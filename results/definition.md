## 3. GO TO DEFINITION (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/definition request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.5 ⚡ | 9.1 ⚡ | 8.5 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 420ms]
```json
{"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so...
```

**solc**  [diag: 1 in 131ms]
```
[]
```

**nomicfoundation**
```
FAIL: wait_for_diagnostics: timeout
```


solc returns [], Our LSP {"range":{"end":{"character":8,"line":9},"start":{"character":8,"line":9}},"uri":"file:///Users/meek/developer/mmsaki/so..., nomicfoundation timeout.
