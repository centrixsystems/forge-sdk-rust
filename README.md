# forge-sdk

Rust SDK for the [Forge](https://github.com/centrixsystems/forge) rendering engine. Converts HTML/CSS to PDF, PNG, and other formats via a running Forge server.

## Installation

```sh
cargo add forge-sdk
```

Or add to `Cargo.toml`:

```toml
[dependencies]
forge-sdk = "0.1"
```

## Quick Start

```rust
use forge_sdk::{ForgeClient, OutputFormat};

#[tokio::main]
async fn main() -> Result<(), forge_sdk::ForgeError> {
    let client = ForgeClient::new("http://localhost:3000");

    let pdf = client.render_html("<h1>Invoice #1234</h1>")
        .format(OutputFormat::Pdf)
        .paper("a4")
        .send()
        .await?;

    std::fs::write("invoice.pdf", &pdf)?;
    Ok(())
}
```

## Usage

### Render HTML to PDF

```rust
let pdf = client.render_html("<h1>Hello</h1>")
    .format(OutputFormat::Pdf)
    .paper("a4")
    .orientation(Orientation::Portrait)
    .margins("25.4,25.4,25.4,25.4")
    .flow(Flow::Paginate)
    .send()
    .await?;
```

### Render URL to PNG

```rust
let png = client.render_url("https://example.com")
    .format(OutputFormat::Png)
    .width(1280)
    .height(800)
    .send()
    .await?;
```

### Color Quantization (e-ink, limited palettes)

```rust
use forge_sdk::{Palette, DitherMethod};

let eink = client.render_html("<h1>Dashboard</h1>")
    .format(OutputFormat::Png)
    .palette(Palette::Eink)
    .dither(DitherMethod::FloydSteinberg)
    .send()
    .await?;
```

### Custom Client Configuration

```rust
use std::time::Duration;

let client = ForgeClient::builder("http://forge:3000")
    .timeout(Duration::from_secs(120))
    .build()?;
```

### Health Check

```rust
let healthy = client.health().await?;
```

## API Reference

### `ForgeClient`

| Method | Description |
|--------|-------------|
| `new(base_url)` | Create a client with default settings |
| `builder(base_url)` | Create a client builder for advanced configuration |
| `render_html(html)` | Start a render request from an HTML string |
| `render_url(url)` | Start a render request from a URL |
| `health()` | Check server health |

### `RenderRequestBuilder`

All methods return `Self` for chaining. Call `.send().await` to execute.

| Method | Type | Description |
|--------|------|-------------|
| `format` | `OutputFormat` | Output format (default: `Pdf`) |
| `width` | `u32` | Viewport width in CSS pixels |
| `height` | `u32` | Viewport height in CSS pixels |
| `paper` | `&str` | Paper size: a3, a4, a5, b4, b5, letter, legal, ledger |
| `orientation` | `Orientation` | Portrait or Landscape |
| `margins` | `&str` | Preset (default, none, narrow) or "T,R,B,L" in mm |
| `flow` | `Flow` | Auto, Paginate, or Continuous |
| `density` | `f64` | Output DPI (default: 96) |
| `background` | `&str` | CSS background color |
| `timeout` | `u32` | Page load timeout in seconds |
| `colors` | `u16` | Quantization color count (2-256) |
| `palette` | `Palette` | Color palette preset or custom hex colors |
| `dither` | `DitherMethod` | Dithering algorithm |

### Enums

**`OutputFormat`**: `Pdf`, `Png`, `Jpeg`, `Bmp`, `Tga`, `Qoi`, `Svg`

**`Orientation`**: `Portrait`, `Landscape`

**`Flow`**: `Auto`, `Paginate`, `Continuous`

**`Palette`**: `Auto`, `BlackWhite`, `Grayscale`, `Eink`, `Custom(Vec<String>)`

**`DitherMethod`**: `None`, `FloydSteinberg`, `Atkinson`, `Ordered`

### Errors

`ForgeError` has two variants:

- **`Http`** — network/connection errors (wraps `reqwest::Error`)
- **`Server { status, message }`** — 4xx/5xx responses from the server

## License

MIT
