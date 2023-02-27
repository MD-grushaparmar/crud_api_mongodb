use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStudentSchema{
    pub roll : String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub course: Option<String>,
    pub university: Option<String>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateStudentSchema{
    pub roll : String,
    pub name : String,
    pub email: String,
    pub course: String,
    pub university: String,
}