use dotenv::dotenv;
use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;

use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod gpio {
    include!("../proto/gpio.rs");
}
use gpio::gpio_server::{Gpio, GpioServer};
use gpio::{
    device_info_response::Payload, CommandType, DeviceInfoResponse, DeviceModelResponse,
    LedRequest, LedResponse,
};

mod utils;
use utils::{info, led};

#[derive(Debug, Default)]
pub struct GpioService {}

#[tonic::async_trait]
impl Gpio for GpioService {
    async fn toggle_led(&self, req: Request<LedRequest>) -> Result<Response<LedResponse>, Status> {
        info!("Got a request: {:?}", req);

        let req_body = req.into_inner();

        match req_body.command_type() {
            CommandType::Switch => match led::switch(req_body.pin as u8).await {
                Ok(msg) => Ok(Response::new(LedResponse {
                    successful: true,
                    message: msg,
                })),
                Err(e) => Err(Status::unavailable(e.to_string())),
            },
            CommandType::Blink => match led::blink(req_body.pin as u8, req_body.duration).await {
                Ok(msg) => Ok(Response::new(LedResponse {
                    successful: true,
                    message: msg,
                })),
                Err(e) => {
                    error!("{}", e);
                    Err(Status::unavailable(e.to_string()))
                }
            },
        }
    }

    async fn get_device_info(
        &self,
        _: Request<()>,
    ) -> Result<Response<DeviceInfoResponse>, Status> {
        info!("Retreiving device information.");

        match info::get_gpio_info().await {
            Ok(stats) => Ok(Response::new(DeviceInfoResponse {
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
            Err(e) => {
                error!("{}", e);
                Err(Status::unavailable(e.to_string()))
            }
        }
    }

    async fn get_device_model(
        &self,
        _: Request<()>,
    ) -> Result<Response<DeviceModelResponse>, Status> {
        info!("Retreiving device model.");

        match info::get_device_model().await {
            Ok(model) => Ok(Response::new(DeviceModelResponse {
                message: model.to_string(),
            })),
            Err(e) => {
                error!("{}", e);
                Err(Status::unavailable(e.to_string()))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))?;

    let _handle = log4rs::init_config(config)?;

    let host = env::var("GRPC_HOST").expect("environment setting 'GRPC_HOST' not set");
    let port = env::var("GRPC_PORT").expect("environment setting 'GRPC_PORT' not set");

    let addr = format!("{}:{}", host, port).parse()?;
    let btc_service = GpioService::default();

    info!("Started listening to {}.", addr);

    Server::builder()
        .add_service(GpioServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use gpio::gpio_client::GpioClient;
    use gpio::LedRequest;

    use dotenv::dotenv;
    use std::env;

    use self::gpio::CommandType;

    pub mod gpio {
        include!("../proto/gpio.rs");
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
    async fn get_device_info() -> Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let host = env::var("GRPC_HOST").expect("environment setting 'GRPC_HOST' not set");
        let port = env::var("GRPC_PORT").expect("environment setting 'GRPC_PORT' not set");

        let mut client = GpioClient::connect(format!("https://{}:{}", host, port)).await?;

        let response = client.get_device_info(()).await?;
        println!("RESPONSE={:?}", response);

        Ok(())
    }
}
