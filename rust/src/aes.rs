use {
    aes::Aes128,
    base64::{prelude::BASE64_STANDARD, Engine},
    cbc::{
        cipher::{block_padding::ZeroPadding, BlockEncryptMut, KeyIvInit},
        Encryptor,
    },
    serde::Serialize,
};

#[derive(Serialize)]

pub struct EncryptedData {
    pub data: String, // 加密后的数据
    pub iv: String,
}

pub fn crypto_encode(data: &str, iv: &str, key: &str) -> EncryptedData {
    let mut buf = vec![0; data.len() * 2];
    buf[..data.len()].copy_from_slice(&data.as_bytes());
    let e = match Encryptor::<Aes128>::new(key.as_bytes().into(), iv.as_bytes().into())
        .encrypt_padded_mut::<ZeroPadding>(&mut buf, data.len())
    {
        Ok(e) => e,
        Err(e) => panic!("{:?}", e),
    };
    let b64 = BASE64_STANDARD.encode(&e);
    // println!("{:?}", b64);
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
