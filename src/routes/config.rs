use actix_web::web;

use super::post::{get_posts, get_post_by_id, create_post, update_post, delete_post};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_posts)
        .service(get_post_by_id)
        .service(create_post)
        .service(update_post)
        .service(delete_post);

    conf.service(scope);
}