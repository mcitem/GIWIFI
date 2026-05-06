use clap::Parser;
use clap::Subcommand;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const DEFAULT_BASE: &str = "http://100.100.9.2";
const DEFAULT_KEY: &str = "1234567887654321";
const DEFAULT_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0";
const DEFAULT_CONFIG_PATH: &str = "/etc/config/luci-app-giwifi.json";

#[derive(Debug, Clone, Default, Deserialize)]
struct FileConfig {
    client: ClientConfig,
    login: LoginConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct LoginConfig {
    username: Option<String>,
    password: Option<String>,
    force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ClientConfig {
    base: Option<String>,
    key: Option<String>,
    ua: Option<String>,
}

#[derive(Parser)]
pub struct Args {
    /// JSON 配置文件
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    pub config: PathBuf,

    /// 认证网页IP
    #[arg(short, long)]
    pub base: Option<String>,

    /// AES-CBC 加密密钥
    #[arg(long)]
    pub key: Option<String>,

    /// User-Agent
    #[arg(long)]
    pub ua: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 登录
    Login {
        /// 账号
        #[arg(short, long)]
        username: Option<String>,

        /// 密码
        #[arg(short, long)]
        password: Option<String>,

        /// 强制重新登录
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },
    /// 退出登录
    Logout,
}

fn load_config(path: &Path) -> anyhow::Result<FileConfig> {
    Ok(match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content)?,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => FileConfig::default(),
        Err(err) => return Err(err.into()),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let file_config = load_config(&args.config)?;

    let base = url::Url::parse(
        args.base
            .or(file_config.client.base)
            .as_deref()
            .unwrap_or(DEFAULT_BASE),
    )?;
    let key = args
        .key
        .or(file_config.client.key)
        .unwrap_or(String::from(DEFAULT_KEY));
    let ua = args.ua.or(file_config.client.ua);
    let ua = ua.as_deref().unwrap_or(DEFAULT_UA);

    println!("Using base URL: {}", base);
    println!("Using key: {}", key);
    println!("Using User-Agent: {}", ua);

    let client = giwifi::Client {
        base,
        key,
        c: reqwest::Client::builder().user_agent(ua).build()?,
    };

    match args.command {
        Some(Commands::Login {
            username,
            password,
            force,
        }) => {
            let login_config = file_config.login;
            let username = username
                .or(login_config.username)
                .ok_or_else(|| anyhow::anyhow!("missing username"))?;

            let password = password
                .or(login_config.password)
                .ok_or_else(|| anyhow::anyhow!("missing password"))?;

            let force = force || login_config.force.unwrap_or(false);

            println!("Using username: {}", username);
            println!("Using password: {}", "*".repeat(password.len()));
            println!("Force login: {}", force);

            if force {
                if let Ok(si) = client.si().await {
                    client.logout_(&si).await?;
                }
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
            client.login(&username, &password).await?
        }
        Some(Commands::Logout) => {
            println!("Logging out...");
            client.logout().await?;
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    };
    Ok(())
}
