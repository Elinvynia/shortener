use crate::utils::{html, user};
use crate::{ctx, State};
use tide::{Request, Response};

pub async fn index_get(req: Request<State>) -> tide::Result<Response> {
    let context = ctx!("user" => user(req.session()));
    let rendered = req.state().with_context("index.html", &context)?;
    let response = html(rendered);
    Ok(response)
}
