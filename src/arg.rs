use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum ResizeType {
	Relative,
	Stretch,
	Crop,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Filter {
	Nearest,
	Triangle,
	CatmullRom,
	Gaussian,
	Lanczos3,
}

impl Filter {
	pub const fn into(self) -> image::imageops::FilterType {
		match self {
			Self::Nearest => image::imageops::FilterType::Nearest,
			Self::Triangle => image::imageops::FilterType::Triangle,
			Self::CatmullRom => image::imageops::FilterType::CatmullRom,
			Self::Gaussian => image::imageops::FilterType::Gaussian,
			Self::Lanczos3 => image::imageops::FilterType::Lanczos3,
		}
	}
}

fn parse_unsharpen(s: &str) -> clap::error::Result<(f32, i32)> {
	let mut split = s.split(",");
	if let Some(sigma) = split.next()
		&& let Some(threshold) = split.next()
	{
		Ok((
			sigma
				.parse()
				.map_err(|_| clap::error::Error::new(clap::error::ErrorKind::InvalidValue))?,
			threshold
				.parse()
				.map_err(|_| clap::error::Error::new(clap::error::ErrorKind::InvalidValue))?,
		))
	} else {
		Err(clap::error::Error::new(clap::error::ErrorKind::InvalidValue))
	}
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Color {
	RGB8,
	RGB16,
	RGB32,
	RGBA8,
	RGBA16,
	RGBA32,
	Luma8,
	Luma16,
	LumaA8,
	LumaA16,
}

/// Tiny image processing tool.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	/// Path to the input image file or a directory containing multiple images.
	#[arg(short, long)]
	pub input: std::path::PathBuf,

	/// Output directory path (default: current directory). Can be either a directory or a file.
	#[arg(short, long, default_value = "./")]
	pub output: std::path::PathBuf,

	/// Output image format (e.g., "png", "jpg"). If not set, inferred from input.
	#[arg(long)]
	pub output_format: Option<String>,

	// ---------------- Resize ----------------
	/// Resampling filter to use when resizing.
	#[arg(short, long, value_enum, default_value_t = Filter::Nearest)]
	pub filter: Filter,

	/// Target width:
	/// - Values 2–10 → original_width / value
	/// - Values > 11 → absolute pixel width (e.g., 1920)
	/// - None → keep original width
	#[arg(long)]
	pub width: Option<u32>,

	/// Target height:
	/// - Values 2–10 → original_height / value
	/// - Values > 11 → absolute pixel height (e.g., 1080)
	/// - None → keep original height
	#[arg(long)]
	pub height: Option<u32>,

	/// Resize mode (relative scaling vs. absolute dimensions).
	#[arg(long, value_enum, default_value_t = ResizeType::Relative)]
	pub resize_type: ResizeType,

	// ---------------- Adjustments ----------------
	/// Apply Gaussian blur. Value is the blur sigma.
	#[arg(long)]
	pub blur: Option<f32>,

	/// Adjust image contrast. Positive values increase contrast, negative decrease.
	#[arg(long)]
	pub contrast: Option<f32>,

	/// Brighten/darken image. Positive values brighten, negative values darken.
	#[arg(long)]
	pub brighten: Option<i32>,

	/// Apply unsharpen mask: `"sigma,threshold"`.
	/// Example: `"0.5,8"` → sigma=0.5, threshold=8.
	#[arg(long, value_parser = parse_unsharpen)]
	pub unsharpen: Option<(f32, i32)>,

	/// Flip image horizontally.
	#[arg(long)]
	pub fliph: bool,

	/// Flip image vertically.
	#[arg(long)]
	pub flipv: bool,

	/// Rotate image 90 degrees clockwise.
	#[arg(long)]
	pub rotate90: bool,

	/// Rotate image 180 degrees.
	#[arg(long)]
	pub rotate180: bool,

	/// Rotate image 270 degrees clockwise.
	#[arg(long)]
	pub rotate270: bool,

	/// Hue rotation in degrees (0–360). 0 and 360 = no change.
	#[arg(long)]
	pub hue: Option<i32>,

	/// Convert image to a different color format.
	#[arg(long, value_enum)]
	pub color: Option<Color>,

	/// Internal buffer size (bytes) used for file I/O.
	#[arg(long, default_value_t = 1024)]
	pub buffer_size: usize,

	/// Number of worker threads (0 = auto).
	#[arg(long, default_value_t = 0)]
	pub threads: usize,
}

impl Args {
	pub fn new() -> Self {
		Args::parse()
	}
}
