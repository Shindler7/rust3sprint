//! Работа с JWT токенами.

use crate::{
    domain::types::{DataId, Username},
    settings::JWT_LIFETIME,
};
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

/// Используемый алгоритм шифрования токена.
const JWT_ALGORITHM: Algorithm = Algorithm::HS256;

/// Структура `Claims` для обработки JWT-токенов.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    /// Id пользователя.
    pub user_id: DataId,
    /// Username пользователя.
    pub username: Username,
    /// Время истечения.
    pub exp: usize,
}

impl Claims {
    /// Создать новый экземпляр [`Claims`]. Время истечения токена формируется
    /// автоматически.
    pub(crate) fn new(user_id: &DataId, username: &Username) -> Self {
        let expiration = Utc::now() + JWT_LIFETIME;
        let user_id = user_id.clone();
        let username = username.clone();

        Self {
            user_id,
            username,
            exp: expiration.timestamp() as usize,
        }
    }
}

/// Фабрика формирования и проверки токенов.
#[derive(Clone)]
pub(crate) struct JwtService {
    /// Ключ для кодирования JWT-токена.
    sign_key: EncodingKey,
    /// Ключ для декодирования JWT-токена.
    verify_key: DecodingKey,
}

impl JwtService {
    /// Создание нового экземпляра [`JwtService`]. Используется один секретный
    /// ключ для кодирования и верификации (декодирования).
    pub(crate) fn from_secret<S: AsRef<[u8]>>(secret: S) -> Self {
        let bytes = secret.as_ref();
        let sign_key = EncodingKey::from_secret(bytes);
        let verify_key = DecodingKey::from_secret(bytes);

        Self {
            sign_key,
            verify_key,
        }
    }

    /// Генерация нового токена.
    pub(crate) fn generate_token(
        &self,
        user_id: &DataId,
        username: &Username,
    ) -> Result<String, JwtError> {
        let claim = Claims::new(user_id, username);
        let header = Header::new(JWT_ALGORITHM);

        encode(&header, &claim, &self.sign_key)
    }

    /// Проверка валидности токена.
    pub(crate) fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let mut validation = Validation::new(JWT_ALGORITHM);
        validation.validate_exp = true;

        let token_message = decode(token, &self.verify_key, &validation)?;
        Ok(token_message.claims)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_secret_key() -> String {
        "Nj2Do6TANdZIP2k9fWjcJVw6A5GVbiqDmmX3DCAHBrE".to_string()
    }

    fn get_jwt_service() -> JwtService {
        let secret = get_secret_key();
        JwtService::from_secret(secret)
    }

    fn get_user_data() -> (DataId, Username) {
        let data_id = DataId(5);
        let username: Username = String::from("JennyK").try_into().unwrap();

        (data_id, username)
    }

    #[test]
    fn test_generate_token() {
        let jwt_serv = get_jwt_service();
        let (user_id, username) = get_user_data();

        let result = jwt_serv.generate_token(&user_id, &username);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_token() {
        let jwt_serv = get_jwt_service();
        let (user_id, username) = get_user_data();

        let result = jwt_serv.generate_token(&user_id, &username).unwrap();
        let token_verified = jwt_serv.verify_token(&result);
        assert!(token_verified.is_ok());

        let claims = token_verified.unwrap();
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.username, username);
    }
}
