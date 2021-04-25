use crate::data::UrlData;
use crate::utils::{error, html, id, user};
use crate::State;
use tide::{Request, Response};
use url::Url;

pub async fn create_post(mut req: Request<State>) -> tide::Result<Response> {
    let data: UrlData = req.body_form().await?;
    let url = match Url::parse(&data.url) {
        Ok(u) => u,
        Err(_) => return error(req.state(), "index.html", "Invalid URL."),
    };
    let user = user(req.session());
    let id = id();
    let pool = req.state().pool();
    if user.id != 0 {
        sqlx::query!(
            "INSERT INTO shortcut (id, destination, datetime, created_by) VALUES (?, ?, NOW(), ?)",
            &id,
            url.as_str(),
            &user.id
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            "INSERT INTO shortcut (id, destination, datetime) VALUES (?, ?, NOW())",
            &id,
            url.as_str()
        )
        .execute(pool)
        .await?;
    }
    Ok(html("Create handler."))
}
