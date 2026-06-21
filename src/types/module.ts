/** 板块配置类型 */
export type ViewMode = 'single' | 'multi'

export type ModuleLevel = 'full' | 'lite' | 'off'

export type ModuleId =
  // 必要板块
  | 'M1' | 'M2' | 'M3' | 'M4' | 'M5' | 'M6' | 'M7'
  // 拓展板块
  | 'E1' | 'E2' | 'E3' | 'E4' | 'E5' | 'E6' | 'E7' | 'E8'

export interface ModuleConfig {
  mode: ViewMode
  themeName?: string
  themeDescription?: string
  patents: PatentModuleConfig[]
  globalExtended: Partial<Record<ModuleId, ModuleLevel>>
}

export interface PatentModuleConfig {
  patentId: string
  isKey: boolean
  levels: Partial<Record<ModuleId, ModuleLevel>>
}

/** 板块元信息 */
export interface ModuleMeta {
  id: ModuleId
  name: string
  required: boolean
  aiGenerated: boolean
  description: string
}

/** 所有板块元信息 */
export const MODULE_REGISTRY: ModuleMeta[] = [
  { id: 'M1', name: '专利基本信息', required: true, aiGenerated: true, description: '著录信息' },
  { id: 'M2', name: '法律状态与关键日期', required: true, aiGenerated: true, description: '时间轴' },
  { id: 'M3', name: '同族保护情况', required: true, aiGenerated: true, description: '同族概要' },
  { id: 'M4', name: '一句话概要', required: true, aiGenerated: true, description: '问题/手段/效果' },
  { id: 'M5', name: '权利要求范围解读', required: true, aiGenerated: true, description: '权要树+解读' },
  { id: 'M6', name: '实施例归纳', required: true, aiGenerated: true, description: '方案+参数' },
  { id: 'M7', name: '其他揭示方案', required: true, aiGenerated: true, description: '替代方案' },
  { id: 'E1', name: 'PDF 原文浏览', required: false, aiGenerated: false, description: 'pdf.js 内嵌' },
  { id: 'E2', name: '附图对照', required: false, aiGenerated: true, description: '图库+标注' },
  { id: 'E3', name: '批注', required: false, aiGenerated: false, description: '标注+分享' },
  { id: 'E4', name: '多专利对比矩阵', required: false, aiGenerated: true, description: '横向对比' },
  { id: 'E5', name: '技术演进时间线', required: false, aiGenerated: true, description: '演进脉络' },
  { id: 'E6', name: '申请人画像', required: false, aiGenerated: true, description: '布局概览' },
  { id: 'E7', name: '规避/设计空间提示', required: false, aiGenerated: true, description: '仅供参考' },
  { id: 'E8', name: '引用关系网络', required: false, aiGenerated: false, description: '前后向引用' },
]
