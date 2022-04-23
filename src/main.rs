use std::path::PathBuf;

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
        Command::Dealpha { path } => image::open(&path)?.to_rgb8().save(path)?,
    }

    Ok(())
}
