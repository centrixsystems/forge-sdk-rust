//! Rust SDK for the [Forge](https://github.com/centrixsystems/forge) rendering engine.
//!
//! Forge converts HTML/CSS to PDF, PNG, and other formats via an HTTP API.
//! This crate provides a typed, ergonomic client.
//!
//! # Quick start
//!
//! ```no_run
//! use forge_sdk::{ForgeClient, OutputFormat};
//!
//! # async fn example() -> Result<(), forge_sdk::ForgeError> {
//! let client = ForgeClient::new("http://localhost:3000");
//!
//! let pdf = client.render_html("<h1>Hello</h1>")
//!     .format(OutputFormat::Pdf)
//!     .paper("a4")
//!     .send()
//!     .await?;
//!
//! std::fs::write("output.pdf", &pdf).unwrap();
//! # Ok(())
//! # }
//! ```

mod error;
mod types;

pub use error::ForgeError;
pub use types::{DitherMethod, Flow, Orientation, OutputFormat, Palette};

use types::{ErrorResponse, QuantizePayload, RenderPayload};

use std::time::Duration;

/// Client for a Forge rendering server.
#[derive(Debug, Clone)]
pub struct ForgeClient {
    base_url: String,
    http: reqwest::Client,
}

impl ForgeClient {
    /// Create a client with default settings.
    ///
    /// ```
    /// let client = forge_sdk::ForgeClient::new("http://localhost:3000");
    /// ```
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
            http: reqwest::Client::new(),
        }
    }

    /// Create a builder for advanced client configuration.
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// let client = forge_sdk::ForgeClient::builder("http://localhost:3000")
    ///     .timeout(Duration::from_secs(120))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder(base_url: &str) -> ForgeClientBuilder {
        ForgeClientBuilder {
            base_url: base_url.trim_end_matches('/').to_owned(),
            timeout: None,
        }
    }

    /// Start building a render request from an HTML string.
    pub fn render_html<'a>(&'a self, html: &'a str) -> RenderRequestBuilder<'a> {
        RenderRequestBuilder {
            client: self,
            html: Some(html),
            url: None,
            format: OutputFormat::Pdf,
            width: None,
            height: None,
            paper: None,
            orientation: None,
            margins: None,
            flow: None,
            density: None,
            background: None,
            timeout: None,
            colors: None,
            palette: None,
            dither: None,
        }
    }

    /// Start building a render request from a URL.
    pub fn render_url<'a>(&'a self, url: &'a str) -> RenderRequestBuilder<'a> {
        RenderRequestBuilder {
            client: self,
            html: None,
            url: Some(url),
            format: OutputFormat::Pdf,
            width: None,
            height: None,
            paper: None,
            orientation: None,
            margins: None,
            flow: None,
            density: None,
            background: None,
            timeout: None,
            colors: None,
            palette: None,
            dither: None,
        }
    }

    /// Check if the server is healthy.
    pub async fn health(&self) -> Result<bool, ForgeError> {
        let resp = self
            .http
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;
        Ok(resp.status().is_success())
    }
}

/// Builder for configuring a [`ForgeClient`].
pub struct ForgeClientBuilder {
    base_url: String,
    timeout: Option<Duration>,
}

impl ForgeClientBuilder {
    /// Set the HTTP request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<ForgeClient, ForgeError> {
        let mut builder = reqwest::Client::builder();
        if let Some(t) = self.timeout {
            builder = builder.timeout(t);
        }
        Ok(ForgeClient {
            base_url: self.base_url,
            http: builder.build()?,
        })
    }
}

/// Builder for a render request. Created via [`ForgeClient::render_html`] or
/// [`ForgeClient::render_url`].
pub struct RenderRequestBuilder<'a> {
    client: &'a ForgeClient,
    html: Option<&'a str>,
    url: Option<&'a str>,
    format: OutputFormat,
    width: Option<u32>,
    height: Option<u32>,
    paper: Option<&'a str>,
    orientation: Option<Orientation>,
    margins: Option<&'a str>,
    flow: Option<Flow>,
    density: Option<f64>,
    background: Option<&'a str>,
    timeout: Option<u32>,
    colors: Option<u16>,
    palette: Option<Palette>,
    dither: Option<DitherMethod>,
}

impl<'a> RenderRequestBuilder<'a> {
    /// Output format (default: `OutputFormat::Pdf`).
    pub fn format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    /// Viewport width in CSS pixels.
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Viewport height in CSS pixels.
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Paper size: `"a3"`, `"a4"`, `"a5"`, `"b4"`, `"b5"`, `"letter"`,
    /// `"legal"`, `"ledger"`.
    pub fn paper(mut self, paper: &'a str) -> Self {
        self.paper = Some(paper);
        self
    }

    /// Page orientation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    /// Page margins. Preset: `"default"`, `"none"`, `"narrow"`.
    /// Custom: `"T,R,B,L"` in mm.
    pub fn margins(mut self, margins: &'a str) -> Self {
        self.margins = Some(margins);
        self
    }

    /// Document flow mode.
    pub fn flow(mut self, flow: Flow) -> Self {
        self.flow = Some(flow);
        self
    }

    /// Output DPI (default: 96).
    pub fn density(mut self, dpi: f64) -> Self {
        self.density = Some(dpi);
        self
    }

    /// Background CSS color (e.g. `"#ffffff"`).
    pub fn background(mut self, color: &'a str) -> Self {
        self.background = Some(color);
        self
    }

    /// Page load timeout in seconds (default: 30).
    pub fn timeout(mut self, seconds: u32) -> Self {
        self.timeout = Some(seconds);
        self
    }

    /// Number of colors for quantization (2-256).
    pub fn colors(mut self, n: u16) -> Self {
        self.colors = Some(n);
        self
    }

    /// Color palette preset or custom colors.
    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = Some(palette);
        self
    }

    /// Dithering algorithm for quantization.
    pub fn dither(mut self, method: DitherMethod) -> Self {
        self.dither = Some(method);
        self
    }

    /// Send the render request and return the raw output bytes.
    pub async fn send(self) -> Result<Vec<u8>, ForgeError> {
        let has_quantize =
            self.colors.is_some() || self.palette.is_some() || self.dither.is_some();

        let payload = RenderPayload {
            html: self.html,
            url: self.url,
            format: self.format,
            width: self.width,
            height: self.height,
            paper: self.paper,
            orientation: self.orientation,
            margins: self.margins,
            flow: self.flow,
            density: self.density,
            background: self.background,
            timeout: self.timeout,
            quantize: if has_quantize {
                Some(QuantizePayload {
                    colors: self.colors,
                    palette: self.palette.as_ref(),
                    dither: self.dither,
                })
            } else {
                None
            },
        };

        let resp = self
            .client
            .http
            .post(format!("{}/render", self.client.base_url))
            .json(&payload)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let message = match resp.json::<ErrorResponse>().await {
                Ok(e) => e.error,
                Err(_) => format!("HTTP {status}"),
            };
            return Err(ForgeError::Server {
                status: status.as_u16(),
                message,
            });
        }

        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_format_serializes() {
        assert_eq!(json_str(&OutputFormat::Pdf), "\"pdf\"");
        assert_eq!(json_str(&OutputFormat::Png), "\"png\"");
        assert_eq!(json_str(&OutputFormat::Jpeg), "\"jpeg\"");
        assert_eq!(json_str(&OutputFormat::Bmp), "\"bmp\"");
        assert_eq!(json_str(&OutputFormat::Tga), "\"tga\"");
        assert_eq!(json_str(&OutputFormat::Qoi), "\"qoi\"");
        assert_eq!(json_str(&OutputFormat::Svg), "\"svg\"");
    }

    #[test]
    fn orientation_serializes() {
        assert_eq!(json_str(&Orientation::Portrait), "\"portrait\"");
        assert_eq!(json_str(&Orientation::Landscape), "\"landscape\"");
    }

    #[test]
    fn flow_serializes() {
        assert_eq!(json_str(&Flow::Auto), "\"auto\"");
        assert_eq!(json_str(&Flow::Paginate), "\"paginate\"");
        assert_eq!(json_str(&Flow::Continuous), "\"continuous\"");
    }

    #[test]
    fn dither_method_serializes() {
        assert_eq!(json_str(&DitherMethod::None), "\"none\"");
        assert_eq!(json_str(&DitherMethod::FloydSteinberg), "\"floyd-steinberg\"");
        assert_eq!(json_str(&DitherMethod::Atkinson), "\"atkinson\"");
        assert_eq!(json_str(&DitherMethod::Ordered), "\"ordered\"");
    }

    #[test]
    fn palette_preset_serializes() {
        assert_eq!(json_str(&Palette::Auto), "\"auto\"");
        assert_eq!(json_str(&Palette::BlackWhite), "\"bw\"");
        assert_eq!(json_str(&Palette::Grayscale), "\"grayscale\"");
        assert_eq!(json_str(&Palette::Eink), "\"eink\"");
    }

    #[test]
    fn palette_custom_serializes() {
        let p = Palette::Custom(vec!["#ff0000".into(), "#00ff00".into()]);
        assert_eq!(json_str(&p), "[\"#ff0000\",\"#00ff00\"]");
    }

    #[test]
    fn minimal_html_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client.render_html("<h1>Hi</h1>");
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        assert_eq!(v["html"], "<h1>Hi</h1>");
        assert_eq!(v["format"], "pdf");
        assert!(v.get("url").is_none());
        assert!(v.get("width").is_none());
        assert!(v.get("quantize").is_none());
    }

    #[test]
    fn url_payload_with_options() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_url("https://example.com")
            .format(OutputFormat::Png)
            .width(1280)
            .height(800)
            .paper("letter")
            .orientation(Orientation::Landscape)
            .margins("10,20,10,20")
            .flow(Flow::Paginate)
            .density(300.0)
            .background("#ffffff")
            .timeout(60);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        assert!(v.get("html").is_none());
        assert_eq!(v["url"], "https://example.com");
        assert_eq!(v["format"], "png");
        assert_eq!(v["width"], 1280);
        assert_eq!(v["height"], 800);
        assert_eq!(v["paper"], "letter");
        assert_eq!(v["orientation"], "landscape");
        assert_eq!(v["margins"], "10,20,10,20");
        assert_eq!(v["flow"], "paginate");
        assert_eq!(v["density"], 300.0);
        assert_eq!(v["background"], "#ffffff");
        assert_eq!(v["timeout"], 60);
        assert!(v.get("quantize").is_none());
    }

    #[test]
    fn quantize_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<p>test</p>")
            .format(OutputFormat::Png)
            .colors(16)
            .palette(Palette::Auto)
            .dither(DitherMethod::FloydSteinberg);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let q = &v["quantize"];
        assert_eq!(q["colors"], 16);
        assert_eq!(q["palette"], "auto");
        assert_eq!(q["dither"], "floyd-steinberg");
    }

    #[test]
    fn quantize_with_custom_palette() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<p>test</p>")
            .palette(Palette::Custom(vec![
                "#000000".into(),
                "#ffffff".into(),
                "#ff0000".into(),
            ]))
            .dither(DitherMethod::Atkinson);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let q = &v["quantize"];
        assert_eq!(q["palette"], serde_json::json!(["#000000", "#ffffff", "#ff0000"]));
        assert_eq!(q["dither"], "atkinson");
    }

    #[test]
    fn no_quantize_when_unset() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client.render_html("<p>test</p>").format(OutputFormat::Png);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert!(v.get("quantize").is_none());
    }

    #[test]
    fn base_url_trailing_slash_stripped() {
        let client = ForgeClient::new("http://localhost:3000/");
        assert_eq!(client.base_url, "http://localhost:3000");
    }

    #[test]
    fn builder_creates_client() {
        let client = ForgeClient::builder("http://localhost:3000")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        assert_eq!(client.base_url, "http://localhost:3000");
    }

    // -- helpers --

    fn json_str<T: serde::Serialize>(val: &T) -> String {
        serde_json::to_string(val).unwrap()
    }

    fn build_payload(builder: &RenderRequestBuilder<'_>) -> String {
        let has_quantize =
            builder.colors.is_some() || builder.palette.is_some() || builder.dither.is_some();

        let payload = RenderPayload {
            html: builder.html,
            url: builder.url,
            format: builder.format,
            width: builder.width,
            height: builder.height,
            paper: builder.paper,
            orientation: builder.orientation,
            margins: builder.margins,
            flow: builder.flow,
            density: builder.density,
            background: builder.background,
            timeout: builder.timeout,
            quantize: if has_quantize {
                Some(QuantizePayload {
                    colors: builder.colors,
                    palette: builder.palette.as_ref(),
                    dither: builder.dither,
                })
            } else {
                None
            },
        };
        serde_json::to_string(&payload).unwrap()
    }
}
