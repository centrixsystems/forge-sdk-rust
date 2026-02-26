use serde::Serialize;

/// Output format for rendered content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Pdf,
    Png,
    Jpeg,
    Bmp,
    Tga,
    Qoi,
    Svg,
}

impl Serialize for OutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Pdf => "pdf",
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Bmp => "bmp",
            Self::Tga => "tga",
            Self::Qoi => "qoi",
            Self::Svg => "svg",
        })
    }
}

/// Page orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Serialize for Orientation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Portrait => "portrait",
            Self::Landscape => "landscape",
        })
    }
}

/// Document flow mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Auto,
    Paginate,
    Continuous,
}

impl Serialize for Flow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Auto => "auto",
            Self::Paginate => "paginate",
            Self::Continuous => "continuous",
        })
    }
}

/// Dithering algorithm for color quantization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DitherMethod {
    None,
    FloydSteinberg,
    Atkinson,
    Ordered,
}

impl Serialize for DitherMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::None => "none",
            Self::FloydSteinberg => "floyd-steinberg",
            Self::Atkinson => "atkinson",
            Self::Ordered => "ordered",
        })
    }
}

/// Color palette preset for quantization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Palette {
    Auto,
    BlackWhite,
    Grayscale,
    Eink,
    /// Custom palette of hex color strings (e.g. `"#ff0000"`).
    Custom(Vec<String>),
}

impl Serialize for Palette {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::BlackWhite => serializer.serialize_str("bw"),
            Self::Grayscale => serializer.serialize_str("grayscale"),
            Self::Eink => serializer.serialize_str("eink"),
            Self::Custom(colors) => colors.serialize(serializer),
        }
    }
}

/// Watermark layer position relative to page content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatermarkLayer {
    Over,
    Under,
}

impl Serialize for WatermarkLayer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Over => "over",
            Self::Under => "under",
        })
    }
}

/// PDF standard compliance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfStandard {
    /// Standard PDF (no archival compliance).
    None,
    /// PDF/A-2b — modern archival.
    A2B,
    /// PDF/A-3b — archival with embedded files (ZUGFeRD/Factur-X).
    A3B,
}

impl Serialize for PdfStandard {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::None => "none",
            Self::A2B => "pdf/a-2b",
            Self::A3B => "pdf/a-3b",
        })
    }
}

/// Relationship of an embedded file to the PDF document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedRelationship {
    /// Alternative representation (ZUGFeRD: XML invoice).
    Alternative,
    /// Supplements the document.
    Supplement,
    /// Data used to derive visual presentation.
    Data,
    /// Source file the PDF was created from.
    Source,
    /// No specific relationship.
    Unspecified,
}

impl Serialize for EmbedRelationship {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(match self {
            Self::Alternative => "alternative",
            Self::Supplement => "supplement",
            Self::Data => "data",
            Self::Source => "source",
            Self::Unspecified => "unspecified",
        })
    }
}

/// Watermark settings within a PDF render request.
#[derive(Debug, Serialize)]
pub(crate) struct WatermarkPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<WatermarkLayer>,
}

/// Embedded file settings within a PDF render request.
#[derive(Debug, Serialize)]
pub(crate) struct EmbeddedFilePayload {
    pub path: String,
    pub data: String, // base64-encoded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<EmbedRelationship>,
}

/// JSON payload for the /render endpoint.
#[derive(Debug, Serialize)]
pub(crate) struct RenderPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<&'a str>,
    pub format: OutputFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margins: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantize: Option<QuantizePayload<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<PdfPayload<'a>>,
}

/// Quantization settings within a render request.
#[derive(Debug, Serialize)]
pub(crate) struct QuantizePayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<&'a Palette>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dither: Option<DitherMethod>,
}

/// PDF metadata settings within a render request.
#[derive(Debug, Serialize)]
pub(crate) struct PdfPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<WatermarkPayload<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard: Option<PdfStandard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_files: Option<Vec<EmbeddedFilePayload>>,
}

/// Server error response body.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ErrorResponse {
    pub error: String,
}
