# Solidity LSP Benchmarks

Benchmarks comparing Solidity LSP servers against Uniswap V4-core (`Pool.sol`, 618 lines).

## Settings

| Setting | Value |
|---------|-------|
| Iterations | 10 |
| Warmup | 2 |
| Timeout | 30s |

## Servers

| Server | Version |
|--------|---------|
| Our LSP | solidity-language-server 0.1.13+commit.843bd50.macos.aarch64 |
| solc | 0.8.33+commit.64118f21.Darwin.appleclang |
| nomicfoundation | @nomicfoundation/solidity-language-server 0.8.25 |
| juanfranblanco | vscode-solidity-server 0.0.187 |
| qiuxiang | solidity-ls 0.5.4 |

## Results

| Benchmark | Our LSP 🏆 | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|--------------|------|-----------------|----------------|----------|
| Spawn + Init | 4.5ms 🥇 | 113.3ms 🥉 | 857.4ms | 508.4ms | 66.7ms 🥈 |
| Diagnostics | 440.3ms 🥈 | 131.9ms 🥇 | timeout | FAIL | timeout |
| Go to Definition | 8.6ms 🥇 | - | timeout | FAIL | timeout |
| Go to Declaration | 8.6ms 🥇 | unsupported | timeout | FAIL | timeout |
| Hover | 13.6ms 🥇 | - | timeout | FAIL | timeout |
| Find References | 10.4ms 🥇 | unsupported | timeout | FAIL | timeout |
| Document Symbols | 8.5ms 🥇 | unsupported | timeout | FAIL | timeout |
| Document Links | 63.3ms 🥇 | unsupported | timeout | FAIL | timeout |

*Generated from `benchmarks/2026-02-13T02-43-22Z.json`*
*Benchmark run: 2026-02-13T02:43:22Z*

See [DOCS.md](./DOCS.md) for usage and installation.
