use crate::utils::{error, redirect};
use crate::State;
use tide::{Request, Response};

pub async fn shortcut_get(req: Request<State>) -> tide::Result<Response> {
    let pool = req.state().pool();
    let url = req.url().path();

    let query = sqlx::query!("SELECT destination FROM shortcut WHERE id = ?", &url[1..])
        .fetch_optional(pool)
        .await?;
    if query.is_none() {
        let response = error(&req, "index.html", "Requested URL not found.")?;
        return Ok(response);
    }

    Ok(redirect(query.unwrap().destination))
}
