use crate::app::App;

use axum::{
	body::Body,
	extract::State,
	http::{Request, Response, StatusCode, Uri},
	response::{IntoResponse, Response as AxumResponse},
};
use leptos::*;
use std::env;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn file_and_error_handler(
	uri: Uri,
	State(options): State<LeptosOptions>,
	req: Request<Body>,
) -> AxumResponse {
	let root = options.site_root.clone();
	let uploads_dir = &format!("{}public/upload_media", env!("UPLOAD_ROOT"));

	match get_static_file(uri.clone(), &root, &uploads_dir).await {
		Ok(res) => res.into_response(),
		Err((status, msg)) => {
			eprintln!("Unable to serve static file: {} - {}", uri.path(), msg);
			if status == StatusCode::NOT_FOUND {
				// Render your application or a custom 404 page
				let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
				handler(req).await.into_response()
			} else {
				// For other errors, respond with the status code and message
				(status, msg).into_response()
			}
		},
	}
}

async fn get_static_file(uri: Uri, root: &str, uploads_dir: &str) -> Result<Response<Body>, (StatusCode, String)> {
	let site_path = PathBuf::from(root).join(uri.path().trim_start_matches('/'));

	if fs::metadata(&site_path).await.map(|md| md.is_file()).unwrap_or(false) {
		return serve_file(&site_path).await;
	}

	let upload_path = PathBuf::from(uploads_dir).join(uri.path().trim_start_matches('/'));

	if fs::metadata(&upload_path).await.map(|md| md.is_file()).unwrap_or(false) {
		return serve_file(&upload_path).await;
	}

	Err((StatusCode::NOT_FOUND, "File not found".to_string()))
}

async fn serve_file(path: &Path) -> Result<Response<Body>, (StatusCode, String)> {
	match fs::read(&path).await {
		Ok(contents) => {
			let mime_type = get_mime_type(&path);
			let response = Response::builder()
				.status(StatusCode::OK)
				.header("Content-Type", mime_type)
				.body(Body::from(contents))
				.unwrap();

			Ok(response)
		},
		Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error reading file: {}", err))),
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
