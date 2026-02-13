# Solidity LSP Benchmarks

Benchmarks comparing Solidity LSP servers against `examples` (`Counter.sol`).

## Settings

| Setting | Value |
|---------|-------|
| Project | `examples` |
| File | `Counter.sol` |
| Target position | line 21, col 8 |
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

| Benchmark | mmsaki | solc 🏆 | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|--------|-----------|-----------------|----------------|----------|
| [Spawn + Init](#spawn--init) | 5.00ms 🥇 | 110.50ms 🥉 | 860.40ms | 516.30ms | 68.90ms 🥈 |
| [Diagnostics](#diagnostics) | 121.40ms 🥈 | 0.80ms 🥇 | 372.50ms | 809.90ms | 151.00ms 🥉 |
| [Go to Definition](#go-to-definition) | 114.50ms | 0.10ms 🥇 | 0.30ms 🥈 | 0.30ms 🥉 | 0.30ms |
| [Hover](#hover) | 114.50ms | 0.10ms 🥇 | 0.30ms 🥈 | 0.30ms 🥉 | - |
| [Find References](#find-references) | 0.40ms 🥈 | unsupported | - | 0.90ms 🥉 | 0.30ms 🥇 |

> **🏆 Overall Winner: solc** — 3 🥇 out of 5 benchmarks

### Medal Tally

| Server | 🥇 Gold | 🥈 Silver | 🥉 Bronze | Score |
|--------|------|----------|----------|-------|
| **solc** 🏆 | 3 | 0 | 1 | 10 |
| **mmsaki** | 1 | 2 | 0 | 7 |
| **qiuxiang** | 1 | 1 | 1 | 6 |
| **nomicfoundation** | 0 | 2 | 0 | 4 |
| **juanfranblanco** | 0 | 0 | 3 | 3 |

## Feature Support

| Feature | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|---------|--------|------|-----------------|----------------|----------|
| Spawn + Init | yes | yes | yes | yes | yes |
| Diagnostics | yes | yes | yes | yes | yes |
| Go to Definition | yes | yes | yes | yes | yes |
| Hover | yes | yes | yes | yes | empty |
| Find References | yes | no | empty | yes | yes |

> yes = supported   no = unsupported   timeout = server timed out   crash = server crashed   empty = returned null/empty

---

## Detailed Results

### Spawn + Init

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 5.00ms | 4.40ms | 11.20ms |
| **solc** | 🥉 | 110.50ms | 110.00ms | 114.40ms |
| **nomicfoundation** | ok | 860.40ms | 861.20ms | 880.20ms |
| **juanfranblanco** | ok | 516.30ms | 515.20ms | 519.90ms |
| **qiuxiang** | 🥈 | 68.90ms | 69.10ms | 70.30ms |

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
| **mmsaki** | 🥈 | 121.40ms | 120.90ms | 123.90ms |
| **solc** | 🥇 | 0.80ms | 0.80ms | 0.90ms |
| **nomicfoundation** | ok | 372.50ms | 372.20ms | 378.80ms |
| **juanfranblanco** | ok | 809.90ms | 814.50ms | 836.40ms |
| **qiuxiang** | 🥉 | 151.00ms | 151.60ms | 154.50ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "diagnostics": [
    {
      "code": "2072",
      "message": "[forge build] Unused local variable.",
      "range": {
        "end": {
          "character": 19,
          "line": 20
        },...
```

**solc**

```json
{
  "diagnostics": [
    {
      "code": 2072,
      "message": "Warning: Unused local variable.",
      "range": {
        "end": {
          "character": 19,
          "line": 20
        },...
```

**nomicfoundation**

```json
{
  "diagnostics": [
    {
      "code": "2072",
      "message": "Unused local variable.",
      "range": {
        "end": {
          "character": 19,
          "line": 20
        },...
```

**juanfranblanco**

```json
{
  "diagnostics": [
    {
      "code": "2072",
      "message": "Unused local variable.",
      "range": {
        "end": {
          "character": 19,
          "line": 20
        },...
```

**qiuxiang**

```json
{
  "diagnostics": [
    {
      "code": "2072",...
```

</details>

### Go to Definition

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | ok | 114.50ms | 114.20ms | 117.40ms |
| **solc** | 🥇 | 0.10ms | 0.10ms | 0.10ms |
| **nomicfoundation** | 🥈 | 0.30ms | 0.30ms | 0.50ms |
| **juanfranblanco** | 🥉 | 0.30ms | 0.30ms | 0.50ms |
| **qiuxiang** | ok | 0.30ms | 0.30ms | 0.40ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "range": {
    "end": {
      "character": 25,
      "line": 9
    },
    "start": {
      "character": 19,
      "line": 9
    }
  },...
```

**solc**

```json
[
  {
    "range": {
      "end": {
        "character": 25,
        "line": 9
      },
      "start": {
        "character": 19,
        "line": 9
      }
    },...
```

**nomicfoundation**

```json
{
  "range": {
    "end": {
      "character": 25,
      "line": 9
    },
    "start": {
      "character": 19,
      "line": 9
    }
  },...
```

**juanfranblanco**

```json
[
  {
    "range": {
      "end": {
        "character": 26,
        "line": 9
      },
      "start": {
        "character": 4,
        "line": 9
      }
    },...
```

**qiuxiang**

```json
{
  "range": {
    "end": {
      "character": 26,
      "line": 19
    },
    "start": {
      "character": 17,
      "line": 19
    }
  },...
```

</details>

### Hover

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | ok | 114.50ms | 114.20ms | 116.90ms |
| **solc** | 🥇 | 0.10ms | 0.10ms | 0.20ms |
| **nomicfoundation** | 🥈 | 0.30ms | 0.30ms | 0.40ms |
| **juanfranblanco** | 🥉 | 0.30ms | 0.30ms | 0.60ms |
| **qiuxiang** | invalid | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```solidity\nuint256 public number\n```\n\nSelector: `0x8381f58a`\n\n---\nThe current count."
  }
}
```

**solc**

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```solidity\nuint256\n```\n\n"
  },
  "range": {
    "end": {
      "character": 14,
      "line": 21
    },
    "start": {...
```

**nomicfoundation**

```json
{
  "contents": {
    "kind": "markdown",
    "value": "```solidity\nuint256 public number\n```"
  }
}
```

**juanfranblanco**

```json
{
  "contents": {
    "kind": "markdown",
    "value": "### State Variable: number\n#### Contract: Counter\n\t/// @notice The current count.\n\n### Type Info: \n### uint256\n"
  }
}
```

**qiuxiang**

```json
null
```

</details>

### Find References

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥈 | 0.40ms | 0.40ms | 0.40ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | invalid | - | - | - |
| **juanfranblanco** | 🥉 | 0.90ms | 0.60ms | 3.10ms |
| **qiuxiang** | 🥇 | 0.30ms | 0.30ms | 0.40ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
[
  {
    "range": {
      "end": {
        "character": 14,
        "line": 26
      },
      "start": {
        "character": 8,
        "line": 26
      }
    },...
```

**solc**

```json
error: Unknown method textDocument/references
```

**nomicfoundation**

```json
null
```

**juanfranblanco**

```json
[
  {
    "range": {
      "end": {
        "character": 26,
        "line": 9
      },
      "start": {
        "character": 4,
        "line": 9
      }
    },...
```

**qiuxiang**

```json
[
  {
    "range": {
      "end": {
        "character": 26,
        "line": 19
      },
      "start": {
        "character": 17,
        "line": 19
      }
    },...
```

</details>

---

*Generated from [`benchmarks/counter/2026-02-13T07-55-04Z.json`](benchmarks/counter/2026-02-13T07-55-04Z.json) — benchmark run: 2026-02-13T07:55:04Z*

See [DOCS.md](./DOCS.md) for usage and installation.
