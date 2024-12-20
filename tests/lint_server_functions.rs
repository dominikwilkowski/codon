use std::fs;
use std::path::{Path, PathBuf};
use syn::{
	Attribute,
	Expr::{Lit, Path as ExprPath},
	ExprCall, ExprLit,
	Item::Fn,
	Lit::Str,
	Meta::{List, NameValue, Path as MetaPath},
	visit::Visit,
};

struct GetUserCallVisitor {
	found: bool,
}

impl<'ast> Visit<'ast> for GetUserCallVisitor {
	fn visit_expr_call(&mut self, node: &'ast ExprCall) {
		match *node.func {
			ExprPath(ref expr_path) => match expr_path.path.segments.last() {
				Some(segment) => {
					if segment.ident == "get_user" {
						self.found = true;
					}
				},
				_ => {},
			},
			_ => {},
		}
	}
}

fn is_server_function(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| match &attr.meta {
		MetaPath(path) => path.is_ident("server"),
		List(meta_list) => meta_list.path.is_ident("server"),
		NameValue(meta_name_value) => meta_name_value.path.is_ident("server"),
	})
}

fn has_doc_comment(attrs: &[Attribute], comment: &str) -> bool {
	attrs
		.iter()
		.filter_map(|attr| match &attr.meta {
			NameValue(meta) => {
				if meta.path.is_ident("doc") {
					match &meta.value {
						Lit(ExprLit { lit: Str(lit_str), .. }) => Some(lit_str.value()),
						_ => None,
					}
				} else {
					None
				}
			},
			_ => None,
		})
		.any(|doc_string| doc_string.contains(comment))
}

#[test]
fn all_server_functions_call_get_user() {
	let src_dir = Path::new("src");
	let mut server_functions_missing_get_user = Vec::new();

	// Recursively collect all Rust source files in src/
	fn collect_rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
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
			match item {
				Fn(ref func) => {
					if is_server_function(&func.attrs) {
						let mut visitor = GetUserCallVisitor { found: false };
						visitor.visit_item_fn(func);

						let opt_out = has_doc_comment(&func.attrs, "![allow_no_get_user]");

						if !visitor.found && !opt_out {
							server_functions_missing_get_user.push(format!("{} in file {:?}", func.sig.ident, file_path));
						}
					}
				},
				_ => {},
			}
		}
	}

	if !server_functions_missing_get_user.is_empty() {
		panic!(
			"The following server functions ({}) do not call `get_user()` and have not opted out explicitly:\n{}",
			server_functions_missing_get_user.len(),
			server_functions_missing_get_user.join("\n")
		);
	}
}
