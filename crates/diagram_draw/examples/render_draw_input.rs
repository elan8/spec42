//! Regenerate a native SVG golden from its checked-in draw input.
//! cargo run -p diagram_draw --example render_draw_input -- input.json output.svg
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or("expected draw-input JSON path")?;
    let output = args.next().ok_or("expected output SVG path")?;
    let markers_path = args.next();
    if args.next().is_some() {
        return Err("expected input, SVG and optional marker-summary paths".into());
    }
    let svg = diagram_draw::render_svg_from_str(&fs::read_to_string(input)?, 1280.0, 900.0)?;
    if let Some(path) = markers_path {
        let summary = diagram_draw::svg_markers::summarize_svg_markers(&svg);
        let json = serde_json::json!({"classCounts": summary.class_counts, "markerIds":summary.marker_ids});
        fs::write(path, format!("{}\n", serde_json::to_string_pretty(&json)?))?;
    }
    fs::write(output, svg)?;
    Ok(())
}
