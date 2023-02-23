use crate::{db::DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentRequest {
    pub name: String,
    pub email: String,
    pub course: String,
    pub university: String,
}

pub async fn student_list_handler(db: DB) -> WebResult<impl Reply> {
    let books = db.fetch_student().await.map_err(|e| reject::custom(e))?;
    Ok(json(&books))
}

pub async fn create_book_handler(body: StudentRequest, db: DB) -> WebResult<impl Reply> {
    db.create_student(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_book_handler(id: String, body: StudentRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_student(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_student_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_student(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}