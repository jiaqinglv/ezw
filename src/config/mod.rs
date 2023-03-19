pub mod build;

use serde::{Deserialize, Serialize};

use build::BuildConfig;

/// 程序配置
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// 输入路径,匹配目录下的文件或设置单文件
    pub input_path: String,
    /// 生成设置
    pub build: BuildConfig,
}
