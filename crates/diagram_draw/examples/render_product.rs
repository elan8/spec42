//! Render an exported diagram product without rebuilding its semantic publication.
//! cargo run -p diagram_draw --example render_product -- diagram.json diagram.svg
use std::{env, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let input = PathBuf::from(args.next().ok_or("expected diagram.json")?);
    let output = PathBuf::from(args.next().ok_or("expected output.svg")?);
    if args.next().is_some() {
        return Err("expected exactly two paths".into());
    }
    let payload = fs::read_to_string(input)?;
    let svg = diagram_draw::pipeline::render_svg_from_payload_str_nested(
        &payload,
        diagram_draw::nesting::Nesting::Levels(1),
    )?;
    fs::write(output, svg)?;
    Ok(())
}
