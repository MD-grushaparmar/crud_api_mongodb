//use std::async_iter;

use crate::{
    db::DB,
    response::GenericResponse,
    schema::UpdateStudentSchema,
    schema::CreateStudentSchema,
    WebResult,
};
use warp::{http::StatusCode,reject,reply::json,reply::with_status,Reply};
pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

pub async fn students_list_handler(db:DB)-> WebResult<impl Reply>{
    let result_json = db.fetch_students().await.map_err(|e| reject::custom(e))?;
    Ok(json(&result_json))
}

pub async fn create_student_handler(body: CreateStudentSchema, db: DB) -> WebResult<impl Reply>{
    let student = db.create_student(&body).await.map_err(|e| reject::custom(e))?;

    Ok(with_status(json(&student), StatusCode::CREATED))
}

pub async fn get_student_handler(id: String,db:DB)-> WebResult<impl Reply>{
    let student = db.get_student(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse{
        status: "fail".to_string(),
        message: format!("Studet with Id: {} not found",id),
    };

    if student.is_none(){
        return Ok(with_status(json(&error_response),StatusCode::NOT_FOUND));
    }
    Ok(with_status(json(&student), StatusCode::OK))
}

pub async fn edit_student_handler(id:String,body: UpdateStudentSchema, db: DB,)-> WebResult<impl Reply>{
    let student = db.edit_student(&id, &body).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse{
        status: "fail".to_string(),
        message: format!("student with Id:{} not found", id),
    };

    if student.is_none(){
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }
    Ok(with_status(json(&student), StatusCode::OK))
}
pub async fn delete_student_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let result = db.delete_student(&id).await.map_err(|e| reject::custom(e))?;

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Note with ID: {} not found", id),
    };

    if result.is_none() {
        return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
    }

    Ok(with_status(json(&""), StatusCode::NO_CONTENT))
}
