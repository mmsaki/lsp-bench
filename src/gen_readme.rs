use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Find the JSON file to use: explicit path or latest in benchmarks/
    let json_path = if args.len() > 1 {
        args[1].clone()
    } else {
        find_latest_json("benchmarks").unwrap_or_else(|| {
            eprintln!("No JSON files found in benchmarks/");
            eprintln!("Usage: gen-readme [path/to/benchmark.json]");
            std::process::exit(1);
        })
    };

    eprintln!("Reading: {}", json_path);
    let content = std::fs::read_to_string(&json_path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", json_path, e);
        std::process::exit(1);
    });
    let data: Value = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Error parsing JSON: {}", e);
        std::process::exit(1);
    });

    let mut lines: Vec<String> = Vec::new();

    // Title
    lines.push("# Solidity LSP Benchmarks".to_string());
    lines.push(String::new());
    lines.push("Benchmarks comparing Solidity LSP servers against Uniswap V4-core (`Pool.sol`, 618 lines).".to_string());
    lines.push(String::new());

    // Settings
    if let Some(settings) = data.get("settings") {
        let iterations = settings
            .get("iterations")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let warmup = settings.get("warmup").and_then(|v| v.as_u64()).unwrap_or(0);
        let timeout = settings
            .get("timeout_secs")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        lines.push("## Settings".to_string());
        lines.push(String::new());
        lines.push("| Setting | Value |".to_string());
        lines.push("|---------|-------|".to_string());
        lines.push(format!("| Iterations | {} |", iterations));
        lines.push(format!("| Warmup | {} |", warmup));
        lines.push(format!("| Timeout | {}s |", timeout));
        lines.push(String::new());
    }

    // Servers
    if let Some(servers) = data.get("servers").and_then(|s| s.as_array()) {
        lines.push("## Servers".to_string());
        lines.push(String::new());
        lines.push("| Server | Version |".to_string());
        lines.push("|--------|---------|".to_string());
        for srv in servers {
            let name = srv.get("name").and_then(|n| n.as_str()).unwrap_or("?");
            let version = srv.get("version").and_then(|v| v.as_str()).unwrap_or("?");
            lines.push(format!("| {} | {} |", name, version));
        }
        lines.push(String::new());
    }

    // Results table with medals and trophy
    if let Some(benchmarks) = data.get("benchmarks").and_then(|b| b.as_array()) {
        if !benchmarks.is_empty() {
            // Collect server names from first benchmark
            let server_names: Vec<&str> = benchmarks[0]
                .get("servers")
                .and_then(|s| s.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.get("server").and_then(|n| n.as_str()))
                        .collect()
                })
                .unwrap_or_default();

            let medal_icons = ["\u{1F947}", "\u{1F948}", "\u{1F949}"]; // 🥇🥈🥉

            // Pre-compute medals per benchmark and count wins
            let mut wins: HashMap<String, usize> = HashMap::new();
            let mut all_medals: Vec<Vec<&str>> = Vec::new();

            for bench in benchmarks {
                let servers = bench.get("servers").and_then(|s| s.as_array());
                if let Some(servers) = servers {
                    // Rank valid servers by mean
                    let mut ranked: Vec<(usize, f64)> = servers
                        .iter()
                        .enumerate()
                        .filter(|(_, s)| {
                            let status = s.get("status").and_then(|v| v.as_str()).unwrap_or("");
                            let response = s.get("response").and_then(|v| v.as_str()).unwrap_or("");
                            status == "ok"
                                && response != "null"
                                && response != "no result"
                                && !response.is_empty()
                        })
                        .filter_map(|(i, s)| {
                            s.get("mean_ms").and_then(|v| v.as_f64()).map(|m| (i, m))
                        })
                        .collect();
                    ranked.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    let mut row_medals = vec![""; servers.len()];
                    for (place, (idx, _)) in ranked.iter().enumerate() {
                        if place < medal_icons.len() {
                            row_medals[*idx] = medal_icons[place];
                        }
                        if place == 0 {
                            if let Some(name) = servers[*idx].get("server").and_then(|n| n.as_str())
                            {
                                *wins.entry(name.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                    all_medals.push(row_medals);
                }
            }

            // Find overall winner
            let trophy_winner = wins
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(label, _)| label.clone());

            // Header
            lines.push("## Results".to_string());
            lines.push(String::new());

            let mut header = "| Benchmark |".to_string();
            let mut separator = "|-----------|".to_string();
            for name in &server_names {
                let trophy = if trophy_winner.as_deref() == Some(*name) {
                    " \u{1F3C6}" // 🏆
                } else {
                    ""
                };
                header.push_str(&format!(" {}{} |", name, trophy));
                separator.push_str(&"-".repeat(name.len() + trophy.len() + 2));
                separator.push('|');
            }
            lines.push(header);
            lines.push(separator);

            // Rows
            for (i, bench) in benchmarks.iter().enumerate() {
                let bench_name = bench.get("name").and_then(|n| n.as_str()).unwrap_or("?");
                let mut row = format!("| {} |", bench_name);

                if let Some(servers) = bench.get("servers").and_then(|s| s.as_array()) {
                    for (j, srv) in servers.iter().enumerate() {
                        let status = srv.get("status").and_then(|v| v.as_str()).unwrap_or("");
                        let cell = match status {
                            "ok" => {
                                let mean =
                                    srv.get("mean_ms").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let medal = if i < all_medals.len() && j < all_medals[i].len() {
                                    all_medals[i][j]
                                } else {
                                    ""
                                };
                                let suffix = if medal.is_empty() {
                                    "".to_string()
                                } else {
                                    format!(" {}", medal)
                                };
                                format!(" {:.1}ms{} |", mean, suffix)
                            }
                            "invalid" => {
                                let response =
                                    srv.get("response").and_then(|v| v.as_str()).unwrap_or("");
                                if response.contains("Unknown method")
                                    || response.contains("unsupported")
                                {
                                    " unsupported |".to_string()
                                } else {
                                    " - |".to_string()
                                }
                            }
                            _ => {
                                let error = srv.get("error").and_then(|v| v.as_str()).unwrap_or("");
                                if error.contains("timeout") {
                                    " timeout |".to_string()
                                } else {
                                    " FAIL |".to_string()
                                }
                            }
                        };
                        row.push_str(&cell);
                    }
                }
                lines.push(row);
            }
            lines.push(String::new());
        }
    }

    // Footer
    if let Some(ts) = data.get("timestamp").and_then(|t| t.as_str()) {
        lines.push(format!("*Generated from `{}`*", json_path));
        lines.push(format!("*Benchmark run: {}*", ts));
        lines.push(String::new());
    }

    lines.push("See [DOCS.md](./DOCS.md) for usage and installation.".to_string());
    lines.push(String::new());

    let out = lines.join("\n");
    std::fs::write("README.md", &out).unwrap();
    println!("{}", out);
    eprintln!("  -> README.md");
}

/// Find the most recent .json file in the given directory (non-recursive).
fn find_latest_json(dir: &str) -> Option<String> {
    let path = Path::new(dir);
    if !path.is_dir() {
        return None;
    }
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    entries
        .last()
        .map(|e| e.path().to_string_lossy().to_string())
}
