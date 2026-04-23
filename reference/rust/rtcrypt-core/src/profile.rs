#[derive(Debug, Clone)]
pub struct SecurityProfile {
    pub id: String,
    pub allowed_algs: Vec<String>,
    pub typing_required: bool,
    pub default_effect: String,
}
