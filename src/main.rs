use std::env;

pub mod config;
pub mod ezw;
pub mod template;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // #[allow(unused_assignments)] // 取消错误的告警提示
    let mut config_path = "build.yaml".to_string();

    println!("args: {:#?}", args);

    let args_len = args.clone().len();
    let mut e = ezw::Ezw::new("ezw");

    let global_template = match std::option_env!("EZW") {
        Some(gt) => gt.to_string(),
        None => {
            panic!("global template path is not specified");
        }
    };

    if args_len >= 2 {
        match args[1].to_lowercase().as_str() {
            // 解析
            "paser" | "-p" => {
                if args_len == 3 {
                    // 获取要解析的配置路径
                    config_path = args[2].to_string();

                    e.get_config(config_path).await?;
                    println!("[FILE] [PASER] 获取配置成功");
                    tokio::spawn(async move {
                        // 生成文件
                        e.build().await.unwrap();
                    })
                    .await
                    .unwrap();
                    return Ok(());
                } else {
                    // 无效参数
                    panic!("invalid parameters")
                }
            }
            "new" | "-n" => {
                if args_len == 4 {
                    let template_dir = args[2].to_string();
                    let build_name = args[3].to_string();
                    config_path = format!("{global_template}/{template_dir}/build.yaml");

                    e.build_type = config::BuildType::CMD;
                    e.build_name = build_name;

                    // 获取配置文件
                    e.get_config(config_path).await?;
                    println!("[CMD] [NEW] 获取配置成功");
                    tokio::spawn(async move {
                        // 生成文件
                        e.build().await.unwrap();
                    })
                    .await
                    .unwrap();
                    return Ok(());
                } else {
                    // 无效参数
                    panic!("invalid parameters")
                }
            }
            _ => {
                // 获取配置文件
                e.get_config(config_path).await?;
                println!("获取配置成功");
                tokio::spawn(async move {
                    // 生成文件
                    e.build().await.unwrap();
                })
                .await
                .unwrap();
                return Ok(());
            }
        }
    } else {
        // 获取配置文件
        e.get_config(config_path).await?;
        println!("获取配置成功");
        tokio::spawn(async move {
            // 生成文件
            e.build().await.unwrap();
        })
        .await
        .unwrap();
    }

    // if args.len() < 2 {
    //     config_path = "build.yaml".to_string();
    // } else {
    //     config_path = args[1].to_string();
    // }

    // let mut e = ezw::Ezw::new("app");
    // // 获取配置文件
    // e.get_config(config_path).await?;
    // println!("获取配置成功");
    // tokio::spawn(async move {
    //     // 生成文件
    //     e.build().await.unwrap();
    // }).await.unwrap();

    Ok(())
}
