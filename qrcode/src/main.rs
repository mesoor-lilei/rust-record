use qrcode::{render::svg, QrCode};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let code = QrCode::new("https://baidu.com")?;
    let code = code.render::<svg::Color>().build();
    fs::write("qrcode.svg", code)?;
    Ok(())
}
