use crate::model::asset::{ImageAsset, ImageFormat};
use crate::model::SVGElement;

pub fn extract_base64_images(element: &SVGElement) -> Vec<ImageAsset> {
	let mut out = Vec::new();
	collect_images(element, &mut out);
	out
}

pub fn mark_for_webp_conversion(image: &mut ImageAsset) {
	if !matches!(image.format, ImageFormat::Webp) {
		image.format = ImageFormat::Webp;
	}
}

fn collect_images(element: &SVGElement, out: &mut Vec<ImageAsset>) {
	for key in ["href", "xlink:href", "src"] {
		if let Some(value) = element.attributes.get(key) {
			if let Some((mime, data)) = parse_data_url(value) {
				let format = detect_format(mime, &data);
				out.push(ImageAsset {
					id: element
						.id
						.clone()
						.unwrap_or_else(|| format!("img-{}", out.len())),
					format,
					data,
					width: 0,
					height: 0,
				});
			}
		}
	}

	for child in &element.children {
		collect_images(child, out);
	}
}

fn parse_data_url(raw: &str) -> Option<(&str, Vec<u8>)> {
	let pref = "data:";
	let b64_marker = ";base64,";
	if !raw.starts_with(pref) {
		return None;
	}

	let meta_end = raw.find(',')?;
	let meta = &raw[pref.len()..meta_end];
	if !meta.contains("base64") {
		return None;
	}

	let mime = meta.trim_end_matches(";base64");
	let payload = &raw[(meta_end + 1)..];
	if !meta.contains(b64_marker.trim_matches(',')) && !meta.ends_with(";base64") {
		return None;
	}

	decode_base64(payload).map(|bytes| (mime, bytes))
}

fn detect_format(mime: &str, data: &[u8]) -> ImageFormat {
	if data.starts_with(&[0x89, b'P', b'N', b'G']) || mime.contains("png") {
		ImageFormat::Png
	} else if data.starts_with(&[0xFF, 0xD8, 0xFF]) || mime.contains("jpeg") || mime.contains("jpg") {
		ImageFormat::Jpeg
	} else if data.starts_with(b"RIFF") && data.get(8..12) == Some(b"WEBP") || mime.contains("webp") {
		ImageFormat::Webp
	} else if data.starts_with(b"GIF8") || mime.contains("gif") {
		ImageFormat::Gif
	} else if mime.contains("svg") {
		ImageFormat::Svg
	} else {
		ImageFormat::Unknown
	}
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
	let mut out = Vec::with_capacity(input.len() * 3 / 4);
	let mut buf = [0u8; 4];
	let mut buf_len = 0usize;

	for ch in input.bytes() {
		if ch == b'=' {
			break;
		}
		if ch == b'\n' || ch == b'\r' || ch == b'\t' || ch == b' ' {
			continue;
		}

		let val = base64_value(ch)?;
		buf[buf_len] = val;
		buf_len += 1;

		if buf_len == 4 {
			out.push((buf[0] << 2) | (buf[1] >> 4));
			out.push((buf[1] << 4) | (buf[2] >> 2));
			out.push((buf[2] << 6) | buf[3]);
			buf_len = 0;
		}
	}

	if buf_len == 2 {
		out.push((buf[0] << 2) | (buf[1] >> 4));
	} else if buf_len == 3 {
		out.push((buf[0] << 2) | (buf[1] >> 4));
		out.push((buf[1] << 4) | (buf[2] >> 2));
	}

	Some(out)
}

fn base64_value(ch: u8) -> Option<u8> {
	match ch {
		b'A'..=b'Z' => Some(ch - b'A'),
		b'a'..=b'z' => Some(ch - b'a' + 26),
		b'0'..=b'9' => Some(ch - b'0' + 52),
		b'+' => Some(62),
		b'/' => Some(63),
		_ => None,
	}
}
