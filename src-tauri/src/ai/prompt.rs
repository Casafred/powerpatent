use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Prompt 模板定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default = "default_model_hint")]
    pub model_hint: String,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default)]
    pub input_fields: Vec<String>,
    #[serde(default)]
    pub output_schema: Option<serde_json::Value>,
    pub prompt_template: String,
}

fn default_model_hint() -> String { "strong".to_string() }
fn default_temperature() -> f32 { 0.3 }

/// Prompt 模板管理器
pub struct PromptManager {
    templates: HashMap<String, PromptTemplate>,
    handlebars: Handlebars<'static>,
}

impl PromptManager {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            handlebars: Handlebars::new(),
        }
    }

    /// 从 prompts 目录加载所有 YAML 模板
    pub fn load_from_dir(dir: &str) -> Result<Self> {
        let mut manager = Self::new();
        let prompts_dir = PathBuf::from(dir);

        if !prompts_dir.exists() {
            log::warn!("Prompt 目录不存在: {}", dir);
            return Ok(manager);
        }

        let entries = std::fs::read_dir(dir)
            .map_err(|e| anyhow::anyhow!("读取 Prompt 目录失败: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "yaml" || e == "yml").unwrap_or(false) {
                match Self::load_template(&path) {
                    Ok(template) => {
                        log::info!("加载 Prompt 模板: {} ({})", template.id, template.name);
                        manager.handlebars.register_template_string(&template.id, &template.prompt_template)
                            .map_err(|e| anyhow::anyhow!("注册模板 {} 失败: {}", template.id, e))?;
                        manager.templates.insert(template.id.clone(), template);
                    }
                    Err(e) => {
                        log::error!("加载 Prompt 模板失败 {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(manager)
    }

    fn load_template(path: &PathBuf) -> Result<PromptTemplate> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("读取文件失败: {}", e))?;
        let template: PromptTemplate = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("解析 YAML 失败: {}", e))?;
        Ok(template)
    }

    /// 获取模板
    pub fn get(&self, id: &str) -> Option<&PromptTemplate> {
        self.templates.get(id)
    }

    /// 渲染 Prompt（将变量注入模板）
    pub fn render(&self, template_id: &str, data: &HashMap<String, String>) -> Result<String> {
        let rendered = self.handlebars.render(template_id, data)
            .map_err(|e| anyhow::anyhow!("渲染模板 {} 失败: {}", template_id, e))?;
        Ok(rendered)
    }

    /// 获取所有模板 ID
    pub fn template_ids(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    /// 获取模板的建议温度
    pub fn get_temperature(&self, template_id: &str) -> f32 {
        self.templates.get(template_id)
            .map(|t| t.temperature)
            .unwrap_or(0.3)
    }

    /// 获取模板的 model_hint
    pub fn get_model_hint(&self, template_id: &str) -> &str {
        self.templates.get(template_id)
            .map(|t| t.model_hint.as_str())
            .unwrap_or("strong")
    }
}
