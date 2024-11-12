#!/bin/bash

# Take OneDrive link as input
onedrive_link="https://1drv.ms/u/c/a77974c33a34d9bb/ERyj4V2-DkhCmwJ45ytPsCMBotTwChnIKDTdUbyPHmoAeA?e=rXAUj8"

# Remove any trailing slash
onedrive_link="${onedrive_link%/}"

# Extract the share ID (everything after 's/')
share_id=$(echo "$onedrive_link" | grep -o 's/.*' | cut -d'/' -f2)

# Base64 encode the share ID
encoded_id=$(echo -n "u!$share_id" | base64)

# Create the direct download link
direct_link="https://api.onedrive.com/v1.0/shares/$encoded_id/root/content"

echo "$direct_link"