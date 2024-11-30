use crate::app::App;

use axum::{
	body::Body,
	extract::State,
	http::{Request, Response, StatusCode, Uri, header::ACCEPT_ENCODING},
	response::{IntoResponse, Response as AxumResponse},
};
use brotli::CompressorReader;
use leptos::*;
use percent_encoding::percent_decode_str;
use std::{
	env,
	io::{Cursor, Read},
	path::{Path, PathBuf},
};
use tokio::{
	fs::{self, File},
	io::AsyncReadExt,
};

pub async fn file_and_error_handler(
	uri: Uri,
	State(options): State<LeptosOptions>,
	req: Request<Body>,
) -> AxumResponse {
	let root = options.site_root.clone();
	let uploads_dir = &format!("{}public", env!("UPLOAD_ROOT"));

	let accept_encoding = req.headers().get(ACCEPT_ENCODING).and_then(|value| value.to_str().ok()).unwrap_or("");

	match get_static_file(uri.clone(), &root, &uploads_dir, accept_encoding).await {
		Ok(res) => res.into_response(),
		Err((status, msg)) => {
			eprintln!("Unable to serve static file: {} - {}", uri.path(), msg);
			if status == StatusCode::NOT_FOUND {
				// Render your application or a custom 404 page
				let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
				handler(req).await.into_response()
			} else {
				// For other errors, respond with a generic message
				(status, "Internal server error").into_response()
			}
		},
	}
}

fn sanitize_path(uri_path: &str) -> Option<PathBuf> {
	let decoded = percent_decode_str(uri_path).decode_utf8().ok()?;
	let mut path = PathBuf::new();
	for component in Path::new(&*decoded).components() {
		match component {
			std::path::Component::Normal(seg) => path.push(seg),
			std::path::Component::RootDir => {},
			_ => return None,
		}
	}
	Some(path)
}

async fn get_static_file(
	uri: Uri,
	root: &str,
	uploads_dir: &str,
	accept_encoding: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
	if let Some(sanitized_path) = sanitize_path(uri.path()) {
		let site_path = PathBuf::from(root).join(&sanitized_path);
		if fs::metadata(&site_path).await.map(|md| md.is_file()).unwrap_or(false) {
			return serve_file(&site_path, accept_encoding).await;
		}

		let upload_path = PathBuf::from(uploads_dir).join(&sanitized_path);
		if fs::metadata(&upload_path).await.map(|md| md.is_file()).unwrap_or(false) {
			return serve_file(&upload_path, accept_encoding).await;
		}
	} else {
		return Err((StatusCode::BAD_REQUEST, "Invalid path".to_string()));
	}

	Err((StatusCode::NOT_FOUND, "File not found".to_string()))
}

async fn serve_file(path: &Path, accept_encoding: &str) -> Result<Response<Body>, (StatusCode, String)> {
	match File::open(&path).await {
		Ok(mut file) => {
			let mut contents = Vec::new();
			file.read_to_end(&mut contents).await.map_err(|err| {
				eprintln!("Error reading file {}: {}", path.display(), err);
				(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
			})?;

			let mime_type = get_mime_type(&path);
			let (body, encoding) = if accept_encoding.contains("br") {
				// Compress the contents using Brotli
				let mut compressed_data = Vec::new();
				{
					let mut compressor = CompressorReader::new(
						Cursor::new(contents),
						4096, // Buffer size
						5,    // Quality level (0-11)
						22,   // lgwin (window size)
					);
					compressor.read_to_end(&mut compressed_data).map_err(|err| {
						eprintln!("Error compressing file {}: {}", path.display(), err);
						(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
					})?;
				}
				(Body::from(compressed_data), Some("br"))
			} else {
				// Send the uncompressed contents
				(Body::from(contents), None)
			};

			let mut response = Response::builder()
				.status(StatusCode::OK)
				.header("Content-Type", mime_type)
				.header("X-Content-Type-Options", "nosniff")
				.header("X-Frame-Options", "SAMEORIGIN")
				.header("Cache-Control", "public, max-age=31536000, immutable");

			if let Some(encoding) = encoding {
				response = response.header("Content-Encoding", encoding);
			}

			let response = response.body(body).unwrap();

			Ok(response)
		},
		Err(err) => {
			eprintln!("Error opening file {}: {}", path.display(), err);
			Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))
		},
	}
}

fn get_mime_type(path: &Path) -> &'static str {
	match path.extension().and_then(|ext| ext.to_str()) {
		Some(ext) if ext.eq_ignore_ascii_case("mov") => "video/quicktime",
		Some(ext) if ext.eq_ignore_ascii_case("mp4") => "video/mp4",
		Some(ext) if ext.eq_ignore_ascii_case("webm") => "video/webm",
		Some(ext) if ext.eq_ignore_ascii_case("png") => "image/png",
		Some(ext) if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") => "image/jpeg",
		Some(ext) if ext.eq_ignore_ascii_case("gif") => "image/gif",
		Some(ext) if ext.eq_ignore_ascii_case("html") => "text/html; charset=utf-8",
		Some(ext) if ext.eq_ignore_ascii_case("css") => "text/css; charset=utf-8",
		Some(ext) if ext.eq_ignore_ascii_case("js") => "application/javascript",
		Some(ext) if ext.eq_ignore_ascii_case("json") => "application/json",
		Some(ext) if ext.eq_ignore_ascii_case("svg") => "image/svg+xml",
		Some(ext) if ext.eq_ignore_ascii_case("woff") => "font/woff",
		Some(ext) if ext.eq_ignore_ascii_case("woff2") => "font/woff2",
		_ => "application/octet-stream",
	}
}
