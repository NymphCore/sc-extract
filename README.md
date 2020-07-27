# sc_extract

`sc_extract` is a very fast tool to extract graphics and decode csv files from Supercell's game files.

This tool is simply intended to get high quality graphics and data from the files. It is in no way an attempt to:

- modify the game in any way
- create a clone or any other game based on Supercell game
- make profit

## About The Tool

This tool is a Rust implementation of [this existing Python script](https://github.com/123456abcdef/cr-sc-dump) to extract graphics and decode csv files.

The Python implementation takes a long time to extract images from huge files, like `ui_tex.sc`. This Rust tool makes the extraction process very quick. For instance, this tool takes **less than 20 seconds** to extract all sprites from Brawl Stars' `27.269` version whereas the Python script takes **over 4 minutes**.

Thus, this tool extracts sprites about **10** times faster than the Python implemenation without leading to any loss in image quality. The time taken by this tool can by further reduced by using parallize flag.

## Installation

`sc_extract` can be installed in three ways. [The first method](#downloading-precompiled-binary-recommended) is the fastest and does not require you to install Rust. [Second](#using-cargo-install) and [third](#building-from-source) methods require you to install [Rust's 2018 version](https://www.rust-lang.org/tools/install). Rust is available for a large number of operating systems. Clicking on the above link above will take you to Rust's installation page.

### Downloading Precompiled Binary (Recommended)

You can find precompiled binaries for multiple operating systems and architectures [here](https://github.com/AriusX7/sc-extract/releases).

Download the binary which is appropriate for your machine. After downloading, unzip the folder. You should see the following three files inside:

- `sc_extract` (or `sc_extract.exe` on Windows)
- `README.md`
- `LICENSE`

Now, `cd` into this directory and follow steps described [here](#usage) to use it!

### Using `cargo install`

sc_extract is available on [crates.io](https://crates.io/crates/sc_extract). You can install and build this tool by simply using `cargo install sc_extract` command from the terminal. The installation process will take a few minutes to build all dependencies, but once that's done, the tool will work very, very fast.

It will also add this tool to the shell path automatically, allowing you to use the tool from *any* directory.

### Building From Source

You can download this tool's [source code](https://codeload.github.com/AriusX7/sc-extract/zip/master) and build it yourself by using `cargo build --release` command. You need to `cd` into this tool's directory before executing that command.

**Note:** In the below example commands, it will be assumed that you have installed the tool using first or second method. If you installed from the source, you will have to replace `sc_extract` with `cargo run --release` in all commands.

## Usage

**Note:** You may need to replace `sc_extract` by `./sc_extract`, `sc_extract.exe` or `cargo run --release` in the commands below.

You will need the `_tex.sc` or `.csv` files of the Supercell game you wish to extract. You can get the files by downloading the APK of the game, changing the extension to `.zip`, unzipping it and navigating to `/assets/sc` (_tex.sc files), `/assets/csv_logic` (csv files) or `csv_client` (csv files) folder inside the unzipped folder.

After installing this tool, `cd` into the directory with the tool (not required if you add it to your path or use the second method).

```sh
cd path_to_sc_extract
```

Then, simply use the following command to extract the required files!

```sh
sc_extract [FLAGS] [OPTIONS] <path>
```

`path` must be a valid path pointing to a single `_tex.sc` or `.csv` file or a directory containing those files. See [Flags and Options](#flags-and-options) section to know more about them.

If you installed the tool using the source code, you may want to build the tool and all the dependencies prior to extracting the files. You can do so by run the `cargo build --release` command in the tool's directory. Building will take a couple of minutes, but running the tool in future will be very fast.

### Flags and Options

|     Flags     | Short |                      Description                      |
|:-------------:|:-----:|:-----------------------------------------------------:|
|    --delete   |   -d  |         Deletes source files after extracting         |
| --parallelize |   -p  | Extracts files in parallel, making the process faster |
|     --help    |   -h  |                Prints help information                |
|   --version   |   -V  |               Prints version information              |

|      Options     |     Short     |                                              Description                                             |                   out_path                   |
|:----------------:|:-------------:|:----------------------------------------------------------------------------------------------------:|:--------------------------------------------:|
| --out <out_path> | -o <out_path> | Specifies the output directory. If not specified, a directory named `extracts` is created in `path`. | `out_path` must be a valid path-like string. |

**Example Command:**

```sh
sc_extract --delete --p ./sc --out ./extracts
```

The above command uses `./sc` as the source directory. It goes over all files in the directory parallelly and extracts all valid `_tex.sc` and `.csv` files. The output is saved in `./extracts` directory. After extracting, all valid `_tex.sc` and `.csv` files are deleted.

## Updating

If you used a pre-compiled binary, you'll simply have to download a new binary for the newer version from the [Releases](https://github.com/AriusX7/sc-extract/releases) page.

If you installed the tool using `cargo install`, you can update the tool by simply reusing the `cargo install sc_extract` command. If it fails to update the tool, you can force it by adding the `--force` flag, like so: `cargo install sc_extract --force`.

If you installed using the source code, you will have to repeat the process described in [Building From Source](#building-from-source) section using the new source code.

## License

sc_extract is available under the `MIT` license. See [LICENSE](LICENSE) for more details.

## Credits

This tool wouldn't exist if the following didn't create [the Python script](https://github.com/123456abcdef/cr-sc-dump#credits) to extract the files.

- [athlan20](https://github.com/athlan20)
- [clanner](https://github.com/clanner)
- [Galaxy1036](https://github.com/Galaxy1036)
- [umop-aplsdn](https://github.com/umop-aplsdn)
