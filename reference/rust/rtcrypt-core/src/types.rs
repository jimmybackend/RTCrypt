#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrincipalKind {
    User,
    Client,
    Workload,
}

#[derive(Debug, Clone)]
pub struct Principal {
    pub kind: PrincipalKind,
    pub issuer: String,
    pub subject: String,
    pub tenant_id: Option<String>,
    pub audiences: Vec<String>,
    pub scopes: Vec<String>,
    pub auth_method: String,
}

#[derive(Debug, Clone)]
pub struct KeyRef {
    pub kid: String,
    pub provider: String,
    pub purpose: String,
    pub tenant_scope: Option<String>,
}
