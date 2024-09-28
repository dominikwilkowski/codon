use crate::app::App;

use axum::{
	body::Body,
	extract::State,
	http::{Request, Response, StatusCode, Uri},
	response::{IntoResponse, Response as AxumResponse},
};
use leptos::*;
use std::path::Path;
use tokio::fs;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
	uri: Uri,
	State(options): State<LeptosOptions>,
	req: Request<Body>,
) -> AxumResponse {
	let root = options.site_root.clone();
	let res = get_static_file(uri.clone(), &root).await.unwrap();

	if res.status() == StatusCode::OK {
		res.into_response()
	} else {
		let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
		handler(req).await.into_response()
	}
}

async fn get_static_file(
	uri: Uri,
	root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
	let path = format!("{}{}", root, uri.path());
	let path = Path::new(&path);

	// Check if the file exists and is a file
	match fs::metadata(&path).await {
		Ok(metadata) if metadata.is_file() => match fs::read(&path).await {
			Ok(contents) => {
				let mime_type = match path.extension().and_then(|ext| ext.to_str()) {
					Some(ext) if ext.eq_ignore_ascii_case("mov") => "video/quicktime",
					Some(ext) if ext.eq_ignore_ascii_case("mp4") => "video/mp4",
					Some(ext) if ext.eq_ignore_ascii_case("webm") => "video/webm",
					Some(ext) if ext.eq_ignore_ascii_case("png") => "image/png",
					Some(ext)
						if ext.eq_ignore_ascii_case("jpg")
							| ext.eq_ignore_ascii_case("jpeg") =>
					{
						"image/jpeg"
					},
					Some(ext) if ext.eq_ignore_ascii_case("gif") => "image/gif",
					Some(ext) if ext.eq_ignore_ascii_case("html") => {
						"text/html; charset=utf-8"
					},
					Some(ext) if ext.eq_ignore_ascii_case("css") => {
						"text/css; charset=utf-8"
					},
					Some(ext) if ext.eq_ignore_ascii_case("js") => {
						"application/javascript"
					},
					Some(ext) if ext.eq_ignore_ascii_case("json") => "application/json",
					Some(ext) if ext.eq_ignore_ascii_case("svg") => "image/svg+xml",
					Some(ext) if ext.eq_ignore_ascii_case("woff") => "font/woff",
					Some(ext) if ext.eq_ignore_ascii_case("woff2") => "font/woff2",
					_ => "application/octet-stream",
				};

				let response = Response::builder()
					.status(StatusCode::OK)
					.header("Content-Type", mime_type)
					.body(Body::from(contents))
					.unwrap();

				Ok(response)
			},
			Err(err) => Err((
				StatusCode::INTERNAL_SERVER_ERROR,
				format!("Error reading file: {}", err),
			)),
		},
		_ => {
			// File not found or it's not a file
			let req =
				Request::builder().uri(uri.clone()).body(Body::empty()).unwrap();

			match ServeDir::new(root).oneshot(req).await {
				Ok(res) => Ok(res.into_response()),
				Err(err) => Err((
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Something went wrong: {}", err),
				)),
			}
		},
	}
}
