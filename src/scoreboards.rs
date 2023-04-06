use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, Read},
    path::Path,
};

/// A scoreboard from the `stats.json` file in the `assets` directory.
#[derive(Deserialize, Serialize, Debug)]
pub struct MCScoreboard {
    pub stat: String,
    pub translation: String,
    pub name: Option<String>,
}

/// Get all scoreboards from the `stats.json` file in the `assets` directory depending on the version of the game.
pub fn get_scoreboards_from_version(version: &str) -> Result<Vec<MCScoreboard>, Error> {
    let assets_path = Path::new(&format!("assets/stats_{}.json", version)).to_owned();

    let mut file = File::open(&assets_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    parse_scoreboards(contents)
}

fn parse_scoreboards(file_content: String) -> Result<Vec<MCScoreboard>, Error> {
    let mut stats: Vec<MCScoreboard> = serde_json::from_str(&file_content)?;

    for stat in &mut stats {
        let parts: Vec<&str> = stat.stat.split(':').collect();
        stat.name = Some(format!(
            "{}-{}",
            &shorten_scoreboard_type(parts[0]),
            &parts[1][10..]
        ));
    }

    Ok(stats)
}

fn shorten_scoreboard_type(s: &str) -> String {
    let mut short_types = HashMap::new();

    short_types.insert("minecraft.mined", "m");
    short_types.insert("minecraft.used", "u");
    short_types.insert("minecraft.crafted", "c");
    short_types.insert("minecraft.broken", "b");
    short_types.insert("minecraft.picked_up", "p");
    short_types.insert("minecraft.dropped", "d");
    short_types.insert("minecraft.killed", "k");
    short_types.insert("minecraft.killed_by", "kb");
    short_types.insert("minecraft.custom", "z");

    short_types.get(s).unwrap_or(&s).to_string()
}
