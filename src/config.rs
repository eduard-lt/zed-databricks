use serde::Deserialize;
use zed_extension_api as zed;

#[derive(Debug, Deserialize)]
pub struct DatabricksSettings {
    pub host: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub token: String,
}

impl Config {
    pub fn load(_worktree: Option<&zed::Worktree>) -> zed::Result<Self> {
        // 1. Try Zed Settings
        /*
        if let Some(settings) =
            zed::settings::read_settings::<DatabricksSettings>("databricks", worktree)?
        {
            if let (Some(host), Some(token)) = (settings.host, settings.token) {
                return Ok(Self { host, token });
            }
        }
        */

        // 2. Fallback to ~/.databrickscfg
        // Note: Direct file access might be restricted.
        // We'll attempt a simple parse of the default profile.
        // For now, let's just return an error if settings aren't found.

        Err(
            "Databricks credentials not found. Please configure 'databricks' in your settings."
                .into(),
        )
    }
}
