#!/usr/bin/env bash

# Fetch the latest release
wget $(curl -s https://api.github.com/repos/47ng/revelio/releases/latest \
  | grep 'browser_download_url' \
  | cut -d\" -f4)

# Install to local bin
chmod +x revelio
mv revelio /usr/local/bin/

# Print version information
revelio --version
