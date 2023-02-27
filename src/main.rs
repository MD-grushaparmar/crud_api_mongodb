mod db;
mod error;
mod handler;
mod response;
mod schema;
mod model;

use db::DB;
use dotenv::dotenv;
use std::convert::Infallible;
use serde::Serialize;
use warp::{http::Method,Filter,Rejection};
type Result<T> = std::result::Result<T,error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse{
    pub status: String,
    pub message: String,
}
#[tokio::main]
async fn main() -> Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();
    dotenv().ok();
    let db = DB::init().await?;

    let cors = warp::cors().allow_methods(&[Method::GET, Method::POST,Method::PATCH, Method::DELETE]).allow_origins(vec!["http://localhost:3000"]).allow_headers(vec!["content-type"]).allow_credentials(true);

    let student_router = warp::path!("api"/"students");

    let student_router_id = warp::path!("api"/"students"/ String);

    let health_checker = warp::path!("api"/"healthchecker").and(warp::get()).and_then(handler::health_checker_handler);

    let student_routes =student_router.and(warp::post()).and(warp::body::json()).and(with_db(db.clone())).and_then(handler::create_student_handler).or(student_router.and(warp::get()).and(with_db(db.clone())).and_then(handler::students_list_handler));

    let student_routes_id = student_router_id.and(warp::patch()).and(warp::body::json()).and(with_db(db.clone())).and_then(handler::edit_student_handler).or(student_router_id.and(warp::get()).and(with_db(db.clone())).and_then(handler::get_student_handler)).or(student_router_id
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(handler::delete_student_handler));
    let routes = student_routes.with(warp::log("api")).or(student_routes_id).or(health_checker).with(cors).recover(error::handle_rejection);

    println!("server started successfully");
    warp::serve(routes).run(([0,0,0,0],8000)).await;
    Ok(())
}

fn with_db(db: DB)-> impl Filter<Extract = (DB,), Error = Infallible> + Clone{
    warp::any().map(move || db.clone())
}