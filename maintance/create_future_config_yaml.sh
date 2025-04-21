#!/bin/bash
# filepath: /workspaces/CTMenu_Runner/maintance/create_future_config_yaml.sh

# Generate YAML configuration
cat > ./menu_config.yaml << EOF
# Button definitions 
button_types:
  start:
    label: "Start"
    order: 1
  freeze:
    label: "Freeze"
    order: 2
  unfreeze:
    label: "Unfreeze"
    order: 3
  kill:
    label: "Kill"
    order: 4

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
    button_states:
      start: false
      freeze: false
      unfreeze: false
      kill: false
    actions: [start, freeze, unfreeze, kill]
    command: "./target/debug/Menu_Runner_system ACTION firefox /path/to/${label}"
EOF
done

# Generate UI colors in a separate file
cat > ./ui_colors.yaml << EOF
# Colors for different button states
button_styles:
  active:
    color: "#FFD32C"
  inactive:
    color: "#E0BC00"

# Colors for specific buttons
button_types:
  start:
    active_color: "#00AA00"
    inactive_color: "#005500"
  freeze:
    active_color: "#0000AA"
    inactive_color: "#000055"
  unfreeze:
    active_color: "#00AAAA"
    inactive_color: "#005555"
  kill:
    active_color: "#AA0000"
    inactive_color: "#550000"
EOF

echo "Generated menu_config.yaml and ui_colors.yaml with simplified button states "