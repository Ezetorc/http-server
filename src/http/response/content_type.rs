#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    // text/*
    Plain,
    Html,
    Css,
    Csv,

    // application/*
    Json,
    Xml,
    FormUrlEncoded,
    Javascript,
    Pdf,
    Zip,
    OctetStream,

    // image/*
    Png,
    Jpeg,
    Gif,
    Webp,
    Svg,

    // audio/*
    Mp3,
    Wav,
    OggAudio,

    // video/*
    Mp4,
    Webm,
    OggVideo,

    // multipart/*
    MultipartFormData,
    MultipartMixed,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            // text/*
            Self::Plain => "text/plain",
            Self::Html => "text/html",
            Self::Css => "text/css",
            Self::Csv => "text/csv",

            // application/*
            Self::Json => "application/json",
            Self::Xml => "application/xml",
            Self::FormUrlEncoded => "application/x-www-form-urlencoded",
            Self::Javascript => "application/javascript",
            Self::Pdf => "application/pdf",
            Self::Zip => "application/zip",
            Self::OctetStream => "application/octet-stream",

            // image/*
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
            Self::Svg => "image/svg+xml",

            // audio/*
            Self::Mp3 => "audio/mpeg",
            Self::Wav => "audio/wav",
            Self::OggAudio => "audio/ogg",

            // video/*
            Self::Mp4 => "video/mp4",
            Self::Webm => "video/webm",
            Self::OggVideo => "video/ogg",

            // multipart/*
            Self::MultipartFormData => "multipart/form-data",
            Self::MultipartMixed => "multipart/mixed",
        }
    }
}
