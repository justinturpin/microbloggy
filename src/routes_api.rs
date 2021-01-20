use super::{State, MessageFlashes};

use tide_tera::prelude::*;
use tide::{Request, Response, Redirect, Result};
use tide::prelude::json;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use std::vec::Vec;


#[derive(Serialize)]
struct DraftImageResponse {
    full_path: String,
    thumbnail_path: String
}

#[derive(Serialize)]
struct IndexResponse {
    draft_images: Vec<DraftImageResponse>
}


pub async fn index_api(req: Request<State>) -> Result<Response> {
    let state = req.state();
    let session = req.session();
    let mut db_conn = state.sqlite_pool.acquire().await?;

    let logged_in = session.get::<bool>("logged_in").unwrap_or(false);

    if logged_in {
        // Query for draft images
        let result = sqlx::query!(
                "SELECT image_thumbnail_path, image_full_path FROM image_drafts"
            )
            .fetch_all(&mut db_conn)
            .await?;

        let draft_images: Vec<DraftImageResponse> = result.into_iter().map(|row| {
            DraftImageResponse {
                full_path: row.image_full_path.unwrap(),
                thumbnail_path: row.image_thumbnail_path.unwrap()
            }
        }).collect();

        Ok(
            json!({
                "draft_images": draft_images
            })
            .into()
        )
    } else {
        Ok(
        // TODO: get the proper 400 response
        //     Response::builder(400)
        //         .body(json!({
        //             "unauthorized": true
        //         }))
        //         .content_type(tide::http::mime::JSON)
        //         .build()

            json!({
                "unauthorized": true
            })
            .into()
        )
    }
}
