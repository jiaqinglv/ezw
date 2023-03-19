use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::{self, skip_serializing_none};

/// 生成配置
#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    /// 全局模板内容值设置
    pub global: Option<HashMap<String, String>>,
    /// 生成文件信息
    pub files: Vec<BuildTemplate>,
}


/// 生成模板
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildTemplate {
    /// 模板名称
    pub name: String,
    /// 生成文件信息列表
    pub build_file: Vec<BuildFileConfig>,
}

/// 生成文件
#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildFileConfig {
    /// 生成文件
    pub out_file: String,
    /// 值设置
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<HashMap<String, String>>,
    /// 是否可生成
    pub build: bool,
}
