use clap::Parser;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Your Zcash Unified Address
    ua: String,

    /// Output PNG file
    #[arg(short, long, default_value = "zcash_knot_qr.png")]
    output: String,

    /// Pure knot mode: no shear, no rotation, raw lattice only
    #[arg(long)]
    raw: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct BaseKnot {
    monomials: Vec<(i32, i32)>,
    denominator: String,
}

// Simple prime checker
fn is_prime(n: u8) -> bool {
    if n <= 1 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5u32;
    while i * i <= n as u32 {
        if n % i as u8 == 0 || n % (i + 2) as u8 == 0 { return false; }
        i += 6;
    }
    true
}

fn main() {
    let args = Args::parse();

    let base_json = fs::read_to_string("base_knot_14a479.json")
        .expect("Run the Sage export command first to create base_knot_14a479.json");
    let base: BaseKnot = serde_json::from_str(&base_json)
        .expect("Invalid base knot JSON");

    let mut ua_padded = args.ua.clone();
    let padding = "====";
    while ua_padded.len() < 217 {
        ua_padded.push_str(padding);
    }
    ua_padded.truncate(217);
    println!("Padded UA length: {}", ua_padded.len());

    // Prime-based mapping (red = prime, blue = non-prime)
    let mut new_poly: HashMap<(i32, i32), i64> = HashMap::new();
    for (i, ch) in ua_padded.chars().enumerate() {
        let mon = base.monomials[i];
        let byte = ch as u8 as i64;
        let coeff = if is_prime(ch as u8) { byte } else { -byte };
        new_poly.insert(mon, coeff);
    }

    render_hex(&new_poly, &args.output, args.raw);

    println!("✅ Done! Your knot QR saved to: {}", args.output);
    if args.raw {
        println!("   (Pure knot mode — no shear / no rotation)");
    }
}

fn render_hex(poly: &HashMap<(i32, i32), i64>, output_path: &str, raw: bool) {
    let root = BitMapBackend::new(output_path, (1400, 1200)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let max_abs = poly.values().map(|&v| v.abs()).max().unwrap_or(1) as f64;

    let mut points: Vec<(f64, f64)> = Vec::new();
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for (&(a, b), _) in poly {
        let x = a as f64;
        let y = if raw {
            b as f64                     // pure knot: no shear
        } else {
            b as f64 + a as f64 * 0.5    // sheared
        };
        points.push((x, y));
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    // For raw mode we skip rotation and centering
    let mut chart = ChartBuilder::on(&root)
        .caption(
            if raw {
                "Your Zcash UA as 14a479 knot QR (pure lattice)"
            } else {
                "Your Zcash UA as 14a479 knot QR (sheared & symmetrized)"
            },
            ("sans-serif", 24)
        )
        .margin(40)
        .build_cartesian_2d(
            (min_x - 1.5)..(max_x + 1.5),
            (min_y - 1.5)..(max_y + 1.5),
        )
        .unwrap();

    chart.configure_mesh()
        .light_line_style(&BLACK.mix(0.05))
        .draw()
        .unwrap();

    for (i, (&(a, b), &coeff)) in poly.iter().enumerate() {
        let (x, y) = points[i];

        let color = if coeff > 0 { RED } else { BLUE };
        let alpha = (coeff.abs() as f64 / max_abs * 0.98 + 0.25).clamp(0.35, 1.0);
        let size = 0.45;

        let style = color.mix(alpha).filled().stroke_width(2);

        let hex_points: Vec<(f64, f64)> = (0..6)
            .map(|j| {
                let ang = std::f64::consts::PI * 2.0 * (j as f64) / 6.0 + std::f64::consts::PI / 6.0;
                let dx = size * ang.cos();
                let dy = size * ang.sin();
                (x + dx, y + dy)
            })
            .collect();

        chart
            .draw_series(std::iter::once(Polygon::new(hex_points, style)))
            .unwrap();
    }

    root.present().unwrap();
    println!("   Plot saved with {} nonzero terms", poly.len());
}