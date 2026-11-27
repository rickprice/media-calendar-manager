use axum_login::AuthUser;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user() -> User {
        User {
            id: Uuid::nil(),
            email: "person@example.com".to_string(),
            password_hash: "$argon2id$v=19$dummy-hash".to_string(),
        }
    }

    #[test]
    fn id_returns_the_users_uuid() {
        let user = test_user();
        assert_eq!(user.id(), user.id);
    }

    #[test]
    fn session_auth_hash_is_the_password_hash_bytes() {
        let user = test_user();
        assert_eq!(user.session_auth_hash(), user.password_hash.as_bytes());
    }

    #[test]
    fn session_auth_hash_changes_when_password_hash_changes() {
        let mut user = test_user();
        let original_hash = user.session_auth_hash().to_vec();

        user.password_hash = "$argon2id$v=19$a-different-hash".to_string();

        assert_ne!(user.session_auth_hash(), original_hash.as_slice());
    }

    #[test]
    fn credentials_deserialize_from_form_encoded_pairs() {
        let creds: Credentials =
            serde_urlencoded::from_str("email=a%40example.com&password=secret").unwrap();

        assert_eq!(creds.email, "a@example.com");
        assert_eq!(creds.password, "secret");
    }

    #[test]
    fn register_form_deserializes_from_form_encoded_pairs() {
        let form: RegisterForm =
            serde_urlencoded::from_str("email=new%40example.com&password=hunter2").unwrap();

        assert_eq!(form.email, "new@example.com");
        assert_eq!(form.password, "hunter2");
    }
}
