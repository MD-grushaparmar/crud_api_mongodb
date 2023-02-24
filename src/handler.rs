use crate::{
    db::DB,
    response::GenericResponse,
    schema::UpdateStudentSchema,
    schema::{CreateStudentSchema},
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