const POINT_BYTES: usize = 32;
use x25519_dalek::{PublicKey as DalekPublic, StaticSecret};

//Public Key
pub struct PublicKey(pub [u8; POINT_BYTES]);

impl PublicKey {
    // Number of bytes
    pub const BYTES: usize = POINT_BYTES;

    //Create a Public Key
    pub fn new(pk: [u8; PublicKey::BYTES]) -> Self {
        PublicKey(pk)
    }
}

pub struct SecretKey(pub [u8; SecretKey::BYTES]);

//Secret Key
impl SecretKey {
    
    //number of bytes
    pub const BYTES: usize = 32;

    //Create a Secret Key
    pub fn new(sk: [u8; SecretKey::BYTES]) -> Self{
        SecretKey(sk)
    }

}


pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}


pub fn from_hex(s: &str) -> Result<Vec<u8>, String> {
    let s = s.trim();
    if s.len() % 2 != 0 {
        return Err("Hex com tamanho ímpar".to_string());
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|_| "Hex inválido".to_string()))
        .collect()
}

pub fn read_32(s: &str) -> Result<[u8; 32], String> {
    from_hex(s)?
        .try_into()
        .map_err(|_| "A chave tem de ter 32 bytes (64 caracteres hex)".to_string())
}

pub fn generate_keys() -> (SecretKey, PublicKey) {
    let sk_bytes: [u8; 32] = rand::random();
    let secret = StaticSecret::from(sk_bytes);
    let public = DalekPublic::from(&secret);
    (SecretKey::new(sk_bytes), PublicKey::new(public.to_bytes()))
}

