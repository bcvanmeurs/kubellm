# KubeLLM

An opinionated LLM proxy written in Rust to be used with Kubernetes or standalone.

> [!WARNING]
> Under development, not ready for production.

## Quick start

```bash
cargo run
```

In another terminal:

```bash
❯ xh 127.0.0.1:3000/v1/chat/completions model=gpt-4o-mini messages[0][role]=user messages[0][content]="Hello, who are you"
HTTP/1.1 200 OK
Content-Length: 747
Content-Type: application/json
Date: Fri, 07 Feb 2025 16:10:10 GMT

{
    "id": "chatcmpl-AyLDdtuquP488gFntfcYARiPKwJBZ",
    "choices": [
        {
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello! I'm an AI language model created by OpenAI. I'm here to help you with information, answer questions, and assist you with a variety of topics. How can I help you today?",
                "refusal": null
            },
            "finish_reason": "stop",
            "logprobs": null
        }
    ],
    "created": 1738944609,
    "model": "gpt-4o-mini-2024-07-18",
    "service_tier": "default",
    "system_fingerprint": "fp_72ed7ab54c",
    "object": "chat.completion",
    "usage": {
        "completion_tokens": 40,
        "prompt_tokens": 12,
        "total_tokens": 52,
        "completion_tokens_details": {
            "accepted_prediction_tokens": 0,
            "audio_tokens": 0,
            "reasoning_tokens": 0,
            "rejected_prediction_tokens": 0
        },
        "prompt_tokens_details": {
            "audio_tokens": 0,
            "cached_tokens": 0
        }
    }
}
```

Output of proxy:

```bash
❯ cargo run
   Compiling kubellm v0.0.1 (/Users/bram.vanmeurs/repos/kubellm)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.57s
     Running `target/debug/kubellm`
Listening on 127.0.0.1:3000
Received request
Prompt tokens:     12
Completion tokens: 40
Total tokens:      52
```

## Design goals

- An API that allows calling different LLM providers based on the OpenAI spec
- Transparent configuration in code
- Provides a way to generate virtual keys in Kubernetes
- Spend and usage tracking
- Logging into local database
- Prometheus metrics

## Personal goals

- Learning Rust
