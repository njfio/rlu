#!/bin/bash
date="$1"

# Fetch entries based on date and process the output
logseq show --date "$date"
