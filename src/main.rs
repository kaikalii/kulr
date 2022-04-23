use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Dealpha { path: PathBuf },
}

fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let app = App::parse();

    match app.command {
        Command::Dealpha { path } => dealpha(&path)?,
    }

    Ok(())
}

fn dealpha(path: &Path) -> anyhow::Result<()> {
    let mut image = image::open(path)?.to_rgba8();
    for pixel in image.pixels_mut() {
        if pixel.0[3] > 0 {
            println!("{}", pixel.0[3]);
            pixel.0[3] = 255;
        }
    }
    image.save(path)?;
    Ok(())
}
