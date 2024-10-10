fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/authenticator_export.proto");
    prost_build::compile_protos(&["src/authenticator_export.proto"], &["src/"])?;

    Ok(())
}
