fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir_proto = "./.protos";
    let proto = &format!("{}/gpio.proto", dir_proto);
    tonic_build::configure()
        .build_server(true)
        .out_dir(dir_proto)
        .compile(&[proto], &["."])?;
    println!("cargo:rerun-if-changed={}", proto);
    Ok(())
}
