use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::Path;

use crate::stats::MCStat;

pub fn generate_datapack(stats: Vec<MCStat>, version: &str) {
    create_datapack_structure(version).expect("Could not create datapack structure");
    create_mcmeta_file(version).expect("Could not create mcmeta file");
    write_create_remove_functions(stats, version).expect("Could not write function files");
}

fn write_create_remove_functions(stats: Vec<MCStat>, version: &str) -> Result<(), Error> {
    let dir_path = Path::new(&format!(
        "datapacks/mc-scoreboards-{}/data/mc-scoreboards/functions",
        version
    ))
    .to_owned();
    fs::create_dir_all(&dir_path)?;

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

fn create_datapack_structure(version: &str) -> Result<(), Error> {
    fs::create_dir_all(format!(
        "datapacks/mc-scoreboards-{version}/data/mc-scoreboards/functions"
    ))?;

    Ok(())
}

fn create_mcmeta_file(version: &str) -> Result<(), Error> {
    let mut file = File::create(format!("datapacks/mc-scoreboards-{version}/pack.mcmeta"))?;

    let content = format!(
        r#"{{"pack":{{"pack_format":{},"description":"All scoreboards for Minecraft {}"}}}}"#,
        get_datapack_version(version),
        version
    );

    file.write_all(&content.as_bytes())?;

    Ok(())
}

fn get_datapack_version(version: &str) -> u8 {
    match version {
        "1.16.5" => 6,
        "1.17.1" => 7,
        "1.18.2" => 9,
        "1.19.1" | "1.19.2" | "1.19.3" => 10,
        "1.19.4" => 12,
        _ => panic!("Unsupported version: {}", version),
    }
}
