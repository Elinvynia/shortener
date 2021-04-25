use crate::utils::html;
use crate::State;
use tide::{Request, Response};

pub async fn index_handler(req: Request<State>) -> tide::Result<Response> {
    let state = req.state();
    let rendered = state.render("index.html")?;
    let response = html(rendered);
    Ok(response)
}
