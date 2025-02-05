# KubeLLM

An opinionated LLM proxy written in Rust to be used with Kubernetes or standalone.

> [!WARNING]
> Under development, not ready for production.

## Design goals

- An API that allows calling different LLM providers based on the OpenAI spec
- Transparent configuration in code
- Provides a way to generate virtual keys in Kubernetes
- Spend and usage tracking
- Logging into local database
- Prometheus metrics

## Personal goals

- Learning Rust
