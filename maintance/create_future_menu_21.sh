#!/bin/bash
# filepath: /workspaces/CTMenu_Runner/maintance/create_future_menu_json.sh

# Start JSON object with button types and profiles
echo "{"

# Define global button types and their state behavior (without colors)
echo "  \"button_types\": {"
echo "    \"start\": {"
echo "      \"applicable_states\": [\"stopped\", \"killed\"],"
echo "      \"next_state\": \"started\""
echo "    },"
echo "    \"freeze\": {"
echo "      \"applicable_states\": [\"started\"],"
echo "      \"next_state\": \"frozen\""
echo "    },"
echo "    \"unfreeze\": {"
echo "      \"applicable_states\": [\"frozen\"],"
echo "      \"next_state\": \"started\""
echo "    },"
echo "    \"kill\": {"
echo "      \"applicable_states\": [\"started\", \"frozen\"],"
echo "      \"next_state\": \"killed\""
echo "    }"
echo "  },"

# Start profiles array
echo "  \"profiles\": ["

# Track if we need a comma separator
first_entry=true

# Loop through all Firefox profiles
for profile in ~/Stuff/Settings/firefox/*; do
    # Extract label from profile path
    label="$(echo ${profile} | cut -d/ -f7 | cut -d. -f2)"
    
    # If this is not the first entry, add a comma
    if [ "$first_entry" = false ]; then
        echo "    },"
    else
        first_entry=false
    fi
    
    # Create JSON object for this profile
    echo "    {"
    echo "      \"label\": \"${label}\","
    echo "      \"current_state\": \"stopped\","
    echo "      \"actions\": ["
    
    # Add standard action buttons - could filter these from words.txt if needed
    echo "        \"start\","
    echo "        \"freeze\","
    echo "        \"unfreeze\","
    echo "        \"kill\""
    
    echo "      ]"
done

echo "    }"
echo "  ]" 
echo "}"