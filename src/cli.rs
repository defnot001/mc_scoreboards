use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
#[command(name = "mc-scoreboards")]
pub struct Cli {
    /// The version of the game to generate the datapack for. Supported versions: 1.19.3, 1.19.4
    pub game_version: String,

    /// The path to the stats directory. If this is passed, the program will also create an update.mcfunction file.
    #[arg(short = 's', long = "stats")]
    pub stats_dir: Option<PathBuf>,

    /// The directory to output the datapack to. Defaults to the current directory. You can't pass this argument, if you don't pass the stats argument.
    #[arg(short = 'o', long = "outdir", default_value = ".")]
    pub output_dir: PathBuf,

    /// The path to the whitelist.json file. If this is passed, the program will only update the stats for whitelisted players, which can significantly reduce the size of the datapack.
    #[arg(short = 'w', long = "whitelist")]
    pub whitelist_path: Option<PathBuf>,
}
