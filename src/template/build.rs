use std::{fmt, collections::HashMap};
use tera::{Tera, Context};
use tokio;

use crate::config::build::BuildFileConfig;



/// 生成文件
#[derive(Debug, Clone)]
pub struct BuildFile {
    pub config: BuildFileConfig,
    /// 生成文件内容
    body: Option<String>,
}

impl BuildFile {
    /// 创建生成文件实例
    pub fn new(config: BuildFileConfig) -> Self {
        BuildFile {
            config,
            body: None,
        }
    }

    /// 设置文件内容
    pub async fn set_body(
        &mut self, 
        t: &Tera, //模板引擎
        template_name: String, // 模板名称
        global_context: &Context, // 全局内容值
    ) -> Result<(), BuildError> {
        let mut context = Context::new();
        // 如果没有值的情况将新的HashMap给它
        let now_value = &HashMap::new();

        // 判断值是否存在
        let context_value = match &self.config.value {
            Some(value) => value,
            None => now_value,
        };
        // 设置文件内容值
        for (key, val) in context_value {
            context.insert(key, &val);
        }
        // 与全局内容值合并
        context.extend(global_context.clone());

        // 渲染
        let res = t.render(&template_name.clone(), &context);
        match res {
            Ok(body) => {
                self.body = Some(body);
                return  Ok(());
            },
            Err(e) => {
                dbg!(e.to_string(), &template_name);
                return Err(BuildError::new(e.to_string()));
            },
        }
    }

    /// 生成文件
    pub async fn build(&self) -> Result<(), BuildError> {
        match &self.body {
            Some(data) => {
                let write_body = data.as_bytes();
                // 生成文件,写入内容
                let r = tokio::fs::write(&self.config.out_file, write_body).await;

                // 结果判断
                match r {
                    Ok(_) => {
                        return  Ok(());
                    },
                    Err(e) => {
                        return Err(BuildError { msg: e.to_string() });
                    },
                };
            },
            None => {
                return Err(BuildError { msg: String::from("build file is none") });
            },
        };
    }
}

/// 生成错误
#[derive(Debug)]
pub struct BuildError {
    pub msg: String,
}
/// 错误需要错误信息输出
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "build error: {} \n", self.msg)
    }
}

/// 自定义方法
impl BuildError {
    pub fn new(msg: String) -> BuildError {
        BuildError { msg }
    }
}
