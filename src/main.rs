use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::Rgba;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Dealpha {
        path: PathBuf,
        #[clap(long)]
        outline: bool,
    },
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let app = App::parse();

    match app.command {
        Command::Dealpha { path, outline } => dealpha(&path, outline)?,
    }

    Ok(())
}

fn dealpha(path: &Path, outline: bool) -> anyhow::Result<()> {
    let mut image = image::open(path)?.to_rgba8();
    for pixel in image.pixels_mut() {
        pixel[3] = if pixel[3] < 127 { 0 } else { 255 };
    }

    if outline {
        let mut border_pixels = HashSet::new();
        for x in 0..image.width() {
            for y in 0..image.height() {
                if image.get_pixel(x, y)[3] == 0 {
                    for [i, j] in neighbors_of(x, y, image.width(), image.height()) {
                        if image.get_pixel(i, j)[3] == 255 {
                            border_pixels.insert([x, y]);
                            break;
                        }
                    }
                }
            }
        }
        for [x, y] in border_pixels {
            image.put_pixel(x, y, Rgba([0, 0, 0, 255]));
        }
    }

    image.save(path)?;
    Ok(())
}

fn neighbors_of(x: u32, y: u32, width: u32, height: u32) -> impl Iterator<Item = [u32; 2]> {
    [
        (x > 0).then(|| [x - 1, y]),
        (y > 0).then(|| [x, y - 1]),
        (x < width - 1).then(|| [x + 1, y]),
        (y < height - 1).then(|| [x, y + 1]),
    ]
    .into_iter()
    .flatten()
}
