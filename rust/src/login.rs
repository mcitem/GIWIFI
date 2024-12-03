use {
    crate::{aes::crypto_encode, logout::get_si, Args},
    reqwest::{blocking::Client, header::CONTENT_TYPE},
    std::error::Error,
    urlencoding::encode,
};

pub fn main(args: &Args, client: &Client) -> Result<(), Box<dyn Error>> {
    match get_si(&args, client) {
        Some(si) => match args.force {
            false => {
                return Err(format!("当前已登录 会话id为{}", si).into());
            }
            true => {
                println!("当前已登录 会话id为{}", si);
                crate::logout::main(args, client)?;
            }
        },
        None => {}
    };
    let doc = &client
        .get(format!("{}/gportal/web/login", args.base))
        .send()?
        .text()?;
    // println!("{}", doc);
    let doc = scraper::Html::parse_document(&doc);
    let f = doc
        .select(&scraper::Selector::parse("#loginForm")?)
        .next()
        .ok_or("找不到登录表单")?;

    let mut inputs = Vec::new();
    let mut iv = "";
    for input in f.select(&scraper::Selector::parse("input")?) {
        let name = input.value().attr("name").ok_or("找不到登录参数")?;
        let value = match name {
            "user_account" => &args.username,
            "user_password" => &args.password,
            "iv" => {
                iv = input.value().attr("value").ok_or("找不到加密参数iv")?;
                iv
            }
            _ => input.value().attr("value").unwrap_or(""),
        };
        inputs.push((name, value));
    }

    let data = inputs
        .iter()
        .map(|(k, v)| format!("{}={}", k, encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let data = crypto_encode(&data, &iv, &args.key);

    let data = serde_urlencoded::to_string(data)?;

    let res = client
        .post(format!("{}/gportal/Web/loginAction", args.base))
        .header(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .body(data)
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}
