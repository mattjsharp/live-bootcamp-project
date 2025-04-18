use rand::Rng;
use uuid::Uuid;

use super::{Email, Password, User};

#[async_trait::async_trait]
pub trait UserStore: Sync + Send {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(pub String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        let result = Uuid::parse_str(&id);
        match result {
            Ok(_) => Ok(LoginAttemptId(id)),
            Err(_) => Err("Not a valid uuid".to_owned()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(pub String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.len() == 6 {
            Ok(Self(code))
        } else {
            Err("Invalid 2FA Code".to_owned())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut code = String::new();
        for _ in 0..6 {
            code.push_str(&format!("{}", rand::thread_rng().gen_range(0..10)));
        }
        Self(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
