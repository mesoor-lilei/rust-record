use std::io;

fn main() -> io::Result<()> {
    tonic_build::compile_protos("proto/echo.proto")?;
    Ok(())
}
