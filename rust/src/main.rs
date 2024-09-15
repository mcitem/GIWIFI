mod aes;

use aes::crypto_encode;
use clap::Parser;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde::Serialize;
// use urlencoding::encode;

#[derive(Parser)]
struct Args {
    /// 认证网页IP
    #[arg(short, default_value = "100.100.9.2")]
    base: String,

    /// 账号
    #[arg(short)]
    username: String,

    /// 密码
    #[arg(short)]
    password: String,
}
const UA: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3";
const CT: &'static str = "application/x-www-form-urlencoded";
fn main() {
    let args = Args::parse();
    let base = args.base;
    let username = args.username;
    let password = args.password;

    let login_html_url = format!("http://{}/gportal/web/login", base);
    let login_action_url = format!("http://{}/gportal/Web/loginAction", base);

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(login_html_url)
        .header(USER_AGENT, UA)
        .send()
        .unwrap();

    let body = res.text().unwrap();
    let doc = scraper::Html::parse_document(&body);

    let data = FormData {
        sign: query(&doc, "sign"),
        sta_vlan: query(&doc, "sta_vlan"),
        sta_port: query(&doc, "sta_port"),
        sta_ip: query(&doc, "sta_ip"),
        nas_ip: query(&doc, "nas_ip"),
        nas_name: query(&doc, "nas_name"),
        last_url: query(&doc, "last_url"),
        request_ip: query(&doc, "request_ip"),
        device_mode: query(&doc, "device_mode"),
        device_type: query(&doc, "device_type"),
        device_os_type: query(&doc, "device_os_type"),
        is_mobile: query(&doc, "is_mobile"),
        iv: query(&doc, "iv"),
        login_type: query(&doc, "login_type"),
        account_type: query(&doc, "account_type"),
        user_account: username,
        user_password: password,
    };
    let iv = data.iv.clone();
    let data = serde_urlencoded::to_string(&data).unwrap();
    let data = crypto_encode(&data, &iv);
    let data = serde_urlencoded::to_string(&data).unwrap();
    let res = client
        .post(login_action_url)
        .header(USER_AGENT, UA)
        .header(CONTENT_TYPE, CT)
        .body(data)
        .send()
        .unwrap();

    println!("{}", res.text().unwrap());
}

fn query(doc: &scraper::Html, name: &str) -> String {
    let data = doc
        .select(&scraper::Selector::parse(format!("input[name={}]", name).as_str()).unwrap())
        .next()
        .and_then(|input| input.value().attr("value"))
        .unwrap();
    serde_urlencoded::from_str(data).unwrap()
}

#[derive(Serialize)]
struct FormData {
    sign: String,
    sta_vlan: String,
    sta_port: String,
    sta_ip: String,
    nas_ip: String,
    nas_name: String,
    last_url: String,
    request_ip: String,
    device_mode: String,
    device_type: String,
    device_os_type: String,
    is_mobile: String,
    pub iv: String,
    login_type: String,
    account_type: String,
    user_account: String,
    user_password: String,
}
