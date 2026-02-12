## 5. HOVER (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/hover request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | - | - | - |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 414ms]
```
error: Method not found
```

**solc**  [diag: 1 in 194ms]
```
null
```

**nomicfoundation**
```
FAIL: wait_for_diagnostics: timeout
```

**juanfranblanco**
```
FAIL: wait_for_diagnostics: EOF
```

**qiuxiang**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP error: Method not found, solc null, nomicfoundation timeout, juanfranblanco timeout, qiuxiang timeout.
