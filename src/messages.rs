use serde::{Deserialize, Serialize};

use crate::schema::Recipes;

#[derive(Debug, Serialize, Deserialize)]
pub enum ListMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    mode: ListMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    mode: ListMode,
    data: Recipes,
    receiver: String,
}

pub enum EventType {
    Response(ListResponse),
    Input(String),
}
