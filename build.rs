fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto = "proto/gpio.proto";
    tonic_build::configure()
        .build_server(true)
        .out_dir("./proto")
        .compile(&[proto], &["."])?;
    println!("cargo:rerun-if-changed={}", proto);
    Ok(())
}
