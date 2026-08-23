use crate::auth::crypto::Crypto;
use crate::auth::rbac::{AuthUser, Role, Session};
use crate::core::error::OxideError;
use sqlx::{Pool, Row, Sqlite};
use tracing::info;

pub struct AuthService {
    pool: Pool<Sqlite>,
}

impl AuthService {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Ensure default superadmin account exists
    pub async fn ensure_admin_user(&self) -> Result<(), OxideError> {
        let count_row = sqlx::query("SELECT COUNT(*) as count FROM _oxide_users")
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = count_row.try_get("count")?;

        if count == 0 {
            let pass_hash = Crypto::hash_password("admin");
            sqlx::query(
                r#"
                INSERT INTO _oxide_users (username, email, password_hash, role, is_active)
                VALUES ('admin', 'admin@example.com', ?, 'Admin', 1)
                "#
            )
            .bind(pass_hash)
            .execute(&self.pool)
            .await?;

            info!("✨ Auto-generated Superadmin account => Username: 'admin', Password: 'admin'");
        }
        Ok(())
    }

    /// Authenticate a user and create a 7-day session
    pub async fn login(
        &self,
        username: &str,
        password: &str,
        ip: Option<String>,
        ua: Option<String>,
    ) -> Result<Option<Session>, OxideError> {
        let row_opt = sqlx::query(
            "SELECT id, username, email, password_hash, role, is_active FROM _oxide_users WHERE username = ? LIMIT 1"
        )
        .bind(username.to_lowercase())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let pass_hash: String = row.try_get("password_hash")?;
            let is_active: i64 = row.try_get("is_active")?;

            if is_active == 0 || !Crypto::verify_password(password, &pass_hash) {
                return Ok(None);
            }

            let user_id: i64 = row.try_get("id")?;
            let role_str: String = row.try_get("role")?;
            let role = Role::from_str(&role_str);

            let token = Crypto::random_token(32);
            let expires_at = (chrono::Utc::now() + chrono::Duration::days(7)).to_rfc3339();

            let session_id = sqlx::query(
                r#"
                INSERT INTO _oxide_sessions (token, user_id, ip_address, user_agent, expires_at)
                VALUES (?, ?, ?, ?, ?)
                "#
            )
            .bind(&token)
            .bind(user_id)
            .bind(ip)
            .bind(ua)
            .bind(&expires_at)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

            let _ = sqlx::query("UPDATE _oxide_users SET last_login = datetime('now') WHERE id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await;

            return Ok(Some(Session {
                id: session_id,
                token,
                user_id,
                username: username.to_string(),
                role,
                expires_at,
            }));
        }

        Ok(None)
    }

    /// Validate session token from Cookie or Bearer header
    pub async fn validate_session(&self, token: &str) -> Result<Option<AuthUser>, OxideError> {
        let row_opt = sqlx::query(
            r#"
            SELECT u.id, u.username, u.email, u.role, u.is_active, s.expires_at
            FROM _oxide_sessions s
            JOIN _oxide_users u ON s.user_id = u.id
            WHERE s.token = ? AND u.is_active = 1
            LIMIT 1
            "#
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row_opt {
            let role_str: String = row.try_get("role")?;
            return Ok(Some(AuthUser {
                id: row.try_get("id")?,
                username: row.try_get("username")?,
                email: row.try_get("email")?,
                role: Role::from_str(&role_str),
                is_active: true,
            }));
        }

        Ok(None)
    }

    /// Invalidate session
    pub async fn logout(&self, token: &str) -> Result<(), OxideError> {
        sqlx::query("DELETE FROM _oxide_sessions WHERE token = ?")
            .bind(token)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
