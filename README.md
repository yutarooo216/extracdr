# extracdr

`extracdr` is a command-line tool for extracting antibody CDR (Complementarity Determining Regions) sequences from FASTA files.  
Internally, it uses [ANARCI](https://github.com/oxpig/ANARCI) for IMGT-based numbering and annotation.  
This version is designed to work seamlessly with Docker Compose.

## Features

- FASTA input support  
- IMGT-based CDR extraction  
- JSON output  
- Simple CLI and Docker Compose integration  

## Getting Started (with Docker Compose)

### 1. Clone and build

```bash
git clone https://github.com/yourname/extracdr.git
cd extracdr
docker compose build
```

### 2. Prepare your input

Create a FASTA file (e.g. `input.fasta`) in the project root:

```fasta
>Trastuzumab_H
EVQLVESGGGLVQPGGSLRLSCAASGYTFSSYWMHWVRQAPGKGLEWVSAISSGGSHTYYADSVKGRFTISRDNAKNSLYLQMNSLRAEDTAVYYCAR

>Trastuzumab_L
DIQMTQSPSSLSASVGDRVTITCRASQDISNYLNWYQQKPGKAPKLLIYAASSLQSGVPSRFSGSGSGTDFTLTISSLQPEDFATYYCQQYNSYPYTFGQGTKVEIK
```

### 3. Run extracdr

```bash
docker compose up -d
docker exec -it extracdr /bin/bash
extracdr -i test.fasta --json
```

This command will:

- Mount `test.fasta` to `/app/test.fasta` inside the container  
- Extract CDRs using ANARCI  
- Write results to `/app/outputs/results.json` and `/app/outputs/anarci.tsv`

### 4. Output Structure

```
outputs/
├── anarci.tsv     # Raw ANARCI output
└── results.json   # CDR JSON output
```

Example output (`results.json`):

```json
{
  "Trastuzumab_H": {
    "cdr1": "GYTF----SSYW",
    "cdr2": "ISSG--GSHT",
    "cdr3": "A-----------R"
  },
  "Trastuzumab_L": {
    "cdr1": "QDI------SNY",
    "cdr2": "AA-------S",
    "cdr3": "QQYNS----YPYT"
  }
}
```

## CLI Options

| Option             | Description                                  |
|--------------------|----------------------------------------------|
| `-i`, `--input`     | Input FASTA file (required)                  |
| `-o`, `--output`    | Output directory (default: `outputs/`)       |
| `--anarci-path`     | Path to the ANARCI executable (optional)     |
| `--json`            | Output results as `results.json`             |
| `-h`, `--help`      | Show help message                            |
| `-V`, `--version`   | Show version information                     |

## License

- extracdr: MIT License  
- ANARCI: [BSD 3-Clause](https://github.com/oxpig/ANARCI/blob/master/LICENSE)

## Author

Yutaro Ito  
https://github.com/yourname