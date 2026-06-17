use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub d: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<FillRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
}
