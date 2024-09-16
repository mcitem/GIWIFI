use base64::{prelude::BASE64_STANDARD, Engine};
use crypto::{
    aes::{cbc_encryptor, KeySize},
    blockmodes::NoPadding,
    buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer},
};
use serde::Serialize;

fn zero_padding(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padded_data = data.to_vec();
    let padding_length = block_size - (data.len() % block_size);
    padded_data.extend(vec![0; padding_length]);
    padded_data
}

pub fn crypto_encode(data: &str, iv: &str, key: &str) -> CryptoData {
    let mut encryptor = cbc_encryptor(
        KeySize::KeySize128,
        key.as_bytes(),
        iv.as_bytes(),
        NoPadding,
    );
    let data = zero_padding(data.as_bytes(), 16);
    let mut input = RefReadBuffer::new(&data);
    let mut output = vec![0; data.len() * 2];
    let mut output_buffer = RefWriteBuffer::new(&mut output);
    let s = match encryptor.encrypt(&mut input, &mut output_buffer, true) {
        Ok(BufferResult::BufferUnderflow) => {
            let encrypted_data = output_buffer.take_read_buffer().take_remaining().to_vec();
            BASE64_STANDARD.encode(&encrypted_data)
        }
        Ok(BufferResult::BufferOverflow) => {
            panic!("Buffer overflow occurred");
        }
        Err(error) => {
            panic!("Encryption error: {:?}", error);
        }
    };
    CryptoData {
        data: s,
        iv: iv.to_string(),
    }
}
#[derive(Serialize)]
pub struct CryptoData {
    pub data: String,
    pub iv: String,
}
