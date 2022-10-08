use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event{
    pub project_id: String,
    pub account_id: String,
    pub user_id: i32,
    pub message: String
}