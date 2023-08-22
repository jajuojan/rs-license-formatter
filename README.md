# rs-license-formatter
Convert rust's yaml-based 3rd party license-file into markdown

## Usage
```
Convert third-party license-information from toml to markdown

Usage: rs-license-formatter.exe [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>
          Location of yaml license-file

Options:
  -f, --fail-on-missing
          Fail on missing license-texts

  -o, --output <CHOICE>
          Choose what to output

          [default: all]
          [possible values: all, toc-only, license-texts-only]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example
```
cargo bundle-licenses --format yaml --output THIRDPARTY.yml
cargo run THIRDPARTY.yml > THIRDPARTY.md
```
