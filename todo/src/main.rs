use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;
use askama::Template;

// TODOエントリ
struct TodoEntry {
    id: u32,
    text: String,
}

// index.htmlのテンプレート
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

// 自作エラー
#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}
impl ResponseError for MyError {}

// コントローラ
#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let mut entries = Vec::new();
    entries.push(TodoEntry {
        id: 1,
        text: "First entry".to_string(),
    });
    entries.push(TodoEntry {
        id: 2,
        text: "Second entry".to_string(),
    });


    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    // index関数を渡してHTTPサーバを起動
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
