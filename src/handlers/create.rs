use crate::data::UrlData;
use crate::utils::{error, generate_id, message, user};
use crate::State;
use tide::{Request, Response};
use url::Url;

pub async fn create_post(mut req: Request<State>) -> tide::Result<Response> {
    let data: UrlData = req.body_form().await?;
    let url = match Url::parse(&data.url) {
        Ok(u) => u,
        Err(_) => return error(&req, "index.html", "Invalid URL."),
    };

    let user = user(req.session());
    let shortcut_id = generate_id(5);
    let delete_code = generate_id(10);
    let pool = req.state().pool();
    if user.logged_in {
        sqlx::query!(
            "INSERT INTO shortcut (id, destination, datetime, created_by, delete_code) VALUES (?, ?, NOW(), ?, ?)",
            &shortcut_id,
            url.as_str(),
            &user.id,
            &delete_code
        )
        .execute(pool)
        .await?;
    } else {
        sqlx::query!(
            "INSERT INTO shortcut (id, destination, datetime, delete_code) VALUES (?, ?, NOW(), ?)",
            &shortcut_id,
            url.as_str(),
            &delete_code,
        )
        .execute(pool)
        .await?;
    }

    let url = format!("{}/{}", &req.url().host_str().unwrap(), &shortcut_id);
    let delete_url = format!("{}/delete/{}", &req.url().host_str().unwrap(), &delete_code);
    let response = message(
        &req,
        "index.html",
        &format!(
            "URL created at <a href='/{}'>{}</a>!<br>You can delete it via <a href='/delete/{}'>{}</a>'>",
            &shortcut_id, &url, &delete_code, &delete_url
        ),
    )?;

    Ok(response)
}
