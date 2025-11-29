use crate::arg;

pub fn bulk_process(dir_entries: Vec<std::path::PathBuf>, args: &arg::Args) {
	for path in dir_entries {
		if path.is_file()
			&& let Err(e) = process(args, &path)
		{
			println!("{e}");
		}
	}
}

pub fn process(args: &arg::Args, input: &std::path::PathBuf) -> std::io::Result<()> {
	let img_format = parse_type(input)?;
	let out_format = if let Some(format_string) = &args.output_format {
		image::ImageFormat::from_extension(format_string).ok_or(std::io::Error::other("invalid output format"))?
	} else {
		img_format
	};

	let mut input_br = std::io::BufReader::with_capacity(args.buffer_size * 1024, std::fs::File::open(input)?);
	let mut output_bw = std::io::BufWriter::with_capacity(
		args.buffer_size * 1024,
		output_file(&args.output, input.file_name(), img_format, out_format)?,
	);

	let img = filtering_options(
		args,
		image::load(&mut input_br, img_format).map_err(std::io::Error::other)?,
	);
	img.write_to(&mut output_bw, out_format)
		.map_err(std::io::Error::other)?;
	Ok(())
}

fn parse_type(input: &std::path::Path) -> std::io::Result<image::ImageFormat> {
	if let Some(ext) = input.extension()
		&& let Some(ext) = image::ImageFormat::from_extension(ext)
	{
		Ok(ext)
	} else {
		Err(std::io::Error::other("parsing extension failed"))
	}
}

fn output_file(
	path: &std::path::PathBuf,
	fname: Option<&std::ffi::OsStr>,
	img_format: image::ImageFormat,
	out_format: image::ImageFormat,
) -> std::io::Result<std::fs::File> {
	if path.is_dir() {
		let fname = fname.unwrap_or(std::ffi::OsStr::new("newfile"));
		let mut dist = path.join(fname);
		if img_format != out_format {
			dist.set_extension(out_format.extensions_str()[0]);
		}
		if dist.is_file() {
			dist.set_file_name(format!("new_{}", fname.to_string_lossy()));
		}

		std::fs::OpenOptions::new()
			.create(true)
			.truncate(true)
			.write(true)
			.open(dist)
	} else {
		std::fs::OpenOptions::new()
			.create(true)
			.truncate(true)
			.write(true)
			.open(path)
	}
}

fn filtering_options(args: &arg::Args, mut img: image::DynamicImage) -> image::DynamicImage {
	if args.height.is_some() || args.width.is_some() {
		let mut width = args.width.unwrap_or(img.width());
		let mut height = args.height.unwrap_or(img.height());
		if 1 < width && width < 11 {
			width = img.width() / width
		}
		if 1 < height && height < 11 {
			height = img.height() / height
		}
		img = match args.resize_type {
			arg::ResizeType::Relative => img.resize(width, height, args.filter.into()),
			arg::ResizeType::Crop => img.resize_to_fill(width, height, args.filter.into()),
			arg::ResizeType::Stretch => img.resize_exact(width, height, args.filter.into()),
		}
	};

	if let Some(sigma) = args.blur {
		img = img.blur(sigma);
	};
	if let Some(contrast) = args.contrast {
		img = img.adjust_contrast(contrast);
	};
	if let Some(brighten) = args.brighten {
		img = img.brighten(brighten);
	};
	if let Some((sigma, threshold)) = args.unsharpen {
		img = img.unsharpen(sigma, threshold);
	};
	if args.fliph {
		img = img.fliph();
	};
	if args.flipv {
		img = img.flipv();
	};
	if args.rotate90 {
		img = img.rotate90();
	};
	if args.rotate180 {
		img = img.rotate180();
	};
	if args.rotate270 {
		img = img.rotate270();
	};

	if let Some(hue) = args.hue {
		img = img.huerotate(hue);
	};
	if let Some(color) = args.color {
		img = convet_color_format(img, color);
	};

	img
}

fn convet_color_format(img: image::DynamicImage, format: arg::Color) -> image::DynamicImage {
	match format {
		arg::Color::RGB8 => img.into_rgb8().into(),
		arg::Color::RGB16 => img.into_rgb16().into(),
		arg::Color::RGB32 => img.into_rgb32f().into(),
		arg::Color::RGBA8 => img.into_rgba8().into(),
		arg::Color::RGBA16 => img.into_rgba16().into(),
		arg::Color::RGBA32 => img.into_rgba32f().into(),
		arg::Color::Luma8 => img.into_luma8().into(),
		arg::Color::Luma16 => img.into_luma16().into(),
		arg::Color::LumaA8 => img.into_luma_alpha8().into(),
		arg::Color::LumaA16 => img.into_luma_alpha16().into(),
	}
}
