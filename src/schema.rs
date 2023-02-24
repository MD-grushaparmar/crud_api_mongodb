use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions{
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateStudentSchema{
    pub name: String,
    pub email: String,
    pub course: String,
    pub university: String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UpdateStudentSchema{
    pub name : String,
    pub email: String,
    pub course: String,
    pub university: String,
}