#!/bin/bash
# This script fetches fields from Logseq

# Replace with the actual command to fetch fields
fields_json=$(logseq get-fields)  # Replace with actual command

# Parse JSON and extract field names
field_names=$(echo "$fields_json" | jq -r '.fields[]')

# Output each field name
echo "$field_names" | while IFS= read -r field; do
  echo $field
done
