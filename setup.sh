#!/bin/bash

# Take OneDrive link as input
onedrive_link="https://1drv.ms/u/c/a77974c33a34d9bb/ERyj4V2-DkhCmwJ45ytPsCMBotTwChnIKDTdUbyPHmoAeA?e=rXAUj8"

# Base64 encode the entire URL
encoded_id=$(echo -n "$onedrive_link" | base64)

# Create the direct download link
direct_link="https://api.onedrive.com/v1.0/shares/$encoded_id/root/content"

echo "$direct_link"