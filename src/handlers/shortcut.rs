use crate::utils::html;
use crate::State;
use tide::{Request, Response};

pub async fn shortcut_handler(_req: Request<State>) -> tide::Result<Response> {
    Ok(html("Shortcut handler."))
}
