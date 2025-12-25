use sea_orm_migration::{ prelude::*, schema::* };

const TABLE_NAME: &str = "users";
const EMAIL_PASSWORD_CREDENTIALS_TABLE_NAME: &str = "email_password_credentials";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(EMAIL_PASSWORD_CREDENTIALS_TABLE_NAME)
                .col(pk_uuid("id").not_null())
                .col(string("user_id").not_null())
                .col(string("email").unique_key().not_null())
                .col(string("password_hash").not_null())
                .col(timestamp("created_at").not_null().default(Expr::current_timestamp()))
                .col(timestamp("updated_at").not_null().default(Expr::current_timestamp()))
                .if_not_exists()
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(TABLE_NAME)
                .if_not_exists()
                .col(pk_uuid("id"))
                .col(string("first_name"))
                .col(string("last_name"))
                .col(string("email").unique_key())
                .col(string("username").unique_key())
                .col(timestamp("created_at").not_null().default(Expr::current_timestamp()))
                .col(timestamp("updated_at").not_null().default(Expr::current_timestamp()))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(TABLE_NAME).to_owned()).await
    }
}
