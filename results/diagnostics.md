## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 648.3 | 668.9 | 650.0 |
| solc | 131.6 ⚡ | 138.1 ⚡ | 132.5 ⚡ |
| nomicfoundation | 914.7 | 922.6 | 914.3 |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | 256.4 | 265.6 | 256.6 |

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

**juanfranblanco**
```
FAIL: EOF
```

**qiuxiang**
```json
"no diagnostics"
```


solc fastest diagnostics (133ms), qiuxiang 257ms, Our LSP 650ms, nomicfoundation 914ms, juanfranblanco fail.
