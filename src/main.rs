use dotenv::dotenv;
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod gpio {
    include!("../proto/gpio.rs");
}
use gpio::gpio_server::{Gpio, GpioServer};
use gpio::{
    status_response::Payload, CommandType, LedRequest, LedResponse, StatusRequest, StatusResponse,
};

mod utils;
use utils::{led, status};

#[derive(Debug, Default)]
pub struct GpioService {}

#[tonic::async_trait]
impl Gpio for GpioService {
    async fn toggle_led(&self, req: Request<LedRequest>) -> Result<Response<LedResponse>, Status> {
        println!("Got a request: {:?}", req);

        let req_body = req.into_inner();

        match req_body.command_type() {
            CommandType::Switch => match led::switch(req_body.pin as u8).await {
                Ok(_) => Ok(Response::new(LedResponse {
                    successful: true,
                    message: "succesfully switch LED".to_string(),
                })),
                Err(e) => Err(Status::unavailable(e.to_string())),
            },
            CommandType::Blink => match led::toggle(req_body.pin as u8, req_body.duration).await {
                Ok(_) => Ok(Response::new(LedResponse {
                    successful: true,
                    message: "succesfully toggled led".to_string(),
                })),
                Err(e) => Err(Status::unavailable(e.to_string())),
            },
        }
    }

    async fn print_status(
        &self,
        _: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        match status::print_status().await {
            Ok(stats) => Ok(Response::new(StatusResponse {
                successful: true,
                payloads: stats
                    .into_iter()
                    .map(|x| Payload {
                        gpio: x.gpio,
                        level: x.level,
                        mode: x.mode,
                        pin: x.pin,
                    })
                    .collect(),
            })),
            Err(e) => Err(Status::unknown(e.to_string())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("GRPC_HOST").expect("environment setting 'GRPC_HOST' not set");
    let port = env::var("GRPC_PORT").expect("environment setting 'GRPC_PORT' not set");

    let addr = format!("{}:{}", host, port).parse()?;
    let btc_service = GpioService::default();

    println!("Started listening to {}.....", addr);

    Server::builder()
        .add_service(GpioServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use gpio::gpio_client::GpioClient;
    use gpio::{LedRequest, StatusRequest};

    use dotenv::dotenv;
    use std::env;

    use self::gpio::CommandType;

    pub mod gpio {
        tonic::include_proto!("gpio");
    }

    #[tokio::test]
    async fn toggle_led() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let host = env::var("GRPC_HOST").expect("environment setting 'GRPC_HOST' not set");
        let port = env::var("GRPC_PORT").expect("environment setting 'GRPC_PORT' not set");

        let mut client = GpioClient::connect(format!("https://{}:{}", host, port)).await?;

        for pin in vec![22, 23, 24, 25] {
            let request = tonic::Request::new(LedRequest {
                pin: pin,
                command_type: CommandType::Blink.into(),
                duration: 500,
            });

            let response = client.toggle_led(request).await?;
            println!("RESPONSE={:?}", response);
        }

        Ok(())
    }

    #[tokio::test]
    async fn print_status() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let host = env::var("GRPC_HOST").expect("environment setting 'GRPC_HOST' not set");
        let port = env::var("GRPC_PORT").expect("environment setting 'GRPC_PORT' not set");

        let mut client = GpioClient::connect(format!("https://{}:{}", host, port)).await?;

        let response = client.print_status(StatusRequest::default()).await?;
        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
