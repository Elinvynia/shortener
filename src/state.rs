use sqlx::MySqlPool;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Debug, Clone)]
pub struct State {
    pool: Arc<MySqlPool>,
    tera: Arc<Tera>,
}

impl State {
    pub fn new(pool: Arc<MySqlPool>, tera: Arc<Tera>) -> Self {
        State { pool, tera }
    }

    pub fn tera(&self) -> &Tera {
        self.tera.as_ref()
    }

    pub fn pool(&self) -> &MySqlPool {
        self.pool.as_ref()
    }

    pub fn render<T: AsRef<str>>(&self, template: T) -> tide::Result<String> {
        self.with_context(template, &Context::new())
    }

    pub fn with_context<T: AsRef<str>>(&self, template: T, context: &Context) -> tide::Result<String> {
        let template = template.as_ref();
        Ok(self.tera().render(template, &context)?)
    }

    pub fn error<T: AsRef<str>>(&self, template: T, error: T) -> tide::Result<String> {
        let template = template.as_ref();
        let error = error.as_ref();
        let mut context = Context::new();
        context.insert("error", error);
        self.with_context(template, &context)
    }
}
