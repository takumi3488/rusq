use actix_web::{get, post, web, HttpResponse, Responder};
use redis::Commands;

use crate::utils::queue::{AllQueues, NewQueue, Queue, Queues};

#[get("/")]
pub async fn get_queues(client: web::Data<redis::Client>) -> impl Responder {
    let mut conn = client.get_connection().unwrap();
    let keys = conn.keys::<&str, Vec<String>>("rusq:*").unwrap();
    let queues: Vec<Queues> = keys
        .iter()
        .map(|k| Queues {
            name: k.to_string(),
            total: conn.llen::<&str, usize>(&k).unwrap(),
        })
        .collect();
    HttpResponse::Ok().json(AllQueues { queues })
}

#[post("/{queue}")]
pub async fn create_queue(
    path: web::Path<(String,)>,
    client: web::Data<redis::Client>,
    body: web::Json<NewQueue>,
) -> impl Responder {
    let queue = path.into_inner().0;
    let message = &body.message;
    let mut conn = client.get_connection().unwrap();
    let res = conn.rpush::<String, &str, usize>(format!("rusq:{}", &queue), message);
    match res {
        Ok(_) => HttpResponse::Created().json(Queue {
            queue,
            message: message.to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/{queue}")]
pub async fn get_first_queue(
    path: web::Path<(String,)>,
    client: web::Data<redis::Client>,
) -> impl Responder {
    let queue = path.into_inner().0;
    let mut conn = client.get_connection().unwrap();
    let res = conn.lpop::<String, String>(format!("rusq:{}", &queue), None);
    if res.is_err() {
        return HttpResponse::NotFound().body("message not found");
    }
    let message = res.unwrap();

    HttpResponse::Ok().json(Queue { queue, message })
}
