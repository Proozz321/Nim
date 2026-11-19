const POINT_BYTES: usize = 32;

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

pub fn generate_keys() -> (SecretKey, PublicKey) {
    let sk = SecretKey::new(rand::random::<[u8; SecretKey::BYTES]>());
    // TEMPORÁRIO: uma chave pública a sério é derivada da privada
    let pk = PublicKey::new(rand::random::<[u8; PublicKey::BYTES]>());
    (sk, pk)
}

