mod cli;
mod compare;
mod datapack;
mod scoreboards;

use clap::Parser;
use cli::Cli;
use datapack::generate_datapack;

const SUPPORTED_VERSIONS: &[&str] = &["1.16.5", "1.17.1", "1.18.2", "1.19.2", "1.19.3", "1.19.4"];

fn main() {
    let cli = Cli::parse();

    if !check_version(&cli.game_version) {
        panic!("Unsupported version: {}", &cli.game_version);
    }

    let scoreboards = scoreboards::get_scoreboards_from_version(&cli.game_version)
        .expect("Could not get scoreboards");

    generate_datapack(scoreboards, &cli).expect("Could not generate datapack");

    println!(
        "Successfully generated datapack for Minecraft {}",
        &cli.game_version
    );
}

fn check_version(version: &str) -> bool {
    SUPPORTED_VERSIONS.contains(&version)
}
