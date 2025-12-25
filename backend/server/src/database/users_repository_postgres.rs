use async_trait::async_trait;
use domain::{
    dtos::user::{ CreateUserDTO, UserDTO },
    repositories::users_repository::UsersRepository,
};
use errors::ZwitterError;

pub struct UsersRepositoryPostgres {}

#[async_trait]
impl UsersRepository for UsersRepositoryPostgres {
    async fn create(&self, dto: CreateUserDTO) -> Result<(), ZwitterError> {
        todo!();
    }

    async fn get(&self, user_id: String) -> Result<UserDTO, ZwitterError> {
        todo!();
    }
}
