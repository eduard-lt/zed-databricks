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
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "databricks" => {
                if args.is_empty() {
                    return Err("Usage: /databricks <command> (e.g., test-connection)".to_string());
                }

                match args[0].as_str() {
                    "test-connection" => {
                        let text = "Testing Databricks connection... (Mock)".to_string();
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
                    _ => Err(format!("Unknown command: {}", args[0])),
                }
            }
            _ => Err("Unknown slash command".to_string()),
        }
    }
}

zed::register_extension!(DatabricksExtension);
