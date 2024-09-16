use {crate::Args, reqwest::blocking::Client};

pub fn main(args: &Args, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let logout_action = client
        .post(format!("{}/gportal/Web/logoutAction", args.base))
        .body(serde_urlencoded::to_string([(
            "si",
            get_si(&args, client).ok_or("无法获取登录状态，当前可能已离线")?,
        )])?)
        .send()?
        .text()?;
    println!("{}", logout_action);
    Ok(())
}

pub fn get_si(args: &Args, client: &Client) -> Option<String> {
    scraper::Html::parse_document(
        &client
            .get(format!("{}/gportal/web/logout", args.base))
            .send()
            .ok()?
            .text()
            .ok()?,
    )
    .select(&scraper::Selector::parse("input[name=si]").ok()?)
    .next()
    .and_then(|input| input.value().attr("value"))
    .and_then(|f| Some(f.to_string()))
}
