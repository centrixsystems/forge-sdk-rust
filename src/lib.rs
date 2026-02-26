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
pub use types::{DitherMethod, EmbedRelationship, Flow, Orientation, OutputFormat, Palette, PdfStandard, WatermarkLayer};

use types::{EmbeddedFilePayload, ErrorResponse, PdfPayload, QuantizePayload, RenderPayload, WatermarkPayload};

use std::time::Duration;

/// Client for a Forge rendering server.
#[derive(Debug, Clone)]
#[must_use]
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
            pdf_title: None,
            pdf_author: None,
            pdf_subject: None,
            pdf_keywords: None,
            pdf_creator: None,
            pdf_bookmarks: None,
            pdf_watermark_text: None,
            pdf_watermark_image: None,
            pdf_watermark_opacity: None,
            pdf_watermark_rotation: None,
            pdf_watermark_color: None,
            pdf_watermark_font_size: None,
            pdf_watermark_scale: None,
            pdf_watermark_layer: None,
            pdf_standard: None,
            pdf_embedded_files: vec![],
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
            pdf_title: None,
            pdf_author: None,
            pdf_subject: None,
            pdf_keywords: None,
            pdf_creator: None,
            pdf_bookmarks: None,
            pdf_watermark_text: None,
            pdf_watermark_image: None,
            pdf_watermark_opacity: None,
            pdf_watermark_rotation: None,
            pdf_watermark_color: None,
            pdf_watermark_font_size: None,
            pdf_watermark_scale: None,
            pdf_watermark_layer: None,
            pdf_standard: None,
            pdf_embedded_files: vec![],
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
#[must_use]
pub struct ForgeClientBuilder {
    base_url: String,
    timeout: Option<Duration>,
}

impl ForgeClientBuilder {
    /// Set the HTTP request timeout.
    #[must_use]
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
#[must_use]
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
    timeout: Option<u64>,
    colors: Option<u16>,
    palette: Option<Palette>,
    dither: Option<DitherMethod>,
    pdf_title: Option<&'a str>,
    pdf_author: Option<&'a str>,
    pdf_subject: Option<&'a str>,
    pdf_keywords: Option<&'a str>,
    pdf_creator: Option<&'a str>,
    pdf_bookmarks: Option<bool>,
    pdf_watermark_text: Option<&'a str>,
    pdf_watermark_image: Option<&'a str>,
    pdf_watermark_opacity: Option<f32>,
    pdf_watermark_rotation: Option<f32>,
    pdf_watermark_color: Option<&'a str>,
    pdf_watermark_font_size: Option<f32>,
    pdf_watermark_scale: Option<f32>,
    pdf_watermark_layer: Option<WatermarkLayer>,
    pdf_standard: Option<PdfStandard>,
    pdf_embedded_files: Vec<(String, String, Option<String>, Option<String>, Option<EmbedRelationship>)>,
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
    pub fn timeout(mut self, seconds: u64) -> Self {
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

    /// PDF metadata: document title.
    pub fn pdf_title(mut self, title: &'a str) -> Self {
        self.pdf_title = Some(title);
        self
    }

    /// PDF metadata: document author.
    pub fn pdf_author(mut self, author: &'a str) -> Self {
        self.pdf_author = Some(author);
        self
    }

    /// PDF metadata: document subject.
    pub fn pdf_subject(mut self, subject: &'a str) -> Self {
        self.pdf_subject = Some(subject);
        self
    }

    /// PDF metadata: comma-separated keywords.
    pub fn pdf_keywords(mut self, keywords: &'a str) -> Self {
        self.pdf_keywords = Some(keywords);
        self
    }

    /// PDF metadata: creator application name.
    pub fn pdf_creator(mut self, creator: &'a str) -> Self {
        self.pdf_creator = Some(creator);
        self
    }

    /// Enable PDF bookmarks generated from headings.
    pub fn pdf_bookmarks(mut self, enabled: bool) -> Self {
        self.pdf_bookmarks = Some(enabled);
        self
    }

    /// Watermark text overlay on each PDF page.
    pub fn pdf_watermark_text(mut self, text: &'a str) -> Self {
        self.pdf_watermark_text = Some(text);
        self
    }

    /// Watermark image (base64-encoded PNG/JPEG).
    pub fn pdf_watermark_image(mut self, base64_data: &'a str) -> Self {
        self.pdf_watermark_image = Some(base64_data);
        self
    }

    /// Watermark opacity (0.0-1.0, default 0.15).
    pub fn pdf_watermark_opacity(mut self, opacity: f32) -> Self {
        self.pdf_watermark_opacity = Some(opacity);
        self
    }

    /// Watermark rotation in degrees (default -45).
    pub fn pdf_watermark_rotation(mut self, degrees: f32) -> Self {
        self.pdf_watermark_rotation = Some(degrees);
        self
    }

    /// Watermark text color as hex (default #888888).
    pub fn pdf_watermark_color(mut self, hex: &'a str) -> Self {
        self.pdf_watermark_color = Some(hex);
        self
    }

    /// Watermark font size in PDF points.
    pub fn pdf_watermark_font_size(mut self, size: f32) -> Self {
        self.pdf_watermark_font_size = Some(size);
        self
    }

    /// Watermark image scale (0.0-1.0, default 0.5).
    pub fn pdf_watermark_scale(mut self, scale: f32) -> Self {
        self.pdf_watermark_scale = Some(scale);
        self
    }

    /// Watermark layer position.
    pub fn pdf_watermark_layer(mut self, layer: WatermarkLayer) -> Self {
        self.pdf_watermark_layer = Some(layer);
        self
    }

    /// PDF standard compliance level.
    pub fn pdf_standard(mut self, standard: PdfStandard) -> Self {
        self.pdf_standard = Some(standard);
        self
    }

    /// Attach a file to the PDF. Data must be base64-encoded.
    pub fn pdf_attach(
        mut self,
        path: &str,
        base64_data: &str,
        mime_type: Option<&str>,
        description: Option<&str>,
        relationship: Option<EmbedRelationship>,
    ) -> Self {
        self.pdf_embedded_files.push((
            path.to_owned(),
            base64_data.to_owned(),
            mime_type.map(str::to_owned),
            description.map(str::to_owned),
            relationship,
        ));
        self
    }

    /// Send the render request and return the raw output bytes.
    pub async fn send(self) -> Result<Vec<u8>, ForgeError> {
        let has_quantize =
            self.colors.is_some() || self.palette.is_some() || self.dither.is_some();
        let has_watermark = self.pdf_watermark_text.is_some()
            || self.pdf_watermark_image.is_some()
            || self.pdf_watermark_opacity.is_some()
            || self.pdf_watermark_rotation.is_some()
            || self.pdf_watermark_color.is_some()
            || self.pdf_watermark_font_size.is_some()
            || self.pdf_watermark_scale.is_some()
            || self.pdf_watermark_layer.is_some();
        let has_pdf = self.pdf_title.is_some()
            || self.pdf_author.is_some()
            || self.pdf_subject.is_some()
            || self.pdf_keywords.is_some()
            || self.pdf_creator.is_some()
            || self.pdf_bookmarks.is_some()
            || has_watermark
            || self.pdf_standard.is_some()
            || !self.pdf_embedded_files.is_empty();

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
            pdf: if has_pdf {
                Some(PdfPayload {
                    title: self.pdf_title,
                    author: self.pdf_author,
                    subject: self.pdf_subject,
                    keywords: self.pdf_keywords,
                    creator: self.pdf_creator,
                    bookmarks: self.pdf_bookmarks,
                    watermark: if has_watermark {
                        Some(WatermarkPayload {
                            text: self.pdf_watermark_text,
                            image_data: self.pdf_watermark_image,
                            opacity: self.pdf_watermark_opacity,
                            rotation: self.pdf_watermark_rotation,
                            color: self.pdf_watermark_color,
                            font_size: self.pdf_watermark_font_size,
                            scale: self.pdf_watermark_scale,
                            layer: self.pdf_watermark_layer,
                        })
                    } else {
                        None
                    },
                    standard: self.pdf_standard,
                    embedded_files: if self.pdf_embedded_files.is_empty() {
                        None
                    } else {
                        Some(
                            self.pdf_embedded_files
                                .iter()
                                .map(|(path, data, mime, desc, rel)| EmbeddedFilePayload {
                                    path: path.clone(),
                                    data: data.clone(),
                                    mime_type: mime.clone(),
                                    description: desc.clone(),
                                    relationship: *rel,
                                })
                                .collect(),
                        )
                    },
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
        assert!(v.get("pdf").is_none());
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
    fn pdf_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<h1>Report</h1>")
            .pdf_title("Annual Report")
            .pdf_author("Jane Doe")
            .pdf_subject("Financials")
            .pdf_keywords("finance,annual,report")
            .pdf_creator("Forge SDK")
            .pdf_bookmarks(true);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let p = &v["pdf"];
        assert_eq!(p["title"], "Annual Report");
        assert_eq!(p["author"], "Jane Doe");
        assert_eq!(p["subject"], "Financials");
        assert_eq!(p["keywords"], "finance,annual,report");
        assert_eq!(p["creator"], "Forge SDK");
        assert_eq!(p["bookmarks"], true);
    }

    #[test]
    fn pdf_partial_fields() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<p>test</p>")
            .pdf_title("Title Only");
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let p = &v["pdf"];
        assert_eq!(p["title"], "Title Only");
        assert!(p.get("author").is_none());
        assert!(p.get("subject").is_none());
        assert!(p.get("keywords").is_none());
        assert!(p.get("creator").is_none());
        assert!(p.get("bookmarks").is_none());
    }

    #[test]
    fn no_pdf_when_unset() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client.render_html("<p>test</p>");
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert!(v.get("pdf").is_none());
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

    #[test]
    fn watermark_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<h1>Report</h1>")
            .pdf_watermark_text("DRAFT")
            .pdf_watermark_opacity(0.2)
            .pdf_watermark_rotation(-30.0)
            .pdf_watermark_color("#ff0000")
            .pdf_watermark_layer(WatermarkLayer::Over);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let wm = &v["pdf"]["watermark"];
        assert_eq!(wm["text"], "DRAFT");
        assert_eq!(wm["opacity"], 0.2);
        assert_eq!(wm["rotation"], -30.0);
        assert_eq!(wm["color"], "#ff0000");
        assert_eq!(wm["layer"], "over");
    }

    #[test]
    fn no_watermark_in_pdf_when_unset() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<p>test</p>")
            .pdf_title("Title Only");
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();

        let p = &v["pdf"];
        assert_eq!(p["title"], "Title Only");
        assert!(p.get("watermark").is_none());
    }

    #[test]
    fn pdf_standard_serializes() {
        assert_eq!(json_str(&PdfStandard::None), "\"none\"");
        assert_eq!(json_str(&PdfStandard::A2B), "\"pdf/a-2b\"");
        assert_eq!(json_str(&PdfStandard::A3B), "\"pdf/a-3b\"");
    }

    #[test]
    fn embed_relationship_serializes() {
        assert_eq!(json_str(&EmbedRelationship::Alternative), "\"alternative\"");
        assert_eq!(json_str(&EmbedRelationship::Unspecified), "\"unspecified\"");
    }

    #[test]
    fn pdf_standard_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<h1>Invoice</h1>")
            .pdf_standard(PdfStandard::A3B);
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();
        assert_eq!(v["pdf"]["standard"], "pdf/a-3b");
    }

    #[test]
    fn pdf_attach_payload() {
        let client = ForgeClient::new("http://localhost:3000");
        let builder = client
            .render_html("<h1>Invoice</h1>")
            .pdf_standard(PdfStandard::A3B)
            .pdf_attach("factur-x.xml", "PGludm9pY2U+PC9pbnZvaWNlPg==", Some("text/xml"), Some("Invoice data"), Some(EmbedRelationship::Alternative));
        let payload = build_payload(&builder);
        let v: serde_json::Value = serde_json::from_str(&payload).unwrap();
        let ef = &v["pdf"]["embedded_files"][0];
        assert_eq!(ef["path"], "factur-x.xml");
        assert_eq!(ef["mime_type"], "text/xml");
        assert_eq!(ef["relationship"], "alternative");
    }

    // -- helpers --

    fn json_str<T: serde::Serialize>(val: &T) -> String {
        serde_json::to_string(val).unwrap()
    }

    fn build_payload(builder: &RenderRequestBuilder<'_>) -> String {
        let has_quantize =
            builder.colors.is_some() || builder.palette.is_some() || builder.dither.is_some();
        let has_watermark = builder.pdf_watermark_text.is_some()
            || builder.pdf_watermark_image.is_some()
            || builder.pdf_watermark_opacity.is_some()
            || builder.pdf_watermark_rotation.is_some()
            || builder.pdf_watermark_color.is_some()
            || builder.pdf_watermark_font_size.is_some()
            || builder.pdf_watermark_scale.is_some()
            || builder.pdf_watermark_layer.is_some();
        let has_pdf = builder.pdf_title.is_some()
            || builder.pdf_author.is_some()
            || builder.pdf_subject.is_some()
            || builder.pdf_keywords.is_some()
            || builder.pdf_creator.is_some()
            || builder.pdf_bookmarks.is_some()
            || has_watermark
            || builder.pdf_standard.is_some()
            || !builder.pdf_embedded_files.is_empty();

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
            pdf: if has_pdf {
                Some(PdfPayload {
                    title: builder.pdf_title,
                    author: builder.pdf_author,
                    subject: builder.pdf_subject,
                    keywords: builder.pdf_keywords,
                    creator: builder.pdf_creator,
                    bookmarks: builder.pdf_bookmarks,
                    watermark: if has_watermark {
                        Some(WatermarkPayload {
                            text: builder.pdf_watermark_text,
                            image_data: builder.pdf_watermark_image,
                            opacity: builder.pdf_watermark_opacity,
                            rotation: builder.pdf_watermark_rotation,
                            color: builder.pdf_watermark_color,
                            font_size: builder.pdf_watermark_font_size,
                            scale: builder.pdf_watermark_scale,
                            layer: builder.pdf_watermark_layer,
                        })
                    } else {
                        None
                    },
                    standard: builder.pdf_standard,
                    embedded_files: if builder.pdf_embedded_files.is_empty() {
                        None
                    } else {
                        Some(
                            builder.pdf_embedded_files
                                .iter()
                                .map(|(path, data, mime, desc, rel)| EmbeddedFilePayload {
                                    path: path.clone(),
                                    data: data.clone(),
                                    mime_type: mime.clone(),
                                    description: desc.clone(),
                                    relationship: *rel,
                                })
                                .collect(),
                        )
                    },
                })
            } else {
                None
            },
        };
        serde_json::to_string(&payload).unwrap()
    }
}
