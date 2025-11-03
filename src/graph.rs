use serde::{Deserialize, Serialize};

use crate::JSNode;

#[derive(Deserialize, Serialize)]
pub struct SourceGraph {
    pub root_id: i32,
    pub nodes: Vec<JSNode>,
}
