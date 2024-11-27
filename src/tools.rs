use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: ParameterDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ParameterDefinition {
    #[serde(rename = "type")]
    pub param_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, PropertyDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PropertyDefinition {
    #[serde(rename = "type")]
    pub prop_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToolConfig {
    pub function_declarations: Vec<ToolDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FunctionCallingConfig {
    pub mode: String,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> ParameterDefinition;
    async fn execute(&self, params: Value) -> Result<Value>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn get_tool_definitions(&self) -> ToolConfig {
        ToolConfig {
            function_declarations: self
                .tools
                .values()
                .map(|tool| ToolDefinition {
                    name: tool.name().to_string(),
                    description: tool.description().to_string(),
                    parameters: tool.parameters(),
                })
                .collect(),
        }
    }

    pub async fn execute_tool(&self, name: &str, params: Value) -> Result<Value> {
        self.tools
            .get(name)
            .ok_or_else(|| crate::error::Error::ToolError(format!("Tool {} not found", name)))?
            .execute(params)
            .await
    }
}

// Example tool implementation
pub struct EnableLights;

#[async_trait]
impl Tool for EnableLights {
    fn name(&self) -> &str {
        "enable_lights"
    }

    fn description(&self) -> &str {
        "Turn on the lighting system."
    }

    fn parameters(&self) -> ParameterDefinition {
        ParameterDefinition {
            param_type: "object".to_string(),
            properties: None,
            required: None,
        }
    }

    async fn execute(&self, _params: Value) -> Result<Value> {
        // Implementation would go here
        Ok(Value::Bool(true))
    }
}

pub struct SetLightColor;

#[async_trait]
impl Tool for SetLightColor {
    fn name(&self) -> &str {
        "set_light_color"
    }

    fn description(&self) -> &str {
        "Set the light color. Lights must be enabled for this to work."
    }

    fn parameters(&self) -> ParameterDefinition {
        ParameterDefinition {
            param_type: "object".to_string(),
            properties: Some({
                let mut map = HashMap::new();
                map.insert(
                    "rgb_hex".to_string(),
                    PropertyDefinition {
                        prop_type: "string".to_string(),
                        description: "The light color as a 6-digit hex string, e.g. ff0000 for red."
                            .to_string(),
                    },
                );
                map
            }),
            required: Some(vec!["rgb_hex".to_string()]),
        }
    }

    async fn execute(&self, params: Value) -> Result<Value> {
        // Implementation would go here
        Ok(params)
    }
}
