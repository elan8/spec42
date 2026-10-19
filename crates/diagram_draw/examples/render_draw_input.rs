//! Regenerate a native SVG golden from its checked-in draw input.
//! cargo run -p diagram_draw --example render_draw_input -- input.json output.svg
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or("expected draw-input JSON path")?;
    let output = args.next().ok_or("expected output SVG path")?;
    if args.next().is_some() {
        return Err("expected exactly two paths".into());
    }
    let svg = diagram_draw::render_svg_from_str(&fs::read_to_string(input)?, 1280.0, 900.0)?;
    fs::write(output, svg)?;
    Ok(())
}
