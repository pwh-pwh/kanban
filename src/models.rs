use serde::{Deserialize, Serialize};

// for get req

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Board {
    pub id:i64,
    pub name:String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct Card {
    pub id:i64,
    pub board_id:i64,
    pub description:String,
    pub status:Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize,Deserialize)]
#[serde(rename_all="camelCase")]
pub enum Status {
    Todo,
    Done,
    Doing,
}

#[derive(Serialize)]
pub struct BoardSummary {
    pub todo:i64,
    pub done:i64,
    pub doing:i64,
}

//for post req

#[derive(Deserialize)]
pub struct CreateBoard{
    pub name:String,
}

#[derive(Deserialize)]
pub struct CreateCard {
    pub board_id:i64,
    pub description:String,
}

//for patch req

#[derive(Deserialize)]
pub struct UpdateCard {
    pub description:String,
    pub status:Status,
}