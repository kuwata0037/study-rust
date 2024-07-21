use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("descriptor.bin"))
        .include_file("_include.rs")
        .compile(&["./proto/example/v1/hello_world.proto"], &["./proto"])?;

    Ok(())
}
