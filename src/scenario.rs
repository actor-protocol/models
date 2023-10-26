use crate::utils::NullableU32;
use chrono::{DateTime, Utc};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

#[derive(FromRow, ToSchema, Serialize, Debug, Clone)]
pub struct Scenario {
    #[sqlx(try_from = "i32")]
    pub id: u16,
    #[sqlx(try_from = "i64")]
    pub network: u32,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(try_from = "Option<i64>")]
    #[schema(value_type = Option<u32>)]
    pub on_chain_id: NullableU32,
    pub owner: String,
    #[sqlx(try_from = "Option<i64>")]
    #[schema(value_type = Option<u32>)]
    pub actor_id: NullableU32,
    pub actor_address: Option<String>,
    pub input_token: Option<String>,
    pub input_amount: Option<String>,
    #[schema(value_type = Vec<Script>)]
    pub scripts: sqlx::types::Json<Vec<Script>>,
    #[schema(value_type = ScenarioStatus)]
    pub status: sqlx::types::Json<ScenarioStatus>,
    #[sqlx(try_from = "Option<i64>")]
    #[schema(value_type = Option<u32>)]
    pub updated_at_block: NullableU32,
    #[serde(skip_serializing)]
    pub deleted: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, Deserialize, Clone, Copy, PartialEq)]
#[serde(tag = "code")]
pub enum ScenarioStatus {
    Active,
    Draft,
    Finished,
    Conflict,
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug, Clone)]
pub struct Script {
    #[validate(range(min = 0, max = 1))]
    pub trigger_type: u8,
    pub triggers: Vec<Trigger>,
    pub on_chain_actions: Vec<Action>,
    pub off_chain_actions: Vec<Action>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug, Clone, PartialEq)]
pub struct Trigger {
    pub source: String,
    pub kind: String,
    pub data: HashMap<String, String>,
    pub condition: u8,
    pub on_chain_verification: bool,
}

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug, Clone, PartialEq)]
pub struct Action {
    pub action: String,
    pub input_token: Option<String>,
    pub output: Option<String>,
    pub data: HashMap<String, String>,
}

#[derive(Iden)]
pub enum ScenarioIden {
    #[iden = "scenarios"]
    Table,
    Id,
    Network,
    Name,
    Description,
    OnChainId,
    Owner,
    ActorId,
    ActorAddress,
    InputToken,
    InputAmount,
    Scripts,
    Status,
    UpdatedAtBlock,
    Deleted,
    UpdatedAt,
    CreatedAt,
}

pub fn get_create_statement() -> String {
    Table::create()
        .table(ScenarioIden::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(ScenarioIden::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key(),
        )
        .col(
            ColumnDef::new(ScenarioIden::Network)
                .big_integer()
                .not_null(),
        )
        .col(ColumnDef::new(ScenarioIden::Name).string_len(32).not_null())
        .col(ColumnDef::new(ScenarioIden::Description).string_len(256))
        .col(ColumnDef::new(ScenarioIden::OnChainId).big_integer())
        .col(
            ColumnDef::new(ScenarioIden::Owner)
                .string_len(42)
                .not_null(),
        )
        .col(ColumnDef::new(ScenarioIden::ActorId).big_integer())
        .col(ColumnDef::new(ScenarioIden::ActorAddress).string_len(42))
        .col(ColumnDef::new(ScenarioIden::InputToken).string_len(42))
        .col(ColumnDef::new(ScenarioIden::InputAmount).string())
        .col(
            ColumnDef::new(ScenarioIden::Scripts)
                .json_binary()
                .not_null(),
        )
        .col(
            ColumnDef::new(ScenarioIden::Status)
                .json_binary()
                .not_null(),
        )
        .col(ColumnDef::new(ScenarioIden::UpdatedAtBlock).big_integer())
        .col(
            ColumnDef::new(ScenarioIden::Deleted)
                .boolean()
                .not_null()
                .default(false),
        )
        .col(
            ColumnDef::new(ScenarioIden::UpdatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .col(
            ColumnDef::new(ScenarioIden::CreatedAt)
                .timestamp_with_time_zone()
                .not_null(),
        )
        .build(PostgresQueryBuilder)
}
