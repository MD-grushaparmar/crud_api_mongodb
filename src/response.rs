use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse{
    pub status: String,
    pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct StudentResponse{
    pub id : String,
    pub roll: String,
    pub name: String,
    pub email: String,
    pub course: String,
    pub university: String,
}

#[derive(Serialize, Debug)]
pub struct StudentData{
    pub student: StudentResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleStudentResponse{
    pub status: String,
    pub data: StudentData,
}

#[derive(Serialize, Debug)]
pub struct StudentListResponse{
    pub status: String,
    pub results: usize,
    pub students: Vec<StudentResponse>,
}