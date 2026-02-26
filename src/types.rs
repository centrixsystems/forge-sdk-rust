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
    pub timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantize: Option<QuantizePayload<'a>>,
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

/// Server error response body.
#[derive(Debug, serde::Deserialize)]
pub(crate) struct ErrorResponse {
    pub error: String,
}
