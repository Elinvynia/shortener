use sqlx::MySqlPool;
use std::sync::Arc;
use tide::sessions::{Session, SessionStore};

#[derive(Debug, Clone)]
pub struct DbStore {
    pool: Arc<MySqlPool>,
}

impl DbStore {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        DbStore { pool }
    }

    pub fn pool(&self) -> &MySqlPool {
        self.pool.as_ref()
    }
}

#[async_trait::async_trait]
impl SessionStore for DbStore {
    async fn load_session(&self, cookie_value: String) -> anyhow::Result<Option<Session>> {
        let pool = self.pool();
        let query = sqlx::query!("SELECT data FROM session WHERE session = ?", &cookie_value)
            .fetch_optional(pool)
            .await?;
        if let Some(q) = query {
            let session: Session = serde_json::from_str(&q.data)?;
            return Ok(Some(session));
        }
        Ok(None)
    }

    async fn store_session(&self, session: Session) -> anyhow::Result<Option<String>> {
        let user_id: u64 = match session.get("user_id") {
            Some(uid) => uid,
            None => return Ok(None),
        };

        let pool = self.pool();
        let data = serde_json::to_string(&session)?;
        let cookie_value = match session.into_cookie_value() {
            Some(cv) => cv,
            None => return Ok(None),
        };
        sqlx::query!(
            "INSERT INTO session (user_id, session, data) VALUES (?, ?, ?)",
            &user_id,
            &cookie_value,
            &data
        )
        .execute(pool)
        .await?;
        Ok(Some(cookie_value))
    }

    async fn destroy_session(&self, session: Session) -> anyhow::Result<()> {
        if let Some(id) = session.get::<u64>("user_id") {
            let pool = self.pool();
            sqlx::query!("DELETE FROM session WHERE user_id = ?", &id)
                .execute(pool)
                .await?;
        } else if let Some(cookie) = session.into_cookie_value() {
            let pool = self.pool();
            sqlx::query!("DELETE FROM session WHERE session = ?", &cookie)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

    async fn clear_store(&self) -> anyhow::Result<()> {
        let pool = self.pool();
        sqlx::query!("DELETE FROM session").execute(pool).await?;
        Ok(())
    }
}
