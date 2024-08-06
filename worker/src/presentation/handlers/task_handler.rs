
use actix_web::{HttpResponse, post, Responder, web::{self}};
use crate::application::dtos::task_dtos::NewTask;
use crate::shared::app_state::AppState;
use crate::shared::errors::Result;

#[post("/task")]
pub async fn submit(task: web::Json<NewTask>, state: web::Data<AppState>) -> Result<impl Responder> {
    log::info!("Received task submission request");
    let task = task.into_inner();
    let state = state.get_ref();
    let task_actor = state.task_actor_pool.get_actor();
    match task_actor.send(task).await {
        Ok(res) => {
            match res {
                Ok(task_id) => {
                    log::info!("Task submitted successfully");
                    Ok(HttpResponse::Ok().body(task_id.to_string()))
                },
                Err(e) => {
                    log::error!("Error submitting task: {}", e);
                    Ok(HttpResponse::InternalServerError().body("Error submitting task"))
                }
            }
        },
        Err(e) => {
            log::error!("Error submitting task: {}", e);
            Ok(HttpResponse::InternalServerError().body("Error submitting task"))
        }
    }
}