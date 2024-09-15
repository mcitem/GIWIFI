use crate::aes::crypto_encode;
use crate::query;
use crate::Args;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde::Serialize;
pub fn login(args: Args) {
    let base = args.base;
    let ua = args.ua.as_str();
    let username = args.username;
    let password = args.password;

    let client = reqwest::blocking::Client::new();

    let doc_client = client
        .get(format!("{}/gportal/web/login", base))
        .header(USER_AGENT, ua);

    let action_client = client
        .post(format!("{}/gportal/Web/loginAction", base))
        .header(USER_AGENT, ua)
        .header(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=UTF-8",
        );

    let res = doc_client.send().unwrap();
    let status = res.status();
    if status != 200 {
        println!("请求出错, 状态码: {}", status);
        return;
    }

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

    let iv = &data.iv;
    let data = serde_urlencoded::to_string(&data).unwrap();
    let data = crypto_encode(&data, &iv, &args.key);
    let data = serde_urlencoded::to_string(&data).unwrap();
    let res = action_client.body(data).send().unwrap();
    println!("{}", res.text().unwrap());
}

#[derive(Debug, Serialize)]
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
    iv: String,
    login_type: String,
    account_type: String,
    user_account: String,
    user_password: String,
}
