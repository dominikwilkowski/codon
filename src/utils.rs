pub fn string_to_option(s: String) -> Option<String> {
	if s.is_empty() {
		None
	} else {
		Some(s)
	}
}
