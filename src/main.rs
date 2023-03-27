use std::env;

pub mod ezw;
pub mod template;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    #[allow(unused_assignments)] // 取消错误的告警提示
    let mut config_path = "build.yaml".to_string();
    if args.len() < 2 {
        config_path = "build.yaml".to_string();
    } else {
        config_path = args[1].to_string();
    }
    
    let mut e = ezw::Ezw::new("app");
    // 获取配置文件
    e.get_config(config_path).await?;
    println!("获取配置成功");
    tokio::spawn(async move {
        // 生成文件
        e.build().await.unwrap();
    }).await.unwrap();
    Ok(())
}
