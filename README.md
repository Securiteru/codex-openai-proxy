# Codex OpenAI Proxy

A proxy server that allows CLINE (Claude Code) and other OpenAI-compatible extensions to use ChatGPT Plus tokens from Codex authentication instead of requiring separate OpenAI API keys.

## Overview

This proxy bridges the gap between:
- **Input**: Standard OpenAI Chat Completions API format (what CLINE expects)
- **Output**: ChatGPT Responses API format (what ChatGPT backend uses)

## Features

- ✅ **OpenAI API Compatibility**: Accepts standard OpenAI Chat Completions requests
- ✅ **ChatGPT Plus Integration**: Uses your existing ChatGPT Plus tokens  
- ✅ **Cloudflare Bypass**: Handles ChatGPT's Cloudflare protection with browser-like headers
- ✅ **HTTPS Support**: Works with extensions requiring secure connections (via ngrok)
- ✅ **Streaming Responses**: Full streaming support for real-time responses
- ✅ **CLINE Compatible**: Tested extensively with CLINE VS Code extension
- ✅ **Array Content Support**: Handles both string and array message formats from OpenAI SDK
- ✅ **Universal Routing**: Bulletproof request routing that bypasses complex warp conflicts

## Quick Start

### 1. Build and Run

```bash
git clone https://github.com/Securiteru/codex-openai-proxy.git
cd codex-openai-proxy
cargo build --release
./target/release/codex-openai-proxy --port 8888 --auth-path ~/.codex/auth.json
```

### 2. Setup HTTPS Tunnel (Required for CLINE)

Most VS Code extensions require HTTPS:

```bash
# Install ngrok and create your own static domain at https://dashboard.ngrok.com/domains
# Replace 'your-static-domain' with your unique domain name
ngrok http 8888 --domain=your-static-domain.ngrok-free.app
```

**Security Note**: Always use your own unique ngrok domain. Do not share your domain publicly to prevent unauthorized access to your proxy.

### 3. Configure CLINE Extension

In VS Code CLINE settings:
- **Base URL**: `https://your-static-domain.ngrok-free.app`
- **Model**: `gpt-5` (or `gpt-4`)
- **API Key**: Any value (not used, but required by extension)

### 4. Test Connection

```bash
# Health check
curl https://your-static-domain.ngrok-free.app/health

# Test completion
curl -X POST https://your-static-domain.ngrok-free.app/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer test-key" \
  -d '{
    "model": "gpt-5",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## How It Works

### Request Flow

1. **CLINE** → Chat Completions format → **Proxy**
2. **Proxy** → Converts to Responses API → **ChatGPT Backend**
3. **ChatGPT Backend** → Responses API format → **Proxy**
4. **Proxy** → Converts to Chat Completions → **CLINE**

### Format Conversion

**Chat Completions Request:**
```json
{
  "model": "gpt-5",
  "messages": [
    {"role": "user", "content": "Hello!"}
  ]
}
```

**Responses API Request:**
```json
{
  "model": "gpt-5", 
  "instructions": "You are a helpful AI assistant.",
  "input": [
    {
      "type": "message",
      "role": "user", 
      "content": [{"type": "input_text", "text": "Hello!"}]
    }
  ],
  "tools": [],
  "tool_choice": "auto",
  "store": false,
  "stream": false
}
```

## Configuration

### Command Line Options

```bash
codex-openai-proxy [OPTIONS]

Options:
  -p, --port <PORT>          Port to listen on [default: 8080]
      --auth-path <PATH>     Path to Codex auth.json [default: ~/.codex/auth.json]
  -h, --help                 Print help
  -v, --version              Print version
```

### Authentication

The proxy automatically reads authentication from your Codex `auth.json` file:

```json
{
  "access_token": "eyJ...",
  "account_id": "db1fc050-5df3-42c1-be65-9463d9d23f0b",
  "api_key": "sk-proj-..."
}
```

**Priority**: Uses `access_token` + `account_id` for ChatGPT Plus accounts, falls back to `api_key` for standard OpenAI accounts.

## API Endpoints

### Health Check
- **GET** `/health`
- Returns service status

### Chat Completions
- **POST** `/v1/chat/completions`
- OpenAI-compatible chat completions endpoint
- Supports: messages, model, temperature, max_tokens, stream, tools

## Troubleshooting

### Common Issues

**Connection Refused:**
```bash
# Check if proxy is running
curl http://localhost:8080/health
```

**Authentication Errors:**
```bash
# Verify auth.json exists and has valid tokens
cat ~/.codex/auth.json | jq .
```

**Backend Errors:**
```bash
# Check proxy logs for detailed error messages
RUST_LOG=debug cargo run
```

### Debug Mode

```bash
# Run with debug logging
RUST_LOG=debug cargo run -- --port 8080

# Test with verbose curl
curl -v -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-5", "messages": [{"role": "user", "content": "Test"}]}'
```

## Development

### Building

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```

### Adding Features

The proxy is designed to be extensible:

- **New endpoints**: Add routes in `main.rs`
- **Format conversion**: Modify conversion functions
- **Authentication**: Extend `AuthData` structure
- **Streaming**: Add SSE support for real-time responses

## License

This project is part of the Codex ecosystem and follows the same licensing as the main Codex repository.