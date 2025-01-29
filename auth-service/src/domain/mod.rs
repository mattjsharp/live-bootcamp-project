pub mod data_stores;
pub mod error;
pub mod user;
pub mod email_client;

pub use data_stores::*;
pub use error::*;
pub use user::*;
pub use email_client::*;

use core::convert::AsRef;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, String> {
        if !email.contains("@") || email.len() < 8 {
            return Err(format!("Invalid Email Address: {}", email));
        }
        Ok(Self(email.to_owned()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0[..]
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, String> {
        if password.len() < 8 {
            return Err("Invalid Password".to_owned());
        }
        Ok(Self(password.to_owned()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0[..]
    }
}
