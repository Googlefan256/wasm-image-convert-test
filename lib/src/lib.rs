use std::io::{BufWriter, Cursor};

use image::guess_format;
use image::ImageFormat as RawImageFormat;
use image::ImageOutputFormat;
use image::load_from_memory;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png = 0,
    Jpeg = 1,
    Gif = 2,
    Bmp = 4,
    Ico = 5,
    Tiff = 6,
    Tga = 7,
    Dds = 8,
    Hdr = 9,
    Farbfeld = 10,
    Pnm = 11,
    OpenExr = 12,
    Qoi = 13,
    Unknown = 14,
}

impl ToString for ImageFormat {
    fn to_string(&self) -> String {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Ico => "ico",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Tga => "tga",
            ImageFormat::Dds => "dds",
            ImageFormat::Hdr => "hdr",
            ImageFormat::Farbfeld => "farbfeld",
            ImageFormat::Pnm => "pnm",
            ImageFormat::OpenExr => "exr",
            ImageFormat::Qoi => "qoi",
            ImageFormat::Unknown => "unknown",
        }.to_string()
    }
}

impl From<String> for ImageFormat {
    fn from(fmt: String) -> Self {
        match fmt.as_str() {
            "png" => ImageFormat::Png,
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "bmp" => ImageFormat::Bmp,
            "ico" => ImageFormat::Ico,
            "tiff" => ImageFormat::Tiff,
            "tga" => ImageFormat::Tga,
            "dds" => ImageFormat::Dds,
            "hdr" => ImageFormat::Hdr,
            "farbfeld" => ImageFormat::Farbfeld,
            "pnm" => ImageFormat::Pnm,
            "exr" => ImageFormat::OpenExr,
            "qoi" => ImageFormat::Qoi,
            _ => ImageFormat::Unknown,
        }
    }
}

impl From<RawImageFormat> for ImageFormat {
    fn from(fmt: RawImageFormat) -> Self {
        match fmt {
            RawImageFormat::Png => ImageFormat::Png,
            RawImageFormat::Jpeg => ImageFormat::Jpeg,
            RawImageFormat::Gif => ImageFormat::Gif,
            RawImageFormat::Bmp => ImageFormat::Bmp,
            RawImageFormat::Ico => ImageFormat::Ico,
            RawImageFormat::Tiff => ImageFormat::Tiff,
            RawImageFormat::Tga => ImageFormat::Tga,
            RawImageFormat::Dds => ImageFormat::Dds,
            RawImageFormat::Hdr => ImageFormat::Hdr,
            RawImageFormat::Farbfeld => ImageFormat::Farbfeld,
            RawImageFormat::Pnm => ImageFormat::Pnm,
            RawImageFormat::OpenExr => ImageFormat::OpenExr,
            RawImageFormat::Qoi => ImageFormat::Qoi,
            _ => ImageFormat::Unknown,
        }
    }

}

impl From<ImageFormat> for RawImageFormat {
    fn from(fmt: ImageFormat) -> Self {
        match fmt {
            ImageFormat::Png => RawImageFormat::Png,
            ImageFormat::Jpeg => RawImageFormat::Jpeg,
            ImageFormat::Gif => RawImageFormat::Gif,
            ImageFormat::Bmp => RawImageFormat::Bmp,
            ImageFormat::Ico => RawImageFormat::Ico,
            ImageFormat::Tiff => RawImageFormat::Tiff,
            ImageFormat::Tga => RawImageFormat::Tga,
            ImageFormat::Dds => RawImageFormat::Dds,
            ImageFormat::Hdr => RawImageFormat::Hdr,
            ImageFormat::Farbfeld => RawImageFormat::Farbfeld,
            ImageFormat::Pnm => RawImageFormat::Pnm,
            ImageFormat::OpenExr => RawImageFormat::OpenExr,
            ImageFormat::Qoi => RawImageFormat::Qoi,
            ImageFormat::Unknown => unreachable!("Unknown image format"),
        }
    }
}

impl From<ImageOutputFormat> for ImageFormat {
    fn from(fmt: ImageOutputFormat) -> Self {
        match fmt {
            ImageOutputFormat::Png => ImageFormat::Png,
            ImageOutputFormat::Jpeg(_) => ImageFormat::Jpeg,
            ImageOutputFormat::Gif => ImageFormat::Gif,
            ImageOutputFormat::Bmp => ImageFormat::Bmp,
            ImageOutputFormat::Ico => ImageFormat::Ico,
            ImageOutputFormat::Tiff => ImageFormat::Tiff,
            ImageOutputFormat::Tga => ImageFormat::Tga,
            ImageOutputFormat::Farbfeld => ImageFormat::Farbfeld,
            ImageOutputFormat::Pnm(_) => ImageFormat::Pnm,
            ImageOutputFormat::OpenExr => ImageFormat::OpenExr,
            ImageOutputFormat::Qoi => ImageFormat::Qoi,
            ImageOutputFormat::Unsupported(_) => ImageFormat::Unknown,
            _ => ImageFormat::Unknown,
        }
    }
}

pub fn convert(buf: Vec<u8>, fmt: ImageFormat) -> Option<Vec<u8>> {
    if fmt == ImageFormat::Unknown {
        return None;
    }
    let img = load_from_memory(&buf).ok()?;
    let mut buf = BufWriter::new(Cursor::new(Vec::new()));
    img.write_to(&mut buf, Into::<RawImageFormat>::into(fmt)).ok()?;
    Some(buf.into_inner().ok()?.into_inner())
}

pub fn guess(buf: Vec<u8>) -> Option<ImageFormat> {
    Some(guess_format(&buf).ok()?.into())
}