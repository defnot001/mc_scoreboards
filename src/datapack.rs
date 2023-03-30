use std::fs::{self, File};
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

use crate::cli::Cli;
use crate::scoreboards::MCScoreboard;

pub fn generate_datapack(scoreboards: Vec<MCScoreboard>, options: &Cli) -> Result<(), Error> {
    let datapack_path = create_datapack_directory(&options)?;
    let datapack_function_path = create_function_directory(&datapack_path)?;

    create_mcmeta_file(&datapack_path, &options.game_version)?;
    write_create_remove_functions(scoreboards, &datapack_function_path)?;

    Ok(())
}

fn create_datapack_directory(options: &Cli) -> Result<PathBuf, Error> {
    let datapack_path = Path::join(
        &options.output_dir,
        format!("datapacks/mc-scoreboards-{}", options.game_version),
    );
    fs::create_dir_all(&datapack_path)?;

    Ok(datapack_path)
}

fn create_function_directory(datapack_path: &Path) -> Result<PathBuf, Error> {
    let datapack_function_path = Path::join(datapack_path, "data/mc-scoreboards/functions");
    if datapack_function_path.exists() {
        fs::remove_dir_all(&datapack_function_path)?;
    }
    fs::create_dir_all(&datapack_function_path)?;

    Ok(datapack_function_path)
}

fn create_mcmeta_file(datapack_path: &Path, version: &str) -> Result<(), Error> {
    let mut file = File::create(datapack_path.join("pack.mcmeta"))?;

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

    file.write_all(&content.as_bytes())?;

    Ok(())
}

fn write_create_remove_functions(
    scoreboards: Vec<MCScoreboard>,
    functions_path: &PathBuf,
) -> Result<(), Error> {
    let mut create_file = File::create(Path::join(&functions_path, "create.mcfunction"))?;
    let mut remove_file = File::create(Path::join(&functions_path, "remove.mcfunction"))?;

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
        _ => panic!("Unsupported version: {}", version),
    }
}
