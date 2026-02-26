# forge-sdk

Rust SDK for the [Forge](https://github.com/centrixsystems/forge) rendering engine. Converts HTML/CSS to PDF, PNG, and other formats via a running Forge server.

Uses [`reqwest`](https://crates.io/crates/reqwest) with `rustls-tls`. Fully async.

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
use forge_sdk::{ForgeClient, OutputFormat, Orientation, Flow};

let client = ForgeClient::new("http://localhost:3000");

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
    .density(2.0)
    .send()
    .await?;
```

### Color Quantization

Reduce colors for e-ink displays or limited-palette output.

```rust
use forge_sdk::{Palette, DitherMethod};

let eink = client.render_html("<h1>Dashboard</h1>")
    .format(OutputFormat::Png)
    .palette(Palette::Eink)
    .dither(DitherMethod::FloydSteinberg)
    .send()
    .await?;
```

### Custom Palette

```rust
let img = client.render_html("<h1>Brand</h1>")
    .format(OutputFormat::Png)
    .palette(Palette::Custom(vec![
        "#000000".into(),
        "#ffffff".into(),
        "#ff0000".into(),
    ]))
    .dither(DitherMethod::Atkinson)
    .send()
    .await?;
```

### PDF Metadata

Embed metadata and generate bookmarks in PDF output.

```rust
let pdf = client.render_html("<h1>Annual Report</h1><h2>Q4 Results</h2>")
    .format(OutputFormat::Pdf)
    .paper("a4")
    .pdf_title("Annual Report 2025")
    .pdf_author("Jane Doe")
    .pdf_subject("Financial Results")
    .pdf_keywords("finance,annual,report")
    .pdf_creator("Centrix ERP")
    .pdf_bookmarks(true)
    .send()
    .await?;
```

### Custom Client Configuration

```rust
use std::time::Duration;

let client = ForgeClient::builder("http://forge:3000")
    .timeout(Duration::from_secs(300))
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
| `new(base_url)` | Create a client with default settings (120s timeout) |
| `builder(base_url)` | Create a client builder for advanced configuration |
| `render_html(html)` | Start a render request from an HTML string |
| `render_url(url)` | Start a render request from a URL |
| `health()` | Check server health (`true` if reachable and healthy) |

### `ForgeClientBuilder`

| Method | Description |
|--------|-------------|
| `timeout(Duration)` | Set the HTTP request timeout |
| `build()` | Build the client |

### `RenderRequestBuilder`

All methods consume and return `Self` for chaining. Call `.send().await` to execute.

| Method | Type | Description |
|--------|------|-------------|
| `format` | `OutputFormat` | Output format (default: `Pdf`) |
| `width` | `u32` | Viewport width in CSS pixels |
| `height` | `u32` | Viewport height in CSS pixels |
| `paper` | `&str` | Paper size: a3, a4, a5, b4, b5, letter, legal, ledger |
| `orientation` | `Orientation` | Portrait or Landscape |
| `margins` | `&str` | Preset (`default`, `none`, `narrow`) or `"T,R,B,L"` in mm |
| `flow` | `Flow` | Auto, Paginate, or Continuous |
| `density` | `f64` | Output DPI (default: 96) |
| `background` | `&str` | CSS background color (e.g. `"#ffffff"`) |
| `timeout` | `u64` | Page load timeout in seconds |
| `colors` | `u16` | Quantization color count (2-256) |
| `palette` | `Palette` | Color palette preset or custom hex colors |
| `dither` | `DitherMethod` | Dithering algorithm |
| `pdf_title` | `&str` | PDF metadata: document title |
| `pdf_author` | `&str` | PDF metadata: document author |
| `pdf_subject` | `&str` | PDF metadata: document subject |
| `pdf_keywords` | `&str` | PDF metadata: comma-separated keywords |
| `pdf_creator` | `&str` | PDF metadata: creator application name |
| `pdf_bookmarks` | `bool` | Generate PDF bookmarks from headings |

### Enums

**`OutputFormat`**: `Pdf`, `Png`, `Jpeg`, `Bmp`, `Tga`, `Qoi`, `Svg`

**`Orientation`**: `Portrait`, `Landscape`

**`Flow`**: `Auto`, `Paginate`, `Continuous`

**`Palette`**: `Auto`, `BlackWhite`, `Grayscale`, `Eink`, `Custom(Vec<String>)`

**`DitherMethod`**: `None`, `FloydSteinberg`, `Atkinson`, `Ordered`

### Errors

`ForgeError` has two variants:

| Variant | Description |
|---------|-------------|
| `Http(reqwest::Error)` | Network/connection errors (DNS, timeout, connection refused) |
| `Server { status: u16, message: String }` | Server returned 4xx/5xx with error message |

## Requirements

- Rust 1.63+
- A running [Forge](https://github.com/centrixsystems/forge) server

## License

MIT
