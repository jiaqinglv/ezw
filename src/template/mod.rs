use tera::{Tera, Context};

use crate::config::build::BuildTemplate;

use self::build::BuildFile;

pub mod build;

/// 模板
#[derive( Debug)]
pub struct Template{
    /// 文件名称
    pub name: String,
    pub build_file: Vec<BuildFile>,
}

/// 模板设置
impl Template {
    /// 创建模板对应实例
    pub async fn new(config: &BuildTemplate,tera: &Tera, global_context: &Context, list:&mut Vec<BuildFile>) -> Self {
        let mut bf_list = Vec::new();
        // 添加生成文件到列表
        for bfc in &config.build_file {
            let mut bf = BuildFile::new(bfc.clone());
            // 设置生成内容
            bf.set_body(tera, config.name.clone(), global_context).await.unwrap();
            // 模板列表中添加生成文件
            bf_list.push(bf.clone());
            // 程序中记录要生成的文件
            list.push(bf.clone());
        }

        Template{
            name: config.name.clone(),
            build_file: bf_list,
        }
    }
}
