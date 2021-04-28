use crate::data::LoginData;
use crate::state::State;
use crate::utils::{error, html, redirect, user};
use bcrypt::verify;
use tide::{Request, Response};

pub async fn login_get(req: Request<State>) -> tide::Result<Response> {
    let user = user(req.session());
    if user.logged_in {
        return Ok(redirect("/"));
    }

    let rendered = req.state().render("login.html")?;
    let response = html(rendered);
    Ok(response)
}

pub async fn login_post(mut req: Request<State>) -> tide::Result<Response> {
    let data: LoginData = req.body_form().await?;
    let pool = req.state().pool();
    let query = sqlx::query!(
        "SELECT id, username, password FROM user WHERE username = ?",
        &data.username
    )
    .fetch_optional(pool)
    .await?;
    if query.is_none() {
        let response = error(&req, "login.html", "User not found.")?;
        return Ok(response);
    }

    let query = query.unwrap();
    let result = verify(&data.password, &query.password)?;
    if !result {
        let response = error(&req, "login.html", "Invalid password.")?;
        return Ok(response);
    }

    req.session_mut().insert("user_id", &query.id)?;
    req.session_mut().insert("username", &query.username)?;

    Ok(redirect("/"))
}
