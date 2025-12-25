use server::bootstrap::Dependencies;

pub fn build_dependencies() -> Dependencies {
    use async_trait::async_trait;
    use domain::{ dtos::user::CreateUserDTO, features::feature::Feature };
    use errors::ZwitterError;

    struct FakeCreateUser;

    #[async_trait]
    impl Feature<CreateUserDTO, ()> for FakeCreateUser {
        async fn execute(&self, _dto: CreateUserDTO) -> Result<(), ZwitterError> {
            Ok(())
        }
    }

    Dependencies {
        create_user: Box::new(FakeCreateUser),
    }
}
