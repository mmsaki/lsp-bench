## 1. SPAWN + INITIALIZE (ms) — 1 iterations, 0 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 8.9 ⚡ | 8.9 ⚡ | 8.9 ⚡ |
| solc | 121.1 | 121.1 | 121.1 |
| nomicfoundation | 862.9 | 862.9 | 862.9 |
| juanfranblanco | 519.5 | 519.5 | 519.5 |
| qiuxiang | 83.1 | 83.1 | 83.1 |

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

**juanfranblanco**
```json
"ok"
```

**qiuxiang**
```json
"ok"
```


Our LSP fastest startup (9ms), qiuxiang 83ms, solc 121ms, juanfranblanco 520ms, nomicfoundation 863ms.
