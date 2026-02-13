# Benchmark Analysis

Analysis of `examples` (`Counter.sol`) — 10 iterations per benchmark.

## Servers

| Server | Description | Version |
|--------|-------------|---------|
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | Solidity Language Server by mmsaki | `solidity-language-server 0.1.14+commit.3d6a3d1.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | Official Solidity compiler LSP | `0.8.33+commit.64118f21.Darwin.appleclang` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | Hardhat/Nomic Foundation Solidity Language Server | `@nomicfoundation/solidity-language-server 0.8.25` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | VSCode Solidity by Juan Blanco | `vscode-solidity-server 0.0.187` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | Solidity Language Server by qiuxiang | `solidity-ls 0.5.4` |

## Capability Matrix

| Benchmark | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|--------|------|-----------------|----------------|----------|
| initialize | ok | ok | ok | ok | ok |
| textDocument/diagnostic | ok | ok | ok | ok | ok |
| textDocument/definition | ok | ok | ok | ok | ok |
| textDocument/declaration | ok | no | empty | empty | empty |
| textDocument/hover | ok | ok | ok | ok | empty |
| textDocument/references | ok | no | ok | ok | ok |

| Server | Working | Failed | Success Rate |
|--------|---------|--------|--------------|
| mmsaki | 6/6 | 0/6 | 100% |
| solc | 4/6 | 2/6 | 67% |
| nomicfoundation | 5/6 | 1/6 | 83% |
| juanfranblanco | 5/6 | 1/6 | 83% |
| qiuxiang | 4/6 | 2/6 | 67% |

## initialize

| Server | Status | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | Overhead | vs mmsaki |
|--------|--------|------|-----|-----|--------|-------|-----|-----|-------|----------|-----------|
| mmsaki | ok | 4.29ms | 4.3ms | 5.1ms | 0.8ms | 1.20x | 3.56ms | 5.11ms | 1.55ms | **1.0x (fastest)** | - |
| solc | ok | 115.57ms | 116.4ms | 117.1ms | 0.7ms | 1.01x | 111.40ms | 117.12ms | 5.72ms | **26.9x** | **26.9x slower** |
| nomicfoundation | ok | 869.85ms | 866.9ms | 892.8ms | **25.9ms** | 1.03x | 853.10ms | 892.83ms | **39.73ms** | **202.8x** | **202.8x slower** |
| juanfranblanco | ok | 521.23ms | 521.5ms | 524.3ms | 2.8ms | 1.01x | 519.05ms | 524.34ms | 5.29ms | **121.5x** | **121.5x slower** |
| qiuxiang | ok | 69.99ms | 70.1ms | 71.7ms | 1.6ms | 1.02x | 68.88ms | 71.66ms | 2.78ms | **16.3x** | **16.3x slower** |

## textDocument/diagnostic

| Server | Status | Mem | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | Overhead | vs mmsaki |
|--------|--------|-----|------|-----|-----|--------|-------|-----|-----|-------|----------|-----------|
| mmsaki | ok | 4.9 MB | 123.84ms | 123.9ms | 125.7ms | 1.8ms | 1.01x | 121.56ms | 125.73ms | 4.17ms | **145.7x** | - |
| solc | ok | 26.2 MB | 0.85ms | 0.9ms | 0.9ms | 0.1ms | 1.10x | 0.75ms | 0.95ms | 0.20ms | **1.0x (fastest)** | **145.7x faster** |
| nomicfoundation | ok | 361.2 MB | 375.33ms | 376.5ms | 378.2ms | 1.7ms | 1.00x | 367.82ms | 378.25ms | **10.43ms** | **441.6x** | 3.0x slower |
| juanfranblanco | ok | 380.8 MB | 816.74ms | 813.6ms | 969.0ms | **155.4ms** | 1.19x | 770.79ms | 969.00ms | **198.21ms** | **960.9x** | 6.6x slower |
| qiuxiang | ok | 60.4 MB | 157.10ms | 157.0ms | 159.0ms | 2.0ms | 1.01x | 155.55ms | 159.01ms | 3.46ms | **184.8x** | 1.3x slower |

## textDocument/definition

| Server | Status | Mem | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | Overhead | vs mmsaki |
|--------|--------|-----|------|-----|-----|--------|-------|-----|-----|-------|----------|-----------|
| mmsaki | ok | 4.9 MB | 122.79ms | 122.9ms | 123.8ms | 0.9ms | 1.01x | 121.53ms | 123.85ms | 2.32ms | **2046.5x** | - |
| solc | ok | 26.1 MB | 0.06ms | 0.1ms | 0.1ms | 0.1ms | **1.83x** | 0.04ms | 0.11ms | 0.07ms | **1.0x (fastest)** | **2046.5x faster** |
| nomicfoundation | ok | 357.5 MB | 0.31ms | 0.3ms | 0.4ms | 0.1ms | 1.30x | 0.20ms | 0.43ms | 0.23ms | 5.2x | **396.1x faster** |
| juanfranblanco | ok | 378.6 MB | 0.33ms | 0.3ms | 0.4ms | 0.1ms | 1.33x | 0.19ms | 0.44ms | 0.25ms | 5.5x | **372.1x faster** |
| qiuxiang | ok | 60.1 MB | 0.25ms | 0.2ms | 0.4ms | 0.1ms | **1.54x** | 0.17ms | 0.37ms | 0.20ms | 4.2x | **491.2x faster** |

## textDocument/declaration

| Server | Status | Mem | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | vs mmsaki |
|--------|--------|-----|------|-----|-----|--------|-------|-----|-----|-------|-----------|
| mmsaki | ok | 4.8 MB | 123.04ms | 122.9ms | 129.4ms | 6.5ms | 1.05x | 118.70ms | 129.45ms | **10.75ms** | - |
| solc | no | 26.0 MB | - | - | - | - | - | - | - | - | empty |
| nomicfoundation | empty | 357.1 MB | - | - | - | - | - | - | - | - | empty |
| juanfranblanco | empty | 380.3 MB | - | - | - | - | - | - | - | - | empty |
| qiuxiang | empty | 60.1 MB | - | - | - | - | - | - | - | - | empty |

## textDocument/hover

| Server | Status | Mem | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | Overhead | vs mmsaki |
|--------|--------|-----|------|-----|-----|--------|-------|-----|-----|-------|----------|-----------|
| mmsaki | ok | 4.9 MB | 123.31ms | 123.7ms | 124.7ms | 1.0ms | 1.01x | 122.12ms | 124.67ms | 2.55ms | **2055.2x** | - |
| solc | ok | 26.0 MB | 0.06ms | 0.1ms | 0.1ms | 0.0ms | 1.50x | 0.04ms | 0.09ms | 0.05ms | **1.0x (fastest)** | **2055.2x faster** |
| nomicfoundation | ok | 361.3 MB | 0.33ms | 0.3ms | 0.5ms | 0.1ms | 1.31x | 0.20ms | 0.46ms | 0.26ms | 5.5x | **373.7x faster** |
| juanfranblanco | ok | 378.6 MB | 0.31ms | 0.3ms | 0.4ms | 0.1ms | 1.45x | 0.27ms | 0.42ms | 0.15ms | 5.2x | **397.8x faster** |
| qiuxiang | empty | 60.2 MB | - | - | - | - | - | - | - | - | - | empty |

## textDocument/references

| Server | Status | Mem | Mean | p50 | p95 | Spread | Spike | Min | Max | Range | Overhead | vs mmsaki |
|--------|--------|-----|------|-----|-----|--------|-------|-----|-----|-------|----------|-----------|
| mmsaki | ok | 4.9 MB | 0.32ms | 0.3ms | 0.4ms | 0.1ms | 1.39x | 0.28ms | 0.43ms | 0.15ms | 1.3x | - |
| solc | no | 26.1 MB | - | - | - | - | - | - | - | - | - | empty |
| nomicfoundation | ok | 360.2 MB | 0.29ms | 0.3ms | 0.4ms | 0.1ms | 1.35x | 0.18ms | 0.42ms | 0.24ms | 1.2x | 1.1x faster |
| juanfranblanco | ok | 379.2 MB | 0.87ms | 0.7ms | 2.9ms | 2.2ms | **4.09x** | 0.44ms | 2.86ms | 2.42ms | 3.5x | 2.7x slower |
| qiuxiang | ok | 60.0 MB | 0.25ms | 0.3ms | 0.3ms | 0.1ms | 1.35x | 0.17ms | 0.35ms | 0.18ms | **1.0x (fastest)** | 1.3x faster |

## Peak Memory (RSS)

| mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|--------|------|-----------------|----------------|----------|
| 4.9 MB | 26.2 MB | 361.3 MB | 380.8 MB | 60.4 MB |

---

*Generated from [`benchmarks/counter/2026-02-13T10-50-40Z.json`](benchmarks/counter/2026-02-13T10-50-40Z.json) — benchmark run: 2026-02-13T10:50:40Z*
