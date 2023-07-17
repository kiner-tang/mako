use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::compiler::Context;
use crate::load::{handle_asset, Content, LoadError};
use crate::plugin::{Plugin, PluginLoadParam};

pub struct AssetsPlugin {}

impl Plugin for AssetsPlugin {
    fn name(&self) -> &str {
        "assets"
    }

    fn load(&self, param: &PluginLoadParam, context: &Arc<Context>) -> Result<Option<Content>> {
        if matches!(param.ext_name.as_str(), "less" | "sass" | "scss" | "stylus") {
            return Err(anyhow!(LoadError::UnsupportedExtName {
                ext_name: param.ext_name.clone(),
                path: param.path.clone(),
            }));
        }

        let asset_content = handle_asset(context, param.path.as_str())?;
        Ok(Some(Content::Js(format!(
            "module.exports = \"{}\";",
            asset_content
        ))))
    }
}