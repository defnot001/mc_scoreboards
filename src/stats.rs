use core::convert::TryFrom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
struct PlayerProfile {
    name: String,
    uuid: Uuid,
}

#[derive(Debug)]
struct PlayerProfiles(HashMap<Uuid, String>);

impl TryFrom<&Path> for PlayerProfiles {
    type Error = Box<dyn Error>;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parsed: Vec<PlayerProfile> = serde_json::from_reader(reader)?;

        let mut map: HashMap<Uuid, String> = HashMap::new();

        for profile in parsed {
            map.insert(profile.uuid, profile.name);
        }

        if map.is_empty() {
            return Err(Box::new(IOError::new(
                ErrorKind::NotFound,
                "No player profiles found in the file.",
            )));
        }

        Ok(PlayerProfiles(map))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MCStats {
    #[serde(rename = "minecraft:mined")]
    #[serde(default)]
    pub mined: HashMap<String, u32>,

    #[serde(rename = "minecraft:crafted")]
    #[serde(default)]
    pub crafted: HashMap<String, u32>,

    #[serde(rename = "minecraft:used")]
    #[serde(default)]
    pub used: HashMap<String, u32>,

    #[serde(rename = "minecraft:broken")]
    #[serde(default)]
    pub broken: HashMap<String, u32>,

    #[serde(rename = "minecraft:dropped")]
    #[serde(default)]
    pub dropped: HashMap<String, u32>,

    #[serde(rename = "minecraft:picked_up")]
    #[serde(default)]
    pub picked_up: HashMap<String, u32>,

    #[serde(rename = "minecraft:killed")]
    #[serde(default)]
    pub killed: HashMap<String, u32>,

    #[serde(rename = "minecraft:killed_by")]
    #[serde(default)]
    pub killed_by: HashMap<String, u32>,

    #[serde(rename = "minecraft:custom")]
    #[serde(default)]
    pub custom: HashMap<String, u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct StatFile {
    stats: MCStats,
}

impl TryFrom<&Path> for StatFile {
    type Error = Box<dyn Error>;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let parsed: StatFile = serde_json::from_reader(reader)?;

        Ok(parsed)
    }
}

#[derive(Debug)]
pub struct PlayerStats {
    pub stats: MCStats,
    pub player_name: String,
}

#[derive(Debug)]
struct PlayerFile {
    path: PathBuf,
    player_name: String,
    player_uuid: Uuid,
}

pub fn get_stats(
    whitelist_path: &Path,
    stats_path: &Path,
) -> Result<Vec<PlayerStats>, Box<dyn Error>> {
    let player_profiles = PlayerProfiles::try_from(whitelist_path)?;
    let player_files = get_player_files(stats_path, player_profiles)?;

    let stats: Vec<PlayerStats> = player_files
        .into_iter()
        .filter_map(|player_file| {
            let stats = StatFile::try_from(player_file.path.as_ref());

            stats
                .map(|stats| PlayerStats {
                    stats: stats.stats,
                    player_name: player_file.player_name.to_string(),
                })
                .map_err(|e| {
                    println!(
                        "Error parsing stats file for {} ({}): {}",
                        player_file.player_name, player_file.player_uuid, e
                    );

                    if let Ok(failed_file_str) = fs::read_to_string(player_file.path) {
                        if failed_file_str[2..=5] == "stat".to_string() {
                            println!(
                                "{} still has the old stat format. Skipping...",
                                player_file.player_name
                            );
                            return;
                        }
                        return;
                    };
                })
                .ok()
        })
        .collect();

    Ok(stats)
}

fn get_player_files(
    stats_dir: &Path,
    player_profiles: PlayerProfiles,
) -> io::Result<Vec<PlayerFile>> {
    let stats_dir = fs::read_dir(stats_dir)?;

    let mut player_files: Vec<PlayerFile> = Vec::new();

    for file in stats_dir {
        match file {
            Ok(e) => {
                let filepath = e.path();

                if !filepath.is_file() || filepath.extension() != Some(OsStr::new("json")) {
                    println!("Skipping {}", filepath.to_string_lossy());
                    continue;
                }

                let Some(file_stem) = filepath.file_stem() else {
                    println!("Failed to get filestem from {}", filepath.to_string_lossy());
                    continue;
                };

                let Some(stem_str) = file_stem.to_str() else {
                    println!("Failed to convert filestem {} into string", file_stem.to_string_lossy());
                    continue;
                };

                let Ok(uuid) = Uuid::from_str(stem_str) else {
                    println!("Failed to get uuid from {}", stem_str);
                    continue;
                };

                let Some(profile) = player_profiles.0.get_key_value(&uuid) else {
                    continue;
                };

                player_files.push(PlayerFile {
                    path: filepath,
                    player_name: profile.1.clone(),
                    player_uuid: *profile.0,
                });
            }
            Err(e) => {
                println!("Failed to get file from stats directory: {}", e);
                continue;
            }
        }
    }

    if player_files.is_empty() {
        Err(IOError::new(
            ErrorKind::NotFound,
            "No JSON files found in the directory.",
        ))
    } else {
        Ok(player_files)
    }
}
