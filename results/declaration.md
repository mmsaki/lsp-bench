## 4. GO TO DECLARATION (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/declaration request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.3 ⚡ | 9.3 ⚡ | 8.4 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 424ms]
```json
{"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...
```

**solc**  [diag: 1 in 132ms]
```
error: Unknown method textDocument/declaration
```

**nomicfoundation**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., solc error: Unknown method textDocument/declaration, nomicfoundation timeout.
