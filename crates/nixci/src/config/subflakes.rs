use std::collections::BTreeMap;

use serde::Deserialize;

use super::subflake::SubflakeConfig;

#[derive(Debug, Deserialize)]
pub struct SubflakesConfig(
    // NB: we use BTreeMap instead of HashMap here so that we always iterate
    // configs in a determinitstic (i.e. asciibetical) order
    pub BTreeMap<String, SubflakeConfig>,
);

impl Default for SubflakesConfig {
    /// Default value contains a single entry for the root flake.
    fn default() -> Self {
        let mut subflakes = BTreeMap::new();
        subflakes.insert("<root>".to_string(), SubflakeConfig::default());
        SubflakesConfig(subflakes)
    }
}