#[rustfmt::skip]
#[macro_export]
macro_rules! display_default_for_string_struct {
	($struct_name:ident) => {
		impl std::fmt::Display for $struct_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}

		impl std::default::Default for $struct_name {
			fn default() -> Self {
				$struct_name(Default::default())
			}
		}
	};
}
