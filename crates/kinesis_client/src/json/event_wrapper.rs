use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventWrapper{
    pub id: String,
    pub event_type: i32,
    pub event_json: String
}