use crate::utils::{redirect, user};
use crate::State;
use tide::{Request, Response};

pub async fn logout_get(mut req: Request<State>) -> tide::Result<Response> {
    let user = user(req.session());
    if !user.logged_in {
        return Ok(redirect("/"));
    }

    req.session_mut().destroy();

    Ok(redirect("/"))
}
