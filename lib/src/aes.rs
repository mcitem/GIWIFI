use {
    aes::Aes128,
    base64::{Engine, prelude::BASE64_STANDARD},
    cbc::{
        Encryptor,
        cipher::{BlockEncryptMut, KeyIvInit, block_padding::ZeroPadding},
    },
    serde::Serialize,
};

#[derive(Serialize)]

pub struct EncryptedData {
    pub data: String, // 加密后的数据
    pub iv: String,
}

pub fn crypto_encode(data: &str, iv: &str, key: &str) -> EncryptedData {
    let e = Encryptor::<Aes128>::new(key.as_bytes().into(), iv.as_bytes().into())
        .encrypt_padded_vec_mut::<ZeroPadding>(data.as_bytes());
    let b64 = BASE64_STANDARD.encode(&e);
    EncryptedData {
        data: b64,
        iv: iv.into(),
    }
}

#[test]
fn test_crypto_decode() {
    let data = "Hello, World!";
    let iv = "0123456789abcdef";
    let key = "0123456789abcdef";
    let crypto_data = crypto_encode(data, iv, key);
    assert_eq!(crypto_data.data, "BY4GP9KAMVmefx9XMXA1Hg==");
}
