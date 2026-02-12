## 1. SPAWN + INITIALIZE (ms) — 10 iterations, 2 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 5.0 ⚡ | 5.4 ⚡ | 4.7 ⚡ |
| solc | 121.6 | 124.0 | 121.7 |
| nomicfoundation | 882.0 | 891.9 | 878.9 |

### Responses

**Our LSP**
```json
"ok"
```

**solc**
```json
"ok"
```

**nomicfoundation**
```json
"ok"
```


Our LSP fastest startup (5ms), solc 122ms, nomicfoundation 879ms.
