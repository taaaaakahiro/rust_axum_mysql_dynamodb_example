use async_trait::async_trait;

use crate::model::user::User;

#[async_trait]
pub trait UserRepository {
    async fn find_one(&self, id: &String) -> anyhow::Result<Option<User>>;
}
