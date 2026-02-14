# I Benchmarked 5 Solidity LSP Servers. Here's What Changed.

In Part 1, I benchmarked 3 Solidity LSP servers against Uniswap V4-core's Pool.sol. My server won everything. People had questions. Is it only fast on big projects? What about small files? What about the LSP servers I didn't include?

Fair. So I expanded the test. 5 servers instead of 3. Two codebases instead of one. A 30-line Counter contract and a 618-line production file from Uniswap V4-core. Same methodology — 10 iterations, 2 warmup rounds, fresh process per spawn, JSON-RPC over stdio.

The small file results made me look bad. Then I looked closer.

## The Five Servers

- **mmsaki** — solidity-language-server, Rust, backed by forge
- **solc** — the Solidity compiler's built-in LSP, C++
- **nomicfoundation** — Hardhat VSCode extension's language server, Node.js
- **juanfranblanco** — vscode-solidity-server, the original VSCode Solidity extension, Node.js
- **qiuxiang** — solidity-ls, a lightweight Node.js language server

## Counter.sol: I Got Destroyed

Counter.sol is 30 lines. One contract, a few functions, some unused variables for diagnostics. The kind of file you write at a hackathon.

On this file, solc won most benchmarks. But even here, startup tells a story.

**Spawn + Init:**

| Server | Mean |
|--------|------|
| mmsaki | 5ms |
| qiuxiang | 69ms |
| solc | 110ms |
| juanfranblanco | 516ms |
| nomicfoundation | 860ms |

My server initializes in 5ms. That's before solc has even loaded. nomicfoundation takes 860ms just to say hello — almost a full second before it can do anything. Every time your editor opens a workspace, every time the server restarts, that cost hits. Startup is the one benchmark I win on both codebases, every time.

**Diagnostics:**

| Server | Mean |
|--------|------|
| solc | 0.8ms |
| mmsaki | 121ms |
| qiuxiang | 151ms |
| nomicfoundation | 372ms |
| juanfranblanco | 810ms |

solc at under a millisecond. My server at 121ms. That's the cost of shelling out to `forge build --ast` — on a 30-line file, you're paying process spawn overhead for a job that a compiler can do in memory instantly.

**Go to Definition:**

| Server | Mean |
|--------|------|
| solc | 0.1ms |
| nomicfoundation | 0.3ms |
| juanfranblanco | 0.3ms |
| qiuxiang | 0.3ms |
| mmsaki | 114ms |

Dead last. Every other server answered in under a millisecond.

solc took 3 golds. I took 1 (startup). qiuxiang took 1 (find references). The overall winner on Counter.sol was solc.

## But Wait. What Did They Actually Return?

Sub-millisecond is suspicious. So I checked the responses.

I targeted `number` on line 22 of Counter.sol — a state variable. Go to Definition should jump to its declaration: `uint256 public number` on line 9, characters 19-25.

- **mmsaki**: line 9, characters 19-25. Correct.
- **solc**: line 9, characters 19-25. Correct.
- **nomicfoundation**: line 9, characters 19-25. Correct.
- **juanfranblanco**: line 9, characters 4-26. Imprecise — selected the whole line instead of just the identifier.
- **qiuxiang**: **line 19**, characters 17-26. Wrong line entirely. It jumped to a *usage* of `number`, not its declaration.

qiuxiang answered in 0.3ms with the wrong location. Fast and wrong. You wouldn't notice the speed, but you'd absolutely notice landing on the wrong line.

This is why I now store the full response for every iteration in the benchmark output. The numbers don't tell the whole story. You have to look at what came back.

## Pool.sol: Everything Flipped

Pool.sol. 618 lines. Dozens of imports, libraries, cross-file dependencies. Real Solidity.

**Spawn + Init:**

| Server | Mean |
|--------|------|
| mmsaki | 4.2ms |
| qiuxiang | 69ms |
| solc | 112ms |
| juanfranblanco | 518ms |
| nomicfoundation | 862ms |

Same story as Counter. Startup doesn't depend on project size — it's pure process initialization. My server at 4.2ms, everyone else in the same ballpark as before. The numbers barely moved between the two codebases because spawn is spawn. But everything after this is where it gets ugly.

**Diagnostics:**

| Server | Mean |
|--------|------|
| solc | 134ms |
| mmsaki | 443ms |
| nomicfoundation | timeout |
| juanfranblanco | crash |
| qiuxiang | timeout |

Three servers couldn't even parse the file. nomicfoundation and qiuxiang timed out at 15 seconds. juanfranblanco sent EOF and died.

**Go to Definition:**

| Server | Mean |
|--------|------|
| mmsaki | 8.9ms |
| solc | empty |
| nomicfoundation | timeout |
| juanfranblanco | crash |
| qiuxiang | timeout |

One server returned a result. Mine. solc accepted the request but came back empty — it couldn't resolve the definition across files. The Node.js servers never got past indexing.

**The full feature matrix on V4-core:**

| Method | mmsaki | solc | nomicfoundation | juanfranblanco | qiuxiang |
|--------|--------|------|-----------------|----------------|----------|
| initialize | ok | ok | ok | ok | ok |
| textDocument/diagnostic | ok | ok | timeout | crash | timeout |
| textDocument/definition | ok | empty | timeout | crash | timeout |
| textDocument/declaration | ok | no | timeout | crash | timeout |
| textDocument/hover | ok | empty | timeout | crash | timeout |
| textDocument/references | ok | no | timeout | crash | timeout |
| textDocument/documentSymbol | ok | no | timeout | crash | timeout |
| textDocument/documentLink | ok | no | timeout | crash | timeout |

8 for 8. Every method. Every request answered. 7 gold medals out of 8.

## The Scaling Story

Put the two codebases side by side:

**Go to Definition — Counter.sol (30 lines):**
- solc: 0.1ms
- nomicfoundation: 0.3ms
- mmsaki: 114ms

**Go to Definition — Pool.sol (618 lines):**
- mmsaki: 8.9ms
- solc: empty
- nomicfoundation: timeout

Read that again. My server went from *last place* on the small file to *the only server that works* on the large one. And it got *faster* — 114ms on Counter, 8.9ms on Pool.

That's not a paradox. My server shells out to `forge build --ast` once, which builds the full project AST. On a 30-line file, that forge overhead dominates. On a 618-line file with real dependencies, forge already understands the import graph, the library structure, the type system. The AST is cached. Subsequent requests just look things up.

The lightweight parsers do the opposite. They parse a single file in memory — fast when the file is small, useless when the answer lives in another file across an import chain. They don't build a project graph. They can't.

The 114ms on Counter.sol is the price of forge. The 8.9ms on Pool.sol is what you get for paying it.

## Consistency Under Load

My LSP was designed to be slow on the first request and fast after that — forge builds the AST once, then everything hits cache. The Node.js servers do the opposite: fast early, then garbage collection and memory pressure creep in.

The p50 vs p95 spread tells the story. This is Counter.sol — the *easy* file:

**Diagnostics — p50 vs p95 spread (Counter.sol):**

| Server | p50 | p95 | Spread | Spike |
|--------|-----|-----|--------|-------|
| mmsaki | 118.5ms | 127.0ms | 8.5ms | 1.07x |
| solc | 0.8ms | 0.8ms | 0ms | 1.0x |
| nomicfoundation | 374.9ms | 382.4ms | 7.5ms | 1.02x |
| juanfranblanco | 800.0ms | 872.7ms | **72.7ms** | **1.09x** |
| qiuxiang | 153.7ms | 156.3ms | 2.6ms | 1.02x |

juanfranblanco's worst iteration is 72.7ms slower than its median — at just 10 iterations. That's a server getting *worse* as it runs.

**Find References — p50 vs p95 spread (Counter.sol):**

| Server | p50 | p95 | Spread | Spike |
|--------|-----|-----|--------|-------|
| mmsaki | 0.3ms | 0.4ms | 0.1ms | 1.3x |
| nomicfoundation | 0.3ms | 0.4ms | 0.1ms | 1.3x |
| juanfranblanco | 0.6ms | 0.8ms | **0.2ms** | **1.3x** |
| qiuxiang | 0.3ms | 0.3ms | 0ms | 1.0x |

Now look at V4-core — where only mmsaki has data:

**V4-core — p50 vs p95 spread (all methods, mmsaki only):**

| Benchmark | p50 | p95 | Spread |
|-----------|-----|-----|--------|
| Spawn + Init | 4.0ms | 4.3ms | 0.3ms |
| Diagnostics | 441.2ms | 444.4ms | 3.2ms |
| Go to Definition | 8.8ms | 10.1ms | 1.3ms |
| Go to Declaration | 8.8ms | 9.8ms | 1.0ms |
| Hover | 13.9ms | 14.5ms | 0.6ms |
| Find References | 10.6ms | 11.8ms | 1.2ms |
| Document Symbols | 8.4ms | 8.6ms | 0.2ms |
| Document Links | 63.5ms | 64.1ms | 0.6ms |

Every benchmark under 4ms spread. On a 618-line file with dozens of cross-file dependencies. Rust has no garbage collector. The tenth iteration takes the same time as the first.

## The Memory Story

Speed is one thing. Memory is another. I added RSS (Resident Set Size) measurement to the benchmark tool — it samples how much physical memory each server holds after indexing.

**Counter.sol (30 lines) — Peak RSS:**

| Server | Peak RSS |
|--------|----------|
| mmsaki | 4.9 MB |
| solc | 26.2 MB |
| qiuxiang | 60.4 MB |
| nomicfoundation | 363.6 MB |
| juanfranblanco | 381.1 MB |

Read that. nomicfoundation and juanfranblanco each consume over **360 MB** to analyze a 30-line contract. My server uses **4.9 MB**. That's a 74x difference.

solc at 26 MB makes sense — it loads the full compiler. qiuxiang at 60 MB is reasonable for a Node.js runtime. But 360+ MB for a file you could fit in a tweet is Node.js dependency bloat at scale.

**V4-core (618 lines) — Peak RSS:**

| Server | Peak RSS | Status |
|--------|----------|--------|
| mmsaki | 39.7 MB | working |
| solc | 26.2 MB | working (diagnostics only) |
| qiuxiang | 70.1 MB | timed out |
| nomicfoundation | 513.5 MB | timed out |
| juanfranblanco | 0.0 MB | crashed |

nomicfoundation allocates **half a gigabyte** and still can't finish indexing. It times out at 15 seconds holding 514 MB of RAM. Meanwhile my server does all 8 features at 40 MB.

juanfranblanco shows 0 MB because it crashes (EOF) before the memory sample can be taken.

The memory story is actually worse than the latency story. A server that's slow is annoying. A server that eats 500 MB per workspace while failing to produce results is a resource leak in your editor. If you have 3 Solidity projects open in VS Code, that's 1.5 GB of memory for a language server that can't even resolve a definition.

## What This Means

Solidity developers don't build toy projects. The moment you write a test contract that inherits from `forge-std/Test.sol`, you're pulling in hundreds of functions across dozens of files. That's day one of any Foundry project.

Here's what the servers actually deliver on real code:

| Capability | Counter.sol (toy) | V4-core (production) |
|------------|-------------------|----------------------|
| Servers that work | 5/5 | **1/5** |
| Servers with correct responses | 4/5 | **1/5** |
| Servers under 10ms definition | 4/5 | **1/5** |
| Servers that don't crash/timeout | 5/5 | **2/5** |

For small standalone contracts, any of these servers will do. For real projects, you need one that survives.

## The Framework

This benchmarking tool is now a framework. Write a YAML config, run it, get structured JSON with per-iteration latency and response data.

```yaml
project: my-protocol
file: src/Core.sol
line: 45
col: 12
servers:
  - label: my-server
    cmd: my-lsp-binary
```

Built with Claude Opus 4.6 via [Claude Code](https://claude.ai/code) in 15 minutes — config parsing, LSP communication over JSON-RPC, per-iteration data, memory measurement, README and analysis generation.

What you get out:

| Output | Format | Contents |
|--------|--------|----------|
| Benchmark data | JSON | Per-iteration latency, response, and memory (RSS) for every server |
| Results README | Markdown | Tables, medals, feature matrix, response details |
| Analysis report | Markdown | Per-feature breakdown with consistency, overhead, memory, and head-to-head |
| Partial results | JSON | Saved after each benchmark in case of crashes |

## Benchmarking Made My LSP Better

The benchmarks revealed that `forge build --ast` is my bottleneck. The numbers made it obvious:

| Operation | solc (in-process) | mmsaki (forge) | Overhead |
|-----------|-------------------|----------------|----------|
| Diagnostics (Counter) | 0.8ms | 121ms | **151x** |
| Diagnostics (V4-core) | 134ms | 443ms | **3.3x** |

Switching to solc for AST generation is on the roadmap.

The benchmarks also caught a bug: my Find References was returning identical start and end positions for every reference location. Right file, right line, wrong ranges. I wouldn't have caught that without comparing responses against other servers side by side. Fixed in [v0.1.14](https://github.com/mmsaki/solidity-language-server/releases/tag/v0.1.14).

And the memory measurement confirmed a design decision. Building on Rust without a garbage collector means my server holds 5-40 MB regardless of project size. The Node.js servers balloon to 350-500+ MB. That's not something you see in a latency table — you see it when your editor starts swapping.

This isn't just a leaderboard. It's a diagnostic tool.

## Try It Yourself

```
git clone --recursive https://github.com/mmsaki/solidity-lsp-benchmarks
cd solidity-lsp-benchmarks
cargo build --release
./target/release/lsp-bench init
# edit benchmark.yaml with your project and servers
./target/release/lsp-bench
```

Benchmark source: [github.com/mmsaki/solidity-lsp-benchmarks](https://github.com/mmsaki/solidity-lsp-benchmarks)

The LSP server: [github.com/mmsaki/solidity-language-server](https://github.com/mmsaki/solidity-language-server)

The numbers speak for themselves. But now you can verify them yourself — every iteration, every response, every failure. On your project, with your code.

---

**P.S.** I only benchmarked the LSP methods that most servers implement — `initialize`, `textDocument/diagnostic`, `textDocument/definition`, `textDocument/hover`, `textDocument/references`, `textDocument/documentSymbol`, and `textDocument/documentLink`. My server supports additional capabilities that the others don't, but including those would have made the comparison unfair. I wanted to test on common ground. Even on common ground — with memory measurement added — the results speak for themselves.
