mod cli;
mod datapack;
mod scoreboards;
mod stats;

use clap::Parser;
use cli::Cli;
use datapack::generate_datapack;

const SUPPORTED_VERSIONS: &[&str] = &["1.16.5", "1.17.1", "1.18.2", "1.19.2", "1.19.3", "1.19.4", "1.20.4"];

fn main() {
    let cli = Cli::parse();

    if !check_version(&cli.game_version) {
        panic!("Unsupported version: {}", &cli.game_version);
    }

    let scoreboards = scoreboards::get_scoreboards_from_version(&cli.game_version)
        .expect("Could not get scoreboards");

    if let Err(e) = generate_datapack(scoreboards, &cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!(
        "Successfully generated datapack for Minecraft {}",
        &cli.game_version
    );
}

fn check_version(version: &str) -> bool {
    SUPPORTED_VERSIONS.contains(&version)
}
