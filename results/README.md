# Solidity LSP Benchmark Results

10 iterations, 2 warmup, 10s timeout

| Benchmark | Our LSP | solc | nomicfoundation | juanfranblanco | qiuxiang |
|-----------|---------|------|-----------------|----------------|----------|
| Spawn + Init | 4.1ms ⚡ | 122.5ms | 860.6ms | 510.2ms | 67.4ms |
| Diagnostics | 650.0ms | 132.5ms ⚡ | 914.3ms | FAIL | 256.6ms |
| Go to Definition | 27.0ms ⚡ | - | timeout | FAIL | timeout |
| Go to Declaration | 31.0ms ⚡ | - | timeout | FAIL | timeout |
| Hover | - | - | timeout | FAIL | timeout |
| Find References | 20.7ms ⚡ | - | timeout | FAIL | timeout |
| Document Symbols | 22.2ms ⚡ | - | timeout | FAIL | timeout |

Detailed results per benchmark:

- [spawn](./spawn.md)
- [diagnostics](./diagnostics.md)
- [definition](./definition.md)
- [declaration](./declaration.md)
- [hover](./hover.md)
- [references](./references.md)
- [documentSymbol](./documentSymbol.md)
