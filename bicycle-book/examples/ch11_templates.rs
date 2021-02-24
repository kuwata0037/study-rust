use actix_web::{error, http, server, App, HttpResponse, Path, State};
use serde_derive::*;
use tera::{compile_templates, Context, Tera};

struct AppSate {
    template: Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_template(
    app: State<AppSate>,
    path: Path<HelloPath>,
) -> Result<HttpResponse, error::Error> {
    // テンプレートに渡す値を作る
    let mut context = Context::new();
    context.insert("name", &path.name);
    let body = app
        .template
        .render("index.html.tera", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    Ok(HttpResponse::Ok().body(body))
}

fn main() {
    server::new(|| {
        let app = AppSate {
            template: compile_templates!("bicycle-book/examples/templates/*"),
        };
        App::with_state(app).route("/{name}", http::Method::GET, hello_template)
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
