use std::fs::{self, File};
use std::io::{Error, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct MCStat {
    stat: String,
    translation: String,
    name: Option<String>,
}

fn main() {
    let path = Path::new("assets/stats_1.19.json");
    let file_content = get_file(path).expect(format!("Could not read file: {:?}", path).as_str());
    let parsed = get_stats(&file_content).expect("Could not parse stats");

    write_files(parsed).expect("Could not write files");
}

fn write_files(stats: Vec<MCStat>) -> Result<(), Error> {
    let dir_path = Path::new("results/mc-scoreboards/functions");
    fs::create_dir_all(dir_path)?;

    let mut create_file = File::create(Path::join(&dir_path, "create.mcfunction"))?;
    let mut remove_file = File::create(Path::join(&dir_path, "remove.mcfunction"))?;

    for stat in stats {
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

fn get_file(path: &Path) -> Result<String, Error> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_stats(file_content: &str) -> Result<Vec<MCStat>, Error> {
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
        "minecraft.mined" => "m".to_string(),
        "minecraft.used" => "u".to_string(),
        "minecraft.crafted" => "c".to_string(),
        "minecraft.broken" => "b".to_string(),
        "minecraft.picked_up" => "p".to_string(),
        "minecraft.dropped" => "d".to_string(),
        "minecraft.killed" => "k".to_string(),
        "minecraft.killed_by" => "kb".to_string(),
        "minecraft.custom" => "z".to_string(),
        _ => s.to_string(),
    }
}
