![Build Status](https://github.com/defnot001/mc_scoreboards/actions/workflows/build.yml/badge.svg)

# MC Scoreboards

A tool to generate scoreboards for all statistics that Minecraft tracks. This CLI program generates a datapack with mcfunctions that can be loaded and executed on a server.

In the [releases tab](https://github.com/defnot001/mc_scoreboards/releases), you can find premade datapacks for all the supported versions.

Currently supported versions:

- 1.16.5
- 1.17.1
- 1.18.2
- 1.19.2
- 1.19.3
- 1.19.4

If you need a datapack an unsupported version, don't hesitate to open an issue or join my [Discord server](https://discord.gg/wmJ3WBYcZF).

## Using the program

If you want to build and execute this program yourself, you can do so by following these steps. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

### Building the program

Clone this repository:

```shell
git clone https://github.com/defnot001/mc_scoreboards.git
```

Go into into the project directory:

```shell
cd mc_scoreboards
```

Build the program:

```shell
cargo build --release --all-features
```

You should now have a binary in `target/release/mc_scoreboards`.

### Creating the datapack

This CLI program accepts multiple arguments and flags that can be used to customize the output. You can see all the options by running mc_scoreboards --help.

The most basic usage on Linux and Mac will look like this:

```shell
./target/release/mc-scoreboards 1.19.3
```

On Windows, you will need to use the `.exe` file:

```shell
./target/release/mc-scoreboards.exe 1.19.3
```

This will create a datapack in the current directory with the name `scoreboards_1.19.3.zip`. You can then load this datapack on your server and execute the functions in it.

```shell
/reload
/function mc-scoreboards:create
```

If you want to remove the scoreboard, you can run the following command:

```shell
/function mc-scoreboards:remove
```

The flags that are additionally available are:

#### `--outdir` (`-o`):

You can use this flag to specify an output directory for the datapack. If this flag is not specified, the datapack will be created in the current directory.

#### `--stats` (`-s`):

Whenever you create a new scoreboard in minecraft, it will initialize all scores to 0.

If you want to update the scores to the current value of the statistic you need to specify this flag and point it to the stats folder of your world.

You can find the `stats` folder in the `world` folder of your server.

If you specify this flag, the program will read the stats files and create a third mcfunction file called `update.mcfunction` that will update the scoreboard to the current value of the statistic.

```shell
./target/release/mc-scoreboards 1.19.3 -s /path/to/stats/folder
```

#### `--whitelist` (`-w`):

If you want to save some time and only update scoreboards for whitelisted players, use this flag and point it at the whitlist.json file of your server.

This flag only works if you also specify the `--stats` flag.

```shell
./target/release/mc-scoreboards 1.19.3 -s /path/to/stats/folder -w /path/to/whitelist.json
```

### Naming Convention

This program follows the naming convention introduced by [every-scroreboard](https://github.com/samipourquoi/every-scoreboard).

The scoreboards are named accordingly:

- `m-<block>` Blocks mined
- `u-<item>` Items used
- `c-<item>` Items crafted
- `b-<item>` Tools broken
- `p-<item>` Items picked up
- `d-<item>` Items dropped
- `k-<mob>` Mobs killed
- `kb-<mob>` Killed by mob
- `z-<stat>` Custom stats (find all possible `stats` [here](https://minecraft.fandom.com/wiki/Statistics#List_of_custom_statistic_names))

#### Acknowledgements

This program is heavily inspired by [every-scroreboard](https://github.com/samipourquoi/every-scoreboard). I would like to thank [samipourquoi](https://github.com/samipourquoi) for creating this project and making it open source aswell as [Syntro42](https://github.com/Syntro42) for maintaining it.

Thank you for reading this far! If you have any questions, feel free to join my [Discord server](https://discord.gg/wmJ3WBYcZF). If you find any bugs, please open an [issue](https://github.com/defnot001/mc_scoreboards/issues) on GitHub. If you liked this project, please consider starring it on GitHub.
