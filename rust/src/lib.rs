pub mod aes;
pub mod login;
pub mod logout;
use clap::Parser;

#[derive(Parser, Debug)]
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

    /// 强制重新登录
    #[arg(short,action = clap::ArgAction::SetTrue)]
    pub force: bool,

    /// 维持控制台不关闭
    #[arg(long,action = clap::ArgAction::SetTrue)]
    pub pause: bool,

    /// 读取自环境变量中的账号密码-todo
    #[arg(long,action = clap::ArgAction::SetTrue)]
    pub env: bool,

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

// 用于ui的默认参数
impl Default for Args {
    fn default() -> Self {
        Self {
            base: "http://100.100.9.2".into(),
            username: "".into(),
            password: "".into(),
            quit: false,
            pause: false,
            force: false,
            env: false,
            key: "1234567887654321".into(),
            ua: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0".into(),
        }
    }
}

use std::io::{self, Write};

pub fn pause() {
    let mut stdout = io::stdout();
    write!(stdout, "按回车键继续...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
