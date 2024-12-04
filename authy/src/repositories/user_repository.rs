
use crate::models::user::User;
pub trait UserRepository {
    async fn add(user: User) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_by_id(&self, id: i32) -> Result<User, Box<dyn std::error::Error>>;
    async fn update(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete(&self, id: i32) -> Result<(), Box<dyn std::error::Error>>;
}