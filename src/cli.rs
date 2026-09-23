use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ray_tracer")]
#[command(about = "A high-performance path tracer in Rust")]
pub struct Cli {
    /// Path to a JSON scene file.
    #[arg(short, long, default_value = "scenes/output.json")]
    pub scene: String,

    /// Output image path.
    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    /// Image width in pixels.
    #[arg(long)]
    pub width: Option<u32>,

    /// Image height in pixels.
    #[arg(long)]
    pub height: Option<u32>,

    /// Samples per pixel.
    #[arg(long)]
    pub samples: Option<u32>,

    /// Random seed for deterministic rendering.
    #[arg(long, default_value_t = 42)]
    pub seed: u64,

    /// Maximum ray depth.
    #[arg(long)]
    pub depth: Option<u32>,
}
