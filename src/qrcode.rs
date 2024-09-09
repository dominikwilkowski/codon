extern crate qrcodegen;
use qrcodegen::{DataTooLong, QrCode, QrCodeEcc};

pub fn generate_qr(text: &str) -> Result<String, DataTooLong> {
	let qr = QrCode::encode_text(text, QrCodeEcc::Medium)?;
	Ok(to_optimized_svg_string(qr.size() as u16, 4, QrCode::get_module, &qr))
}

fn to_optimized_svg_string<T, U: Into<i32> + From<u16>>(
	size: u16,
	border: u8,
	get_module_fn: fn(&T, U, U) -> bool,
	module: &T,
) -> String {
	let border = border as u16;
	let dimension: u32 = size as u32 + (border as u32 * 2);

	let mut result = String::new();
	result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {dimension} {dimension}\">",
	);
	result += "<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>";
	result += "<path d=\"";

	let mut new_row;
	let mut last_cursor_position = 0;
	for y in 0..size {
		new_row = true;
		for x in 0..size {
			if get_module_fn(module, x.into(), y.into()) {
				// "H[x]" is shorter when x < 9
				// "h-1" is at least the same length or shorter when x > 9
				// Example:
				// H1 H2 .. H9 H10 H11 .. H99 H100 H101
				// vs
				// h-1 h-1 .. h-1 h-1 h-1 .. h-1 h-1 h-1
				let horizontal = if x + border > 9 {
					"h-1"
				} else {
					&format!("H{x}", x = x + border)
				};

				// On new rows we move the cursor to the next black box with "M[x] [y]"
				if new_row {
					new_row = false;
					result += &format!(
						"M{x} {y}h1v1{horizontal}z",
						x = x + border,
						y = y + border
					);
				// On a box that's within the same row we move the cursor
				// by [x] - last_cursor_position with "m[delta] 0" because that's shorter
				// Example:
				// M1 1, M2 2 .. M9 9, M10 10, M11 11 .. M99 99, M100 100, M101 101
				// vs
				// m1 0, m2 0 .. m9 0, m10 0, m11 0 .. m99 0, m100 0, m101 0
				} else {
					result += &format!(
						"m{x} 0h1v1{horizontal}z",
						x = x + border - last_cursor_position
					);
				}
				last_cursor_position = x + border;
			}
		}
	}
	result += "\"/></svg>\n";
	result
}

#[cfg(test)]
mod test {
	use super::*;

	struct FakeQr {
		blocks: Vec<&'static str>,
	}

	impl FakeQr {
		pub fn get_module(&self, y: u16, x: u16) -> bool {
			self.blocks.contains(&format!("{x},{y}").as_str())
		}
	}

	#[test]
	fn small_svg() {
		let qr = FakeQr {
			blocks: vec!["0,0", "0,3", "1,1", "2,2", "2,3", "3,3"],
		};
		assert_eq!(
			to_optimized_svg_string(4, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 4 4\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M0 0h1v1H0zm3 0h1v1H3z\
					M1 1h1v1H1z\
					M2 2h1v1H2zm1 0h1v1H3z\
					M3 3h1v1H3z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_empty_line() {
		let qr = FakeQr {
			blocks: vec!["0,0", "0,3", "2,2", "3,3"],
		};
		assert_eq!(
			to_optimized_svg_string(4, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 4 4\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M0 0h1v1H0zm3 0h1v1H3z\
					M2 2h1v1H2z\
					M3 3h1v1H3z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_line_left() {
		let qr = FakeQr {
			blocks: vec!["0,0", "1,0", "2,0", "3,0"],
		};
		assert_eq!(
			to_optimized_svg_string(4, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 4 4\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M0 0h1v1H0z\
					M0 1h1v1H0z\
					M0 2h1v1H0z\
					M0 3h1v1H0z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_line_top() {
		let qr = FakeQr {
			blocks: vec!["0,0", "0,1", "0,2", "0,3"],
		};
		assert_eq!(
			to_optimized_svg_string(4, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 4 4\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M0 0h1v1H0zm1 0h1v1H1zm1 0h1v1H2zm1 0h1v1H3z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_line_center() {
		let qr = FakeQr {
			blocks: vec!["2,0", "2,1", "2,2", "2,3", "2,4"],
		};
		assert_eq!(
			to_optimized_svg_string(5, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 5 5\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M0 2h1v1H0zm1 0h1v1H1zm1 0h1v1H2zm1 0h1v1H3zm1 0h1v1H4z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_dot_center() {
		let qr = FakeQr {
			blocks: vec!["2,2"],
		};
		assert_eq!(
			to_optimized_svg_string(5, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 5 5\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M2 2h1v1H2z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_border() {
		let qr = FakeQr {
			blocks: vec!["2,2"],
		};
		assert_eq!(
			to_optimized_svg_string(5, 5, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 15 15\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M7 7h1v1H7z\
				\"/>\
				</svg>\n"
			)
		);
	}

	#[test]
	fn svg_with_large_numbers() {
		let qr = FakeQr {
			blocks: vec!["12,12"],
		};
		assert_eq!(
			to_optimized_svg_string(24, 0, FakeQr::get_module, &qr),
			String::from(
				"<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\">\
				<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\
				<path d=\"\
					M12 12h1v1h-1z\
				\"/>\
				</svg>\n"
			)
		);
	}
}
