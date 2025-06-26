use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, AeadCore
};
use crate::hex;
use std::env;
use crate::errors::AppError;

type AesCipher = Aes256Gcm;

const NONCE_LENGTH: usize = 12;

fn get_key() -> Result<[u8; 32], AppError> {
    let key_hex = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key_bytes = hex::decode(key_hex).map_err(|_| {
        AppError::InternalServerError("Invalid hex for ENCRYPTION_KEY".to_string())
    })?;
    key_bytes.try_into().map_err(|_| {
        AppError::InternalServerError("ENCRYPTION_KEY must be 32 bytes (64 hex characters)".to_string())
    })
}

// Encrypts a plaintext message.
// It generates a random 12-byte nonce, encrypts the data, and then
// prepends the nonce to the ciphertext. The resulting Vec<u8> is
// `[nonce][ciphertext]`.
pub fn encrypt(plaintext: &str) -> Result<Vec<u8>, AppError> {
    let key = get_key()?;
    let cipher = AesCipher::new(&key.into());

    // Generate a new random nonce for each message. Never reuse a nonce with a given key.
    let nonce = AesCipher::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| AppError::InternalServerError("Message encryption failed.".to_string()))?;
    
    // Prepend the nonce to the ciphertext for storage.
    let mut result = Vec::with_capacity(NONCE_LENGTH + ciphertext.len());
    result.extend_from_slice(nonce.as_slice());
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

// Decrypts a message.
// It expects the input data to be in the format `[nonce][ciphertext]`.
// It extracts the nonce, decrypts the ciphertext, and returns the original plaintext.
pub fn decrypt(encrypted_data: &[u8]) -> Result<String, AppError> {
    if encrypted_data.len() < NONCE_LENGTH {
        return Err(AppError::InternalServerError("Invalid encrypted data format.".to_string()));
    }

    let key = get_key()?;
    let cipher = AesCipher::new(&key.into());

    // Split the nonce and the actual ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_LENGTH);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| AppError::InternalServerError("Message decryption failed.".to_string()))?;

    String::from_utf8(decrypted_bytes)
        .map_err(|_| AppError::InternalServerError("Failed to convert decrypted bytes to UTF-8.".to_string()))
}
