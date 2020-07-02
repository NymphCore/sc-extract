# sc_extract

`sc_extract` is a very fast tool to extract graphics and decode csv files from Supercell's game files.

This tool is simply intended to get high quality graphics and data from the files. It is in no way an attempt to:

- modify the game in any way
- create a clone or any other game based on Clash Royale
- make profit

## About The Tool

This tool is a Rust implementation of [this existing Python script](https://github.com/123456abcdef/cr-sc-dump) to extract graphics and decode csv files.

The Python implementation takes a long time to extract images from huge files, like `ui_tex.sc`. This Rust tool makes the extraction process very quick. For instance, extracting all sprites from Brawl Stars' `27.269` version takes:

- Rust implemenation - **Less than 20 seconds**
- Python implemenation - **About 4 minutes**

This tool extracts sprites about **10** times faster than the Python implemenation without leading to any loss in image quality.

## Installation

To use this tool, all you'll need is [Rust (2018 version)](https://www.rust-lang.org/tools/install) and the tool itself. Rust is available for Windows, Mac and Linux, among others operating systems. Clicking on the above link above will take you to Rust's installation page.

Once you've installed Rust, you can install the sc_extract tool in one of the following three ways:

### Downloading Pre-built Binary (Recommended)

You can find pre-built binaries for multiple operating systems and architectures [here](https://github.com/AriusX7/sc-extract/releases).

Download the binary which is appropriate for your machine. After downloading, unzip the folder. You should see the following three files inside:

- `sc_extract` (or `sc_extract.exe` on Windows)
- `README.md`
- `LICENSE`

Now, `cd` into this directory and follow steps described [here](#usage).

### Using `cargo install`

sc_extract is available on [crates.io](https://crates.io/crates/sc_extract). You can install and build this tool by simply using `cargo install sc_extract` command from the terminal. The installation process will take a few minutes to build all dependencies, but once that's done, the tool will work very, very fast.

It will add this tool to the shell path automatically, allowing you to use the tool from *any* directory.

### Building From Source

You can download this tool's [source code](https://codeload.github.com/AriusX7/sc-extract/zip/master) and build it yourself by using `cargo build --release` command. You need to `cd` into this tool's directory before executing that command.

Each time you want to use this tool, you will have to `cd` into the tool's directory, and run `cargo run --release` command.

**Note:** In the below example commands, it will be assumed that you have installed the tool using `cargo install`. If you installed from the source, you will have to replace `sc_extract` with `cargo run --release` in all commands. You will also have to `cd` into the tool's directory.

Choosing the option to build the tool also affects updating. See [Updating](#updating) section for more info.

## Usage

**Note:** You may need to replace `sc_extract` by `./sc_extract`, `sc_extract.exe` or `cargo run --release` in the commands below.

You will need the `_tex.sc` or `.csv` files of the Supercell game you wish to extract. You can get the files by downloading the APK of the game, changing the extension to `.zip`, unzipping it and navigating to `/assets/sc` (_tex.sc files), `/assets/csv_logic` (csv files) or `csv_client` (csv files) folder inside the unzipped folder.

Once you have Rust, this tool and _tex.sc or .csv files, you can begin extracting with just a single command:

```sh
sc_extract path_to_file_or_directory
```

`path_to_file_or_directory` must be a valid path pointing to the sc or csv files directory or just an individual file.

If you installed the tool using the source code, you may want to build the tool and all the dependencies prior to extracting the files. You can do so by run the `cargo build --release` command in the tool's directory. Building will take a couple of minutes, but running the tool in future will be very fast.

### Options

The tool has quite a few options to fine tune the extraction process. The following is the command signature of the tool:

```sh
sc_extract file_or_directory_path [-o | --out] [-d | --delete]
```

If you supply the path to a directory, all the `_tex.sc` and `.csv` files inside the directory will be extracted. If you supply the path to a single file, only that file will be extracted.

`--out` or `-o` flag tells the tool where to save the extracted files. It will create a directory called `extracts` in the directory specified by this flag. All the extracted files will be saved inside the `extracts` directory.

Example Usage: `sc_extract path_to_sc_folder --out path_to_out_directory`

`--delete` or `-d` flag tells the tool whether to delete the source `_tex.sc` or `.csv` files after the extraction. If this flag is passed, the source files are deleted after extraction.

Example Usage: `sc_extract path_to_sc_folder --delete`

## Updating

If you used a pre-compiled binary, you'll simply have to download a new binary for the newer version from [the releases page](https://github.com/AriusX7/sc-extract/releases).

If you installed the tool using `cargo install`, you can update the tool by simply reusing the `cargo install sc_extract` command. If it fails to update the tool, you can force it by adding the `--force` flag, like so: `cargo install sc_extract --force`.

If you installed using the source code, you will have to repeat the process outlined in [Building From Source](#building-from-source) section using the new source code.

## License

sc_extract is available under the `MIT` license. See [LICENSE](LICENSE) for more details.

## Credits

This tool wouldn't exist if the following didn't create [the Python script](https://github.com/123456abcdef/cr-sc-dump#credits) to extract the files.

- [athlan20](https://github.com/athlan20)
- [clanner](https://github.com/clanner)
- [Galaxy1036](https://github.com/Galaxy1036)
- [umop-aplsdn](https://github.com/umop-aplsdn)
