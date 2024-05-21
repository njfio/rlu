#!/bin/bash
date="$1"

# Fetch entries based on date and process the output
rlu show --date "$date"
