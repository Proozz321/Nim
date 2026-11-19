use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use x25519_dalek::{PublicKey as DalekPublic, StaticSecret};
use crate::keys::{PublicKey, SecretKey};


fn shared_key(my_sk: &SecretKey, their_pk: &PublicKey) -> [u8; 32] {
    let sk = StaticSecret::from(my_sk.0);
    let pk = DalekPublic::from(their_pk.0);
    sk.diffie_hellman(&pk).to_bytes()
}

pub fn encrypt(my_sk: &SecretKey, their_pk: &PublicKey, msg: &str) -> Result<Vec<u8>, String> {
    let key = shared_key(my_sk, their_pk);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));

    // nonce: 12 bytes aleatórios, nunca repetir com a mesma chave
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, msg.as_bytes())
        .map_err(|_| "Falha ao encriptar".to_string())?;

    // guardamos o nonce à frente, o outro lado precisa dele
    let mut out = nonce_bytes.to_vec();
    out.extend(ciphertext);
    Ok(out)
}


pub fn decrypt(my_sk: &SecretKey, their_pk: &PublicKey, data: &[u8]) -> Result<String, String> {
    if data.len() < 12 {
        return Err("Mensagem demasiado curta".to_string());
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);

    let key = shared_key(my_sk, their_pk);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));

    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), ciphertext)
        .map_err(|_| "Chave errada ou mensagem alterada".to_string())?;

    String::from_utf8(plaintext).map_err(|_| "Texto inválido".to_string())
}