use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    #[serde(default)]
    pub source: InputSource,
    #[serde(default)]
    pub needs_ocr: bool,
    #[serde(default)]
    pub pdf_file_path: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FamilyMember {
    pub country: String,
    pub publication_number: String,
    pub status: String,
    pub theme_summary: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum InputSource {
    #[default]
    Pdf,
    Table,
    Mixed,
}
