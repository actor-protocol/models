use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::Serialize;
use sqlx::FromRow;
use std::hash::{Hash, Hasher};
use utoipa::ToSchema;

#[derive(FromRow, ToSchema, Serialize, Debug, Clone)]
pub struct Actor {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    #[sqlx(try_from = "i64")]
    pub network: u32,
    pub name: String,
    pub owner: String,
    #[serde(skip_serializing)]
    pub key_token: String,
    pub address: String,

    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Hash for Actor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Actor {}

#[derive(Iden)]
pub enum ActorIden {
    #[iden = "actors"]
    Table,
    Id,
    Network,
    Name,
    Owner,
    KeyToken,
    Address,
    UpdatedAt,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(ActorIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(ActorIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(ColumnDef::new(ActorIden::Network).big_integer().not_null())
        .col(ColumnDef::new(ActorIden::Name).string().not_null())
        .col(ColumnDef::new(ActorIden::Owner).string_len(42).not_null())
        .col(ColumnDef::new(ActorIden::KeyToken).string().not_null())
        .col(ColumnDef::new(ActorIden::Address).string_len(42).not_null())
        .col(
            ColumnDef::new(ActorIden::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(ActorIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
