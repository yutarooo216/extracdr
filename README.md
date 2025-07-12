# extracdr

`extracdr` is a command-line tool for extracting antibody CDR (Complementarity Determining Regions) sequences from a FASTA file.  
Internally, it calls [ANARCI](https://github.com/oxpig/ANARCI) for numbering and annotation.

## Features

- Supports FASTA input
- CDR annotation using the IMGT scheme
- JSON output option
- Can be used without Rust or Cargo (via prebuilt binary)

## Download

1. Visit the [Releases](https://github.com/yourname/extracdr/releases) page.
2. Download the binary for your platform:
   - `extracdr-macos`
   - `extracdr-linux`
   - `extracdr-windows.exe`
3. (Linux/macOS only) Make the binary executable:

```bash
chmod +x extracdr-macos
```

4. Run the tool:

```bash
./extracdr-macos -i input.fasta
```

## Requirements

- Python 3.x
- ANARCI installed via pip:

```bash
pip install anarci
```

Make sure `anarci` is accessible from your system's PATH.

## Usage

```bash
extracdr -i input.fasta
```

### Options

| Option             | Description                                  |
|--------------------|----------------------------------------------|
| `-i`, `--input`     | Input FASTA file (required)                  |
| `-o`, `--output`    | Output directory (default: `outputs/`)       |
| `--anarci-path`     | Path to the ANARCI executable (optional)     |
| `--json`            | Output results as `results.json`             |
| `-h`, `--help`      | Show help message                            |
| `-V`, `--version`   | Show version information                     |

## Output Structure

```
outputs/
├── results.txt     # Tab-separated CDR annotations
├── results.json    # JSON output (only if --json is specified)
└── log/            # ANARCI logs and intermediate files
```

## License

This software is licensed under the MIT License.  
ANARCI is licensed under the [BSD 3-Clause License](https://github.com/oxpig/ANARCI/blob/master/LICENSE).

## Author

Yutaro Ito  
https://github.com/yourname
