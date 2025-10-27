#!/bin/bash

echo "Setting up Git hooks..."

# Set Git hooks directory
git config core.hooksPath .githooks

# Make hooks executable
chmod +x .githooks/pre-commit .githooks/pre-push

echo "✅ Git hooks setup complete!"
echo "Run this script once to enable the hooks for this repository."