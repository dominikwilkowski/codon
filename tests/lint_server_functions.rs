use std::fs;
use std::path::Path;
use syn::{visit::Visit, Attribute, ExprCall};

struct GetUserCallVisitor {
	found: bool,
}

impl<'ast> Visit<'ast> for GetUserCallVisitor {
	fn visit_expr_call(&mut self, node: &'ast ExprCall) {
		if let syn::Expr::Path(ref expr_path) = *node.func {
			if let Some(segment) = expr_path.path.segments.last() {
				if segment.ident == "get_user" {
					self.found = true;
				}
			}
		}
		syn::visit::visit_expr_call(self, node);
	}
}

fn is_server_function(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| attr.path().is_ident("server") || attr.path().is_ident("server_linter::server"))
}

#[test]
fn all_server_functions_call_get_user() {
	let src_dir = Path::new("src");
	let mut server_functions_missing_get_user = Vec::new();

	// Recursively collect all Rust source files in src/
	fn collect_rust_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
		if dir.is_dir() {
			for entry in fs::read_dir(dir).unwrap() {
				let entry = entry.unwrap();
				let path = entry.path();
				if path.is_dir() {
					collect_rust_files(&path, files);
				} else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
					files.push(path);
				}
			}
		}
	}

	let mut files = Vec::new();
	collect_rust_files(src_dir, &mut files);

	for file_path in files {
		let src = fs::read_to_string(&file_path).expect("Failed to read file");
		let syntax = syn::parse_file(&src).expect("Failed to parse file");

		for item in syntax.items {
			if let syn::Item::Fn(ref func) = item {
				if is_server_function(&func.attrs) {
					let mut visitor = GetUserCallVisitor { found: false };
					visitor.visit_item_fn(func);

					if !visitor.found {
						server_functions_missing_get_user.push(format!("{} in file {:?}", func.sig.ident, file_path));
					}
				}
			}
		}
	}

	if !server_functions_missing_get_user.is_empty() {
		panic!(
			"The following server functions do not call `get_user()`:\n{}",
			server_functions_missing_get_user.join("\n")
		);
	}
}
