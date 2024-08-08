use crate::application::dtos::task_dtos::{GetTask, NewTask};
use crate::shared::app_state::AppState;
use crate::shared::errors::Result;
use actix_web::http::header::ContentType;
use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use uuid::Uuid;

#[post("/task")]
pub async fn submit(
    task: web::Json<NewTask>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    log::info!("Received task submission request");
    let task = task.into_inner();
    let state = state.get_ref();
    let task_actor = state.task_actor_pool.get_actor();
    match task_actor.send(task).await {
        Ok(res) => match res {
            Ok(task_id) => {
                log::info!("Task submitted successfully");
                Ok(HttpResponse::Ok().body(task_id.to_string()))
            }
            Err(e) => {
                log::error!("Error submitting task: {}", e);
                Ok(HttpResponse::InternalServerError().body("Error submitting task"))
            }
        },
        Err(e) => {
            log::error!("Error submitting task: {}", e);
            Ok(HttpResponse::InternalServerError().body("Error submitting task"))
        }
    }
}

#[get("/task/sse/{task_id}")]
pub async fn notify(task_id: web::Path<(Uuid,)>, state: web::Data<AppState>) -> impl Responder {
    let task = GetTask {
        id: task_id.into_inner().0,
    };
    let actor = state.task_actor_pool.get_actor();
    let res = actor.send(task).await;
    match res {
        Ok(Some(task)) => {
            let task = serde_json::to_string(&task).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(task)
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            log::error!("Failed to get task. Cause: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
