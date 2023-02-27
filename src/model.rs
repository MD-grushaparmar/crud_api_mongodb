use mongodb::bson::{self,oid::ObjectId};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student{
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub roll: String,
    pub name: String,
    pub email: String,
    pub course: String,
    pub university: String,
}