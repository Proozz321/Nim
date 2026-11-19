const POINT_BYTES: usize = 32;

//Public Key
pub struct PublicKey([u8; POINT_BYTES]);

impl PublicKey {
    // Number of bytes
    pub const BYTES: usize = POINT_BYTES;

    //Create a Public Key
    pub fn new(pk: [u8; PublicKey::BYTES]) -> Self {
        PublicKey(pk)
    }
}

pub struct SecretKey([u8; SecretKey::BYTES]);

//Secret Key
impl SecretKey {
    
    //number of bytes
    pub const BYTES: usize = 32;

    //Create a Secret Key
    pub fn new(sk: [u8; SecretKey::BYTES]) -> Self{
        SecretKey(sk)
    }

}

