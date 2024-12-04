use serde::{Deserialize, Serialize};
use crate::models::address::Address;

#[derive(Debug, Clone, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    email: String,
    password_hash: String,
    address: Address
}




