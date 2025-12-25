use std::{ sync::Arc };

use domain::{ features::create_user::{ CreateUser, CreateUserFeature } };
use macros::barc;

use crate::database::users_repository_postgres::UsersRepositoryPostgres;

pub struct DatabaseConnector {}

pub struct Dependencies {
    pub create_user: Box<CreateUserFeature>,
}

pub fn build_dependencies(connection: ()) -> Dependencies {
    let c = Arc::new(connection);
    let create_user = Box::new(CreateUser {
        users_repository: barc!(UsersRepositoryPostgres {}),
    });

    Dependencies {
        create_user,
    }
}
