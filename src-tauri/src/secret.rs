use std::collections::HashMap;
use std::sync::Mutex;

const SERVICE: &str = "com.datell1357.human-resources-office";

#[derive(Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub enum SecretError {
    NotFound,
    Backend(String),
}

impl std::fmt::Display for SecretError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(formatter, "저장된 비밀번호를 찾지 못했습니다."),
            Self::Backend(message) => write!(formatter, "비밀번호 저장소 오류: {message}"),
        }
    }
}

/// 비밀번호는 데이터베이스에 남기지 않고 이 저장소로만 다룬다.
#[cfg_attr(not(test), allow(dead_code))]
pub trait SecretStore: Send + Sync {
    fn store(&self, reference: &str, secret: &str) -> Result<(), SecretError>;
    fn retrieve(&self, reference: &str) -> Result<String, SecretError>;
    #[allow(dead_code)]
    fn delete(&self, reference: &str) -> Result<(), SecretError>;
}

pub struct KeychainStore;

impl KeychainStore {
    pub fn new() -> Self {
        Self
    }

    fn entry(reference: &str) -> Result<keyring::Entry, SecretError> {
        keyring::Entry::new(SERVICE, reference)
            .map_err(|error| SecretError::Backend(error.to_string()))
    }
}

impl Default for KeychainStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretStore for KeychainStore {
    fn store(&self, reference: &str, secret: &str) -> Result<(), SecretError> {
        Self::entry(reference)?
            .set_password(secret)
            .map_err(|error| SecretError::Backend(error.to_string()))
    }

    fn retrieve(&self, reference: &str) -> Result<String, SecretError> {
        match Self::entry(reference)?.get_password() {
            Ok(secret) => Ok(secret),
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(error) => Err(SecretError::Backend(error.to_string())),
        }
    }

    fn delete(&self, reference: &str) -> Result<(), SecretError> {
        match Self::entry(reference)?.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Err(SecretError::NotFound),
            Err(error) => Err(SecretError::Backend(error.to_string())),
        }
    }
}

/// 테스트와 데모 실행에서 실제 키체인을 건드리지 않기 위한 대체 구현이다.
#[derive(Default)]
pub struct MemorySecretStore {
    entries: Mutex<HashMap<String, String>>,
}

impl SecretStore for MemorySecretStore {
    fn store(&self, reference: &str, secret: &str) -> Result<(), SecretError> {
        self.entries
            .lock()
            .map_err(|_| SecretError::Backend("잠금 획득 실패".to_string()))?
            .insert(reference.to_string(), secret.to_string());
        Ok(())
    }

    fn retrieve(&self, reference: &str) -> Result<String, SecretError> {
        self.entries
            .lock()
            .map_err(|_| SecretError::Backend("잠금 획득 실패".to_string()))?
            .get(reference)
            .cloned()
            .ok_or(SecretError::NotFound)
    }

    fn delete(&self, reference: &str) -> Result<(), SecretError> {
        self.entries
            .lock()
            .map_err(|_| SecretError::Backend("잠금 획득 실패".to_string()))?
            .remove(reference)
            .map(|_| ())
            .ok_or(SecretError::NotFound)
    }
}
