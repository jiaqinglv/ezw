pub mod build;

use serde::{Deserialize, Serialize};

use build::BuildConfig;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum BuildType {
    /// 使用命令生成
    CMD,
    /// 指定使用文件生成
    FILE
}


/// 生成配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// 输入路径,匹配目录下的文件或设置单文件
    pub input_path: String,
    /// 生成设置
    pub build: BuildConfig,
}
