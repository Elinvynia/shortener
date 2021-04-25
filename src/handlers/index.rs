use crate::utils::{html, user};
use crate::State;
use tera::Context;
use tide::{Request, Response};

pub async fn index_handler(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let mut context = Context::new();
    let user = user(req.session());
    context.insert("user", &user);

    let rendered = state.render("index.html")?;
    let response = html(rendered);
    Ok(response)
}
