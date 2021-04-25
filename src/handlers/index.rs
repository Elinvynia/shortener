use crate::utils::{html, user};
use crate::State;
use tera::Context;
use tide::{Request, Response};

pub async fn index_get(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let mut context = Context::new();
    let user = user(req.session());
    context.insert("user", &user);

    let rendered = state.with_context("index.html", &context)?;
    let response = html(rendered);
    Ok(response)
}
