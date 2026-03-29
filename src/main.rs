//! Factory Agent — Built on OpenDev Framework Crates
//!
//! Demonstrates that OpenDev's modular Rust crates can power non-coding agents.
//! This factory monitoring agent uses the SAME primitives (ToolRegistry, BaseTool,
//! context compaction, safety layers) as the coding agent — just different tools.

mod tools;

use opendev_tools_core::{ToolRegistry, BaseTool, ToolContext};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::tools::{ReadSensorTool, ControlValveTool, AlertTool, ProductionStatusTool};

#[tokio::main]
async fn main() {
    // ─── 1. Build Tool Registry (opendev-tools-core) ─────────────
    // Same registry pattern as OpenDev's coding agent, but with factory tools.
    let registry = ToolRegistry::new();
    registry.register(Arc::new(ReadSensorTool));
    registry.register(Arc::new(ControlValveTool));
    registry.register(Arc::new(AlertTool));
    registry.register(Arc::new(ProductionStatusTool));

    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  🏭 VLS Factory Agent v0.1 — Powered by OpenDev     ║");
    println!("╠══════════════════════════════════════════════════════╣");
    println!("║  Tools registered: 4                                ║");
    println!("║  Safety: Defense-in-depth (5 layers)                ║");
    println!("║  Context: Adaptive compaction enabled               ║");
    println!("║  Memory: Cross-shift playbook                       ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();

    // ─── 2. Create Tool Context (runtime state for execution) ────
    let mut ctx = ToolContext::new("/factory/vls");
    ctx.session_id = Some("shift-2026-03-28-morning".into());

    // ─── 3. Simulate a ReAct Loop Iteration ──────────────────────
    // In the real agent, the LLM would decide which tools to call.
    // Here we simulate what the ReAct executor does.

    println!("── Shift Start: 06:00 ──────────────────────────────────");
    println!();
    println!("👷 Operator (Vietnamese): \"Nhiệt độ dây chuyền 1 bao nhiêu?\"");
    println!("   [→ \"What's the temperature on line 1?\"]");
    println!();

    // ReAct Phase 1: Thinking (no tools)
    println!("🧠 [Thinking Model] Operator asking about line 1 temperature.");
    println!("   Plan: read temp sensor → check if normal → report back.");
    println!();

    // ReAct Phase 2: Action — call read_sensor tool
    println!("🔧 [Action] Calling read_sensor...");
    let mut args = HashMap::new();
    args.insert("sensor_id".into(), json!("temp-line1"));
    args.insert("unit".into(), json!("celsius"));
    
    let result = ReadSensorTool.execute(args, &ctx).await;
    println!("📊 Result: {}", result.output.as_deref().unwrap_or("error"));
    println!();

    // ReAct Phase 3: Agent observes result, decides to investigate
    println!("🧠 [Thinking] 93.2°C is above 90°C threshold. Check coolant flow.");
    println!();

    let mut args2 = HashMap::new();
    args2.insert("sensor_id".into(), json!("flow-line1-coolant"));
    let result2 = ReadSensorTool.execute(args2, &ctx).await;
    println!("🔧 [Action] read_sensor(flow-line1-coolant):");
    println!("📊 Result: {}", result2.output.as_deref().unwrap_or("error"));
    println!();

    // ReAct Phase 4: Root cause → propose valve adjustment
    println!("🧠 [Thinking] Coolant flow 12.3 LPM < 15 LPM minimum.");
    println!("   → Increase v-coolant-1 from 60% to 85%.");
    println!();

    // ─── 4. Safety Layer (opendev-runtime) ───────────────────────
    // The approval system would intercept this — valve change >20% needs approval.
    println!("🔒 [Safety Layer 3: Runtime Approval]");
    println!("   Valve adjustment >20% requires operator confirmation.");
    println!("   ⏳ Awaiting approval...");
    println!();
    println!("👷 Operator: \"Được, điều chỉnh đi.\" [Approved]");
    println!();

    // Execute the valve adjustment
    let mut valve_args = HashMap::new();
    valve_args.insert("valve_id".into(), json!("v-coolant-1"));
    valve_args.insert("action".into(), json!("set_position"));
    valve_args.insert("position".into(), json!(85));
    valve_args.insert("reason".into(), json!("Increase coolant flow to reduce line 1 temp from 93.2°C"));
    
    let valve_result = ControlValveTool.execute(valve_args, &ctx).await;
    println!("🔧 [Action] control_valve:");
    println!("   ✅ {}", valve_result.output.as_deref().unwrap_or("error"));
    println!();

    // Send alert to floor
    let mut alert_args = HashMap::new();
    alert_args.insert("severity".into(), json!("info"));
    alert_args.insert("message".into(), json!("Đã tăng van làm mát dây chuyền 1 lên 85%. Theo dõi nhiệt độ."));
    alert_args.insert("language".into(), json!("vi"));
    
    let alert_result = AlertTool.execute(alert_args, &ctx).await;
    println!("📢 {}", alert_result.output.as_deref().unwrap_or("error"));
    println!();

    // ─── 5. Context Compaction (opendev-context) ─────────────────
    println!("── 4 hours later (10:00) ─────────────────────────────────");
    println!("⚙️  [Context Compactor] Token pressure at 82%");
    println!("   Stage 3: Pruning old tool outputs (>85% threshold)");
    println!("   847 sensor readings → 12 key events");
    println!("   Using compact model for summarization");
    println!();

    // ─── 6. Memory (opendev-memory) ──────────────────────────────
    println!("── Shift End: 18:00 ──────────────────────────────────────");
    println!("📝 [Memory Playbook] New bullets added:");
    println!("   • \"Line 1 coolant valve drifts below 15 LPM in morning shifts\"");
    println!("   • \"Temperature resolved 3x by increasing coolant flow\"");
    println!("   → Next shift: proactively check coolant at start");
    println!();

    // ─── 7. Production Status ────────────────────────────────────
    let mut status_args = HashMap::new();
    status_args.insert("line".into(), json!("line-1"));
    let status = ProductionStatusTool.execute(status_args, &ctx).await;
    println!("📋 End-of-shift status:");
    println!("{}", status.output.as_deref().unwrap_or("error"));
    println!();

    // ─── Summary ─────────────────────────────────────────────────
    println!("═══════════════════════════════════════════════════════════");
    println!("  OpenDev Crates Used in This Factory Agent:");
    println!("  ┌─────────────────────┬────────────────────────────────┐");
    println!("  │ opendev-tools-core  │ Tool traits, registry, dispatch│");
    println!("  │ opendev-runtime     │ Safety/approval, cost tracking │");
    println!("  │ opendev-context     │ Context compaction (shifts)    │");
    println!("  │ opendev-memory      │ Cross-shift playbook/learning │");
    println!("  │ opendev-models      │ Messages, sessions, config     │");
    println!("  │ opendev-mcp         │ SCADA/PLC via MCP servers      │");
    println!("  │ opendev-agents      │ ReAct loop, prompt composition │");
    println!("  └─────────────────────┴────────────────────────────────┘");
    println!("  Same framework, completely different domain. 🏭 ≠ 💻");
    println!("═══════════════════════════════════════════════════════════");
}
