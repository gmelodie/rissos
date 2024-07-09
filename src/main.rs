use std::path::Path;

use clap::{Parser, Subcommand};

use rissos::Database;

#[derive(Subcommand)]
enum Command {
    Update,
    UpdateChannel { url: String },
    GetChannel { url: String },
    AddChannel { url: String },
    AddFromFile { path: String },
    RemoveChannel { url: String },
    Save { path: String },
    Load { path: String },
}

#[derive(Parser)]
struct Cli {
    /// Command: update, add, remove, save, load
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();
    let mut db = Database::new();

    match &cli.command {
        Command::Update => {
            let _ = db.update();
        }
        Command::UpdateChannel { url } => {
            let _ = db.update_channel(url);
        }
        Command::GetChannel { url } => {
            let _ = db.get_channel(url).ok_or("Channel not found");
        }
        Command::AddChannel { url } => {
            let _ = db.add_channel(url);
        }
        Command::AddFromFile { path } => {
            let _ = db.add_channel_from_file(Path::new(path));
        }
        Command::RemoveChannel { url } => {
            let _ = db.remove_channel(url);
        }
        Command::Save { path } => {
            let _ = db.save(Path::new(path));
        }
        Command::Load { path } => {
            db = Database::load(Path::new(path)).expect("Could not load db");
        }
    }
}
