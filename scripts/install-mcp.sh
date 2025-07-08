#!/bin/bash
# scripts/install-mcp.sh

set -e

echo "Installing MCP servers for Timeless..."

# Check prerequisites
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is required for MCP servers"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo "Warning: Docker not found. Some MCP servers may not be available."
fi

# Install Node.js MCP servers
echo "Installing Node.js MCP servers..."
npm install -g @modelcontextprotocol/server-slack || echo "Slack MCP server installation failed"
npm install -g @modelcontextprotocol/server-github || echo "GitHub MCP server installation failed"
npm install -g @modelcontextprotocol/server-atlassian || echo "Atlassian MCP server installation failed"
npm install -g @modelcontextprotocol/server-filesystem || echo "Filesystem MCP server installation failed"

# Pull Docker MCP servers if Docker is available
if command -v docker &> /dev/null; then
    echo "Installing Docker MCP servers..."
    docker pull docker/mcp-servers:email || echo "Email MCP server pull failed"
    docker pull docker/mcp-servers:web-search || echo "Web search MCP server pull failed"
fi

echo "MCP server installation complete!"
echo "Please configure your .env file with the required tokens and credentials."