# Convy

**Convy** is a lightweight image processing CLI built with [image](https://crates.io/crates/image) and [clap](https://crates.io/crates/clap).  
It’s designed for quick batch conversions, resizing, and transformations without the overhead of heavy GUI tools.

---

## Features

- Supports a wide range of formats:  
  **AVIF, BMP, DDS, EXR, FF, GIF, HDR, ICO, JPEG, PNG, PNM, QOI, TGA, TIFF, WebP**
- Batch processing: run on single files or entire directories
- Resizing with multiple filters (nearest, triangle, catmull-rom, gaussian, lanczos3)
- Image transformations: flip, rotate, crop, stretch
- Color adjustments: hue, contrast, brightness, blur, sharpen

---

## Build

```bash
git clone https://github.com/radioactiveAHM/convy
cd convy
RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```

---

## Usage

```txt
Usage: convy [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>
          Path to the input image file or a directory containing multiple images
  -o, --output <OUTPUT>
          Output directory path (default: current directory). Can be either a directory or a file [default: ./]
      --output-format <OUTPUT_FORMAT>
          Output image format (e.g., "png", "jpg"). If not set, inferred from input
  -f, --filter <FILTER>
          Resampling filter to use when resizing [default: lanczos3] [possible values: nearest, triangle, catmull-rom, gaussian, lanczos3]
      --width <WIDTH>
          Target width: - Values 2–10 → original_width / value - Values > 11 → absolute pixel width (e.g., 1920) - None → keep original width
      --height <HEIGHT>
          Target height: - Values 2–10 → original_height / value - Values > 11 → absolute pixel height (e.g., 1080) - None → keep original height
      --resize-type <RESIZE_TYPE>
          Resize mode (relative scaling vs. absolute dimensions) [default: relative] [possible values: relative, stretch, crop]
      --blur <BLUR>
          Apply Gaussian blur. Value is the blur sigma
      --contrast <CONTRAST>
          Adjust image contrast. Positive values increase contrast, negative decrease
      --brighten <BRIGHTEN>
          Brighten/darken image. Positive values brighten, negative values darken
      --unsharpen <UNSHARPEN>
          Apply unsharpen mask: `"sigma,threshold"`. Example: `"0.5,8"` → sigma=0.5, threshold=8
      --fliph
          Flip image horizontally
      --flipv
          Flip image vertically
      --rotate90
          Rotate image 90 degrees clockwise
      --rotate180
          Rotate image 180 degrees
      --rotate270
          Rotate image 270 degrees clockwise
      --hue <HUE>
          Hue rotation in degrees (0–360). 0 and 360 = no change
      --color <COLOR>
          Convert image to a different color format [possible values: rgb8, rgb16, rgb32, rgba8, rgba16, rgba32, luma8, luma16, luma-a8, luma-a16]
  -q, --quality <QUALITY>
          Jpeg/Webp quality [default: 100]
      --method <METHOD>
          Webp method level. possible values: 1-6 [default: 6]
      --hint <HINT>
          Webp hint. DEFAULT = 0, PICTURE = 1, PHOTO = 2, GRAPH = 3, LAST = 4 [default: 0]
      --compression-type <COMPRESSION_TYPE>
          Png compression type. [possible values: 1-9, best, default, fast, uncompressed]
      --png-filtertype <PNG_FILTERTYPE>
          Png filter type. [possible values: adaptive, avg, none, paeth, sub, up]
      --buffer-size <BUFFER_SIZE>
          Internal buffer size (KB) used for file I/O [default: 1024]
      --threads <THREADS>
          Number of worker threads (0 = auto) [default: 0]
  -h, --help
          Print help
  -V, --version
          Print version
```

---

## Examples

Resize an image to 1920×1080 with Lanczos filter:

```bash
convy -i input.jpg -o output.png --width 1920 --height 1080 -f triangle
```

Batch convert all JPEGs in a folder to WebP:

```bash
convy -i ./photos -o ./converted --output-format webp -q 95
```

Apply blur and brighten:

```bash
convy -i portrait.png -o portrait_blur.png --blur 2.0 --brighten 20
```
