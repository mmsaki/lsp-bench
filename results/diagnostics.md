## 2. OPEN FILE -> FIRST DIAGNOSTIC (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: didOpen notification -> first publishDiagnostics response

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 711.6 | 726.9 | 702.9 |
| solc | 134.7 ⚡ | 141.7 ⚡ | 134.7 ⚡ |
| nomicfoundation | 918.2 | 953.0 | 918.6 |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | 256.9 | 258.8 | 256.3 |

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


solc fastest diagnostics (135ms), qiuxiang 256ms, Our LSP 703ms, nomicfoundation 919ms, juanfranblanco fail.
