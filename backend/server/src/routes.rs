use actix_web::{ HttpResponse, post, web };
use domain::{ dtos::user::CreateUserDTO };
use errors::ZwitterError;

use crate::bootstrap::Dependencies;

#[post("/users")]
pub async fn create_user_handler(
    dependencies: web::Data<Dependencies>,
    req_body: String
) -> Result<HttpResponse, ZwitterError> {
    let dto: CreateUserDTO = serde_json::from_str(req_body.as_str()).unwrap();
    dependencies.create_user.execute(dto).await.map(|_| HttpResponse::Created().finish())
}
