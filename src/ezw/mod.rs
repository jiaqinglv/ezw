use std::collections::HashMap;

use crate::{template::{Template, build::{BuildFile, BuildError}}, config::Config};
use tera::{Tera, Context};
use tokio;
use serde_yaml;

/// 应用
pub struct Ezw {
    /// 应用名称
    pub name: String,
    /// 模板列表
    pub templates: HashMap<String, Template>,
    /// 配置
    pub config: Option<Config>,
    /// 全局内容设置
    global_context: Context,
    // 生成文件列表
    build_files: Vec<BuildFile>,
    /// 生成成功数量
    build_count: i64,
    /// 生成失败数量
    build_error_count: i64,
    // 错误列表
    build_errors: Vec<BuildError>,
}

/// 应用方法
impl Ezw {
    /// 创建新的应用
    pub fn new(name: &str) -> Self{
        Ezw {
            name: name.to_string(),
            templates: HashMap::new(),
            config: None,
            global_context: Context::new(),
            build_files: Vec::new(),
            build_count: 0,
            build_error_count: 0,
            build_errors: Vec::new(),
        }
    }

    /// 获取json配置文件
    pub async fn get_json_config(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        // 读取配置文件
        let config_context = tokio::fs::read_to_string(path).await?;
        // 转换成配置
        let configer = serde_json::from_str::<Config>(&config_context)?;
        // 设置应用配置
        self.config = Some(configer);

        Ok(())
    }

    /// 获取配置文件
    pub async fn get_yaml_config(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("配置文件位置:{}", path.clone());
        // 读取配置文件
        let config_context = tokio::fs::read_to_string(path).await?;
        // 转换成配置
        let configer = serde_yaml::from_str::<Config>(&config_context)?;
        // 设置应用配置
        self.config = Some(configer);
        println!("应用配置设置成功");
        Ok(())
    }

    /// 获取配置
    pub async fn get_config(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let p: Vec<&str> = path.split('.').collect();
        if p.len() <= 1 {
            panic!("config file type is not supported(supported [json , yaml])")
        }
        let ct_index = p.len() -1;
        println!("配置类型:{}",p[ct_index]);
        match p[ct_index] {
            "json" => {
                self.get_json_config(path).await?
            },
            "yaml" => {
                self.get_yaml_config(path).await?
            },
            "yml" => {
                self.get_yaml_config(path).await?
            },
            _ => {
                panic!("config file type is not supported(supported [json, yaml])")
            }
        };

        Ok(())
    }

    /// 根据配置生成文件
    pub async fn build(&mut self) ->  Result<(), Box<dyn std::error::Error>>{
        match &self.config {
            Some(c) => {
                println!("{}", &c.input_path);
                // 匹配模板
                let t = match Tera::new(&c.input_path) {
                    Ok(t) => {
                        t
                    },
                    Err(e) => { 
                        dbg!(e);
                        panic!("模板路径错误")
                    }
                };

                // 设置内容
                // 设置全局内容信息值
                let none_global_value = HashMap::new();
                let global_value = match  &c.build.global {
                    Some(value) => value,
                    None => &none_global_value,
                };

                for (key, val) in global_value.iter() {
                    self.global_context.insert(key, val);
                }

                //  设置模板
                for btc in c.build.files.iter() {
                    self.templates.insert(btc.name.clone(), Template::new(
                        btc, 
                        &t,
                        &self.global_context,
                        &mut self.build_files
                    ).await);
                }

                // 生成文件
                for bf in self.build_files.clone().into_iter() {
                    if !bf.config.build {
                        // 文件不参与生成
                        continue;
                    }
                    
                    // 多线程生成
                    let build_res = tokio::spawn(async move {
                        bf.clone().build().await
                    }).await.unwrap();

                    // 生成文件
                    match build_res {
                        Ok(_) =>  {
                            // 生成文件成功
                            self.build_count +=  1;
                        },
                        Err(e) => {
                            self.build_error_count += 1;
                            self.build_errors.push(e);
                        },
                    }
                }
               
                // 统计信息输出
                println!("成功生成文件:{}, 失败生成文件{}", self.build_count, self.build_error_count);
                println!("errors:{:#?}", self.build_errors);
            },
            None => {
                panic!("config file type is none");
            },
        };

        Ok(())
    }

}