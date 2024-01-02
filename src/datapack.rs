use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Error as IOError, ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::cli::Cli;
use crate::scoreboards::MCScoreboard;
use crate::stats::{get_stats, PlayerStats};

#[derive(Debug)]
struct DatapackPaths {
    base_path: PathBuf,
    function_path: PathBuf,
}

struct MCStat {
    player_name: String,
    stat_type: String,
    item: String,
    score: u32,
}

impl DatapackPaths {
    pub fn new(custom_path: &Path, version: &str) -> Self {
        let base_path = Path::join(custom_path, format!("datapacks/mc-scoreboards-{}", version));
        let function_path = Path::join(&base_path, "data/mc-scoreboards/functions");

        DatapackPaths {
            base_path,
            function_path,
        }
    }
}

pub fn generate_datapack(
    scoreboards: Vec<MCScoreboard>,
    options: &Cli,
) -> Result<(), Box<dyn Error>> {
    let paths = DatapackPaths::new(&options.output_dir, &options.game_version);

    create_datapack_structure(&paths.function_path)?;
    create_mcmeta_file(&paths.base_path, &options.game_version)?;
    create_base_functions(scoreboards, &paths.function_path)?;

    if let Some(stats_path) = &options.stats_dir {
        let Some(whitelist_path) = &options.whitelist_path else {
            return Err(Box::new(IOError::new(
                ErrorKind::NotFound,
                "Missing whitelist file argument. Please provide a whitelist file with the --whitelist (-w) argument.",
            )));
        };

        let stats = get_stats(whitelist_path, stats_path)?;

        let mut update_file = File::create(Path::join(&paths.function_path, "update.mcfunction"))?;
        let mut string_buf = String::new();

        for player_stats in stats {
            let mcstats = convert_stats(player_stats);

            for stat in mcstats {
                let formatted = format!(
                    "scoreboard players set {} {}{} {}\n",
                    stat.player_name, stat.stat_type, stat.item, stat.score
                );

                string_buf.push_str(&formatted);
            }
        }

        update_file.write_all(string_buf.as_bytes())?;
    }

    Ok(())
}

fn convert_stats(ps: PlayerStats) -> Vec<MCStat> {
    let mut all_stats: Vec<MCStat> = Vec::new();

    let new_stats = [
        map_stat(&ps.player_name, ps.stats.mined, "m"),
        map_stat(&ps.player_name, ps.stats.crafted, "c"),
        map_stat(&ps.player_name, ps.stats.used, "u"),
        map_stat(&ps.player_name, ps.stats.broken, "b"),
        map_stat(&ps.player_name, ps.stats.dropped, "d"),
        map_stat(&ps.player_name, ps.stats.picked_up, "p"),
        map_stat(&ps.player_name, ps.stats.killed, "k"),
        map_stat(&ps.player_name, ps.stats.killed_by, "kb"),
        map_stat(&ps.player_name, ps.stats.custom, "z"),
    ];

    for mut map in new_stats {
        all_stats.append(&mut map);
    }

    all_stats
}

fn map_stat(player_name: &str, stats: HashMap<String, u32>, short_type: &str) -> Vec<MCStat> {
    let stats: Vec<MCStat> = stats
        .iter()
        .map(|(k, v)| MCStat {
            player_name: player_name.to_string(),
            stat_type: format!("{}-", short_type),
            item: k[10..].to_string(),
            score: *v,
        })
        .collect();

    stats
}

fn create_datapack_structure(functions_path: &Path) -> io::Result<()> {
    if functions_path.exists() {
        fs::remove_dir_all(functions_path)?;
    }

    fs::create_dir_all(functions_path)?;

    Ok(())
}

fn create_mcmeta_file(base_path: &Path, version: &str) -> io::Result<()> {
    let mut file = File::create(base_path.join("pack.mcmeta"))?;

    let content = format!(
        r#"{{
            "pack":{{
                "pack_format": {},
                "description": "All scoreboards for Minecraft {}"
            }}
        }}"#,
        get_datapack_version(version),
        version
    );

    file.write_all(content.as_bytes())?;

    Ok(())
}

fn create_base_functions(scoreboards: Vec<MCScoreboard>, functions_path: &Path) -> io::Result<()> {
    let mut create_file = File::create(Path::join(functions_path, "create.mcfunction"))?;
    let mut remove_file = File::create(Path::join(functions_path, "remove.mcfunction"))?;

    for stat in scoreboards {
        if let Some(name) = stat.name {
            let create = format!(
                "scoreboard objectives add {} {} \"{}\"\n",
                name, stat.stat, stat.translation
            );
            let remove = format!("scoreboard objectives remove {}\n", name);

            create_file.write_all(create.as_bytes())?;
            remove_file.write_all(remove.as_bytes())?;
        } else {
            println!("Could not parse stat: {:?}", stat);
        }
    }

    Ok(())
}

fn get_datapack_version(version: &str) -> u8 {
    match version {
        "1.16.5" => 6,
        "1.17.1" => 7,
        "1.18.2" => 9,
        "1.19.2" | "1.19.3" => 10,
        "1.19.4" => 12,
        "1.20.4" => 26,
        _ => panic!("Unsupported version: {}", version),
    }
}
