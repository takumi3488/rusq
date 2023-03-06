use clap::Parser;
use std::env;

const PORT: u16 = 8080;
const HOST: &str = "127.0.0.1";
const USER: &str = "rusq";
const REDIS_URL: &str = "redis://127.0.0.1";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = PORT)]
    port: u16,

    #[arg(long, default_value_t = HOST.to_string())]
    host: String,

    #[arg(short, long, default_value_t = USER.to_string())]
    user: String,

    #[arg(short, long, default_value_t = REDIS_URL.to_string())]
    redis_url: String,

    #[arg(short = 'P', long, default_value_t = String::from(""))]
    password: String,
}

pub fn get_config() -> (u16, String, String, String, String) {
    let args = Args::parse();

    // Set port
    let port = env::var("RUSQ_PORT")
        .map(|p| p.parse().unwrap())
        .unwrap_or(args.port);

    // Set host
    let host = env::var("RUSQ_HOST").unwrap_or(args.host);

    // Set user
    let user = env::var("RUSQ_USER").unwrap_or(args.user);

    // Set password
    let password = env::var("RUSQ_PASSWORD").unwrap_or(args.password);
    if password == "" {
        panic!("password must be set")
    }

    // Set redis url
    let redis_url = env::var("RUSQ_REDIS_URL").unwrap_or(args.redis_url);

    (port, host, user, password, redis_url)
}
