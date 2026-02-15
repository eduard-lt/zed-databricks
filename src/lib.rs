mod config;

use zed_extension_api as zed;

struct DatabricksExtension;

impl zed::Extension for DatabricksExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "databricks" => {
                let subcommand = args
                    .first()
                    .map(|s| s.as_str())
                    .unwrap_or("test-connection");

                match subcommand {
                    "test-connection" | "" => {
                        // Probe: Try to list directory or read a file
                        let mut probe_result = String::new();

                        // Try std::env::var("HOME")
                        match std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                            Ok(home) => {
                                probe_result.push_str(&format!("Home: {}\n", home));
                                let config_path = format!("{}/.databrickscfg", home);
                                match std::fs::read_to_string(&config_path) {
                                    Ok(_content) => {
                                        probe_result.push_str("Found .databrickscfg!\n")
                                    }
                                    Err(e) => probe_result.push_str(&format!(
                                        "Failed to read .databrickscfg: {}\n",
                                        e
                                    )),
                                }
                            }
                            Err(_) => probe_result.push_str("Could not find HOME env var.\n"),
                        }

                        let text = format!("Probe Result:\n{}", probe_result);
                        Ok(zed::SlashCommandOutput {
                            sections: vec![zed::SlashCommandOutputSection {
                                range: zed::Range {
                                    start: 0,
                                    end: text.len() as u32,
                                },
                                label: "Status".to_string(),
                            }],
                            text,
                        })
                    }
                    _ => Err(format!("Unknown command: {}", subcommand)),
                }
            }
            _ => Err("Unknown slash command".to_string()),
        }
    }
}

zed::register_extension!(DatabricksExtension);
