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
| [Spawn + Init](#spawn--init) | 4.10ms 🥇 | 110.50ms 🥉 | 853.40ms | 507.00ms | 66.00ms 🥈 |
| [Diagnostics](#diagnostics) | 119.50ms 🥈 | 0.80ms 🥇 | 375.70ms | 801.30ms | 153.60ms 🥉 |
| [Go to Definition](#go-to-definition) | 117.60ms | 0.10ms 🥇 | 0.30ms 🥈 | 0.40ms | 0.30ms 🥉 |
| [Hover](#hover) | 116.00ms | 0.10ms 🥇 | 0.30ms 🥈 | 0.30ms 🥉 | - |
| [Find References](#find-references) | 0.30ms 🥈 | unsupported | 0.30ms 🥉 | 0.60ms | 0.20ms 🥇 |

> **🏆 Overall Winner: solc** — 3 🥇 out of 5 benchmarks

### Medal Tally

| Server | 🥇 Gold | 🥈 Silver | 🥉 Bronze | Score |
|--------|------|----------|----------|-------|
| **solc** 🏆 | 3 | 0 | 1 | 10 |
| **mmsaki** | 1 | 2 | 0 | 7 |
| **qiuxiang** | 1 | 1 | 2 | 7 |
| **nomicfoundation** | 0 | 2 | 1 | 5 |
| **juanfranblanco** | 0 | 0 | 1 | 1 |

## Feature Support

| Feature | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|---------|--------|------|-----------------|----------------|----------|
| Spawn + Init | yes | yes | yes | yes | yes |
| Diagnostics | yes | yes | yes | yes | yes |
| Go to Definition | yes | yes | yes | yes | yes |
| Hover | yes | yes | yes | yes | empty |
| Find References | yes | no | yes | yes | yes |

> yes = supported   no = unsupported   timeout = server timed out   crash = server crashed   empty = returned null/empty

---

## Detailed Results

### Spawn + Init

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 4.10ms | 4.10ms | 4.60ms |
| **solc** | 🥉 | 110.50ms | 112.40ms | 113.10ms |
| **nomicfoundation** | ok | 853.40ms | 857.70ms | 874.80ms |
| **juanfranblanco** | ok | 507.00ms | 506.40ms | 513.70ms |
| **qiuxiang** | 🥈 | 66.00ms | 66.50ms | 67.60ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
ok
```

**solc**

```json
ok
```

**nomicfoundation**

```json
ok
```

**juanfranblanco**

```json
ok
```

**qiuxiang**

```json
ok
```

</details>

### Diagnostics

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥈 | 119.50ms | 118.50ms | 127.00ms |
| **solc** | 🥇 | 0.80ms | 0.80ms | 0.80ms |
| **nomicfoundation** | ok | 375.70ms | 374.90ms | 382.40ms |
| **juanfranblanco** | ok | 801.30ms | 800.00ms | 872.70ms |
| **qiuxiang** | 🥉 | 153.60ms | 153.70ms | 156.30ms |

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
| **mmsaki** | ok | 117.60ms | 117.30ms | 122.10ms |
| **solc** | 🥇 | 0.10ms | 0.10ms | 0.20ms |
| **nomicfoundation** | 🥈 | 0.30ms | 0.40ms | 0.40ms |
| **juanfranblanco** | ok | 0.40ms | 0.40ms | 0.40ms |
| **qiuxiang** | 🥉 | 0.30ms | 0.20ms | 0.40ms |

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
| **mmsaki** | ok | 116.00ms | 116.90ms | 121.20ms |
| **solc** | 🥇 | 0.10ms | 0.00ms | 0.10ms |
| **nomicfoundation** | 🥈 | 0.30ms | 0.30ms | 0.50ms |
| **juanfranblanco** | 🥉 | 0.30ms | 0.30ms | 0.50ms |
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
| **mmsaki** | 🥈 | 0.30ms | 0.30ms | 0.40ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | 🥉 | 0.30ms | 0.30ms | 0.40ms |
| **juanfranblanco** | ok | 0.60ms | 0.60ms | 0.80ms |
| **qiuxiang** | 🥇 | 0.20ms | 0.30ms | 0.30ms |

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

*Generated from [`benchmarks/counter/2026-02-13T08-16-42Z.json`](benchmarks/counter/2026-02-13T08-16-42Z.json) — benchmark run: 2026-02-13T08:16:42Z*

See [DOCS.md](./DOCS.md) for usage and installation.
