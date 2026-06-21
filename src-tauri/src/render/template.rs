use anyhow::Result;
use handlebars::Handlebars;
use std::collections::HashMap;

/// HTML 渲染引擎
pub struct HtmlRenderer {
    handlebars: Handlebars<'static>,
}

impl HtmlRenderer {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("json", Box::new(json_helper));

        handlebars.register_template_string("single_patent", SINGLE_PATENT_TEMPLATE)
            .map_err(|e| anyhow::anyhow!("注册单篇模板失败: {}", e))?;
        handlebars.register_template_string("multi_patent", MULTI_PATENT_TEMPLATE)
            .map_err(|e| anyhow::anyhow!("注册多篇模板失败: {}", e))?;

        Ok(Self { handlebars })
    }

    pub fn render_single(&self, data: &serde_json::Value) -> Result<String> {
        let html = self.handlebars.render("single_patent", data)
            .map_err(|e| anyhow::anyhow!("渲染单篇 HTML 失败: {}", e))?;
        Ok(html)
    }

    pub fn render_multi(&self, data: &serde_json::Value) -> Result<String> {
        let html = self.handlebars.render("multi_patent", data)
            .map_err(|e| anyhow::anyhow!("渲染多篇 HTML 失败: {}", e))?;
        Ok(html)
    }
}

/// Handlebars helper: 将值序列化为 JSON 字符串
fn json_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(param)?;
    Ok(())
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

    let mut patent_list = Vec::new();
    for (i, patent) in patents.iter().enumerate() {
        let mut patent_data = if let Some(obj) = patent.as_object() {
            obj.clone()
        } else {
            serde_json::Map::new()
        };

        let patent_id = patent_data.get("publicationNumber")
            .or_else(|| patent_data.get("publication_number"))
            .or_else(|| patent_data.get("applicationNumber"))
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
    <div class='header-inner'>
      <div class='logo'>PatentReader</div>
      <h1>专利解读报告</h1>
      <p class='meta'>生成时间：{{generated_at}}</p>
    </div>
  </header>

  {{#each patents}}
  <article class='patent-section' id='patent-{{index}}'>
    <div class='patent-header-bar'>
      <div class='patent-title-area'>
        <span class='patent-index'>#{{index}}</span>
        <h2>{{#if title}}{{title}}{{else}}未识别标题{{/if}}</h2>
      </div>
      <div class='patent-id-area'>
        {{#if publicationNumber}}<span class='patent-id-tag'>{{publicationNumber}}</span>{{/if}}
        {{#if publication_number}}<span class='patent-id-tag'>{{publication_number}}</span>{{/if}}
      </div>
    </div>

    <div class='tab-container'>
      <div class='tab-nav'>
        {{#if modules.M1}}<button class='tab-btn active' data-tab='M1'>M1 基本信息</button>{{/if}}
        {{#if modules.M2}}<button class='tab-btn' data-tab='M2'>M2 法律状态</button>{{/if}}
        {{#if modules.M3}}<button class='tab-btn' data-tab='M3'>M3 同族保护</button>{{/if}}
        {{#if modules.M4}}<button class='tab-btn' data-tab='M4'>M4 一句话概要</button>{{/if}}
        {{#if modules.M5}}<button class='tab-btn' data-tab='M5'>M5 权利要求</button>{{/if}}
        {{#if modules.M6}}<button class='tab-btn' data-tab='M6'>M6 实施例</button>{{/if}}
        {{#if modules.M7}}<button class='tab-btn' data-tab='M7'>M7 替代方案</button>{{/if}}
        {{#if modules.M8}}<button class='tab-btn' data-tab='M8'>M8 同族权要差异</button>{{/if}}
      </div>
      <div class='tab-content'>
        {{#if modules.M1}}<div class='tab-pane active' id='tab-M1'>{{{modules.M1}}}</div>{{/if}}
        {{#if modules.M2}}<div class='tab-pane' id='tab-M2'>{{{modules.M2}}}</div>{{/if}}
        {{#if modules.M3}}<div class='tab-pane' id='tab-M3'>{{{modules.M3}}}</div>{{/if}}
        {{#if modules.M4}}<div class='tab-pane' id='tab-M4'>{{{modules.M4}}}</div>{{/if}}
        {{#if modules.M5}}<div class='tab-pane' id='tab-M5'>{{{modules.M5}}}</div>{{/if}}
        {{#if modules.M6}}<div class='tab-pane' id='tab-M6'>{{{modules.M6}}}</div>{{/if}}
        {{#if modules.M7}}<div class='tab-pane' id='tab-M7'>{{{modules.M7}}}</div>{{/if}}
        {{#if modules.M8}}<div class='tab-pane' id='tab-M8'>{{{modules.M8}}}</div>{{/if}}
      </div>
    </div>
  </article>
  {{/each}}

  <footer class='footer'>
    <p>由 PatentReader 生成 · 仅供参考，不构成法律意见</p>
  </footer>
</div>

<script>
document.querySelectorAll('.tab-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const container = btn.closest('.tab-container');
    container.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    container.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    container.querySelector('#tab-' + btn.dataset.tab).classList.add('active');
  });
});
</script>
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
    <div class='header-inner'>
      <div class='logo'>PatentReader</div>
      <h1>{{theme_name}}</h1>
      {{#if theme_description}}<p class='theme-desc'>{{theme_description}}</p>{{/if}}
      <p class='meta'>共 {{patent_count}} 篇专利 · 生成时间：{{generated_at}}</p>
    </div>
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
    <div class='patent-header-bar'>
      <div class='patent-title-area'>
        <span class='patent-index'>#{{index}}</span>
        <h2>{{#if title}}{{title}}{{else}}未识别标题{{/if}}</h2>
      </div>
      <div class='patent-id-area'>
        {{#if publicationNumber}}<span class='patent-id-tag'>{{publicationNumber}}</span>{{/if}}
        {{#if publication_number}}<span class='patent-id-tag'>{{publication_number}}</span>{{/if}}
      </div>
    </div>

    <div class='tab-container'>
      <div class='tab-nav'>
        {{#if modules.M1}}<button class='tab-btn active' data-tab='M1'>M1 基本信息</button>{{/if}}
        {{#if modules.M4}}<button class='tab-btn' data-tab='M4'>M4 一句话概要</button>{{/if}}
        {{#if modules.M5}}<button class='tab-btn' data-tab='M5'>M5 权利要求</button>{{/if}}
        {{#if modules.M6}}<button class='tab-btn' data-tab='M6'>M6 实施例</button>{{/if}}
        {{#if modules.M3}}<button class='tab-btn' data-tab='M3'>M3 同族保护</button>{{/if}}
      </div>
      <div class='tab-content'>
        {{#if modules.M1}}<div class='tab-pane active' id='tab-M1'>{{{modules.M1}}}</div>{{/if}}
        {{#if modules.M4}}<div class='tab-pane' id='tab-M4'>{{{modules.M4}}}</div>{{/if}}
        {{#if modules.M5}}<div class='tab-pane' id='tab-M5'>{{{modules.M5}}}</div>{{/if}}
        {{#if modules.M6}}<div class='tab-pane' id='tab-M6'>{{{modules.M6}}}</div>{{/if}}
        {{#if modules.M3}}<div class='tab-pane' id='tab-M3'>{{{modules.M3}}}</div>{{/if}}
      </div>
    </div>
  </article>
  {{/each}}

  <footer class='footer'>
    <p>由 PatentReader 生成 · 仅供参考，不构成法律意见</p>
  </footer>
</div>

<script>
document.querySelectorAll('.tab-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    const container = btn.closest('.tab-container');
    container.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    container.querySelectorAll('.tab-pane').forEach(p => p.classList.remove('active'));
    btn.classList.add('active');
    container.querySelector('#tab-' + btn.dataset.tab).classList.add('active');
  });
});
</script>
</body>
</html>"##;
