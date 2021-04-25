use crate::data::User;
use nanoid::nanoid;
use tide::http::mime;
use tide::sessions::Session;
use tide::Response;

const ALPHABET: [char; 57] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
    'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];

pub fn id() -> String {
    nanoid!(5, &ALPHABET)
}

pub fn html<B: AsRef<str>>(body: B) -> Response {
    let body = body.as_ref();
    Response::builder(200).content_type(mime::HTML).body(body).build()
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
