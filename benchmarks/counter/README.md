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
| [initialize](#initialize) | 4.29ms 🥇 | 115.57ms 🥉 | 869.85ms | 521.23ms | 69.99ms 🥈 |
| [textDocument/diagnostic](#textdocumentdiagnostic) | 123.84ms 🥈 | 0.85ms 🥇 | 375.33ms | 816.74ms | 157.10ms 🥉 |
| [textDocument/definition](#textdocumentdefinition) | 122.79ms | 0.06ms 🥇 | 0.31ms 🥉 | 0.33ms | 0.25ms 🥈 |
| [textDocument/declaration](#textdocumentdeclaration) | 123.04ms 🥇 | unsupported | - | - | - |
| [textDocument/hover](#textdocumenthover) | 123.31ms | 0.06ms 🥇 | 0.33ms 🥉 | 0.31ms 🥈 | - |
| [textDocument/references](#textdocumentreferences) | 0.32ms 🥉 | unsupported | 0.29ms 🥈 | 0.87ms | 0.25ms 🥇 |

> **🏆 Overall Winner: solc** — 3 🥇 out of 6 benchmarks

### Medal Tally

| Server | 🥇 Gold | 🥈 Silver | 🥉 Bronze | Score |
|--------|------|----------|----------|-------|
| **solc** 🏆 | 3 | 0 | 1 | 10 |
| **mmsaki** | 2 | 1 | 1 | 9 |
| **qiuxiang** | 1 | 2 | 1 | 8 |
| **nomicfoundation** | 0 | 1 | 2 | 4 |
| **juanfranblanco** | 0 | 1 | 0 | 2 |

## Feature Support

| Feature | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|---------|--------|------|-----------------|----------------|----------|
| initialize | yes | yes | yes | yes | yes |
| textDocument/diagnostic | yes | yes | yes | yes | yes |
| textDocument/definition | yes | yes | yes | yes | yes |
| textDocument/declaration | yes | no | empty | empty | empty |
| textDocument/hover | yes | yes | yes | yes | empty |
| textDocument/references | yes | no | yes | yes | yes |

> yes = supported   no = unsupported   timeout = server timed out   crash = server crashed   empty = returned null/empty

## Memory Usage

Peak resident set size (RSS) measured after indexing.

| Server | Peak RSS | Measured During |
|--------|----------|-----------------|
| **mmsaki** | 4.9 MB | textDocument/diagnostic |
| **solc** | 26.2 MB | textDocument/diagnostic |
| **nomicfoundation** | 361.3 MB | textDocument/hover |
| **juanfranblanco** | 380.8 MB | textDocument/diagnostic |
| **qiuxiang** | 60.4 MB | textDocument/diagnostic |

---

## Detailed Results

### initialize

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 4.29ms | 4.27ms | 5.11ms |
| **solc** | 🥉 | 115.57ms | 116.39ms | 117.12ms |
| **nomicfoundation** | ok | 869.85ms | 866.94ms | 892.83ms |
| **juanfranblanco** | ok | 521.23ms | 521.53ms | 524.34ms |
| **qiuxiang** | 🥈 | 69.99ms | 70.07ms | 71.66ms |

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

### textDocument/diagnostic

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥈 | 123.84ms | 123.89ms | 125.73ms |
| **solc** | 🥇 | 0.85ms | 0.86ms | 0.95ms |
| **nomicfoundation** | ok | 375.33ms | 376.51ms | 378.25ms |
| **juanfranblanco** | ok | 816.74ms | 813.64ms | 969.00ms |
| **qiuxiang** | 🥉 | 157.10ms | 157.01ms | 159.01ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "diagnostics": [
    {
      "code": "2072",...
```

**solc**

```json
{
  "diagnostics": [
    {
      "code": 2072,...
```

**nomicfoundation**

```json
{
  "diagnostics": [
    {
      "code": "2072",...
```

**juanfranblanco**

```json
{
  "diagnostics": [
    {
      "code": "2072",...
```

**qiuxiang**

```json
{
  "diagnostics": [
    {
      "code": "2072",...
```

</details>

### textDocument/definition

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | ok | 122.79ms | 122.90ms | 123.85ms |
| **solc** | 🥇 | 0.06ms | 0.06ms | 0.11ms |
| **nomicfoundation** | 🥉 | 0.31ms | 0.33ms | 0.43ms |
| **juanfranblanco** | ok | 0.33ms | 0.33ms | 0.44ms |
| **qiuxiang** | 🥈 | 0.25ms | 0.24ms | 0.37ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "range": {
    "end": {
      "character": 25,
      "line": 9
    },...
```

**solc**

```json
[
  {
    "range": {
      "end": {
        "character": 25,...
```

**nomicfoundation**

```json
{
  "range": {
    "end": {
      "character": 25,
      "line": 9
    },...
```

**juanfranblanco**

```json
[
  {
    "range": {
      "end": {
        "character": 26,...
```

**qiuxiang**

```json
{
  "range": {
    "end": {
      "character": 26,
      "line": 19
    },...
```

</details>

### textDocument/declaration

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥇 | 123.04ms | 122.91ms | 129.45ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | invalid | - | - | - |
| **juanfranblanco** | invalid | - | - | - |
| **qiuxiang** | invalid | - | - | - |

<details>
<summary>Response details</summary>

**mmsaki**

```json
{
  "range": {
    "end": {
      "character": 25,
      "line": 9
    },...
```

**solc**

```json
error: Unknown method textDocument/declaration
```

**nomicfoundation**

```json
error: Unhandled method textDocument/declaration
```

**juanfranblanco**

```json
error: Unhandled method textDocument/declaration
```

**qiuxiang**

```json
error: Unhandled method textDocument/declaration
```

</details>

### textDocument/hover

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | ok | 123.31ms | 123.65ms | 124.67ms |
| **solc** | 🥇 | 0.06ms | 0.06ms | 0.09ms |
| **nomicfoundation** | 🥉 | 0.33ms | 0.35ms | 0.46ms |
| **juanfranblanco** | 🥈 | 0.31ms | 0.29ms | 0.42ms |
| **qiuxiang** | invalid | - | - | - |

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
{
  "contents": {
    "kind": "markdown",...
```

**nomicfoundation**

```json
{
  "contents": {
    "kind": "markdown",...
```

**juanfranblanco**

```json
{
  "contents": {
    "kind": "markdown",...
```

**qiuxiang**

```json
null
```

</details>

### textDocument/references

| Server | Status | Mean | P50 | P95 |
|--------|--------|------|-----|-----|
| **mmsaki** | 🥉 | 0.32ms | 0.31ms | 0.43ms |
| **solc** | invalid | - | - | - |
| **nomicfoundation** | 🥈 | 0.29ms | 0.31ms | 0.42ms |
| **juanfranblanco** | ok | 0.87ms | 0.70ms | 2.86ms |
| **qiuxiang** | 🥇 | 0.25ms | 0.26ms | 0.35ms |

<details>
<summary>Response details</summary>

**mmsaki**

```json
[
  {
    "range": {
      "end": {
        "character": 14,...
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
        "character": 25,...
```

**juanfranblanco**

```json
[
  {
    "range": {
      "end": {
        "character": 26,...
```

**qiuxiang**

```json
[
  {
    "range": {
      "end": {
        "character": 26,...
```

</details>

---

*Generated from [`benchmarks/counter/2026-02-13T10-50-40Z.json`](benchmarks/counter/2026-02-13T10-50-40Z.json) — benchmark run: 2026-02-13T10:50:40Z*

See [DOCS.md](./DOCS.md) for usage and installation.
