## 1. SPAWN + INITIALIZE (ms) — 10 iterations, 2 warmup

Measures: spawn process -> initialize request -> response -> initialized notification
No files opened.

| Server | p50 | p95 | mean |
|--------|-----|-----|------|
| Our LSP | 4.0 ⚡ | 5.3 ⚡ | 4.1 ⚡ |
| solc | 123.0 | 125.4 | 122.5 |
| nomicfoundation | 861.2 | 886.8 | 860.6 |
| juanfranblanco | 509.5 | 513.8 | 510.2 |
| qiuxiang | 67.3 | 68.4 | 67.4 |

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


Our LSP fastest startup (4ms), qiuxiang 67ms, solc 123ms, juanfranblanco 510ms, nomicfoundation 861ms.
