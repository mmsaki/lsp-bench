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
| [mmsaki](https://github.com/mmsaki/solidity-language-server) | Solidity Language Server by mmsaki | `solidity-language-server 0.1.13+commit.843bd50.macos.aarch64` |
| [solc](https://docs.soliditylang.org) | Official Solidity compiler LSP | `0.8.33+commit.64118f21.Darwin.appleclang` |
| [nomicfoundation](https://github.com/NomicFoundation/hardhat-vscode) | Hardhat/Nomic Foundation Solidity Language Server | `@nomicfoundation/solidity-language-server 0.8.25` |
| [juanfranblanco](https://github.com/juanfranblanco/vscode-solidity) | VSCode Solidity by Juan Blanco | `vscode-solidity-server 0.0.187` |
| [qiuxiang](https://github.com/qiuxiang/solidity-ls) | Solidity Language Server by qiuxiang | `solidity-ls 0.5.4` |

## Results

| Benchmark | mmsaki 🏆 | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|-------------|------|-----------------|----------------|----------|
| [Spawn + Init](#spawn--init) | 4.00ms 🥇 | 111.00ms 🥉 | 844.30ms | 515.40ms | 69.20ms 🥈 |
| [Diagnostics](#diagnostics) | 439.00ms 🥈 | 132.70ms 🥇 | timeout | FAIL | timeout |
| [Go to Definition](#go-to-definition) | 8.60ms 🥇 | - | timeout | FAIL | timeout |
| [Go to Declaration](#go-to-declaration) | 8.70ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Hover](#hover) | 13.50ms 🥇 | - | timeout | FAIL | timeout |
| [Find References](#find-references) | 10.30ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Document Symbols](#document-symbols) | 8.40ms 🥇 | unsupported | timeout | FAIL | timeout |
| [Document Links](#document-links) | 62.30ms 🥇 | unsupported | timeout | FAIL | timeout |

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
| **mmsaki** | 🥇 | 4.00ms | 3.90ms | 4.80ms |
| **solc** | 🥉 | 111.00ms | 111.20ms | 112.80ms |
| **nomicfoundation** | ok | 844.30ms | 844.10ms | 866.40ms |
| **juanfranblanco** | ok | 515.40ms | 516.00ms | 521.50ms |
| **qiuxiang** | 🥈 | 69.20ms | 68.90ms | 76.30ms |

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
| **mmsaki** | 🥈 | 439.00ms | 438.40ms | 451.30ms |
| **solc** | 🥇 | 132.70ms | 132.90ms | 134.00ms |
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
| **mmsaki** | 🥇 | 8.60ms | 8.50ms | 9.90ms |
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
      "character": 8,
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
| **mmsaki** | 🥇 | 8.70ms | 8.60ms | 9.50ms |
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
      "character": 8,
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
| **mmsaki** | 🥇 | 13.50ms | 13.40ms | 13.90ms |
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
| **mmsaki** | 🥇 | 10.30ms | 10.30ms | 11.10ms |
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
        "character": 38,
        "line": 434
      },
      "start": {
        "character": 30,
        "line": 434
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
| **mmsaki** | 🥇 | 8.40ms | 8.50ms | 8.70ms |
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
| **mmsaki** | 🥇 | 62.30ms | 62.30ms | 63.00ms |
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

*Generated from [`benchmarks/v4-core/2026-02-13T07-18-33Z.json`](benchmarks/v4-core/2026-02-13T07-18-33Z.json) — benchmark run: 2026-02-13T07:18:33Z*

See [DOCS.md](./DOCS.md) for usage and installation.
