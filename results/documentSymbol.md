## 7. DOCUMENT SYMBOLS (ms) — 10 iterations, 2 warmup

File: Pool.sol (613 lines)
Measures: textDocument/documentSymbol request -> response
Waits for valid publishDiagnostics before sending requests.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 21.1 ⚡ | 35.8 ⚡ | 22.2 ⚡ |
| solc | - | - | - |
| nomicfoundation | FAIL | FAIL | FAIL |
| juanfranblanco | FAIL | FAIL | FAIL |
| qiuxiang | FAIL | FAIL | FAIL |

### Responses

**Our LSP**  [diag: 4 in 420ms]
```json
[{"kind":15,"name":"solidity ^0.8.0","range":{"end":{"character":23,"line":1},"start":{"character":0,"line":1}},"selecti...
```

**solc**  [diag: 1 in 139ms]
```
error: Unknown method textDocument/documentSymbol
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


Our LSP 22.2ms, solc unsupported, nomicfoundation timeout, juanfranblanco timeout, qiuxiang timeout.
