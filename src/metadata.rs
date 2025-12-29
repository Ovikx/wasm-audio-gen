use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Metadata {
    pub id: usize,
}

pub trait WithMetadata {
    fn metadata(&self) -> Metadata;
}
