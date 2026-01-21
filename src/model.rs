use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Soldier {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub rank: String,
    pub squad: String,
}
