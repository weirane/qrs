use qrcode::render::unicode::Dense1x2;
use qrcode::QrCode;
use std::io::{prelude::*, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().take(3).collect();
    let amount = if args.len() > 2 && args[1] == "-c" {
        args[2].parse()?
    } else {
        1024
    };

    let buf: Vec<_> = stdin().bytes().take(amount).collect::<Result<_, _>>()?;
    if buf.is_empty() {
        return Err("No input".into());
    }

    let code = QrCode::new(&buf)?;
    let image = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Dark)
        .light_color(Dense1x2::Light)
        .build();
    println!("{}", image);
    Ok(())
}
