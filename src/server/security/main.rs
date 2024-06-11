use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use rand::{RngCore, thread_rng};

struct SecurityManager {
    key: [u8; 32],
}

impl SecurityManager {
    fn new() -> Self {
        let mut key = [0u8; 32];
        thread_rng().fill_bytes(&mut key); // Generate a random key
        SecurityManager { key }
    }

    fn encrypt(&self, plaintext: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.key));
        cipher.encrypt(GenericArray::from_slice(nonce), plaintext).expect("encryption failure!")
    }

    fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 12]) -> Option<Vec<u8>> {
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.key));
        cipher.decrypt(GenericArray::from_slice(nonce), ciphertext).ok()
    }
}

fn main() {
    let security_manager = SecurityManager::new();

    let plaintext = b"Hello, world!";
    let nonce = generate_nonce(); // Generate a random nonce
    let ciphertext = security_manager.encrypt(plaintext, &nonce);
    if let Some(decrypted_text) = security_manager.decrypt(&ciphertext, &nonce) {
        assert_eq!(plaintext.to_vec(), decrypted_text);
        println!("Encryption and decryption successful!");
    } else {
        println!("Decryption failed!"); // Handle decryption failure
    }
}

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    thread_rng().fill_bytes(&mut nonce); // Generate a random nonce
    nonce
}
