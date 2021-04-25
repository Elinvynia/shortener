use crate::utils::id;
use crate::State;
use tide::{Request, Response};

pub async fn create_handler(_req: Request<State>) -> tide::Result<Response> {
    let _pool = _req.state().pool();
    let _id = id();
    todo!()
}
