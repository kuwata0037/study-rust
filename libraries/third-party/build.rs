fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .include_file("_include.rs")
        .compile(&["./proto/example/v1/hello_world.proto"], &["./proto"])?;

    Ok(())
}
