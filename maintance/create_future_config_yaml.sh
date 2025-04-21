#!/bin/bash
# filepath: /workspaces/CTMenu_Runner/maintance/create_menu_yaml.sh

# Generate YAML configuration
cat > ./menu_config.yaml << EOF
# Button definitions with their state behavior
button_types:
  start:
    applicable_states: [stopped, killed]
    next_state: running
  freeze:
    applicable_states: [running]
    next_state: frozen
  unfreeze:
    applicable_states: [frozen]
    next_state: running
  kill:
    applicable_states: [running, frozen]
    next_state: killed

# Profiles with their actions
profiles:
EOF

# Loop through Firefox profiles to add them to the YAML file
for profile in ~/Stuff/Settings/firefox/*; do
    # Extract label from profile path
    label="$(echo ${profile} | cut -d/ -f7 | cut -d. -f2)"
    
    # Add profile to YAML configuration
    cat >> ./menu_config.yaml << EOF
  - label: ${label}
    current_state: stopped
    actions: [start, freeze, unfreeze, kill]
    command: "./target/debug/Menu_Runner_system ACTION firefox /path/to/${label}"
EOF
done

# Generate UI colors in a separate file
cat > ./ui_colors.yaml << EOF
# Colors for different states
states:
  stopped:
    label_color: "#E0BC00"
  running:
    label_color: "#FFD32C"
  frozen:
    label_color: "#E0BC00"
  killed:
    label_color: "#E0BC00"

# Colors for different button types
button_types:
  start:
    highlight_color: "#00AA00"
    inactive_color: "#005500"
  freeze:
    highlight_color: "#0000AA"
    inactive_color: "#000055"
  unfreeze:
    highlight_color: "#00AAAA"
    inactive_color: "#005555"
  kill:
    highlight_color: "#AA0000"
    inactive_color: "#550000"
EOF

echo "Generated menu_config.yaml and ui_colors.yaml"