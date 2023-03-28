use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct MCStat {
    pub stat: String,
    pub translation: String,
    pub name: Option<String>,
}

pub fn parse_stats(file_content: String) -> Result<Vec<MCStat>, Error> {
    let mut stats: Vec<MCStat> = serde_json::from_str(&file_content)?;

    for stat in &mut stats {
        let parts: Vec<&str> = stat.stat.split(':').collect();
        stat.name = Some(format!(
            "{}-{}",
            &shorten_stat_type(&parts[0]),
            &parts[1][10..]
        ));
    }

    Ok(stats)
}

fn shorten_stat_type(s: &str) -> String {
    match s {
        "minecraft.mined" => "m",
        "minecraft.used" => "u",
        "minecraft.crafted" => "c",
        "minecraft.broken" => "b",
        "minecraft.picked_up" => "p",
        "minecraft.dropped" => "d",
        "minecraft.killed" => "k",
        "minecraft.killed_by" => "kb",
        "minecraft.custom" => "z",
        _ => s,
    }
    .to_string()
}
