const PROTO_FILE: &str = "authenticator.export.proto";

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/{PROTO_FILE}");
    prost_build::compile_protos(&[format!("src/{PROTO_FILE}")], &["src/"])?;

    Ok(())
}
