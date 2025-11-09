use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InterestFormData {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Interest {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct PaginatedResponse {
    pub items: Vec<Interest>,
    pub next_token: Option<String>,
    pub count: usize,
}
