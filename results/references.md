## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 21.1 ⚡ | 124.1 ⚡ | 30.8 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 437ms]
```json
[{"range":{"end":{"character":16,"line":8},"start":{"character":8,"line":8}},"uri":"file:///Users/meek/developer/mmsaki/...
```

**solc**  [diag: 1 in 136ms]
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


Our LSP 30.8ms, solc unsupported, nomicfoundation timeout, juanfranblanco timeout, qiuxiang timeout.
