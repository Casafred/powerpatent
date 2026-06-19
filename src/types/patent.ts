/** 专利数据类型 */
export interface PatentData {
  publicationNumber?: string
  grantNumber?: string
  applicationNumber?: string
  applicant?: string
  inventor?: string
  filingDate?: string
  priorityDate?: string
  publicationDate?: string
  grantDate?: string
  legalStatus?: string
  ipc?: string
  cpc?: string
  title?: string
  abstractText?: string
  claimsText?: string
  descriptionText?: string
  familyMembers?: FamilyMember[]
  figures?: FigureImage[]
  pdfBase64?: string
  source: InputSource
  needsOcr?: boolean
}

export interface FamilyMember {
  country: string
  publicationNumber: string
  status: 'granted' | 'pending' | 'expired'
  themeSummary?: string
}

export interface FigureImage {
  figureNum: string
  imageBase64: string
  description?: string
  referencedClaims: number[]
  referencedEmbodiments: number[]
  pageNumber: number
  sourceBBox: BBox
  labelAnnotations: LabelAnnotation[]
}

export interface BBox {
  x: number
  y: number
  width: number
  height: number
}

export interface LabelAnnotation {
  label: string
  partName?: string
  descriptionRef?: string
  relativeBBox: RelativeBBox
}

export interface RelativeBBox {
  x: number
  y: number
  width: number
  height: number
}

export type InputSource = 'pdf' | 'table' | 'mixed'
