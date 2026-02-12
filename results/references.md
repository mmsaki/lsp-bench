## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 16.6 ⚡ | 38.1 ⚡ | 20.7 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 433ms]
```json
[{"range":{"end":{"character":40,"line":354},"start":{"character":32,"line":354}},"uri":"file:///Users/meek/developer/mm...
```

**solc**  [diag: 1 in 133ms]
```
error: Unknown method textDocument/references
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


Our LSP 20.7ms, solc unsupported, nomicfoundation timeout, juanfranblanco timeout, qiuxiang timeout.
