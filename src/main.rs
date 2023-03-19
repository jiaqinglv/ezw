use tera::{Tera, Context};
use std::process::exit;

pub mod ezw;
pub mod template;
pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut e = ezw::Ezw::new("demo");
    // 获取配置文件
    e.get_config("build.yaml".to_string()).await?;
    println!("获取配置成功");
    // 生成文件
    e.build().await?;
    Ok(())
}


#[allow(dead_code)]
async fn demo() -> Result<(), Box<dyn std::error::Error>> {
      // 匹配模板
      let t = match Tera::new("/run/media/ljq/Data/Code/Rust/ezw/templates/**/*.js") {
        Ok(t) => {
            t
        },
        Err(e) => {
            dbg!(e);
            println!("not found template");
            exit(1);
        }
    };

    // 设置内容
    let mut context = Context::new();
    context.insert("name", "hello world");
    context.insert("name2", "hello world");

    // 生成代码
    let main_res = t.render("index.js", &context);
    let res = t.render("api/hello.js", &context);

    println!("{:#}",main_res.unwrap());
    println!("{:#}",res.unwrap());
    Ok(())
}
