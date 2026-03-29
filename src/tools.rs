//! Factory-specific tools built on OpenDev's BaseTool trait.

use async_trait::async_trait;
use opendev_tools_core::traits::{BaseTool, ToolContext, ToolResult};
use serde_json::{json, Value};
use std::collections::HashMap;

// ── Read Sensor ──────────────────────────────────────────────────

#[derive(Debug)]
pub struct ReadSensorTool;

#[async_trait]
impl BaseTool for ReadSensorTool {
    fn name(&self) -> &str { "read_sensor" }
    fn description(&self) -> &str { "Read current value from a factory sensor" }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "sensor_id": { "type": "string" },
                "unit": { "type": "string", "enum": ["celsius","psi","lpm","ph"] }
            },
            "required": ["sensor_id"]
        })
    }

    async fn execute(&self, args: HashMap<String, Value>, _ctx: &ToolContext) -> ToolResult {
        let sensor_id = args.get("sensor_id").and_then(|v| v.as_str()).unwrap_or("?");
        let reading = match sensor_id {
            "temp-line1" => json!({
                "sensor_id": "temp-line1", "value": 93.2, "unit": "celsius",
                "normal_range": [80.0, 90.0], "status": "above_normal",
                "trend": "+2.1°C/30min"
            }),
            "flow-line1-coolant" => json!({
                "sensor_id": "flow-line1-coolant", "value": 12.3, "unit": "lpm",
                "normal_range": [15.0, 20.0], "status": "below_normal"
            }),
            _ => json!({ "sensor_id": sensor_id, "value": 42.0, "status": "normal" }),
        };
        ToolResult::ok(serde_json::to_string_pretty(&reading).unwrap())
    }
}

// ── Control Valve ────────────────────────────────────────────────

#[derive(Debug)]
pub struct ControlValveTool;

#[async_trait]
impl BaseTool for ControlValveTool {
    fn name(&self) -> &str { "control_valve" }
    fn description(&self) -> &str { "Adjust valve position. Critical valves require approval." }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "valve_id": { "type": "string" },
                "action": { "type": "string", "enum": ["open","close","set_position"] },
                "position": { "type": "number" },
                "reason": { "type": "string" }
            },
            "required": ["valve_id", "action", "reason"]
        })
    }

    async fn execute(&self, args: HashMap<String, Value>, _ctx: &ToolContext) -> ToolResult {
        let valve = args.get("valve_id").and_then(|v| v.as_str()).unwrap_or("?");
        let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("?");
        let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("");
        let pos = args.get("position").and_then(|v| v.as_f64());
        
        let msg = match action {
            "set_position" => format!("✅ Valve {} → {}%. Reason: {}", valve, pos.unwrap_or(0.0), reason),
            "open" => format!("✅ Valve {} opened. Reason: {}", valve, reason),
            "close" => format!("✅ Valve {} closed. Reason: {}", valve, reason),
            _ => format!("❌ Unknown action: {}", action),
        };
        ToolResult::ok(msg)
    }
}

// ── Alert ────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct AlertTool;

#[async_trait]
impl BaseTool for AlertTool {
    fn name(&self) -> &str { "send_alert" }
    fn description(&self) -> &str { "Send alert to operators" }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "severity": { "type": "string", "enum": ["info","warning","critical","emergency"] },
                "message": { "type": "string" },
                "language": { "type": "string", "enum": ["en","vi","zh"] }
            },
            "required": ["severity", "message"]
        })
    }

    async fn execute(&self, args: HashMap<String, Value>, _ctx: &ToolContext) -> ToolResult {
        let sev = args.get("severity").and_then(|v| v.as_str()).unwrap_or("info");
        let msg = args.get("message").and_then(|v| v.as_str()).unwrap_or("");
        let lang = args.get("language").and_then(|v| v.as_str()).unwrap_or("en");
        let icon = match sev { "emergency" => "🚨", "critical" => "⛔", "warning" => "⚠️", _ => "ℹ️" };
        ToolResult::ok(format!("{} [{}][{}] {}", icon, sev.to_uppercase(), lang, msg))
    }
}

// ── Production Status ────────────────────────────────────────────

#[derive(Debug)]
pub struct ProductionStatusTool;

#[async_trait]
impl BaseTool for ProductionStatusTool {
    fn name(&self) -> &str { "production_status" }
    fn description(&self) -> &str { "Get production line status" }

    fn parameter_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": { "line": { "type": "string" } },
            "required": ["line"]
        })
    }

    async fn execute(&self, args: HashMap<String, Value>, _ctx: &ToolContext) -> ToolResult {
        let status = json!({
            "line": args.get("line").and_then(|v| v.as_str()).unwrap_or("all"),
            "status": "running", "uptime_hours": 14.5,
            "output_kg": 2340, "target_kg": 3000, "efficiency": "78%",
            "active_alerts": 1, "operators_on_shift": 3
        });
        ToolResult::ok(serde_json::to_string_pretty(&status).unwrap())
    }
}
