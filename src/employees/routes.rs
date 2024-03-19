use crate::employees::{Employee, Employees};
use crate::error_handlers::CustomError;
use actix_web::web::{Json, Path, ServiceConfig};
use actix_web::{delete, get, post, put, HttpResponse};
use serde_json::json;

#[get("/employees")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let res = Employees::find_all()?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/employee/{id}")]
async fn find(id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee_id = id.into_inner();
    let res = Employees::find(employee_id)?;
    Ok(HttpResponse::Ok().json(res))
}

#[post("/employee")]
async fn create(body: Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = body.into_inner();
    let res = Employees::create(employee)?;
    Ok(HttpResponse::Ok().json(json!(res)))
}

#[put("/employee/{id}")]
async fn update(id: Path<i32>, body: Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = body.into_inner();
    let employee_id = id.into_inner();
    let res = Employees::update(employee_id, employee)?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/employee/{id}")]
async fn delete(id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee_id = id.into_inner();
    let res = Employees::delete(employee_id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routers(config: &mut ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
