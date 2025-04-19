use std::collections::HashMap;
use crate::models::{CommandInfo, GroupedMenuEntry, SlintMenuEntry};
use std::path::Path;
use tokio::fs as tokio_fs;

pub async fn load_menu_async() -> Vec<crate::models::CommandInfo> {
    let menu_path = Path::new("configs/future_menu.txt");
    
    if !menu_path.exists() {
        println!("Error: configs/future_menu.txt not found");
        return Vec::new();
    }
    
    match tokio_fs::read_to_string(menu_path).await {
        Ok(content) => parse_future_menu_format(&content),
        Err(e) => {
            println!("Error reading future_menu.txt: {}", e);
            Vec::new()
        }
    }
}

// Parse the future_menu.txt format into CommandInfo objects
fn parse_future_menu_format(content: &str) -> Vec<CommandInfo> {
    let mut commands = Vec::new();
    let mut current_label = String::new();
    let mut current_actions: Vec<String> = Vec::new();
    let mut current_command = String::new();
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some(label) = trimmed.strip_prefix("Label:") {
            // Process previous entry if complete
            if !current_label.is_empty() && !current_actions.is_empty() && !current_command.is_empty() {
                for action in &current_actions {
                    commands.push(CommandInfo {
                        name: format!("{} {}", current_label, action),
                        command: current_command.replace("<Action>", action),
                        description: format!("{} operation for {}", action, current_label),
                        category: current_label.clone(),
                    });
                }
            }
            
            // Start a new entry
            current_label = label.trim().trim_matches('"').to_string();
            current_actions.clear();
            current_command.clear();
        } else if let Some(actions) = trimmed.strip_prefix("Actions:") {
            current_actions = actions
                .split_whitespace()
                .map(|s| s.trim_matches('"').to_string())
                .collect();
        } else if let Some(cmd) = trimmed.strip_prefix("Command:") {
            current_command = cmd.trim().trim_matches('"').to_string();
        }
    }
    
    // Process the last entry
    if !current_label.is_empty() && !current_actions.is_empty() && !current_command.is_empty() {
        for action in &current_actions {
            commands.push(CommandInfo {
                name: format!("{} {}", current_label, action),
                command: current_command.replace("<Action>", action),
                description: format!("{} operation for {}", action, current_label),
                category: current_label.clone(),
            });
        }
    }
    
    println!("Parsed {} commands from future menu format", commands.len());
    commands
}

// Helper function for grouping commands by category (Label)
pub fn group_menu_commands(commands: &[CommandInfo]) -> HashMap<String, Vec<CommandInfo>> {
    let mut grouped: HashMap<String, Vec<CommandInfo>> = HashMap::new();
    
    for cmd in commands {
        grouped
            .entry(cmd.category.clone())
            .or_insert_with(Vec::new)
            .push(cmd.clone());
    }
    
    println!("Grouped into {} categories", grouped.len());
    grouped
}

// Create Slint menu entries for the new GUI format
pub fn create_slint_menu_entries(commands: &[CommandInfo]) -> Vec<SlintMenuEntry> {
    let grouped = group_menu_commands(commands);
    let mut result = Vec::new();
    
    for (category, cmds) in grouped {
        if !cmds.is_empty() {
            // Extract all actions for this category
            let actions: Vec<String> = cmds.iter()
                .map(|cmd| {
                    let parts: Vec<&str> = cmd.name.split(' ').collect();
                    if parts.len() > 1 {
                        parts[parts.len() - 1].to_string()
                    } else {
                        cmd.name.clone()
                    }
                })
                .collect();
            
            // Get the command template - but format it to work with concatenation
            // instead of using replace() since that's not available in Slint
            let first_cmd = &cmds[0];
            let cmd_parts: Vec<&str> = first_cmd.command.split(' ').collect();
            
            // Find which part contains the action and remove it to create a template
            // that can be used with concatenation in Slint
            let mut command_parts = Vec::new();
            let mut action_index = 0;
            
            for (i, part) in cmd_parts.iter().enumerate() {
                if actions.iter().any(|action| part.contains(action)) {
                    action_index = i;
                    break;
                }
                command_parts.push(*part);
            }
            
            // Add the remaining parts after the action
            for i in (action_index + 1)..cmd_parts.len() {
                command_parts.push(cmd_parts[i]);
            }
            
            let command_template = command_parts.join(" ");
            
            // Create the SlintMenuEntry
            result.push(SlintMenuEntry {
                label: category,
                actions,
                command_template,
            });
        }
    }
    
    result
}

// Build a list of GroupedMenuEntry objects for hierarchical display
pub fn build_grouped_entries(commands: &[CommandInfo]) -> Vec<GroupedMenuEntry> {
    let grouped = group_menu_commands(commands);
    let mut result = Vec::new();
    
    for (category, cmds) in grouped {
        if !cmds.is_empty() {
            // Extract the command template and path from the first command
            let first_cmd = &cmds[0];
            let path_parts: Vec<&str> = first_cmd.command.split("firefox").collect();
            let path_name = if path_parts.len() > 1 {
                path_parts[1].trim().to_string()
            } else {
                "unknown".to_string()
            };
            
            // Extract just the action names from the commands
            let actions = cmds.iter()
                .map(|cmd| {
                    let parts: Vec<&str> = cmd.name.split(' ').collect();
                    if parts.len() > 1 {
                        parts[parts.len() - 1].to_string()
                    } else {
                        cmd.name.clone()
                    }
                })
                .collect();
            
            // Store all the full command strings
            let commands = cmds.iter()
                .map(|cmd| cmd.command.clone())
                .collect();
            
            result.push(GroupedMenuEntry {
                program: category.clone(),
                path_name,
                actions,
                commands,
            });
        }
    }
    
    result
}

// This function is kept for compatibility with the existing code structure,
// although it always returns None with the new menu format.
// It's used in the original lib.rs exports.
pub fn extract_command_info(_line: &str) -> Option<CommandInfo> {
    None // Not used with the future_menu.txt format
}
