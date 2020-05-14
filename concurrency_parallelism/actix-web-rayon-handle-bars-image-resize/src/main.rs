use actix_files as fs;
use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};
use std::collections::BTreeMap;
use std_logger::request;
use tokio::sync::mpsc;

use futures::executor;

use bytes::BytesMut;
use handlebars::Handlebars;
use image_utils;

async fn handle_multipart_post(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut handlebars = Handlebars::new();
    let mut data = BTreeMap::new();

    let image_template = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
        100x100 <p><img src={{image_url_1}}/> <p>
        400x400 <p><img src={{image_url_2}}/> <p>
        </body>
    </html>"#;

    match handlebars.register_template_string("result", image_template) {
        Ok(res) => {
            request!("handle bars registered {:?}", res);
        }
        Err(reason) => {
            request!("handle bars registered {:?}", reason);
        }
    };

    let (send, mut recv) = mpsc::unbounded_channel();
    let mut buf = Vec::with_capacity(1024);

    while let Ok(Some(mut field)) = payload.try_next().await {
        buf.clear();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            buf.extend_from_slice(&data);
        }

        let img = image::load_from_memory(&buf).unwrap();
        let send = send.clone();

        rayon::spawn(move || {
            let thumbnail_task_mem = || -> String {
                match image_utils::read_img_mem_resize(&img, 100, 100) {
                    Ok(res) => res,
                    rest => rest.unwrap().to_string(),
                }
            };

            let half_task_mem = || -> String {
                match image_utils::read_img_mem_resize(&img, 400, 400) {
                    Ok(res) => res,
                    rest => rest.unwrap().to_string(),
                }
            };

            let (tk_res, ht_res) = rayon::join(thumbnail_task_mem, half_task_mem);
            send.send(("image_url_1".to_string(), tk_res)).unwrap();
            send.send(("image_url_2".to_string(), ht_res)).unwrap();
        });
    }

    drop(send);

    while let Some((url, res)) = recv.recv().await {
        data.insert(url, res);
    }

    let html = handlebars.render_template(image_template, &data).unwrap();
    Ok(HttpResponse::Ok().body(html).into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std_logger::init();

    let ip = "0.0.0.0:3000";

    request!("Started server {:?}", ip);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index))
                    .route(web::post().to(handle_multipart_post)),
            )
            .service(
                fs::Files::new("/test-image/", "./test-image")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind(ip)?
    .run()
    .await
}
