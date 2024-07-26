use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUser {
    pub name: String,
    pub surname: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
