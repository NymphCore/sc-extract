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

To use this tool, all you'll need is [Rust (2018 version)](https://www.rust-lang.org/tools/install) and [the tool's source code](https://codeload.github.com/AriusX7/sc-extract/zip/master). Rust is available for Windows, Mac and Linux, among others operating systems. Clicking on the first link above will take you to Rust's installation page. The second link will download the source code of this tool.

## Usage

You will need the `_tex.sc` or `.csv` files of the Supercell game you wish to extract. You can get the files by downloading the APK of the game, changing the extension to `.zip`, unzipping it and navigating to `/assets/sc` (_tex.sc files), `/assets/csv_logic` (csv files) or `csv_client` (csv files) folder inside the unzipped folder.

Once you have Rust, this tool and _tex.sc or .csv files, you can begin extracting with just these two commands:

Changing `current_directory` to this tool's directory:

`cd path_to_this_tool`

Running the tool:

`cargo run --release path_to_file_or_directory`

Running the above command for the first time will take quite long. Rust needs to build up all the dependencies and it can take a couple of minutes. Subsequent uses of the tool will be very, very quick.

If you'd like to build the tool prior to extracting the files, you can run `cargo build --release` in the tool's directory. Once again, building will take a couple of minutes, but running the tool in future will be very fast.

### Options

The tool has quite a few options to fine tune the extraction process. The following is the command signature of the tool:

```cargo run --release file_or_directory_path [-o | --out] [-d | --delete]```

If you supply the path to a directory, all the `_tex.sc` and `.csv` files inside the directory will be extracted. If you supply the path to a single file, only that file will be extracted.

`--out` or `-o` flag tells the tool where to save the extracted files. It will create a directory called `extracts` in the directory specified by this flag. All the extracted files will be saved inside the `extracts` directory.

Example Usage: `cargo run --release path_to_sc_folder --out path_to_out_directory`

`--delete` or `-d` flag tells the tool whether to delete the source `_tex.sc` or `.csv` files after the extraction. If this flag is passed, the source files are deleted after extraction.

Example Usage: `cargo run --release path_to_sc_folder --delete`

## License

sc_extract is available under the `MIT` license. See [LICENSE](LICENSE) for more details.

## Credits

This tool wouldn't exist if the following didn't create [the Python script](https://github.com/123456abcdef/cr-sc-dump#credits) to extract the files.

- [athlan20](https://github.com/athlan20)
- [clanner](https://github.com/clanner)
- [Galaxy1036](https://github.com/Galaxy1036)
- [umop-aplsdn](https://github.com/umop-aplsdn)
