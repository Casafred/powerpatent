# PatentReader · 专利解读生成器 — 开发规范文档

> 版本：v1.1 | 日期：2026-06-19 | 作者：Alfred Shi
> v1.1 更新：细化附图智能标注功能（版面识别 + 标号 OCR + 说明书关联 + SVG 自动标注）

---

## 一、项目概述

### 1.1 定位

面向"专利同事 → 研发同事"单向分发的专利解读生成工具。专利同事通过桌面应用输入专利材料（PDF/表格），AI 自动生成结构化解读，最终导出为离线自包含的 HTML 文件，研发打开即用、可标注、可分享。

### 1.2 核心原则

| 原则 | 说明 |
|------|------|
| 离线自包含 | 输出 HTML 单文件，CSS/JS/图/PDF 全 base64 内嵌，研发无需联网 |
| 模块化组合 | 必要板块恒定，拓展板块可勾选；Full/Lite/Off 三级 |
| AI 可溯源 | 每段 AI 解读标注来源（段落号/图号/权要号）+ 置信度 |
| 板块级重跑 | 输入固定，AI 输出可按板块单独重跑，满意后导出 |
| 输入容错 | 表格/PDF/混合输入均支持，缺字段标记"⚠️ 输入缺失" |

### 1.3 参考项目

| 项目 | 技术栈 | 参考要点 |
|------|--------|----------|
| **patent2pic** | Tauri 2.0 + Vue 3 + TS + Element Plus + AntV X6 | Tauri 打包配置、Vue 组件架构、AI 服务集成模式、NSIS 安装包 |
| **history-helper** | Electron + 原生 HTML/JS + Tauri 备选 | AI 多 provider SSE 流式调用、OCR 双引擎（PaddleOCR-VL/GLM）、PDF 处理、SQLite 缓存、API 代理模式 |

---

## 二、技术架构

### 2.1 整体架构图

```
┌──────────────────────────────────────────────────────────────┐
│                    Tauri 2.0 桌面应用                         │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │  前端 (Vue 3 + TypeScript + Element Plus)              │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │  │
│  │  │ 输入配置  │ │ 板块勾选  │ │ AI缓存管理│ │ 预览导出  │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │  │
│  │  ┌──────────────────────────────────────────────────┐  │  │
│  │  │  HTML 模板引擎 (Handlebars 预编译)                │  │  │
│  │  │  single.hbs / multi.hbs / partials/*             │  │  │
│  │  └──────────────────────────────────────────────────┘  │  │
│  └──────────────────────────┬─────────────────────────────┘  │
│                             │ Tauri IPC (invoke)              │
│  ┌──────────────────────────▼─────────────────────────────┐  │
│  │  Rust 后端                                              │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │  │
│  │  │ PDF 抽取  │ │ 表格解析  │ │ AI 调用   │ │ HTML 渲染 │  │  │
│  │  │ pdf-extract│ │ calamine │ │ reqwest   │ │ handlebars│  │  │
│  │  │ mupdf     │ │ xlsx     │ │ SSE流式   │ │ minify    │  │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐               │  │
│  │  │ OCR 模块  │ │ 缓存模块  │ │ 字段映射  │               │  │
│  │  │ paddleocr │ │ SQLite   │ │ 智能识别  │               │  │
│  │  │ glm-ocr   │ │ JSON     │ │          │               │  │
│  │  └──────────┘ └──────────┘ └──────────┘               │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
            │
            ▼
    独立 .html 文件（离线自包含）
```

### 2.2 技术栈选型

| 层级 | 技术 | 选型理由 | 参考来源 |
|------|------|----------|----------|
| 桌面框架 | **Tauri 2.0** | 体积 ~10-20MB，Rust 原生 PDF/OCR 处理高效，与 patent2pic 一致 | patent2pic |
| 前端框架 | **Vue 3 + TypeScript** | 与 patent2pic 一致，组件化开发板块配置面板 | patent2pic |
| UI 组件 | **Element Plus** | 与 patent2pic 一致，中文友好 | patent2pic |
| 状态管理 | **Pinia** | 与 patent2pic 一致 | patent2pic |
| 构建工具 | **Vite** | 与 patent2pic 一致 | patent2pic |
| PDF 文本抽取 | **pdf-extract** (Rust crate) | Rust 原生，无需 Python 依赖 | 新选型 |
| PDF 图像抽取 | **mupdf** (Rust crate) | 高质量渲染 PDF 页面为图片 | 新选型 |
| 表格解析 | **calamine** (Rust crate) | 纯 Rust，支持 xlsx/csv，无需 Excel 环境 | 新选型 |
| AI 调用 | **reqwest** + SSE 流式 | Rust HTTP 客户端，SSE 流式与 history-helper 一致 | history-helper |
| OCR | **PaddleOCR-VL API** + **GLM OCR API** | 双引擎，与 history-helper 一致 | history-helper |
| HTML 渲染 | **Handlebars** (Rust) | 模板与数据分离，板块 partial 可组合 | 新选型 |
| 缓存 | **SQLite** (via rusqlite) | AI 输出缓存 + 板块重跑，与 history-helper Tauri 后端一致 | history-helper |
| 输出 PDF Viewer | **pdf.js** (内嵌到输出 HTML) | 与 history-helper 一致 | history-helper |
| 安装包 | **NSIS** | 与 patent2pic/history-helper 一致 | 两者 |

### 2.3 为什么选 Tauri 而非 Electron

| 维度 | Tauri 2.0 | Electron |
|------|-----------|----------|
| 安装包体积 | ~10-20MB | ~80-120MB |
| PDF/OCR 处理 | Rust 原生 crate，无需 Python | 需 Python 子进程或 Node.js 库 |
| AI 调用 | Rust reqwest，高性能 | Node.js http，可接受 |
| 与现有项目一致性 | patent2pic 已用 Tauri | history-helper 已用 Electron |
| **结论** | **主选**，PDF/OCR 处理链更优 | 备选 |

> 注：history-helper 同时维护 Electron + Tauri 双打包，本项目仅用 Tauri，简化维护。

---

## 三、功能模块详细设计

### 3.1 输入层

#### 3.1.1 支持的输入形态

| 输入形态 | 处理方式 | 优先级 |
|----------|----------|--------|
| **PDF 原文** | Rust 侧 pdf-extract 抽文本 + mupdf 抽图 → AI 全流程生成 | 基础输入 |
| **结构化表格**（CSV/XLSX） | calamine 解析 → 字段映射 → 直接填充，AI 仅做解读类生成 | 补充输入 |
| **混合输入**（表格 + PDF） | 表格字段优先，PDF 补全缺失项并提取附图 | 最优输入 |

#### 3.1.2 字段映射规范

表格列名自动识别 + 手动映射，兼容中英文常见列名：

```typescript
interface FieldMapping {
  // 著录信息
  publicationNumber: string;   // 公开号/PN/Publication Number
  grantNumber: string;         // 授权号/Grant Number
  applicationNumber: string;   // 申请号/AN/Application Number
  applicant: string;           // 申请人/Assignee/Applicant
  inventor: string;            // 发明人/Inventor
  filingDate: string;          // 申请日/Filing Date
  priorityDate: string;        // 优先权日/Priority Date
  publicationDate: string;     // 公开日/Publication Date
  grantDate: string;           // 授权日/Grant Date
  legalStatus: string;         // 法律状态/Legal Status
  ipc: string;                 // IPC/IPC分类号
  cpc: string;                 // CPC/CPC分类号
  familyMembers: string;       // 同族/Family/Family Members
  // 文本内容
  claims: string;              // 权利要求/Claims
  description: string;         // 说明书/Description/Specification
  abstract: string;            // 摘要/Abstract
  title: string;               // 标题/Title/发明名称
}
```

缺失字段 → 对应模块内显示"⚠️ 输入缺失"，AI 不编造。

#### 3.1.3 字段映射 UI 流程

1. 用户上传表格 → 自动扫描列头 → 匹配已知列名
2. 展示映射预览表：每行一个字段，下拉选择对应列
3. 已匹配字段标绿 ✅，未匹配标黄 ⚠️，缺失标红 ❌
4. 用户可手动调整映射 → 确认

### 3.2 板块体系

#### 3.2.1 必要板块（Required，恒定输出）

| 编号 | 板块 | 单篇模式 | 多篇模式 | AI 生成 |
|------|------|----------|----------|---------|
| M1 | 专利基本信息 | Full | Full | 否（直接填充） |
| M2 | 法律状态与关键日期 | Full | Full | 否（直接填充） |
| M3 | 同族保护情况 | Full | Full | 部分（同族主题概要需 AI） |
| M4 | 一句话概要 | Full | Full | 是 |
| M5 | 权利要求范围解读 | Full | Lite（仅独权） | 是 |
| M6 | 实施例归纳 | Full | Lite（仅首例） | 是 |
| M7 | 其他揭示方案 | Full | Lite | 是 |

#### 3.2.2 拓展板块（Extended，可勾选）

| 编号 | 板块 | 适用模式 | AI 生成 | 说明 |
|------|------|----------|---------|------|
| E1 | PDF 原文浏览 | 单篇/多篇 | 否 | pdf.js 内嵌，可选是否内嵌（体积权衡） |
| E2 | 附图对照 | 单篇/多篇 | 部分（图示说明需 AI） | 图库 + 双向跳转 + 框选标注 |
| E3 | 批注 | 单篇/多篇 | 否 | localStorage + JSON 导入导出 |
| E4 | 多专利对比矩阵 | 仅多篇 | 是 | 横向维度对比 |
| E5 | 技术演进时间线 | 仅多篇 | 是 | 申请人同主题演进 |
| E6 | 申请人画像 | 单篇/多篇 | 是 | 同领域布局概览 |
| E7 | 规避/设计空间提示 | 单篇/多篇 | 是 | 标注"仅供参考" |
| E8 | 引用关系网络 | 单篇/多篇 | 否（需外部数据） | 前后向引用可视化 |

#### 3.2.3 板块级别

| 级别 | 说明 | 适用板块 |
|------|------|----------|
| **Full** | 完整输出，所有细节 | M5（全部权要树+解读）、M6（全部实施例详述） |
| **Lite** | 精简输出，仅核心 | M5（仅独权概要）、M6（仅首例）、M7（列表无详述） |
| **Off** | 不输出 | 仅拓展板块可关闭，必要板块锁定 |

### 3.3 单篇模式 vs 多篇模式

#### 3.3.1 单篇模式

- 所有必要板块 Full + 勾选拓展板块
- 左侧 sticky TOC，纵向深读
- 适合：重点专利深研

#### 3.3.2 多篇模式（用户策展主题）

用户预设主题（如"美工刀系列"）→ 批量导入该主题专利 → 每篇可精简介绍 → 统一选择模块组合 → 输出统一模板 HTML。

**模块配置三层机制**：

1. **全局默认**：所有专利统一用某套模块组合（如 M1-M4 Full + M5-M7 Lite + E4）
2. **批量覆盖**：勾选第 1-5 篇 → 应用"完整版"模块组合
3. **单篇微调**：某篇标为"⭐ 重点" → 单独切换为 Full

**输出 HTML 结构**：

```
封面页（主题名、专利数、生成日期、全局 TOC）
├── 主题概览
│   ├── E4 对比矩阵（横向对比表）
│   ├── E5 技术演进时间线（可选）
│   └── 主题综述（AI 基于全部专利生成）
├── 专利 1（按其模块配置渲染）
├── 专利 2
├── ...
└── 专利 N
```

**导航**：左侧双层 TOC —— 一级=专利列表，二级=当前专利的模块列表

### 3.4 AI 输出缓存与重跑机制

#### 3.4.1 核心流程

```
1. 用户配置 → 点击"生成"
2. 逐板块调用 AI → 每个板块输出 JSON 存入 SQLite 缓存
3. 全部生成完 → 应用内 WebView 预览
4. 用户对某板块不满意 → 点"🔄 重跑此板块"
5. 重跑结果覆盖缓存 → 预览实时更新
6. 用户也可手动编辑某板块的 AI JSON 输出（高级模式）
7. 满意后 → "导出 HTML"（基于当前缓存渲染，不再调 AI）
```

#### 3.4.2 缓存数据结构

```sql
CREATE TABLE ai_cache (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id TEXT NOT NULL,        -- 项目唯一标识
  patent_id TEXT NOT NULL,         -- 专利公开号
  module_id TEXT NOT NULL,         -- 板块编号 (M4, M5, E4...)
  level TEXT NOT NULL DEFAULT 'full', -- full/lite
  output_json TEXT NOT NULL,       -- AI 输出 JSON
  model TEXT NOT NULL,             -- 使用的模型
  provider TEXT NOT NULL,          -- AI 提供方
  prompt_version TEXT NOT NULL,    -- Prompt 模板版本
  temperature REAL,                -- 生成温度
  rerun_count INTEGER DEFAULT 0,   -- 重跑次数
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(project_id, patent_id, module_id, level)
);
```

#### 3.4.3 重跑 UI

- 每个板块右上角显示：AI 模型名 + 置信度 + 🔄 重跑按钮
- 点击重跑 → 弹出选项：更换模型 / 调整温度 / 直接重跑
- 重跑后覆盖缓存，预览自动刷新

### 3.5 批注系统

#### 3.5.1 批注交互

- 选中任意文字 → 弹出批注气泡
- 气泡内容：文本输入框 + 颜色标签（疑问/重点/规避思路/其他）
- 批注以高亮色显示在原文上
- 右侧侧栏列出所有批注，可编辑/删除/跳转

#### 3.5.2 批注数据结构

```typescript
interface Annotation {
  id: string;                    // UUID
  patentId: string;              // 专利公开号
  moduleId: string;              // 板块编号
  text: string;                  // 批注内容
  selectedText: string;          // 被标注的原文
  tag: 'question' | 'key' | 'avoid' | 'other';  // 标签
  color: string;                 // 显示颜色
  author: string;                // 作者
  createdAt: string;             // ISO 时间
  rangeInfo: {                   // DOM Range 信息，用于重新定位
    startOffset: number;
    endOffset: number;
    startContainerPath: string;  // XPath
    endContainerPath: string;
  };
}
```

#### 3.5.3 持久化

- **localStorage**：实时保存，页面刷新不丢失
- **导出**：下载 `patent-{专利号}-annotations.json`
- **导入**：加载同事分享的 JSON，合并显示（不同人用不同颜色）
- 多篇模式下批注按专利隔离

---

## 四、AI 服务集成

### 4.1 多 Provider 架构

参考 history-helper 的 `web-ai.js` 模式，统一封装：

```typescript
// 前端 AI 配置 Store (Pinia)
interface AIProviderConfig {
  type: 'openai' | 'zhipu' | 'deepseek';
  apiKey: string;
  baseUrl: string;
  model: string;
}

interface AIConfig {
  analysis: AIProviderConfig;     // 分析用模型（强模型）
  translate: AIProviderConfig;    // 翻译用模型（快速模型）
  ocr: 'paddleocr' | 'glm';      // OCR 引擎选择
}
```

### 4.2 默认配置

| Provider | Base URL | 分析模型 | 翻译/快速模型 |
|----------|----------|----------|---------------|
| DeepSeek | https://api.deepseek.com | deepseek-chat | deepseek-v4-flash |
| 智谱 GLM | https://open.bigmodel.cn/api/paas | glm-4-plus | glm-4-flash |
| OpenAI 兼容 | 可自定义 | gpt-4o | gpt-4o-mini |

### 4.3 Rust 后端 AI 调用

参考 history-helper 的 SSE 流式模式，Rust 侧实现：

```rust
// src-tauri/src/ai/mod.rs

pub struct AIClient {
    provider: AIProvider,
    api_key: String,
    base_url: String,
    model: String,
}

pub enum AIProvider {
    OpenAI,
    Zhipu,
    DeepSeek,
}

impl AIClient {
    /// SSE 流式调用，逐 token 返回
    pub async fn stream_chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<impl Stream<Item = Result<String>>> {
        // 统一 OpenAI 兼容接口格式
        // POST {base_url}/v1/chat/completions
        // SSE: data: {"choices":[{"delta":{"content":"..."}}]}
    }

    /// 非流式调用，等待完整 JSON 返回
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<String> {
        // 用于板块级生成，等待完整 JSON
    }
}
```

### 4.4 OCR 双引擎 + 版面识别

参考 history-helper 的 `extract_pdf.py` + `electron-main.js` OCR 模块。OCR 在本项目承担三重职责：①PDF 文本抽取（扫描件）；②**版面识别定位附图**；③**附图标号 OCR + 坐标定位**。

#### PaddleOCR-VL（免费，推荐默认）

```
流程：提交异步 Job → 轮询状态 → 获取 JSONL 结果
API: https://paddleocr.aistudio-app.com/api/v2/ocr/jobs
认证: Bearer Token（内置）
模型: PaddleOCR-VL-1.6
返回：每页的 region 列表，含 type（text/figure/table/title）+ bbox + text/ocr_blocks
```

Rust 侧实现：通过 reqwest 调用 PaddleOCR API（与 history-helper 的 Python 脚本逻辑一致，但用 Rust 重写）。

#### GLM OCR（付费，精度更高）

```
API: https://open.bigmodel.cn/api/paas/v4/layout_parsing
认证: 智谱 API Key
优势: 版面识别精度更高，适合复杂专利文档
返回：版面区域 + 文字块坐标，格式与 PaddleOCR 类似
```

#### 4.4.1 版面识别 → 附图定位流程

OCR 返回的版面识别结果是附图智能标注的起点。处理流程：

```rust
// src-tauri/src/ocr/mod.rs

pub async fn parse_pdf_layout(
    pdf_path: &str,
    engine: OcrEngine,
) -> Result<LayoutParsingResult> {
    // 1. 调用 OCR API（PaddleOCR 或 GLM），获取版面识别结果
    let raw_result = match engine {
        OcrEngine::Paddle => paddle::parse_layout(pdf_path).await?,
        OcrEngine::Glm => glm::parse_layout(pdf_path).await?,
    };

    // 2. 解析为统一的 LayoutParsingResult 结构
    let result = parse_layout_response(raw_result)?;

    // 3. 筛选 Figure 区域 + 关联 Caption（图说明文字）
    //    规则：Figure 区域附近（下方/右侧）的 Caption 文本即为图号
    let figures = extract_figures_with_captions(&result);

    Ok(result)
}

/// 从版面识别结果提取附图 + 图号
fn extract_figures_with_captions(result: &LayoutParsingResult) -> Vec<FigureCandidate> {
    // 遍历每页的 regions
    // 对每个 Figure 区域，查找相邻的 Caption 区域（"图1""图2"等）
    // 返回 FigureCandidate { page, bbox, figure_num }
}

/// 对附图区域做标号 OCR，返回标号 + 坐标
pub async fn ocr_figure_labels(
    figure_image: &[u8],  // 附图 PNG 字节
    engine: OcrEngine,
) -> Result<Vec<OcrBlock>> {
    // 1. 对附图图片单独做 OCR
    let raw_blocks = match engine {
        OcrEngine::Paddle => paddle::ocr_image(figure_image).await?,
        OcrEngine::Glm => glm::ocr_image(figure_image).await?,
    };

    // 2. 筛选标号：纯数字或短字母（1-3字符），过滤掉长文本
    let labels: Vec<OcrBlock> = raw_blocks
        .into_iter()
        .filter(|b| is_label_text(&b.text))
        .collect();

    Ok(labels)
}

/// 判断 OCR 文本是否为附图标号
fn is_label_text(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() || trimmed.len() > 4 {
        return false;
    }
    // 匹配：纯数字（1, 2, 10）、数字+字母（10a, 10b）、数字-数字（1-1）
    let re = regex::Regex::new(r"^\d+[a-zA-Z]?$|^\d+-\d+$").unwrap();
    re.is_match(trimmed)
}
```

#### 4.4.2 附图标号 → 部件名关联

标号 OCR 得到坐标后，需关联说明书文字得到部件名。这是 AI 任务（见 4.5 Prompt 体系的 `figure_label_mapping.yaml`）：

```rust
// src-tauri/src/ai/figure_labels.rs

pub async fn build_label_annotations(
    figure: &FigureCandidate,
    label_ocr_blocks: &[OcrBlock],
    description_text: &str,
    ai_client: &AIClient,
) -> Result<Vec<LabelAnnotation>> {
    // 1. 调用 AI 提取说明书中的"标号-部件名"映射
    let mapping = ai_client
        .call_prompt("figure_label_mapping", &{"description": description_text})
        .await?;

    // 2. 将 OCR 标号坐标转换为相对图片的百分比坐标
    let figure_width = figure.image_width as f32;
    let figure_height = figure.image_height as f32;

    let annotations: Vec<LabelAnnotation> = label_ocr_blocks
        .iter()
        .map(|block| {
            let part_info = mapping.labels.iter().find(|l| l.label == block.text);
            LabelAnnotation {
                label: block.text.clone(),
                part_name: part_info.map(|p| p.part_name.clone()),
                description_ref: part_info.map(|p| p.description_ref.clone()),
                relative_bbox: RelativeBBox {
                    x: block.bbox.x / figure_width,
                    y: block.bbox.y / figure_height,
                    width: block.bbox.width / figure_width,
                    height: block.bbox.height / figure_height,
                },
            }
        })
        .collect();

    Ok(annotations)
}
```

#### 4.4.3 完整附图处理流水线

```rust
// src-tauri/src/pdf/figure_pipeline.rs

pub async fn process_figures(
    pdf_path: &str,
    description_text: &str,
    ocr_engine: OcrEngine,
    ai_client: &AIClient,
) -> Result<Vec<FigureImage>> {
    // ① 版面识别
    let layout = ocr::parse_pdf_layout(pdf_path, ocr_engine).await?;

    // ② 提取附图候选（Figure 区域 + 图号）
    let candidates = extract_figures_with_captions(&layout);

    let mut figures = Vec::new();
    for candidate in candidates {
        // ③ 用 mupdf 按 bbox 裁剪附图，渲染为 PNG
        let png_bytes = pdf::render::render_region(
            pdf_path,
            candidate.page_number,
            &candidate.bbox,
        )?;

        // ④ 对附图做标号 OCR
        let label_blocks = ocr::ocr_figure_labels(&png_bytes, ocr_engine).await?;

        // ⑤ 关联说明书，生成标注层
        let annotations = ai::build_label_annotations(
            &candidate,
            &label_blocks,
            description_text,
            ai_client,
        ).await?;

        // ⑥ 组装 FigureImage
        figures.push(FigureImage {
            figure_num: candidate.figure_num,
            image_base64: base64::encode(&png_bytes),
            description: None,  // 后续由 E2 图示说明 AI 生成
            referenced_claims: vec![],  // 后续由 M5 关联
            referenced_embodiments: vec![],  // 后续由 M6 关联
            page_number: candidate.page_number,
            source_bbox: candidate.bbox,
            label_annotations: annotations,
        });
    }

    Ok(figures)
}
```

### 4.5 Prompt 模板体系

#### 4.5.1 目录结构

```
src-tauri/prompts/
├── _shared/
│   ├── system.yaml              # 系统提示（角色、输出规范、禁编造规则）
│   └── output_schema.yaml       # 通用 JSON 输出规范
├── M4_summary.yaml              # 一句话概要
├── M5_claims.yaml               # 权要解读（含 Full/Lite 两套）
├── M6_embodiments.yaml          # 实施例归纳（含 Full/Lite 两套）
├── M7_other_solutions.yaml      # 其他揭示方案
├── M3_family_themes.yaml        # 同族保护主题概要
├── E2_figure_desc.yaml          # 附图说明（图示内容描述）
├── figure_label_mapping.yaml    # 附图标号-部件名关联提取（v1.1 新增）
├── E4_comparison.yaml           # 多篇对比矩阵
├── E5_evolution.yaml            # 技术演进
├── E7_design_around.yaml        # 规避提示
└── theme_overview.yaml          # 多篇主题综述
```

#### 4.5.2 Prompt 模板格式（YAML）

```yaml
# M4_summary.yaml
id: M4_summary
name: 一句话概要
description: 提取技术问题、技术手段、技术效果三段式概要
model_hint: strong          # 建议使用强模型
temperature: 0.3            # 低温度，确保准确性
input_fields:
  - claims                  # 权利要求文本
  - description             # 说明书文本
  - abstract                # 摘要
output_schema:
  type: object
  properties:
    technical_problem:
      type: string
      description: 现有技术存在的具体问题
    technical_means:
      type: string
      description: 本专利采用的核心技术手段
    technical_effect:
      type: string
      description: 实现的技术效果及相比现有技术的提升
    source_refs:
      type: array
      items: { type: string }
      description: 来源段落号列表
    confidence:
      type: number
      description: 置信度 0-1
prompt_template: |
  你是一位资深专利分析师。请基于以下专利文本，提取一句话概要。

  ## 专利文本

  ### 权利要求
  {{claims}}

  ### 说明书
  {{description}}

  ### 摘要
  {{abstract}}

  ## 输出要求

  请严格按照以下 JSON 格式输出，不要添加任何其他内容：

  ```json
  {
    "technical_problem": "现有XXX存在YYY问题",
    "technical_means": "通过采用ZZZ结构/方法...",
    "technical_effect": "实现AAA效果，相比现有技术提升BBB",
    "source_refs": ["[0005]", "[0012]-[0015]"],
    "confidence": 0.92
  }
  ```

  ## 规则
  1. 技术问题：必须具体，不能泛泛而谈
  2. 技术手段：必须包含核心结构/方法特征
  3. 技术效果：必须量化或可对比
  4. source_refs：必须标注来源段落号，不可编造
  5. confidence：根据信息充分程度自评
  6. 如果输入信息不足以生成某项，填 "⚠️ 输入缺失" 而非编造
```

#### 4.5.3 各板块 Prompt 输出格式

**M5 权要解读**：
```json
{
  "scope_summary": "一种XXX，包含A、B、C特征",
  "claims": [
    {
      "claim_num": 1,
      "type": "independent",
      "scope_summary": "一种XXX，包含A、B、C特征",
      "full_text": "一种美工刀，包括...",
      "key_limitations": [
        {
          "text": "特征C",
          "interpretation": "限定为...",
          "source_ref": "[0023]",
          "figure_ref": "图2"
        }
      ],
      "depends_on": null
    }
  ]
}
```

**M6 实施例归纳**：
```json
{
  "embodiments": [
    {
      "num": 1,
      "title": "基础双重锁定结构",
      "core_scheme": "采用直齿圆柱棘齿+压缩弹簧...",
      "parameters": [
        {"name": "棘齿齿距", "value": "1.2mm"},
        {"name": "弹簧力", "value": "8N"}
      ],
      "variations": "滑块表面可增加防滑纹...",
      "figure_refs": ["图2", "图3", "图4"],
      "source_refs": ["[0020]-[0030]"]
    }
  ]
}
```

**E4 对比矩阵**：
```json
{
  "dimensions": ["技术问题", "技术手段", "技术效果", "独权范围", "同族国家"],
  "patents": [
    {
      "patent_id": "CN114567890A",
      "values": {
        "技术问题": "刀片易意外滑出",
        "技术手段": "棘齿+弹簧双重锁定",
        "技术效果": "防误触、寿命>5000次",
        "独权范围": "含棘齿、弹簧、滑块",
        "同族国家": "CN/JP/US"
      },
      "diff_flags": {
        "技术手段": true,
        "独权范围": false
      }
    }
  ]
}
```

### 4.6 多模型路由策略

| 板块 | 建议模型类型 | 理由 |
|------|-------------|------|
| M4 概要 | 强模型（GLM-4-Plus / DeepSeek-V3） | 需准确提炼问题/手段/效果 |
| M5 权要解读 | 强模型 | 需精确解读限定词范围 |
| M6 实施例归纳 | 通用模型 | 结构化归纳，难度适中 |
| M7 其他方案 | 通用模型 | 列表整理 |
| E2 附图说明 | 多模态模型（如支持） | 需理解图片内容 |
| E4 对比矩阵 | 强模型 | 需跨专利对比分析 |
| E7 规避提示 | 强模型 | 需深度理解权要范围 |
| 主题综述 | 强模型 | 需综合多专利分析 |

---

## 五、HTML 输出模板设计

### 5.1 模板架构

```
src/templates/
├── single.hbs              # 单篇模板
├── multi.hbs               # 多篇模板
└── partials/
    ├── head.hbs             # <head> 内联 CSS/JS
    ├── toolbar.hbs          # 顶部工具栏
    ├── sidebar.hbs          # 左侧导航
    ├── cover.hbs            # 多篇封面
    ├── M1_basic_info.hbs    # 专利基本信息
    ├── M2_legal_status.hbs  # 法律状态时间轴
    ├── M3_family.hbs        # 同族保护
    ├── M4_summary.hbs       # 一句话概要
    ├── M5_claims.hbs        # 权要解读
    ├── M6_embodiments.hbs   # 实施例归纳
    ├── M7_other.hbs         # 其他揭示方案
    ├── E1_pdf_viewer.hbs    # PDF 原文浏览
    ├── E2_figures.hbs       # 附图对照
    ├── E3_annotations.hbs   # 批注侧栏
    ├── E4_comparison.hbs    # 对比矩阵
    ├── E5_evolution.hbs     # 技术演进
    ├── E6_applicant.hbs     # 申请人画像
    ├── E7_design_around.hbs # 规避提示
    ├── E8_citations.hbs     # 引用网络
    └── scripts.hbs          # 内联 JS（交互逻辑）
```

### 5.2 板块组合渲染逻辑

```handlebars
{{!-- single.hbs --}}
<!DOCTYPE html>
<html lang="zh-CN">
{{> head}}
<body>
{{> toolbar}}
<div class="layout">
  {{> sidebar}}
  <main class="main">
    {{> M1_basic_info}}
    {{> M2_legal_status}}
    {{> M3_family}}
    {{> M4_summary}}
    {{#if (ne module_config.M5 "off")}}{{> M5_claims}}{{/if}}
    {{#if (ne module_config.M6 "off")}}{{> M6_embodiments}}{{/if}}
    {{#if (ne module_config.M7 "off")}}{{> M7_other}}{{/if}}
    {{#if (eq module_config.E1 "full")}}{{> E1_pdf_viewer}}{{/if}}
    {{#if (ne module_config.E2 "off")}}{{> E2_figures}}{{/if}}
    {{#if (ne module_config.E3 "off")}}{{> E3_annotations}}{{/if}}
  </main>
</div>
{{> scripts}}
</body>
</html>
```

### 5.3 内联资源策略

| 资源类型 | 内联方式 | 说明 |
|----------|----------|------|
| CSS | `<style>` 标签内联 | 全部内联，无外部依赖 |
| JS | `<script>` 标签内联 | 全部内联，含 pdf.js（如启用 E1） |
| 附图 | `<img src="data:image/png;base64,...">` | base64 内嵌 |
| PDF 原文 | `<iframe src="data:application/pdf;base64,...">` | 可选内嵌，体积较大 |
| 字体 | 不内联 | 使用系统字体栈 |

### 5.4 输出 HTML 交互功能

| 功能 | 实现方式 | 说明 |
|------|----------|------|
| 左侧 TOC 导航 | 原生 JS + IntersectionObserver | 当前位置高亮，平滑滚动 |
| 板块显隐开关 | 顶部工具栏 + localStorage | 必要板块锁定不可关 |
| 全文搜索 | 原生 JS + CSS 高亮 | Ctrl+F 增强 |
| 权要树交互 | 原生 JS | 点击切换详情面板 |
| 实施例折叠 | 原生 JS | 点击展开/收起 |
| 附图对照 | 原生 JS + Canvas | 缩放/平移/框选标注 |
| 批注 | 原生 JS + Selection API | 选中文字弹出气泡 |
| 批注导入导出 | JSON Blob 下载 + FileReader | localStorage 持久化 |
| 打印优化 | `@media print` CSS | 隐藏工具栏/侧栏 |
| 深色模式 | CSS 变量 + `prefers-color-scheme` | 自动跟随系统 |

---

## 六、Rust 后端模块设计

### 6.1 模块划分

```
src-tauri/src/
├── main.rs                  # Tauri 入口
├── lib.rs                   # 模块注册
├── commands/                # Tauri IPC 命令
│   ├── mod.rs
│   ├── input.rs             # 输入处理命令
│   ├── generate.rs          # 生成命令
│   ├── cache.rs             # 缓存管理命令
│   └── export.rs            # 导出命令
├── pdf/                     # PDF 处理
│   ├── mod.rs
│   ├── extract.rs           # 文本抽取 (pdf-extract)
│   ├── images.rs            # 图像抽取 (mupdf)
│   ├── render.rs            # PDF 页面/区域渲染为 PNG
│   └── figure_pipeline.rs   # 附图处理流水线（版面识别→裁剪→标号OCR→关联→标注层）
├── table/                   # 表格解析
│   ├── mod.rs
│   ├── xlsx.rs              # Excel 解析 (calamine)
│   ├── csv.rs               # CSV 解析
│   └── mapping.rs           # 字段映射
├── ai/                      # AI 服务
│   ├── mod.rs
│   ├── client.rs            # HTTP 客户端 + SSE
│   ├── provider.rs          # Provider 配置
│   ├── prompt.rs            # Prompt 模板加载
│   ├── figure_labels.rs     # 附图标号-部件名关联（v1.1 新增）
│   └── models.rs            # 数据结构
├── ocr/                     # OCR 模块（版面识别 + 标号定位）
│   ├── mod.rs               # 统一接口 + 版面识别流程
│   ├── paddle.rs            # PaddleOCR-VL API（layout parsing + 图像 OCR）
│   ├── glm.rs               # GLM OCR API（layout parsing + 图像 OCR）
│   └── label_filter.rs      # 标号筛选（从 OCR 结果过滤出附图标号）
├── render/                  # HTML 渲染
│   ├── mod.rs
│   ├── template.rs          # Handlebars 注册
│   └── assets.rs            # 资源内联 (base64)
├── cache/                   # 缓存模块
│   ├── mod.rs
│   └── sqlite.rs            # SQLite 操作
└── types/                   # 共享类型
    ├── mod.rs
    ├── patent.rs            # 专利数据结构
    └── module_config.rs     # 板块配置结构
```

### 6.2 核心 Tauri IPC 命令

```rust
// 输入处理
#[tauri::command]
async fn process_input(files: Vec<InputFile>) -> Result<Vec<PatentData>, String>;

// 字段映射
#[tauri::command]
async fn map_fields(table_path: String, mapping: FieldMapping) -> Result<PatentData, String>;

// PDF 文本抽取
#[tauri::command]
async fn extract_pdf_text(pdf_path: String) -> Result<PdfExtractResult, String>;

// PDF 图像抽取
#[tauri::command]
async fn extract_pdf_images(pdf_path: String) -> Result<Vec<FigureImage>, String>;

// OCR 识别
#[tauri::command]
async fn ocr_pdf(pdf_path: String, engine: OcrEngine) -> Result<OcrResult, String>;

// AI 生成（单板块）
#[tauri::command]
async fn generate_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    level: String,
    provider: AIProviderConfig,
) -> Result<ModuleOutput, String>;

// AI 流式生成（SSE → 前端 EventSource）
#[tauri::command]
async fn stream_generate_module(/* ... */) -> Result<(), String>;

// 缓存查询
#[tauri::command]
async fn get_cached_module(
    project_id: String,
    patent_id: String,
    module_id: String,
) -> Result<Option<ModuleOutput>, String>;

// 板块重跑
#[tauri::command]
async fn rerun_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    options: RerunOptions,
) -> Result<ModuleOutput, String>;

// HTML 渲染导出
#[tauri::command]
async fn render_html(
    project_id: String,
    module_config: ModuleConfig,
    embed_pdf: bool,
) -> Result<String, String>;

// 导出 HTML 文件
#[tauri::command]
async fn export_html(
    project_id: String,
    output_path: String,
    module_config: ModuleConfig,
    embed_pdf: bool,
) -> Result<(), String>;
```

### 6.3 核心数据结构

```rust
// types/patent.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct PatentData {
    pub publication_number: Option<String>,
    pub grant_number: Option<String>,
    pub application_number: Option<String>,
    pub applicant: Option<String>,
    pub inventor: Option<String>,
    pub filing_date: Option<String>,
    pub priority_date: Option<String>,
    pub publication_date: Option<String>,
    pub grant_date: Option<String>,
    pub legal_status: Option<String>,
    pub ipc: Option<String>,
    pub cpc: Option<String>,
    pub title: Option<String>,
    pub abstract_text: Option<String>,
    pub claims_text: Option<String>,
    pub description_text: Option<String>,
    pub family_members: Option<Vec<FamilyMember>>,
    pub figures: Option<Vec<FigureImage>>,
    pub pdf_base64: Option<String>,
    pub source: InputSource,  // 来自表格/PDF/混合
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyMember {
    pub country: String,
    pub publication_number: String,
    pub status: String,       // granted / pending / expired
    pub theme_summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FigureImage {
    pub figure_num: String,   // "图1", "图2"...
    pub image_base64: String,
    pub description: Option<String>,  // AI 生成的图示说明
    pub referenced_claims: Vec<u32>,
    pub referenced_embodiments: Vec<u32>,
    // 智能标注相关（v1.1 新增）
    pub page_number: u32,                  // 来源 PDF 页码
    pub source_bbox: BBox,                 // 在 PDF 页面中的位置（版面识别结果）
    pub label_annotations: Vec<LabelAnnotation>,  // 标号标注层
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BBox {
    pub x: f32,      // 左上角 x（PDF 坐标系，点）
    pub y: f32,      // 左上角 y
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabelAnnotation {
    pub label: String,              // 标号文本，如 "1"、"10a"
    pub part_name: Option<String>,  // 部件名称，如 "壳体"（来自说明书关联）
    pub description_ref: Option<String>,  // 说明书段落号，如 "[0012]"
    pub relative_bbox: RelativeBBox,  // 相对图片的百分比坐标（0-1）
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelativeBBox {
    pub x: f32,      // 相对图片左上角的 x 百分比 (0-1)
    pub y: f32,      // 相对图片左上角的 y 百分比 (0-1)
    pub width: f32,  // 宽度百分比 (0-1)
    pub height: f32, // 高度百分比 (0-1)
}

// 版面识别结果（OCR 返回）
#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutParsingResult {
    pub pages: Vec<LayoutPage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutPage {
    pub page_number: u32,
    pub regions: Vec<LayoutRegion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutRegion {
    pub region_type: RegionType,  // Text / Figure / Table / Title
    pub bbox: BBox,
    pub text: Option<String>,     // 文本区域的内容
    pub ocr_blocks: Vec<OcrBlock>, // 附图区域的标号 OCR 结果
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RegionType {
    Text,
    Figure,
    Table,
    Title,
    Caption,  // 图说明文字，如 "图1"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OcrBlock {
    pub text: String,   // OCR 识别的文本
    pub bbox: BBox,     // 在 PDF 页面中的位置
    pub confidence: f32,
}

// types/module_config.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub mode: ViewMode,       // Single / Multi
    pub theme_name: Option<String>,
    pub theme_description: Option<String>,
    pub patents: Vec<PatentModuleConfig>,
    pub global_extended: HashMap<String, ModuleLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatentModuleConfig {
    pub patent_id: String,
    pub is_key: bool,         // ⭐ 重点标记
    pub levels: HashMap<String, ModuleLevel>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ModuleLevel {
    Full,
    Lite,
    Off,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ViewMode {
    Single,
    Multi,
}
```

---

## 七、前端架构设计

### 7.1 目录结构

```
src/
├── App.vue                  # 根组件
├── main.ts                  # 入口
├── router/                  # 路由（步骤式导航）
│   └── index.ts
├── stores/                  # Pinia Store
│   ├── project.ts           # 项目状态
│   ├── input.ts             # 输入数据
│   ├── moduleConfig.ts      # 板块配置
│   ├── aiConfig.ts          # AI 配置
│   └── cache.ts             # AI 输出缓存
├── components/
│   ├── layout/
│   │   ├── AppHeader.vue    # 标题栏
│   │   ├── StepNav.vue      # 左侧步骤导航
│   │   └── MainLayout.vue   # 主布局
│   ├── input/
│   │   ├── FileDrop.vue     # 拖拽上传
│   │   ├── FileList.vue     # 文件列表
│   │   └── FieldMapping.vue # 字段映射表
│   ├── config/
│   │   ├── ModeSelect.vue   # 模式选择
│   │   ├── ThemeInfo.vue    # 主题信息
│   │   ├── ModuleGrid.vue   # 板块勾选网格
│   │   ├── PresetSelect.vue # 预设选择
│   │   └── PatentList.vue   # 逐篇配置表
│   ├── ai/
│   │   ├── AIProviderConfig.vue  # AI 服务配置
│   │   ├── OCREngineSelect.vue   # OCR 引擎选择
│   │   └── PromptEditor.vue      # Prompt 模板编辑
│   ├── generate/
│   │   ├── ProgressPanel.vue     # 生成进度
│   │   ├── ModuleRerun.vue       # 板块重跑
│   │   └── CacheEditor.vue       # 缓存编辑（高级）
│   └── preview/
│       ├── PreviewPanel.vue      # HTML 预览
│       └── ExportPanel.vue       # 导出面板
├── services/
│   ├── tauri.ts             # Tauri invoke 封装
│   └── ai.ts                # AI 配置管理
└── types/
    ├── patent.ts            # 专利数据类型
    └── module.ts            # 板块配置类型
```

### 7.2 页面流程

```
Step 1: 输入 → 拖拽上传文件 → 自动识别类型 → 字段映射
Step 2: 模式与板块 → 选择模式 → 勾选板块 → 逐篇配置
Step 3: AI 配置 → 选择 Provider → 填入 API Key → 选择模型
Step 4: 生成与重跑 → 点击生成 → 逐板块进度 → 不满意重跑
Step 5: 预览与导出 → 实时预览 → 导出 HTML
```

### 7.3 与 patent2pic 的组件复用

| 组件 | patent2pic 对应 | 复用方式 |
|------|-----------------|----------|
| AI Provider 配置 | `src/components/ai/` | 参考架构，适配本项目的多板块需求 |
| Element Plus 布局 | 整体布局模式 | 参考暗色主题 + 中文适配 |
| Pinia Store 模式 | `src/stores/` | 参考状态管理模式 |

### 7.4 与 history-helper 的模式复用

| 模式 | history-helper 对应 | 复用方式 |
|------|---------------------|----------|
| AI SSE 流式调用 | `web-ai.js` | Rust 侧重写流式逻辑，前端 EventSource 接收 |
| OCR 双引擎 | `extract_pdf.py` + `electron-main.js` | Rust 侧重写 API 调用逻辑 |
| PDF 渲染 | pdf.js + Canvas | 输出 HTML 内复用相同 pdf.js 模式 |
| 缓存机制 | SQLite (Tauri) | 直接复用 SQLite 缓存模式 |
| API 代理 | Electron main process | Tauri IPC 替代，逻辑一致 |

---

## 八、项目目录结构

```
patent-reader/
├── src/                          # 前端源码 (Vue 3 + TS)
│   ├── App.vue
│   ├── main.ts
│   ├── router/
│   ├── stores/
│   ├── components/
│   │   ├── layout/
│   │   ├── input/
│   │   ├── config/
│   │   ├── ai/
│   │   ├── generate/
│   │   └── preview/
│   ├── services/
│   └── types/
├── src-tauri/                    # Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── commands/
│   │   ├── pdf/
│   │   ├── table/
│   │   ├── ai/
│   │   ├── ocr/
│   │   ├── render/
│   │   ├── cache/
│   │   └── types/
│   ├── prompts/                  # Prompt 模板 (YAML)
│   │   ├── _shared/
│   │   ├── M4_summary.yaml
│   │   ├── M5_claims.yaml
│   │   └── ...
│   └── templates/                # HTML 输出模板 (Handlebars)
│       ├── single.hbs
│       ├── multi.hbs
│       └── partials/
├── package.json
├── vite.config.ts
├── tsconfig.json
└── .trae/rules/
    └── project_rules.md
```

---

## 九、实现关键路径

### 9.1 Phase 1：骨架搭建

1. Tauri 2.0 项目初始化（参考 patent2pic 的 tauri.conf.json）
2. Vue 3 + Element Plus + Pinia + Vite 配置
3. 步骤式导航框架
4. 基础布局（标题栏 + 步骤导航 + 配置区 + 预览区）

### 9.2 Phase 2：输入处理

1. 文件拖拽上传（Tauri dialog + fs 插件）
2. PDF 文本抽取（Rust pdf-extract）
3. 表格解析 + 字段映射（Rust calamine）
4. OCR 双引擎集成（Rust reqwest 调 PaddleOCR/GLM API）
   - 版面识别（layout parsing）→ 定位附图区域
   - 附图区域裁剪渲染（mupdf）
   - 附图标号 OCR + 坐标定位
   - 标号筛选（正则过滤纯数字/短字母）

### 9.3 Phase 3：AI 生成

1. AI 多 Provider 客户端（Rust reqwest + SSE）
2. Prompt 模板加载与渲染（YAML + Handlebars）
3. 逐板块生成 + SQLite 缓存
4. **附图标号-部件名关联**（AI 从说明书提取映射表，关联 OCR 标号坐标）
5. **附图标注层生成**（标号坐标 + 部件名 → SVG overlay 数据）
6. 板块级重跑机制
7. 前端进度展示 + 流式输出

### 9.4 Phase 4：HTML 输出

1. Handlebars 模板开发（单篇 + 多篇 + 各板块 partial）
2. 资源内联（CSS/JS/图/PDF base64）
3. 输出 HTML 交互功能（TOC/搜索/批注）
4. **E2 附图对照板块**（含 SVG 标注层渲染 + 三向联动交互）
   - 附图 `<img>` + SVG overlay 叠加
   - 悬停标号 → 部件名气泡
   - 点击标号 → 跳转说明书段落
   - 标号清单侧栏 + 高亮联动
   - 用户手动框选标注（补充层）
5. 导出文件功能

### 9.5 Phase 5：打磨

1. 板块显隐开关 + 预设系统
2. 逐篇板块配置（Full/Lite/Off）
3. 批注导入导出
4. 打印样式优化
5. 深色模式
6. NSIS 安装包打包

---

## 十、开发规范

### 10.1 代码规范

- **Rust**：`cargo fmt` + `cargo clippy`，禁止 `unwrap()` 在命令层使用
- **TypeScript**：严格模式，ESLint + Prettier
- **Vue**：`<script setup>` + Composition API
- **CSS**：BEM 命名，CSS 变量管理主题色

### 10.2 Tauri 安全

- CSP 策略：`default-src 'self'; img-src 'self' data:; script-src 'unsafe-inline'`
- API Key 存储：使用 `@tauri-apps/plugin-store` 加密存储
- 文件访问：仅通过 Tauri dialog 插件，不直接暴露文件系统

### 10.3 错误处理

- AI 调用失败：重试 3 次，间隔 2s/4s/8s，最终失败标记板块为"⚠️ 生成失败"
- PDF 抽取失败：标记对应字段为"⚠️ 抽取失败"，不阻塞其他板块
- OCR 失败：回退到纯文本抽取模式
- 网络断开：检测离线状态，提示用户

### 10.4 性能约束

- 单篇专利 HTML 输出 < 30MB（含内嵌 PDF）
- 多篇（5篇）HTML 输出 < 100MB
- AI 单板块生成超时 60s
- PDF 图像抽取：每页 < 2s
- 应用启动 < 3s

### 10.5 测试策略

- Rust 单元测试：PDF 抽取、表格解析、字段映射、AI 客户端
- 前端组件测试：Vitest + Vue Test Utils
- 集成测试：端到端生成流程（输入 → 生成 → 导出）
- 手动测试：输出 HTML 在 Chrome/Edge/Firefox 的兼容性

---

## 十一、与现有项目的集成预留

### 11.1 未来统一平台集成

三个工具（patent2pic / history-helper / patent-reader）未来可能统一为一个平台：

- **共享模块**：AI Provider 配置、OCR 引擎、缓存层
- **数据流**：history-helper 获取审查历史 → patent-reader 生成解读 → patent2pic 生成结构图
- **集成方式**：Tauri 多窗口 + 共享 SQLite + 统一 AI 配置

### 11.2 本项目的集成预留

- AI 配置格式与 history-helper 的 `web-ai.js` 兼容
- SQLite 缓存表结构预留 `source_project` 字段
- 输出 HTML 预留 `window.__PATENT_READER__` 全局对象，供其他工具嵌入
- Tauri 插件注册使用统一前缀 `patent-reader-`

---

## 附录 A：输出 HTML 关键交互规格

### A.1 M5 权要解读交互

- 左侧：权要树状图（独权根节点，从权子节点，可折叠）
- 右侧：选中权要的全文 + AI 解读 + 关键限定词高亮
- 限定词旁标注来源段落号 `[0023]`，点击跳转 PDF/说明书原文
- 顶部：权要范围一句话总结

### A.2 M6 实施例归纳交互

- 每个实施例一张折叠卡片
- 卡片头：实施例编号 + 一句话核心
- 卡片体：核心方案、关键参数（表格化）、变形/替代方案、对应附图缩略图（点击跳 E2）

### A.3 E2 附图对照交互（含智能标注）

#### A.3.1 智能标注生成流程（生成阶段，Rust 后端）

```
PDF 输入
  │
  ▼
①版面识别（PaddleOCR-VL / GLM OCR layout parsing）
  │  返回 JSONL，每区域含 bbox + type（text/figure/table）
  │  筛出 type=figure 的区域 → 得到附图位置
  ▼
②附图抽取
  │  用 mupdf 按 bbox 裁剪附图区域 → 渲染为独立 PNG
  │  记录图号（从相邻文本块提取"图1""图2"等）
  ▼
③附图标号 OCR
  │  对每张附图单独做 OCR（PaddleOCR 返回 text + bbox）
  │  筛选规则：纯数字或短字母（1-3字符）+ 位于图内 → 判定为标号
  │  输出：[{label:"1", bbox:{x,y,w,h}}, {label:"2", ...}]
  ▼
④说明书"标号-部件名"关联提取（AI）
  │  从说明书文字提取映射表
  │  常见句式："图中：1-壳体，2-刀片" / "1为壳体，2为刀片" / "标号1表示..."
  │  AI Prompt 提取 → 输出：{"1":"壳体", "2":"刀片", "3":"滑块"}
  ▼
⑤生成 SVG 标注层
  │  标号 bbox 转为相对图片的百分比坐标
  │  生成 SVG <circle> + <title> 叠加层
  │  存入 FigureImage.label_annotations
  ▼
⑥输出 HTML 渲染
  │  附图 <img> + SVG overlay 叠加
  │  悬停标号 → 显示部件名气泡
  │  点击标号 → 跳转说明书对应段落
```

#### A.3.2 标号-部件名关联的 AI Prompt

```yaml
# prompts/figure_label_mapping.yaml
id: figure_label_mapping
name: 附图标号-部件名关联提取
description: 从说明书文字提取附图标号与部件名称的对应关系
model_hint: strong
temperature: 0.1
input_fields:
  - description    # 说明书文字
output_schema:
  type: object
  properties:
    labels:
      type: array
      items:
        type: object
        properties:
          label: { type: string, description: "标号，如 1、2、10a" }
          part_name: { type: string, description: "部件名称，如 壳体、刀片" }
          description_ref: { type: string, description: "说明书段落号，如 [0012]" }
prompt_template: |
  你是一位专利分析专家。请从以下专利说明书文字中，提取所有"附图标号-部件名称"的对应关系。

  ## 说明书文字
  {{description}}

  ## 提取规则
  1. 识别所有形如 "1-壳体"、"1为壳体"、"标号1表示壳体"、"1，壳体" 的对应关系
  2. 标号可能是数字（1,2,10）、带字母后缀（10a, 10b）、带连字符（1-1, 1-2）
  3. 部件名称为名词短语，如"壳体""刀片""棘齿条""压缩弹簧"
  4. 记录该对应关系首次出现的说明书段落号
  5. 如果说明书中没有明确的标号说明，返回空数组

  ## 输出格式（严格 JSON）
  ```json
  {
    "labels": [
      {"label": "1", "part_name": "壳体", "description_ref": "[0012]"},
      {"label": "2", "part_name": "刀片", "description_ref": "[0012]"},
      {"label": "3", "part_name": "滑块", "description_ref": "[0013]"}
    ]
  }
  ```
```

#### A.3.3 输出 HTML 交互

- 顶部：图号索引条（图1、图2、图3...可滚动）
- 主区：当前图大图 + **SVG 标注层叠加**
  - 支持缩放/平移（SVG 标注层同步缩放）
  - 标号位置渲染为半透明圆形锚点
  - **悬停标号 → 显示部件名气泡**（"1 → 壳体"）
  - **点击标号 → 跳转说明书对应段落**（高亮该段）
  - 标注层可一键显隐（默认显示）
- 右侧面板四栏：
  - "引用本图的权要"（点击跳 M5）
  - "引用本图的实施例"（点击跳 M6）
  - "本图标号清单"（标号-部件名列表，点击高亮图上对应锚点）
  - "用户标注"（用户框选区域 + 文字标注，存 localStorage）
- 底部：AI 生成的图示说明
- **三向联动**：附图标号 ↔ 说明书文字 ↔ 权要引用
  - 悬停附图标号 → 说明书对应文字高亮
  - 悬停说明书"标号1" → 附图对应锚点闪烁

### A.4 E3 批注交互

- 选中任意文字 → 弹气泡（输入 + 颜色标签：疑问/重点/规避/其他）
- 批注高亮显示，右侧侧栏列出
- 导出：`patent-{专利号}-annotations.json`
- 导入：合并显示，不同人不同颜色
- 多篇模式下批注按专利隔离

### A.5 E4 对比矩阵交互

- 行：对比维度（可自定义增减）
- 列：各篇专利
- 单元格：AI 提炼的一句话总结 + 跳转链接
- 差异显著项黄色高亮
- 点击单元格 → 跳转对应专利对应板块

---

## 附录 B：预设配置

| 预设名称 | M1-M4 | M5 | M6 | M7 | E1 | E2 | E3 | E4 | E5-E8 |
|----------|-------|----|----|----|----|----|----|----|-------|
| 快速概览 | Full | Lite | Lite | Lite | Off | Off | Off | Off | Off |
| 标准解读 | Full | Full | Full | Full | Off | Full | Full | Off | Off |
| 深度研读 | Full | Full | Full | Full | Full | Full | Full | Off | Off |
| 完整版 | Full | Full | Full | Full | Full | Full | Full | Full | Full |

---

## 附录 C：版本规划

| 版本 | 目标 | 关键交付 |
|------|------|----------|
| v0.1.0 | 骨架 + 单篇核心 | Tauri 骨架 + PDF 抽取 + M4/M5/M6 生成 + 单篇 HTML 输出 |
| v0.2.0 | 多篇 + 对比 | 多篇模式 + E4 对比矩阵 + 逐篇配置 |
| v0.3.0 | 拓展板块 | E2 附图对照 + E3 批注 + E1 PDF viewer |
| v0.4.0 | OCR + 完善 | OCR 双引擎 + E5-E8 拓展板块 + 板块重跑 |
| v1.0.0 | 打磨发布 | 安装包 + 用户手册 + 性能优化 + 深色模式 |
