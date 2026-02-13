# Solidity LSP Benchmarks

Benchmarks comparing Solidity LSP servers against `v4-core` (`src/libraries/Pool.sol`).

## Settings

| Setting | Value |
|---------|-------|
| Project | `v4-core` |
| File | `src/libraries/Pool.sol` |
| Target position | line 102, col 15 |
| Iterations | 10 |
| Warmup | 2 |
| Request timeout | 10s |
| Index timeout | 15s |

## Servers

| Server | Description | Version |
|--------|-------------|---------|
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | Solidity Language Server by mmsaki | `solidity-language-server 0.1.14+commit.3d6a3d1.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | Official Solidity compiler LSP | `0.8.33+commit.64118f21.Darwin.appleclang` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | Hardhat/Nomic Foundation Solidity Language Server | `@nomicfoundation/solidity-language-server 0.8.25` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | VSCode Solidity by Juan Blanco | `vscode-solidity-server 0.0.187` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | Solidity Language Server by qiuxiang | `solidity-ls 0.5.4` |

## Results

| Benchmark | mmsaki 🏆 | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|-------------|------|-----------------|----------------|----------|
| [Spawn + Init](#spawn--init) | 4.20ms 🥇 | 112.00ms 🥉 | 862.40ms | 518.40ms | 68.90ms 🥈 |
| [Diagnostics](#diagnostics) | 443.10ms 🥈 | 134.40ms 🥇 | timeout | FAIL | timeout |
| [Go to Definition](#go-to-definition) | 8.90ms 🥇 | - | timeout | FAIL | timeout |
| [Go to Declaration](#go-to-declaration) | 9.00ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Hover](#hover) | 13.70ms 🥇 | - | timeout | FAIL | timeout |
| [Find References](#find-references) | 10.20ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Document Symbols](#document-symbols) | 8.40ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Document Links](#document-links) | 64.40ms 🥇 | unsupported | timeout | FAIL | timeout |

> **🏆 Overall Winner: mmsaki** — 7 🥇 out of 8 benchmarks

### Medal Tally

| Server | 🥇 Gold | 🥈 Silver | 🥉 Bronze | Score |
|--------|------|----------|----------|-------|
| **mmsaki** 🏆 | 7 | 1 | 0 | 23 |
| **solc** | 1 | 0 | 1 | 4 |
| **qiuxiang** | 0 | 1 | 0 | 2 |
| **nomicfoundation** | 0 | 0 | 0 | 0 |
| **juanfranblanco** | 0 | 0 | 0 | 0 |

## Feature Support

| Feature | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|---------|--------|------|-----------------|----------------|----------|
| Spawn + Init | yes | yes | yes | yes | yes |
| Diagnostics | yes | yes | timeout | crash | timeout |
| Go to Definition | yes | empty | timeout | crash | timeout |
| Go to Declaration | yes | no | timeout | crash | timeout |
| Hover | yes | empty | timeout | crash | timeout |
| Find References | yes | no | timeout | crash | timeout |
| Document Symbols | yes | no | timeout | crash | timeout |
| Document Links | yes | no | timeout | crash | timeout |

> yes = supported   no = unsupported   timeout = server timed out   crash = server crashed   empty = returned null/empty

---

## Detailed Results

### Spawn + Init

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 4.20ms | 4.20ms | 6.40ms |
| **solc** | 🥉 | 112.00ms | 112.00ms | 117.10ms |
| **nomicfoundation** | ok | 862.40ms | 862.60ms | 879.60ms |
| **juanfranblanco** | ok | 518.40ms | 517.80ms | 524.90ms |
| **qiuxiang** | 🥈 | 68.90ms | 69.00ms | 70.30ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
"ok"
```

**solc**

```json
"ok"
```

**nomicfoundation**

```json
"ok"
```

**juanfranblanco**

```json
"ok"
```

**qiuxiang**

```json
"ok"
```

</details>

### Diagnostics

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥈 | 443.10ms | 443.80ms | 451.10ms |
| **solc** | 🥇 | 134.40ms | 134.40ms | 138.80ms |
| **nomicfoundation** | timeout | - | - | - |
| **juanfranblanco** | EOF | - | - | - |
| **qiuxiang** | timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "diagnostics": [
    {
      "code": "mixed-case-function",
      "message": "[forge lint] function names should use mixedCase",
      "range": {
        "end": {
          "character": 21,...
```

**solc**

```json
{
  "diagnostics": [
    {
      "code": 6275,...
```

**nomicfoundation**

Error: `timeout`

**juanfranblanco**

Error: `EOF`

**qiuxiang**

Error: `timeout`

</details>

### Go to Definition

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 8.90ms | 8.70ms | 9.70ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "range": {
    "end": {
      "character": 16,
      "line": 9
    },
    "start": {
      "character": 8,
      "line": 9
    }
  },...
```

**solc**

```json
[]
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

### Go to Declaration

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 9.00ms | 8.80ms | 9.90ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "range": {
    "end": {
      "character": 16,
      "line": 9
    },
    "start": {
      "character": 8,
      "line": 9
    }
  },...
```

**solc**

```json
error: Unknown method textDocument/declaration
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

### Hover

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 13.70ms | 13.70ms | 14.50ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "contents": {
    "kind": "markdown",...
```

**solc**

```json
null
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

### Find References

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 10.20ms | 10.20ms | 11.00ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
[
  {
    "range": {
      "end": {
        "character": 32,
        "line": 95
      },
      "start": {
        "character": 24,
        "line": 95
      }
    },...
```

**solc**

```json
error: Unknown method textDocument/references
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

### Document Symbols

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 8.40ms | 8.40ms | 8.60ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
[
  {
    "kind": 15,
    "name": "solidity ^0.8.0",
    "range": {
      "end": {
        "character": 23,
        "line": 1
      },
      "start": {
        "character": 0,
        "line": 1...
```

**solc**

```json
error: Unknown method textDocument/documentSymbol
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

### Document Links

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 64.40ms | 63.90ms | 70.30ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | wait_for_diagnostics: timeout | - | - | - |
| **juanfranblanco** | wait_for_diagnostics: EOF | - | - | - |
| **qiuxiang** | wait_for_diagnostics: timeout | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
[
  {
    "range": {
      "end": {
        "character": 16,
        "line": 3
      },
      "start": {
        "character": 8,
        "line": 3
      }
    },...
```

**solc**

```json
error: Unknown method textDocument/documentLink
```

**nomicfoundation**

Error: `wait_for_diagnostics: timeout`

**juanfranblanco**

Error: `wait_for_diagnostics: EOF`

**qiuxiang**

Error: `wait_for_diagnostics: timeout`

</details>

---

*Generated from [`benchmarks/v4-core/2026-02-13T08-01-57Z.json`](benchmarks/v4-core/2026-02-13T08-01-57Z.json) — benchmark run: 2026-02-13T08:01:57Z*

See [DOCS.md](./DOCS.md) for usage and installation.
