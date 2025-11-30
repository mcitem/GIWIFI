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
#[cfg(test)]
fn test_crypto_decode() {
    let data = "Hello, World!";
    let iv = "0123456789abcdef";
    let key = "0123456789abcdef";
    let crypto_data = crypto_encode(data, iv, key);
    assert_eq!(crypto_data.data, "BY4GP9KAMVmefx9XMXA1Hg==");
}

mod app {
    use aes::{
        Aes128,
        cipher::{BlockDecryptMut, KeyInit},
    };
    use base64::{Engine, prelude::BASE64_STANDARD};
    #[allow(dead_code)]
    pub fn decrypt(data: &str) -> String {
        const KEY: &[u8] = b"5447c08b53e8dac4";
        // urlend

        let val = urlencoding::decode(data).unwrap();
        let val = BASE64_STANDARD.decode(val.as_bytes()).unwrap();
        use aes::cipher::block_padding::Pkcs7;
        let val = ecb::Decryptor::<Aes128>::new(KEY.into())
            .decrypt_padded_vec_mut::<Pkcs7>(&val)
            .unwrap();
        let val = String::from_utf8(val).unwrap();

        val
    }

    #[test]
    #[cfg(test)]
    fn test_decrypt_app_data() {
        const DATA: &str = "%2BRbWc52eMbDCsA5S%2FQaOt%2BWaRf1L7%2F%2BiGZ7YV90TQMXOFeUoWm5YZ0m7ymY79mArn%2BzGxRhY1XOPpuNakHcVsQ%3D%3D";
        let result = decrypt(DATA);
        assert_eq!(
            result,
            "{\"resultCode\":99,\"resultMsg\":\"error: data is empty\",\"data\":\"\"}"
        );
    }

    // pub fn send() {}

    #[tokio::test]
    #[cfg(test)]
    async fn test_send() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Payload {
            timestamp: String,
            user_ip: String,
            user_name: String,
        }
        impl Default for Payload {
            fn default() -> Self {
                Self {
                    timestamp: chrono::Utc::now().timestamp().to_string(),
                    user_ip: "255.255.255.255".into(),
                    user_name: "".into(),
                }
            }
        }
        impl Payload {
            fn build(self) -> String {
                // md5盐
                const KEY: &str = "5447c08b53e8dac47f81269f98cfeada";

                let raw = serde_urlencoded::to_string(self).unwrap();
                println!("raw: {}", raw);
                let sign = md5::compute(raw.clone() + KEY);
                let sign = hex::encode(*sign);

                let val = format!("{}&sign={}", raw, sign);

                use aes::cipher::{BlockEncryptMut, block_padding::Pkcs7};
                const KEY_AES: &[u8] = b"5447c08b53e8dac4";
                let val = ecb::Encryptor::<Aes128>::new(KEY_AES.into())
                    .encrypt_padded_vec_mut::<Pkcs7>(val.as_bytes());
                let val = BASE64_STANDARD.encode(&val);
                let val = urlencoding::encode(&val);
                val.into_owned()
            }
        }

        let payload = Payload::default().build();
        println!("{}", payload);

        use reqwest::{Client, header::USER_AGENT};

        let client = Client::new();
        const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0";
        let res = client
            .post("http://100.100.9.2/gportal/app/queryAuthState")
            .form(&[("data", payload)])
            .header(USER_AGENT, UA)
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        println!("{}", text);

        #[derive(Deserialize)]
        struct Res {
            data: String,
        }

        let res: Res = serde_json::from_str(&text).unwrap();
        let decrypted = decrypt(&res.data);

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Root {
            pub result_code: i64,
            pub result_msg: String,
            pub data: Data,
        }

        #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            pub contact_phone: String,
            pub suggest_phone: String,
            pub auth_state: i64,
            pub online_time: i64,
            pub nas_name: String,
            pub station_sn: String,
            pub user_mac: String,
        }

        println!("{}", decrypted);

        let res: Root = serde_json::from_str(&decrypted).unwrap();
        println!("{:#?}", res);
    }
}
