use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct Soldier {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50))]
    pub rank: String,
    #[validate(length(min = 1, max = 50))]
    pub squad: String,
}
