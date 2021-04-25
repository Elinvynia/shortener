use crate::data::RegisterData;
use crate::state::State;
use crate::utils::{error, html, redirect, user};
use bcrypt::{hash, DEFAULT_COST};
use tide::{Request, Response};

pub async fn register_get(req: Request<State>) -> tide::Result<Response> {
    let user = user(req.session());
    if user.logged_in {
        return Ok(redirect("/"));
    }

    let rendered = req.state().render("register.html")?;
    let response = html(rendered);
    Ok(response)
}

pub async fn register_post(mut req: Request<State>) -> tide::Result<Response> {
    let data: RegisterData = req.body_form().await?;

    if data.password != data.password_repeat {
        let response = error(req.state(), "template", "Passwords don't match.")?;
        return Ok(response);
    }

    let pool = req.state().pool();
    let query = sqlx::query!("SELECT * FROM user WHERE username = ?", &data.username)
        .fetch_optional(pool)
        .await?;
    if query.is_some() {
        let response = error(req.state(), "template", "User with this name already exists.")?;
        return Ok(response);
    }

    let hashed = hash(&data.password, DEFAULT_COST)?;
    sqlx::query!(
        "INSERT INTO user (username, password) VALUES (?, ?)",
        &data.username,
        &hashed
    )
    .execute(pool)
    .await?;

    Ok(redirect("/"))
}
