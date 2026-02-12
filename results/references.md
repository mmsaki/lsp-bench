## 6. FIND REFERENCES (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Target: `TickMath` at line 103:15
Measures: textDocument/references request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 10.0 ⚡ | 11.0 ⚡ | 10.1 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 427ms]
```json
[{"range":{"end":{"character":32,"line":95},"start":{"character":24,"line":95}},"uri":"file:///Users/meek/developer/mmsa...
```

**solc**  [diag: 1 in 132ms]
```
error: Unknown method textDocument/references
```

**nomicfoundation**
```
FAIL: wait_for_diagnostics: timeout
```


Our LSP [{"range":{"end":{"character":32,"line":95},"start":{"character":24,"line":95}},"uri":"file:///Users/meek/developer/mmsa..., solc error: Unknown method textDocument/references, nomicfoundation timeout.
