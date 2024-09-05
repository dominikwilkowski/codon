#[rustfmt::skip]
#[macro_export]
macro_rules! custom_sql_string_type {
	($struct_name:ident) => {
		#[cfg(feature = "ssr")]
		impl sqlx::Type<sqlx::Postgres> for $struct_name {
			fn type_info() -> sqlx::postgres::PgTypeInfo {
				<String as sqlx::Type<sqlx::Postgres>>::type_info()
			}
		}

		#[cfg(feature = "ssr")]
		impl sqlx::Encode<'_, sqlx::Postgres> for $struct_name {
			fn encode_by_ref(
				&self,
				buf: &mut sqlx::postgres::PgArgumentBuffer,
			) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>>
			{
				<String as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0, buf)
			}
		}

		#[cfg(feature = "ssr")]
		impl<'a> sqlx::Decode<'a, sqlx::Postgres> for $struct_name {
			fn decode(
				value: sqlx::postgres::PgValueRef<'a>,
			) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
				let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
				Ok($struct_name(s))
			}
		}

		impl std::fmt::Display for $struct_name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{}", self.0)
			}
		}
	};
}
