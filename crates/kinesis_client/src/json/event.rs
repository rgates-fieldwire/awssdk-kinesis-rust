use serde::Serialize;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event{
    pub project_id: String,
    pub account_id: String,
    pub user_id: i32,
    pub message: String
}


impl fmt::Display for Event{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Project Id: {}, Account Id: {}, User id: {}, Message: {} \n", self.project_id, self.account_id, self.user_id, self.message)
    }
}