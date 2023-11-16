use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use askama::Template;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let template = IndexTemplate {name: "Robert"};

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(template.render()?.into())?
        )
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: &'static str
}
