pub mod aes;
pub mod login;
pub mod logout;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// 认证网页IP
    #[arg(short, default_value = "http://100.100.9.2")]
    pub base: String,

    /// 账号
    #[arg(short, default_value = "")]
    pub username: String,

    /// 密码
    #[arg(short, default_value = "")]
    pub password: String,

    /// 退出登录
    #[arg(short, action = clap::ArgAction::SetTrue)]
    pub quit: bool,

    /// AES-CBC 加密密钥
    #[arg(long, default_value = "1234567887654321")]
    pub key: String,

    /// User-Agent
    #[arg(
        long,
        default_value = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0"
    )]
    pub ua: String,
}
