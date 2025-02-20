use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

// stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    // Return `UserStoreError::UserAlreadyExists` if the user already exists,
    // otherwise insert the user into the hashmap and return `Ok(())`.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    // Takes an immutable reference to self and an email string slice as arguments.
    // Returns a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        if let Some(user) = self.users.get(email) {
            return Ok(User::new(
                user.email.clone(),
                user.password.clone(),
                user.requires_2fa,
            ));
        }
        Err(UserStoreError::UserNotFound)
    }

    // Takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. returns a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(email) {
            if user.password == *password {
                return Ok(());
            }
            return Err(UserStoreError::InvalidCredentials);
        }
        Err(UserStoreError::UserNotFound)
    }
}

impl HashmapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use fake::faker::internet::en::{Password as FakerPassword, SafeEmail};
    use fake::Fake;

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = Email::parse(&SafeEmail().fake::<String>()).unwrap();

        let new_password: String = FakerPassword(8..16).fake();
        let password = Password::parse(&new_password).unwrap();

        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User::new(email, password, true);

        assert_ne!(
            user_store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let email = Email::parse(&SafeEmail().fake::<String>()).unwrap();

        let new_password: String = FakerPassword(8..16).fake();
        let password = Password::parse(&new_password).unwrap();

        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User::new(email.clone(), password, true);

        user_store
            .add_user(user)
            .await
            .expect("Failed to add account");

        let user = user_store.get_user(&email).await;
        assert_eq!(user.unwrap().email, email.clone());

        // if let Err(_) = user_store.get_user(&Email::parse("goldfish").unwrap()).await {
        //     assert!(true, "get_user Passes");
        // } else {
        //     assert!(false, "get_user Fails");
        // }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let invalid_password = "not_the_password";

        let email = Email::parse(&SafeEmail().fake::<String>()).unwrap();

        let new_password: String = FakerPassword(8..16).fake();
        let password = Password::parse(&new_password).unwrap();

        let mut user_store = HashmapUserStore {
            users: HashMap::new(),
        };

        let user = User::new(email.clone(), password.clone(), true);

        user_store
            .add_user(user)
            .await
            .expect("Failed to add account");

        if let Err(_) = user_store
            .validate_user(&email, &Password::parse(invalid_password).unwrap())
            .await
        {
            assert!(true, "validate_user passes");
        } else {
            assert!(false, "validate_user fails");
        }

        assert_eq!(
            user_store.validate_user(&email, &password).await.unwrap(),
            (),
            "validate_user fails"
        );
    }
}
