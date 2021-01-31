use std::sync::Arc;
use std::fs::{read_to_string, write};
use tide::prelude::Deserialize;
use tide::{Redirect, Request, Response};
use tide::http::{mime, Url};
use tera::{Tera, Context};
use nanoid::nanoid;

const LOCATION: &str = "127.0.0.1:8000";
const ALPHABET: [char; 55] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j',
    'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
    'Z'
];

#[async_std::main]
async fn main() {
    let tera = Arc::new(Tera::new("templates/*.tera").unwrap());
    let state = State { tera };

    let mut app = tide::with_state(state);


    app.at("/").get(|req: Request<State>| async move {
        Ok(req.state().render("index.tera"))
    });

    app.at("create").post(|mut req: Request<State>| async move {
        let urlform: UrlForm = req.body_form().await?;
        let id = nanoid!(4, &ALPHABET);
        write(format!("shortcuts/{}", id), urlform.url.as_str())?;
        let mut context = Context::new();
        context.insert("shortcut", &format!("{}/{}", LOCATION, id));
        Ok(req.state().render_with_context("create.tera", &context))
    });
    
    app.at("*").get(|req: Request<State>| async move {
        let url = read_to_string(format!("shortcuts/{}", &req.url().path()[1..]))?;
        Ok(Redirect::new(url))
    });


    app.listen(LOCATION).await.unwrap();
}

#[derive(Debug, Clone, Deserialize)]
struct UrlForm {
  url: Url,
}

#[derive(Debug, Clone)]
struct State {
    tera: Arc<Tera>
}

impl State {
    fn tera(&self) -> &Tera {
        self.tera.as_ref()
    }
    fn render(&self, name: &str) -> Response {
        let render = self
            .tera()
            .render(name, &Context::new())
            .unwrap_or_else(|err| err.to_string());

        Response::builder(200)
            .content_type(mime::HTML)
            .body(render)
            .build()
    }
    fn render_with_context(&self, name: &str, context: &Context) -> Response {
        let render = self
            .tera()
            .render(name, context)
            .unwrap_or_else(|err| err.to_string());

        Response::builder(200)
            .content_type(mime::HTML)
            .body(render)
            .build()
    }
}
