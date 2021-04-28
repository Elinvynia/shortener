use crate::data::Url;
use crate::utils::{html, redirect, user};
use crate::{ctx, State};
use tide::{Request, Response};

pub async fn dashboard_get(req: Request<State>) -> tide::Result<Response> {
    let user = user(req.session());
    if !user.logged_in {
        return Ok(redirect("/"));
    }

    let pool = req.state().pool();
    let urls = sqlx::query!(
        "SELECT id, destination, datetime, delete_code FROM shortcut WHERE created_by = ? ORDER BY datetime DESC",
        &user.id
    )
    .fetch_all(pool)
    .await?;
    let urls: Vec<Url> = urls
        .into_iter()
        .map(|r| Url {
            id: r.id,
            destination: r.destination,
            created_at: r.datetime.to_string(),
            delete_code: r.delete_code,
        })
        .collect();

    let context = ctx!("urls" => urls, "user" => user);
    let rendered = req.state().with_context("dashboard.html", &context)?;
    let response = html(rendered);
    Ok(response)
}
