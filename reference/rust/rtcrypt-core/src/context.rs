#[derive(Debug, Clone)]
pub struct VerificationContext {
    pub expected_profile: String,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: String,
}
