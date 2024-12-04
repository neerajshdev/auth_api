use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]

pub struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}