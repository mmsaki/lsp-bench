# Documentation

## Prerequisites

- [solidity-language-server](https://github.com/mmsaki/solidity-language-server): `cargo install solidity-language-server`
- [solc](https://docs.soliditylang.org/en/latest/installing-solidity.html)
- [nomicfoundation-solidity-language-server](https://github.com/NomicFoundation/hardhat-vscode) `npm i -g @nomicfoundation/solidity-language-server`
- [vscode-solidity-server](https://github.com/juanfranblanco/vscode-solidity): `npm i -g vscode-solidity-server`
- [solidity-ls](https://github.com/qiuxiang/solidity-ls): `npm i -g solidity-ls`

## Run Benchmarks

```sh
git clone --recursive https://github.com/mmsaki/solidity-lsp-benchmarks.git
cd solidity-lsp-benchmarks
cargo build --release
./target/release/bench [OPTIONS] <COMMAND>
```

### Commands

| Command | Description |
|---------|-------------|
| `all` | Run all benchmarks |
| `spawn` | Spawn + initialize handshake |
| `diagnostics` | Open Pool.sol, time to first diagnostic |
| `definition` | Go-to-definition on TickMath in Pool.sol |
| `declaration` | Go-to-declaration on TickMath in Pool.sol |
| `hover` | Hover on TickMath in Pool.sol |
| `references` | Find references on TickMath in Pool.sol |
| `documentSymbol` | Get document symbols for Pool.sol |
| `documentLink` | Get document links for Pool.sol |

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-n, --iterations` | 10 | Number of measured iterations |
| `-w, --warmup` | 2 | Number of warmup iterations |
| `-t, --timeout` | 30 | Timeout per request in seconds |
| `-h, --help` | | Show help message |

### Examples

```sh
bench all                   # Run all benchmarks
bench all -n 1 -w 0         # Run all benchmarks once, no warmup
bench diagnostics -n 5      # Run diagnostics with 5 iterations
bench spawn definition      # Run specific benchmarks
bench all -t 10             # Run all benchmarks with 10s timeout
```

## Generate README

After running benchmarks, generate the README from JSON data:

```sh
./target/release/gen-readme                           # uses latest JSON in benchmarks/
./target/release/gen-readme benchmarks/2026-02-12.json # use a specific snapshot
```

## Output

`bench` produces JSON snapshots:

- `benchmarks/<timestamp>.json` — full runs
- `benchmarks/<names>/<timestamp>.json` — partial runs (e.g. `benchmarks/diagnostics/`)

`gen-readme` reads a JSON snapshot and writes `README.md` with medals, trophy, and results table.
