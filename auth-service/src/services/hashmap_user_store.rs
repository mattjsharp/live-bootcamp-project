use std::collections::HashMap;

use crate::domain::{User, UserStoreError, UserStore};

// stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    // Return `UserStoreError::UserAlreadyExists` if the user already exists,
    // otherwise insert the user into the hashmap and return `Ok(())`.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists)
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    // Takes an immutable reference to self and an email string slice as arguments.
    // Returns a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            return Ok(User::new(user.email.clone(), user.password.clone(), user.requires_2fa))
        }
        Err(UserStoreError::UserNotFound)
    }

    // Takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. returns a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.password == password {
                return Ok(())
            }
            return Err(UserStoreError::InvalidCredentials)
        }
        Err(UserStoreError::UserNotFound)
    }
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let users = setup_users();

        let mut user_store = HashmapUserStore{users: HashMap::new()};

        for user in users {
            assert_ne!(user_store.add_user(user).await, Err(UserStoreError::UserAlreadyExists));
        }
    }

    #[tokio::test]
    async fn test_get_user() {
        let valid_users = setup_users();
        let invlaid_users = invalid_users_emails();
        let user_store = setup_user_store();

        // Testing for valid users
        for valid_user in &valid_users {
            let user = user_store.get_user(&valid_user.email).await;
            assert_eq!(user.unwrap().email, valid_user.email);
        }

        // Testing for users not present
        for invalid_user in &invlaid_users {
            if let Err(_) = user_store.get_user(&invalid_user).await {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let valid_users = setup_users();
        let invlaid_users = invalid_users_emails();
        let user_store = setup_user_store();
        let invalid_password = "not_the_password";

        // Testing for users not present
        for invalid_user in &invlaid_users {
            if let Err(_) = user_store.validate_user(&invalid_user, &invalid_password).await {
                assert!(true);
            } else {
                assert!(false);
            }
        }

        // Testing for valid users
        for valid_user in &valid_users {
            let user = user_store.validate_user(&valid_user.email, &valid_user.password).await;
            assert_eq!(user.unwrap(), ());
        }

        // Testing for present users with wrong passwords
        for valid_user in &valid_users {
            if let Err(_) = user_store.validate_user(&valid_user.email, &invalid_password).await {
                assert!(true);
            } else {
                assert!(false);
            }
        }
    }

    fn setup_users() -> Vec<User> {
        vec![
            User::new(String::from("joebiden@whitehouse.gov"), String::from("123456"), false),
            User::new(String::from("donaldtrump@whitehouse.gov"), String::from("654321"), false),
            User::new(String::from("barakobama@whitehouse.gov"), String::from("2468"), true)
        ]
    }

    fn invalid_users_emails() -> Vec<String> {
        vec![
            "hillaryclinton@senate.gov".to_owned(), 
            "berniesanders@senate.gov".to_owned(),
            "kamalaharris@whitehouse.gov".to_owned()
            ]
    }

    fn setup_user_store() -> HashmapUserStore {
        let mut user_store = HashMap::new();
        let users = setup_users();

        for user in users {
            user_store.insert(user.email.clone(), user);
        }

        HashmapUserStore {
            users: user_store
        }
    }
}