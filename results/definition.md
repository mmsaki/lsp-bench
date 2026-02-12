## 3. GO TO DEFINITION (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/definition request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 18.0 ⚡ | 89.0 ⚡ | 27.0 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 410ms]
```json
{"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...
```

**solc**  [diag: 1 in 137ms]
```
[]
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


Our LSP 27.0ms, solc no result, nomicfoundation timeout, juanfranblanco timeout, qiuxiang timeout.
