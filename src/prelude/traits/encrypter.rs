use bytes::Bytes;

/// The Encrypter should be implemented by objects intending to encrypt or decrypt bytes.
/// 
/// Due to decompiling client side applications, encryption should not to be considered secure, merely obfuscated / hidden from plainsite.
///
pub trait Encrypter {
    /// Encrypts bytes
    /// Encrypted (or obfuscated) version of the original.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The unencrypted data.
    /// * `secret` - The secret key to encrypt the data with.  Leave blank to use default secret key. (optional, default: empty)
    /// 
    fn encrypt(&self, value: Bytes, secret: Option<String>) -> Bytes;

    /// Decrypts bytes.
    /// Decrypted (or unobfuscated) version of the encrypted data.
    /// 
    /// # Arguments
    /// 
    /// * `value` - The encrypted data.
    /// * `secret` - The secret key to encrypt the data with.  Leave blank to use default secret key. (optional, default: empty)
    /// 
    fn decrypt(&self, value: Bytes, secret: Option<String>) -> Bytes;
}
