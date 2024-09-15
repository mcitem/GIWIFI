use crate::query;
use crate::Args;
use reqwest::header::USER_AGENT;
pub fn logout(args: Args) {
    let base = args.base;

    let client = reqwest::blocking::Client::new();

    let logout_client = client
        .get(format!("{}/gportal/web/logout", base))
        .header(USER_AGENT, args.ua.as_str())
        .send()
        .unwrap();

    let body = logout_client.text().unwrap();
    let doc = scraper::Html::parse_document(&body);
    let si = query(&doc, "si");
    if si == "" {
        println!("当前可能未登录");
        return;
    } else {
        let logout_action = client
            .post(format!("{}/gportal/Web/logoutAction", base))
            .header(USER_AGENT, args.ua.as_str())
            .body(serde_urlencoded::to_string(&[("si", si)]).unwrap())
            .send()
            .unwrap();
        println!("{}", logout_action.text().unwrap())
    }
}
