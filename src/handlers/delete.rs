use crate::utils::{error, message};
use crate::State;
use tide::{Request, Response};

pub async fn delete_get(req: Request<State>) -> tide::Result<Response> {
    let delete_code = req.param("code")?;
    let pool = req.state().pool();
    let query = sqlx::query!("SELECT * FROM shortcut WHERE delete_code = ?", &delete_code)
        .fetch_optional(pool)
        .await?;
    if query.is_none() {
        let response = error(&req, "index.html", "Invalid delete code.")?;
        return Ok(response);
    }

    sqlx::query!("DELETE FROM shortcut WHERE delete_code = ?", &delete_code)
        .execute(pool)
        .await?;

    let response = message(&req, "index.html", "URL shortcut deleted")?;
    Ok(response)
}
