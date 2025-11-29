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
cargo build --release
```

---

## Usage

```txt
Usage: convy [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>                  Path to the input image file or a directory
  -o, --output <OUTPUT>                Output path (file or directory) [default: ./]
      --output-format <FORMAT>         Output format (png, jpg, etc.). Inferred if not set
  -f, --filter <FILTER>                Resampling filter [default: nearest]
                                       [values: nearest, triangle, catmull-rom, gaussian, lanczos3]
      --width <WIDTH>                  Target width:
                                         • 2–10 → original_width / value
                                         • >11  → absolute pixel width (e.g., 1920)
                                         • none → keep original
      --height <HEIGHT>                Target height (same rules as width)
      --resize-type <TYPE>             Resize mode [default: relative]
                                       [values: relative, stretch, crop]
      --blur <SIGMA>                   Apply Gaussian blur
      --contrast <VALUE>               Adjust contrast (+/-)
      --brighten <VALUE>               Brighten/darken (+/-)
      --unsharpen <SIGMA,THRESHOLD>    Apply unsharpen mask (e.g., "0.5,8")
      --fliph                          Flip horizontally
      --flipv                          Flip vertically
      --rotate90                       Rotate 90° clockwise
      --rotate180                      Rotate 180°
      --rotate270                      Rotate 270° clockwise
      --hue <DEGREES>                  Hue rotation (0–360)
      --color <COLOR>                  Convert to color format
                                       [values: rgb8, rgb16, rgb32, rgba8, rgba16, rgba32, luma8, luma16, luma-a8, luma-a16]
      --buffer-size <BYTES>            File I/O buffer size [default: 1024]
      --threads <N>                    Number of worker threads (0 = auto) [default: 0]
  -h, --help                           Show help
  -V, --version                        Show version
```

---

## Examples

Resize an image to 1920×1080 with Lanczos filter:

```bash
convy -i input.jpg -o output.png --width 1920 --height 1080 -f lanczos3
```

Batch convert all JPEGs in a folder to WebP:

```bash
convy -i ./photos -o ./converted --output-format webp
```

Apply blur and brighten:

```bash
convy -i portrait.png -o portrait_blur.png --blur 2.0 --brighten 20
```
