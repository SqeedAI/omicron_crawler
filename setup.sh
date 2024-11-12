#!/bin/bash
profile_link="https://0x0.st/XkAg.b64"

if ! curl -L $profile_link -o ./sessions/encoded.b64 ; then
    echo "Failed to download firefox profile"
    exit 1
fi