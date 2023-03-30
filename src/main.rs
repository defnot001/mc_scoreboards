mod cli;
mod compare;
mod datapack;
mod scoreboards;

use clap::Parser;
use cli::Cli;
use datapack::generate_datapack;

fn main() {
    let cli = Cli::parse();

    if !check_version(&cli.game_version) {
        panic!("Unsupported version: {}", &cli.game_version);
    }

    println!("Generating datapack for Minecraft {}...", &cli.game_version);

    let scoreboards = scoreboards::get_scoreboards_from_version(&cli.game_version)
        .expect("Could not get scoreboards");

    println!("Generating {} scoreboard functions", scoreboards.len() * 2);

    generate_datapack(scoreboards, &cli).expect("Could not generate datapack");
}

fn check_version(version: &str) -> bool {
    let supported_versions = vec!["1.16.5", "1.17.1", "1.18.2", "1.19.2", "1.19.3", "1.19.4"];

    supported_versions.contains(&version)
}
