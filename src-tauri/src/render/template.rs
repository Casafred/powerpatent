use anyhow::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTML 渲染引擎
pub struct HtmlRenderer {
    handlebars: Handlebars<'static>,
}

impl HtmlRenderer {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();

        // 注册内嵌模板
        handlebars.register_template_string("single_patent", SINGLE_PATENT_TEMPLATE)
            .map_err(|e| anyhow::anyhow!("注册单篇模板失败: {}", e))?;
        handlebars.register_template_string("multi_patent", MULTI_PATENT_TEMPLATE)
            .map_err(|e| anyhow::anyhow!("注册多篇模板失败: {}", e))?;
        handlebars.register_template_string("module_basic_info", MODULE_BASIC_INFO)
            .map_err(|e| anyhow::anyhow!("注册基本信息模板失败: {}", e))?;
        handlebars.register_template_string("module_legal_status", MODULE_LEGAL_STATUS)
            .map_err(|e| anyhow::anyhow!("注册法律状态模板失败: {}", e))?;
        handlebars.register_template_string("module_summary", MODULE_SUMMARY)
            .map_err(|e| anyhow::anyhow!("注册概要模板失败: {}", e))?;
        handlebars.register_template_string("module_claims", MODULE_CLAIMS)
            .map_err(|e| anyhow::anyhow!("注册权要模板失败: {}", e))?;
        handlebars.register_template_string("module_embodiments", MODULE_EMBODIMENTS)
            .map_err(|e| anyhow::anyhow!("注册实施例模板失败: {}", e))?;
        handlebars.register_template_string("module_alternatives", MODULE_ALTERNATIVES)
            .map_err(|e| anyhow::anyhow!("注册替代方案模板失败: {}", e))?;
        handlebars.register_template_string("module_family", MODULE_FAMILY)
            .map_err(|e| anyhow::anyhow!("注册同族模板失败: {}", e))?;

        Ok(Self { handlebars })
    }

    /// 渲染单篇专利 HTML
    pub fn render_single(&self, data: &serde_json::Value) -> Result<String> {
        let html = self.handlebars.render("single_patent", data)
            .map_err(|e| anyhow::anyhow!("渲染单篇 HTML 失败: {}", e))?;
        Ok(html)
    }

    /// 渲染多篇专利 HTML
    pub fn render_multi(&self, data: &serde_json::Value) -> Result<String> {
        let html = self.handlebars.render("multi_patent", data)
            .map_err(|e| anyhow::anyhow!("渲染多篇 HTML 失败: {}", e))?;
        Ok(html)
    }

    /// 渲染单个板块
    pub fn render_module(&self, module_id: &str, data: &serde_json::Value) -> Result<String> {
        let template_name = match module_id {
            "M1" => "module_basic_info",
            "M2" => "module_legal_status",
            "M3" => "module_family",
            "M4" => "module_summary",
            "M5" => "module_claims",
            "M6" => "module_embodiments",
            "M7" => "module_alternatives",
            _ => return Ok(format!("<div class='module-placeholder'>板块 {} 待实现</div>", module_id)),
        };
        self.handlebars.render(template_name, data)
            .map_err(|e| anyhow::anyhow!("渲染板块 {} 失败: {}", module_id, e))
    }
}

/// 构建渲染数据
pub fn build_render_data(
    patents: &[serde_json::Value],
    modules: &HashMap<String, serde_json::Value>,
    mode: &str,
    theme_name: Option<&str>,
    theme_description: Option<&str>,
) -> serde_json::Value {
    let mut data = serde_json::Map::new();

    data.insert("mode".to_string(), serde_json::Value::String(mode.to_string()));
    data.insert("theme_name".to_string(), serde_json::Value::String(theme_name.unwrap_or("").to_string()));
    data.insert("theme_description".to_string(), serde_json::Value::String(theme_description.unwrap_or("").to_string()));
    data.insert("generated_at".to_string(), serde_json::Value::String(
        chrono::Local::now().format("%Y-%m-%d %H:%M").to_string()
    ));

    // 专利列表
    let mut patent_list = Vec::new();
    for (i, patent) in patents.iter().enumerate() {
        let mut patent_data = if let Some(obj) = patent.as_object() {
            obj.clone()
        } else {
            serde_json::Map::new()
        };

        // 注入板块输出
        let patent_id = patent_data.get("publication_number")
            .or_else(|| patent_data.get("application_number"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let mut module_outputs = serde_json::Map::new();
        for (key, value) in modules {
            if key.starts_with(&format!("{}_", patent_id)) {
                let module_id = key.split('_').last().unwrap_or("");
                module_outputs.insert(module_id.to_string(), value.clone());
            }
        }
        patent_data.insert("modules".to_string(), serde_json::Value::Object(module_outputs));
        patent_data.insert("index".to_string(), serde_json::Value::Number((i + 1).into()));

        patent_list.push(serde_json::Value::Object(patent_data));
    }

    data.insert("patents".to_string(), serde_json::Value::Array(patent_list));
    data.insert("patent_count".to_string(), serde_json::Value::Number(patents.len().into()));

    serde_json::Value::Object(data)
}

// ============ 内嵌 Handlebars 模板 ============

const SINGLE_PATENT_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang='zh-CN'>
<head>
<meta charset='UTF-8'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
<title>{{patents.0.title}} - 专利解读</title>
<style>
{{{inline_css}}}
</style>
</head>
<body>
<div class='container'>
  <header class='header'>
    <h1>专利解读报告</h1>
    <p class='meta'>生成时间：{{generated_at}}</p>
  </header>

  {{#each patents}}
  <article class='patent-section' id='patent-{{index}}'>
    {{#if modules.M1}}
    <section class='module' id='M1'>
      <h2>M1 · 专利基本信息</h2>
      {{{modules.M1}}}
    </section>
    {{/if}}

    {{#if modules.M2}}
    <section class='module' id='M2'>
      <h2>M2 · 法律状态与关键日期</h2>
      {{{modules.M2}}}
    </section>
    {{/if}}

    {{#if modules.M3}}
    <section class='module' id='M3'>
      <h2>M3 · 同族保护情况</h2>
      {{{modules.M3}}}
    </section>
    {{/if}}

    {{#if modules.M4}}
    <section class='module' id='M4'>
      <h2>M4 · 一句话概要</h2>
      {{{modules.M4}}}
    </section>
    {{/if}}

    {{#if modules.M5}}
    <section class='module' id='M5'>
      <h2>M5 · 权利要求范围解读</h2>
      {{{modules.M5}}}
    </section>
    {{/if}}

    {{#if modules.M6}}
    <section class='module' id='M6'>
      <h2>M6 · 实施例归纳</h2>
      {{{modules.M6}}}
    </section>
    {{/if}}

    {{#if modules.M7}}
    <section class='module' id='M7'>
      <h2>M7 · 其他揭示方案</h2>
      {{{modules.M7}}}
    </section>
    {{/if}}
  </article>
  {{/each}}

  <footer class='footer'>
    <p>由 PatentReader 生成 · 仅供参考，不构成法律意见</p>
  </footer>
</div>
</body>
</html>"##;

const MULTI_PATENT_TEMPLATE: &str = r##"<!DOCTYPE html>
<html lang='zh-CN'>
<head>
<meta charset='UTF-8'>
<meta name='viewport' content='width=device-width, initial-scale=1.0'>
<title>{{theme_name}} - 专利解读</title>
<style>
{{{inline_css}}}
</style>
</head>
<body>
<div class='container'>
  <header class='header'>
    <h1>{{theme_name}}</h1>
    {{#if theme_description}}<p class='theme-desc'>{{theme_description}}</p>{{/if}}
    <p class='meta'>共 {{patent_count}} 篇专利 · 生成时间：{{generated_at}}</p>
  </header>

  <nav class='toc'>
    <h3>目录</h3>
    <ol>
    {{#each patents}}
      <li><a href='#patent-{{index}}'>{{#if title}}{{title}}{{else}}专利 {{index}}{{/if}}</a></li>
    {{/each}}
    </ol>
  </nav>

  {{#each patents}}
  <article class='patent-section' id='patent-{{index}}'>
    <h2>专利 {{index}}：{{#if title}}{{title}}{{else}}未识别标题{{/if}}</h2>

    {{#if modules.M1}}
    <section class='module' id='M1-{{index}}'>
      <h3>M1 · 专利基本信息</h3>
      {{{modules.M1}}}
    </section>
    {{/if}}

    {{#if modules.M4}}
    <section class='module' id='M4-{{index}}'>
      <h3>M4 · 一句话概要</h3>
      {{{modules.M4}}}
    </section>
    {{/if}}

    {{#if modules.M5}}
    <section class='module' id='M5-{{index}}'>
      <h3>M5 · 权利要求范围解读</h3>
      {{{modules.M5}}}
    </section>
    {{/if}}

    {{#if modules.M6}}
    <section class='module' id='M6-{{index}}'>
      <h3>M6 · 实施例归纳</h3>
      {{{modules.M6}}}
    </section>
    {{/if}}
  </article>
  {{/each}}

  <footer class='footer'>
    <p>由 PatentReader 生成 · 仅供参考，不构成法律意见</p>
  </footer>
</div>
</body>
</html>"##;

// ============ 板块子模板 ============

const MODULE_BASIC_INFO: &str = r#"<table class="info-table">
{{#if publication_number}}<tr><th>公开号</th><td>{{publication_number}}</td></tr>{{/if}}
{{#if grant_number}}<tr><th>授权号</th><td>{{grant_number}}</td></tr>{{/if}}
{{#if application_number}}<tr><th>申请号</th><td>{{application_number}}</td></tr>{{/if}}
{{#if applicant}}<tr><th>申请人</th><td>{{applicant}}</td></tr>{{/if}}
{{#if inventor}}<tr><th>发明人</th><td>{{inventor}}</td></tr>{{/if}}
{{#if filing_date}}<tr><th>申请日</th><td>{{filing_date}}</td></tr>{{/if}}
{{#if ipc}}<tr><th>IPC</th><td>{{ipc}}</td></tr>{{/if}}
{{#if cpc}}<tr><th>CPC</th><td>{{cpc}}</td></tr>{{/if}}
</table>"#;

const MODULE_LEGAL_STATUS: &str = r#"<div class="legal-status">
{{#if legal_status}}<p><strong>法律状态：</strong>{{legal_status}}</p>{{/if}}
{{#if filing_date}}<p><strong>申请日：</strong>{{filing_date}}</p>{{/if}}
{{#if publication_date}}<p><strong>公开日：</strong>{{publication_date}}</p>{{/if}}
{{#if grant_date}}<p><strong>授权日：</strong>{{grant_date}}</p>{{/if}}
{{#if priority_date}}<p><strong>优先权日：</strong>{{priority_date}}</p>{{/if}}
</div>"#;

const MODULE_SUMMARY: &str = r#"<div class="summary-card">
{{#if output.technical_problem}}<div class="summary-item"><strong>技术问题</strong><p>{{output.technical_problem}}</p></div>{{/if}}
{{#if output.technical_means}}<div class="summary-item"><strong>技术手段</strong><p>{{output.technical_means}}</p></div>{{/if}}
{{#if output.technical_effect}}<div class="summary-item"><strong>技术效果</strong><p>{{output.technical_effect}}</p></div>{{/if}}
{{#if output.one_line_summary}}<div class="summary-one-line">{{output.one_line_summary}}</div>{{/if}}
</div>"#;

const MODULE_CLAIMS: &str = r#"<div class="claims-analysis">
{{#if output.independent_claims}}
<h4>独立权利要求</h4>
{{#each output.independent_claims}}
<div class="claim-item">
  <div class="claim-header">权利要求 {{claim_number}}</div>
  <p class="claim-text">{{claim_text}}</p>
  <div class="claim-features">
    <strong>必要技术特征：</strong>
    <ul>{{#each core_features}}<li>{{this}}</li>{{/each}}</ul>
  </div>
  <p class="claim-scope">{{scope_summary}}</p>
</div>
{{/each}}
{{/if}}
{{#if output.dependent_claims}}
<h4>从属权利要求</h4>
{{#each output.dependent_claims}}
<div class="claim-item dependent">
  <span>权利要求 {{claim_number}}（引用 {{depends_on}}）</span>
  <p>{{additional_limitation}}</p>
  <p class="scope-narrowing">{{scope_narrowing}}</p>
</div>
{{/each}}
{{/if}}
</div>"#;

const MODULE_EMBODIMENTS: &str = r#"<div class="embodiments">
{{#if output.embodiments}}
{{#each output.embodiments}}
<div class="embodiment-card">
  <h4>{{name}}</h4>
  <p>{{solution}}</p>
  {{#if key_parameters}}
  <table class="param-table">
    <tr><th>参数</th><th>值</th></tr>
    {{#each key_parameters}}<tr><td>{{name}}</td><td>{{value}}</td></tr>{{/each}}
  </table>
  {{/if}}
  {{#if advantages}}<p class="advantage">{{advantages}}</p>{{/if}}
</div>
{{/each}}
{{/if}}
</div>"#;

const MODULE_ALTERNATIVES: &str = r#"<div class="alternatives">
{{#if output.alternatives}}
{{#each output.alternatives}}
<div class="alternative-card">
  <p>{{description}}</p>
  {{#if related_claims}}<p class="related">相关权利要求：{{#each related_claims}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}</p>{{/if}}
  {{#if potential_scope}}<p class="scope">{{potential_scope}}</p>{{/if}}
</div>
{{/each}}
{{/if}}
</div>"#;

const MODULE_FAMILY: &str = r#"<div class="family-overview">
{{#if output.family_overview}}<p>{{output.family_overview}}</p>{{/if}}
{{#if output.key_jurisdictions}}
<table class="info-table">
  <tr><th>国家/地区</th><th>状态</th><th>范围差异</th></tr>
  {{#each output.key_jurisdictions}}
  <tr><td>{{country}}</td><td>{{status}}</td><td>{{scope_difference}}</td></tr>
  {{/each}}
</table>
{{/if}}
</div>"#;
