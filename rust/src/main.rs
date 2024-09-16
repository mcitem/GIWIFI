use {
    clap::Parser,
    giwifi::*,
    reqwest::{
        blocking::Client,
        header::{HeaderMap, HeaderValue, USER_AGENT},
    },
    std::error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(&args.ua)?);

    let client = Client::builder().user_agent(&args.ua).build()?;

    match args.quit {
        true => logout::main(&args, &client),
        false => {
            if args.username.is_empty() || args.password.is_empty() {
                println!("用户名和密码不能为空");
                return Ok(());
            }
            login::main(&args, &client)
        }
    }
}
