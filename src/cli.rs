use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(name = "mc-scoreboards")]
pub struct Cli {
    /// The version of the game to generate the datapack for. Supported versions: 1.16.5, 1.17.1, 1.18.2, 1.19.2, 1.19.3, 1.19.4
    pub game_version: String,

    /// The path to the stats directory. If this is passed, the program will also create an update.mcfunction file.
    #[arg(short = 's', long = "stats")]
    pub stats_dir: Option<PathBuf>,

    /// The directory to output the datapack to. Defaults to the current directory.
    #[arg(short = 'o', long = "outdir", default_value = ".")]
    pub output_dir: PathBuf,

    /// The path to the whitelist.json file. If you pass the stats directory, this is required.
    #[arg(short = 'w', long = "whitelist")]
    pub whitelist_path: Option<PathBuf>,
}
