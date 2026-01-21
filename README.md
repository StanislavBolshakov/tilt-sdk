# T1LT SDK (T1 Layer for Tasks)

T1 Public Cloud API Rust SDK

## Crates

| Crate | Описание |
|-------|-------------|
| `tilt-sdk` | Core SDK - HTTP client, authentication, logging |
| `tilt-sdk-cloudengine` | Compute API resources |

## Установка

```toml
[dependencies]
tilt-sdk = "0.1"
tilt-sdk-cloudengine = "0.1"
```

## Быстрый старт

```rust
use tilt_sdk::ClientBuilder;
use tilt_sdk_cloudengine::ComputeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .base_url("https://api.t1.cloud")
        .token("your-token")
        .project("your-project")
        .build()?;

    let compute = ComputeClient::new(&client);
    let instances = compute.list_instances(None, None).await?;
    
    for instance in instances {
        println!("{}: {:?}", instance.name, instance.status);
    }
    
    Ok(())
}
```

## T1 Publc Cloud CLI

```bash
cargo run --package tilt-cli -- --help
```

### Переменные окружения

| Variable | Description |
|----------|-------------|
| `TILT_TOKEN` | API authentication token |
| `TILT_PROJECT` | Project ID |
| `TILT_API_URL` | API base URL (default: https://api.t1.cloud) |

## Building

```bash
# Build all crates
cargo build --all

# Run tests
cargo test --all

# Run clippy
cargo clippy --all-targets
```

## Лицензия

MIT
