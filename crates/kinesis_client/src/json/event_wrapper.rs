use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use std::fmt;

use crate::Event;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventWrapper{
    pub id: String,
    pub event_type: i32,
    pub event_json: String
}

impl fmt::Display for EventWrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id: {}, Event Type: {}, event json: {} \n", self.id, self.event_type, self.event_json)
    }
}
impl EventWrapper {
    pub fn get_event(&self) -> Result<Event, serde_json::Error>{
        let deserialized_result:Result<Event, serde_json::Error>= serde_json::from_str(&self.event_json);
        Ok(deserialized_result.unwrap())
    }
}