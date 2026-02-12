## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 409.1 | 416.9 | 409.8 |
| solc | 130.2 ⚡ | 132.4 ⚡ | 130.1 ⚡ |
| nomicfoundation | 914.6 | 922.9 | 914.7 |

### Responses

**Our LSP**

```json
"4 diagnostics: [3] [forge lint] function names should use mixedCase (forge-lint); [3] [forge lint] mutable variables sh...
```

**solc**

```json
"no diagnostics"
```

**nomicfoundation**

```json
"no diagnostics"
```

solc fastest diagnostics (130ms), Our LSP 410ms with , nomicfoundation 915ms with no diags.
