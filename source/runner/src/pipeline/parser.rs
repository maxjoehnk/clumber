use std::collections::HashMap;
use indexmap::IndexMap;
use serde_json::Value;
use serde::Deserialize;

pub struct PipelineParser;

impl PipelineParser {
    pub fn parse(&self, pipeline: &str) -> anyhow::Result<RawPipeline> {
        let pipeline: RawPipeline = hcl::from_str(pipeline)?;

        log::trace!("{:#?}", pipeline);

        Ok(pipeline)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawPipeline {
    #[serde(default)]
    pub modules: Vec<String>,
    #[serde(rename = "stage")]
    pub stages: IndexMap<String, RawStage>,
    #[serde(default, rename = "module")]
    pub template_modules: HashMap<String, TemplateModule>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawStage {
    #[serde(default)]
    pub steps: IndexMap<String, Value>,
    #[serde(default)]
    pub parallel: Vec<RawStage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TemplateModule {
    pub result: Value,
}
