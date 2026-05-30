use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::{domain::email::Email, services::data_store::{TwoFACodeStore, TwoFACodeStoreError}};
use super::data_store::{LoginAttemptId, TwoFACode};


#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: RwLock<HashMap<Email, (LoginAttemptId, TwoFACode)>>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let mut guard  = self.codes.write().await;

        if guard.contains_key(&email) {
            return Err(TwoFACodeStoreError::UnexpectedError);
        }
        guard.insert(email, (login_attempt_id, code));

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let mut guard  = self.codes.write().await;

        guard.remove(email).ok_or(TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let guard  = self.codes.read().await;

        let res = guard.get(email).cloned().ok_or(TwoFACodeStoreError::UnexpectedError)?;
        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::email::Email;
    use crate::services::data_store::TwoFACodeStoreError;


    #[tokio::test]
    async fn add_code_then_get_code_returns_inserted_values() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        let (got_login_attempt_id, got_code) = store.get_code(&email).await.unwrap();
        assert_eq!(got_login_attempt_id, login_attempt_id);
        assert_eq!(got_code, code);
    }

    #[tokio::test]
    async fn add_code_twice_for_same_email_returns_error() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id1 = LoginAttemptId::default();
        let code1 = TwoFACode::parse("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id1, code1)
            .await
            .unwrap();

        let login_attempt_id2 = LoginAttemptId::default();
        let code2 = TwoFACode::parse("654321".to_string()).unwrap();

        let err = store
            .add_code(email.clone(), login_attempt_id2, code2)
            .await
            .unwrap_err();

        assert_eq!(err, TwoFACodeStoreError::UnexpectedError);
    }

    #[tokio::test]
    async fn remove_code_then_get_code_returns_error() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::parse("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id, code)
            .await
            .unwrap();

        store.remove_code(&email).await.unwrap();

        let err = store.get_code(&email).await.unwrap_err();
        assert_eq!(err, TwoFACodeStoreError::UnexpectedError);
    }

    #[tokio::test]
    async fn remove_code_for_missing_email_returns_error() {
        let mut store = HashmapTwoFACodeStore::default();

        let email = Email::parse("missing@example.com".to_string()).unwrap();
        let err = store.remove_code(&email).await.unwrap_err();

        assert_eq!(err, TwoFACodeStoreError::UnexpectedError);
    }

    #[tokio::test]
    async fn get_code_for_missing_email_returns_error() {
        let store = HashmapTwoFACodeStore::default();

        let email = Email::parse("missing@example.com".to_string()).unwrap();
        let err = store.get_code(&email).await.unwrap_err();

        assert_eq!(err, TwoFACodeStoreError::UnexpectedError);
    }
}

