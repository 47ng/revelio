#!/usr/bin/env bash

# Fetch the latest release
wget https://github.com/47ng/revelio/releases/download/0.1.2/revelio

# Install to local bin
chmod +x revelio
sudo mv revelio /usr/local/bin

# Print version information
revelio --version
