
use crate::models::post::{PostModel, CreatePostSchema, UpdatePostSchema};

use crate::AppState;


// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use chrono::Utc;
use serde_json::json;

#[get("/posts")]
pub async fn get_posts(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        PostModel,
        "SELECT * FROM posts"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message: &str = "Something bad happened while fetching the posts";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let posts = query_result.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "no. posts": posts.len(),
        "posts": posts
    }))
}

#[post("/posts/post")]
async fn create_post(body: web::Json<CreatePostSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        PostModel,
        "INSERT into posts (message, username, day) values ($1, $2, $3) returning *",
        body.message.to_string(),
        body.username.to_string(),
        body.day.to_string()
    ).fetch_one(&data.db)
    .await;

    match query_result {
        Ok(post) => {
            let post_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "post": post
            })});
            return HttpResponse::Ok().json(post_response);
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}))
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/posts/post/{id}")]
async fn get_post_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let post_id = path.into_inner();
    let query_result = sqlx::query_as!(PostModel, "SELECT * FROM posts WHERE id = $1", post_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(post) => {
            let post_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "post": post
            })});
            return HttpResponse::Ok().json(post_response);
        }
        Err(_) => {
            let message = format!("Post with ID: {} not found", post_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("/posts/post/{id}")]
async fn update_post(path: web::Path<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdatePostSchema>) -> impl Responder {
    let post_id = path.into_inner();
    // make sure post exists before updating
    let query_result = sqlx::query_as!(PostModel, "SELECT * FROM posts where id = $1", post_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("post with ID: {} not found", post_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let post = query_result.unwrap();

    let query_result = sqlx::query_as!(
        PostModel,
        "UPDATE posts set message = $1, username = $2, day = $3, updated_at = $4 where id = $5 returning *",
        body.message.to_owned().unwrap_or(post.message),
        body.username.to_owned().unwrap_or(post.username),
        body.day.to_owned().unwrap_or(post.day),
        now,
        post_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(post) => {
            let post_response = serde_json::json!({"state": "success", "data": serde_json::json!({
                "post": post
            })});
            return HttpResponse::Ok().json(post_response);
        }
        Err (_) => {
            let message = format!("post with ID: {} not found", post_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}))
        }
    }
}

#[delete("/posts/post/{id}")]
async fn delete_post(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let post_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE from posts WHERE id = $1", post_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("post with ID: {} not found", post_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
    }
    HttpResponse::NoContent().finish()
}