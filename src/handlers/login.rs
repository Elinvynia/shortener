use crate::data::LoginData;
use crate::state::State;
use crate::utils::{html, redirect, user};
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
    let _data: LoginData = req.body_form().await?;
    Ok(html("Login POST endpoint."))
}
