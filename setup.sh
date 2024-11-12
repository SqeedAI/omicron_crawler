#!/bin/bash
profile_link="https://0x0.st/XkAg.b64"

if ! mkdir -p ./sessions ./drivers 2>/dev/null; then
    echo "Failed to create necessary directories" >&2
    exit 1
fi

apt_config () {
  if ! sudo cp .config/firefox-no-snap /etc/apt/preferences.d/firefox-no-snap; then
    echo "failed to copy firefox no snap apt config" >&2
    return 1
  fi
  if ! sudo cp .config/mozilla-firefox /etc/apt/preferences.d/mozilla-firefox; then
    echo "failed to copy mozilla-firefox apt config" >&2
    return 1
  fi
  return 0
}


if ! sudo snap remove firefox; then
  echo "failed to remove firefox snap" >&2
  exit 1
fi

if ! sudo add-apt-repository ppa:mozillateam/ppa; then
  echo "failed to add mozilla ppa" >&2
  exit 1
fi

if ! apt_config; then
  echo "failed to configure apt" >&2
  exit 1
fi

if ! sudo apt update; then
  echo "failed to update apt" >&2
  exit 1
fi

if ! sudo apt install firefox; then
  echo "failed to install firefox" >&2
  exit 1
fi

if ! sudo chmod 777 /usr/lib/firefox/libxul.so; then
  echo "failed to set xul permissions" >&2
  exit 1
fi

if ! curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; then
    echo "failed to install rustup" >&2
    exit 1
fi

if ! grep -q 'export PATH="$HOME/.cargo/bin:$PATH"' ~/.bashrc; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
fi

source "$HOME/.cargo/env"

if ! rustup toolchain install stable; then
  echo "failed to install rust stable toolchain" >&2
  exit 1
fi

if ! sudo apt install build-essential; then
  echo "failed to install build-essential" >&2
  exit 1
fi

if ! sudo apt install gdb; then
  echo "failed to install gdb" >&2
  exit 1
fi

if ! curl -L $profile_link -o ./sessions/encoded.b64 ; then
    echo "Failed to download firefox profile" >&2
    exit 1
fi

if ! sudo chmod 777 ./sessions/encoded.b64; then
    echo "Failed to set permissions for encoded.b64" >&2
    exit 1
fi

temp_file=$(mktemp)

if ! curl -L "https://github.com/mozilla/geckodriver/releases/download/v0.35.0/geckodriver-v0.35.0-linux64.tar.gz" -o "$temp_file"; then
  echo "Failed to download geckodriver" >&2
  exit 1
fi

if ! tar -xvzf "$temp_file" -C ./drivers; then
    echo "Failed to extract geckodriver" >&2
    exit 1
fi

if ! sudo chmod 777 ./drivers/geckodriver; then
  echo "failed to set geckodriver permissions" >&2
  exit 1
fi

trap 'rm -f "$temp_file"' EXIT