use anyhow::Result;
use env_logger;
use kitty_proxy::{HttpProxy, MatchProxy};
use log::info;
use url::Url;
use std::str::FromStr;
use std::{path::PathBuf, sync::Arc};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init(); // 初始化日志记录器
    println!("{:?}", Url::parse("http://www.baidu.com:443")?.host().map(|x| x.to_string()));
    println!("{:?}", Url::parse("http://www.baidu.com")?.host());
    println!("{:?}", Url::parse("https://www.baidu.com")?.host());
    println!("{:?}", Url::parse("http://127.0.0.1:443")?.host());
    println!("{:?}", Url::parse("http://127.0.0.1/aaa")?.host());

    println!("{:?}", Url::parse("http://www.baidu.com:443")?.port());
    println!("{:?}", Url::parse("http://www.baidu.com")?.port());
    println!("{:?}", Url::parse("https://www.baidu.com")?.port());
    println!("{:?}", Url::parse("http://127.0.0.1:443")?.port());
    println!("{:?}", Url::parse("http://127.0.0.1/aaa")?.port());

    info!("This is an info log message.");
    let mut proxy = HttpProxy::new("127.0.0.1", 10088, None, "127.0.0.1", 10809).await?;
    let geoip_file = "/home/hezhaozhao/opensource/kitty/src-tauri/binaries/geoip.dat";
    let geosite_file = "/home/hezhaozhao/opensource/kitty/src-tauri/binaries/geosite.dat";
    let match_proxy = MatchProxy::from_geo_dat(
        Some(&PathBuf::from_str(geoip_file).unwrap()),
        Some(&PathBuf::from_str(geosite_file).unwrap()),
    )
    .unwrap();
    let arc_match_proxy = Arc::new(match_proxy);
    let _ = proxy.serve(arc_match_proxy).await;
    Ok(())
}
