# 🏭 Factory Agent — Built on OpenDev Framework

A proof-of-concept factory monitoring AI agent demonstrating that [OpenDev](https://github.com/anthropics/opendev)'s modular Rust crates can power **non-coding agents** in any domain.

**Binary size: 997KB** — a full agent runtime under 1MB. That's Rust.

## What This Proves

OpenDev's architecture isn't coupled to software development. The same primitives that power a coding agent work identically for factory automation:

| OpenDev Crate | Coding Agent | Factory Agent |
|---|---|---|
| `opendev-tools-core` | File read/write, shell exec | Sensor reads, valve control |
| `opendev-runtime` | Sandbox permissions | Safety approval (valve >20%) |
| `opendev-context` | Long coding sessions | 12-hour shift compaction |
| `opendev-memory` | Project knowledge | Cross-shift playbooks |
| `opendev-agents` | ReAct code loop | ReAct diagnostic loop |

**The `BaseTool` trait doesn't care if you're reading a file or a PLC register.**

## The Scenario

A Vietnamese factory operator asks about line temperatures. The agent:

1. **Reads sensors** → detects temperature above threshold (93.2°C > 90°C)
2. **Investigates** → checks coolant flow, finds it low (12.3 LPM < 15 LPM minimum)
3. **Proposes fix** → increase coolant valve from 60% → 85%
4. **Safety layer intercepts** → change >20% requires operator approval
5. **Executes** → adjusts valve after Vietnamese-language confirmation
6. **Compacts context** → after 4 hours, 847 readings → 12 key events
7. **Learns** → adds pattern to shift playbook for next crew

All multilingual. All under 1MB.

## Tools

| Tool | Description |
|---|---|
| `read_sensor` | Read factory sensors (temp, flow, pH, pressure) |
| `control_valve` | Adjust valve positions with safety checks |
| `send_alert` | Multilingual alerts to floor operators |
| `production_status` | Line status, output, efficiency metrics |

## Running

```bash
cargo run
```

## Architecture

```
┌─────────────────────────────────────────────┐
│              Factory Agent                   │
│                                              │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐ │
│  │ Sensors  │  │  Valves  │  │  Alerts   │ │
│  └────┬─────┘  └────┬─────┘  └─────┬─────┘ │
│       │              │              │        │
│  ┌────┴──────────────┴──────────────┴─────┐ │
│  │         opendev-tools-core             │ │
│  │    (BaseTool trait + ToolRegistry)      │ │
│  └────────────────┬───────────────────────┘ │
│                   │                          │
│  ┌────────────────┴───────────────────────┐ │
│  │     opendev-runtime (safety/approval)  │ │
│  │     opendev-context (shift compaction) │ │
│  │     opendev-memory (shift playbooks)   │ │
│  │     opendev-agents (ReAct executor)    │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

## The Framework Play

These crates just need to be published to [crates.io](https://crates.io/) independently:

```toml
[dependencies]
opendev-tools-core = "0.1"
opendev-agents = "0.1"
```

Then anyone can `cargo add` and build domain-specific agents — medical, logistics, agriculture — all on the same battle-tested runtime.

## License

MIT
