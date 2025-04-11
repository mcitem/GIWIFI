#![allow(unused)]

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
pub struct Args {
    /// 认证网页IP
    #[arg(short, default_value = "http://100.100.9.2")]
    pub base: String,

    /// AES-CBC 加密密钥
    #[arg(long, default_value = "1234567887654321")]
    pub key: String,

    /// User-Agent
    #[arg(
        long,
        default_value = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0"
    )]
    pub ua: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 登录
    Login {
        /// 账号
        #[arg(short)]
        username: String,

        /// 密码
        #[arg(short)]
        password: String,

        /// 强制重新登录
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },
    /// 退出登录
    Logout,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let base = url::Url::parse(&args.base)?;
    let mut client = giwifi::Client {
        base,
        key: args.key,
        c: reqwest::Client::builder().user_agent(&args.ua).build()?,
    };

    let r = match args.command {
        Some(Commands::Login {
            username,
            password,
            force,
        }) => {
            if force {
                if let Ok(si) = client.si().await {
                    client.logout_(&si).await?;
                }
            }
            client.login(&username, &password).await?
        }
        Some(Commands::Logout) => {
            client.logout().await?;
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    };
    Ok(())
}
