![Closed RouterBanner](/closed-router-logo.jpeg)

# Closed Router

**Closed Router** is a high-performance, self-hosted **OpenRouter alternative** written in **Rust**.  
It provides a secure and extensible routing layer for LLM providers, giving you full control over model access, keys, traffic, and observability—without relying on third-party gateways.

## Why Closed Router?

- 🚀 **Blazing fast** — built in Rust for low-latency request routing  
- 🔐 **Self-hosted & private** — your API keys never leave your infra  
- 🔄 **Provider-agnostic** — route requests across multiple LLM providers  
- ⚖️ **Smart routing** — model selection, fallbacks, and load balancing  
- 📊 **Observability ready** — logs, metrics, and tracing hooks  
- 🧩 **Extensible** — add new providers and routing rules easily  

## Features

- OpenAI-compatible API surface  
- Multi-provider support (OpenAI, Anthropic, local models, etc.)  
- API key management & request validation  
- Rate limiting and quotas  
- Failover and retry strategies  
- Streaming responses  
- Config-driven routing rules  

## Getting Started

```bash
git clone https://github.com/saksham1387/closed-router
cd closed-router
cargo run --release
