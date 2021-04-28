use crate::ctx;
use crate::data::User;
use crate::state::State;
use nanoid::nanoid;
use tide::http::mime;
use tide::sessions::Session;
use tide::{Request, Response};

const ALPHABET: [char; 57] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn error<T: AsRef<str>>(req: &Request<State>, template: T, error: T) -> tide::Result<Response> {
    let error = error.as_ref();
    let user = user(req.session());
    let context = ctx!("error" => error, "user" => user);
    let rendered = req.state().with_context(template, &context)?;
    Ok(html(rendered))
}

pub fn message<T: AsRef<str>>(req: &Request<State>, template: T, message: T) -> tide::Result<Response> {
    let message = message.as_ref();
    let user = user(req.session());
    let context = ctx!("message" => message, "user" => user);
    let rendered = req.state().with_context(template, &context)?;
    Ok(html(rendered))
}

pub fn generate_id(length: usize) -> String {
    nanoid!(length, &ALPHABET)
}

pub fn html<B: AsRef<str>>(body: B) -> Response {
    let body = body.as_ref();
    Response::builder(200).content_type(mime::HTML).body(body).build()
}

pub fn redirect<L: AsRef<str>>(location: L) -> Response {
    Response::builder(302)
        .body("")
        .header("location", location.as_ref())
        .build()
}

pub fn user(session: &Session) -> User {
    let mut user = User::default();

    if let Some(uid) = session.get::<u64>("user_id") {
        user.logged_in = true;
        user.id = uid;
        user.username = session
            .get_raw("username")
            .expect("User ID is set without an username.")
    }

    user
}

#[macro_export]
macro_rules! ctx {
    ($($key:expr => $value:expr,)+) => { context!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _context = ::tera::Context::new();
            $(
                _context.insert($key, &$value);
            )*
            _context
        }
     };
}
