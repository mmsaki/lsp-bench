# Solidity LSP Benchmark Results

Date: 2026-02-12

## Settings

| Setting | Value |
|---------|-------|
| Iterations | 1 |
| Warmup | 0 |
| Timeout | 30s |

## Servers

| Server | Version |
|--------|---------|
| Our LSP | solidity-language-server 0.1.11+commit..macos.aarch64 |
| solc | solc, the solidity compiler commandline interface |
| nomicfoundation | unknown |
| juanfranblanco | unknown |
| qiuxiang | unknown |

## Results

| Benchmark | Our LSP | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|---------|------|-----------------|----------------|----------|
| Spawn + Init | 6.2ms ⚡ | 123.4ms | 1021.7ms | 538.8ms | 90.2ms |

## Detailed Results

- [spawn](./spawn.md)

