use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(val) => Ok((val.0.clone(), val.1.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut code_store = HashmapTwoFACodeStore { codes: HashMap::new() };

        let email = Email::parse("joebiden@whitehouse.gov").expect("Invalid Email");
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = code_store.add_code(email, login_attempt_id, code).await.expect("Undable to add code");

        assert_eq!(result, ());
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut code_store = HashmapTwoFACodeStore { codes: HashMap::new() };

        let email = Email::parse("joebiden@whitehouse.gov").expect("Invalid Email");
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = code_store.add_code(email.clone(), login_attempt_id, code).await.expect("Undable to add code");

        assert_eq!(result, ());

        let result = code_store.remove_code(&email).await.expect("Undable to remove code");

        assert_eq!(result, ());

        let result = code_store.get_code(&email).await;

        assert_eq!(result, Err(TwoFACodeStoreError::LoginAttemptIdNotFound));
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut code_store = HashmapTwoFACodeStore { codes: HashMap::new() };

        let email = Email::parse("joebiden@whitehouse.gov").expect("Invalid Email");
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = code_store.add_code(email.clone(), login_attempt_id.clone(), code.clone()).await.expect("Undable to add code");

        assert_eq!(result, ());

        let result = code_store.get_code(&email).await.expect("Unable to get code");

        assert_eq!(result, (login_attempt_id, code));
    }
}