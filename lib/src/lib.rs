#![allow(unused)]

use aes::crypto_encode;
use anyhow::anyhow;
use reqwest::header::CONTENT_TYPE;
mod aes;

pub struct Client {
    pub c: reqwest::Client,
    pub base: url::Url,
    pub key: String,
}

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";

impl Default for Client {
    fn default() -> Self {
        Self {
            c: reqwest::Client::builder().user_agent(UA).build().unwrap(),
            base: url::Url::parse("http://100.100.9.2/").unwrap(),
            key: "1234567887654321".to_owned(),
        }
    }
}

impl Client {
    pub async fn login(&self, u: &str, p: &str) -> anyhow::Result<()> {
        let res = self
            .c
            .get(self.base.join("/gportal/web/login")?)
            .send()
            .await?
            .text()
            .await?;
        let doc = scraper::Html::parse_document(&res);

        let mut iv: &str = "";
        let tumple = doc
            .select(&scraper::Selector::parse("form#loginForm input").unwrap())
            .filter(|input| input.attr("name").is_some())
            .filter_map(|input| {
                let name = input.attr("name").unwrap();
                let value = match name {
                    "user_account" => Some(u),
                    "user_password" => Some(p),
                    "iv" => {
                        iv = input.attr("value").expect("iv is None");
                        input.attr("value")
                    }
                    _ => input.attr("value"),
                }
                .unwrap_or("");
                Some((name, value))
            })
            .collect::<Vec<_>>();
        println!("{:?}", tumple);
        println!("{}", iv);
        if iv == "" {
            return Err(anyhow!("iv enpty"));
        };
        let data = serde_urlencoded::to_string(tumple)?;

        let data = crypto_encode(&data, iv, &self.key);
        let data = serde_urlencoded::to_string(data)?;

        let res = self
            .c
            .post(self.base.join("/gportal/Web/loginAction")?)
            .header(
                CONTENT_TYPE,
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .body(data)
            .send()
            .await?
            .text()
            .await?;
        println!("{}", res);
        Ok(())
    }

    pub async fn logout(&self) -> anyhow::Result<bool> {
        self.logout_(&self.si().await?).await
    }

    pub async fn logout_(&self, si: &str) -> anyhow::Result<bool> {
        let body = serde_urlencoded::to_string([("si", si)])?;
        let res = self
            .c
            .post(self.base.join("/gportal/Web/logoutAction")?)
            .body(body)
            .header(
                CONTENT_TYPE,
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .send()
            .await?;
        let res_t = res.text().await?;

        println!("{}", res_t);
        Ok(true)
    }

    pub async fn si(&self) -> anyhow::Result<String> {
        let res = self
            .c
            .get(self.base.join("/gportal/web/logout")?)
            .send()
            .await?;
        let res = scraper::Html::parse_document(&res.text().await?)
            .select(&scraper::Selector::parse("input[name=si]").unwrap())
            .next()
            .and_then(|input| input.value().attr("value"))
            .and_then(|f| Some(f.to_owned()))
            .ok_or(anyhow::anyhow!("获取si失败"))?;
        Ok(res)
    }
}

#[derive(serde::Deserialize, Debug)]
struct Res {
    pub data: Option<String>,
    pub info: String,
    pub status: String,
}

#[test]
fn test_serde() {
    let tump = [("name0", "value")];
    let data = serde_urlencoded::to_string(tump).expect("0");
    println!("{}", data);
    let opt_tump = [("name1", Some("value"))];
    let data = serde_urlencoded::to_string(opt_tump).expect("1");
    println!("{}", data);
    let opt_tump = [("name2", None::<&str>)];
    let data = serde_urlencoded::to_string(opt_tump).expect("2");
}

#[tokio::test]
async fn logout() {
    let c = Client::default();
    let s = c.logout().await;
    println!("{:?}", s);
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test_client_send_sync() {
    assert_send_sync::<Client>();
}
