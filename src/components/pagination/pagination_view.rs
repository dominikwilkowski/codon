use leptos::*;

stylance::import_style!(css, "pagination.module.css");

#[component]
pub fn FieldBuilder(hidden_fields: Vec<(String, String)>) -> impl IntoView {
	hidden_fields
		.into_iter()
		.map(
			|(name, value)| view! { <input type="hidden" name=name value=value /> },
		)
		.collect_view()
}

#[component]
pub fn PaginationPrev(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	hidden_fields: Vec<(String, String)>,
) -> impl IntoView {
	view! {
		<form action=action method="get">
			<FieldBuilder hidden_fields />
			<input
				type="hidden"
				name="page"
				value=if query_page.get() == 1 {
					1
				} else {
					query_page.get() - 1
				}
			/>
			<input type="hidden" name="items_per_page" value=query_ipp.get() />
			<button type="submit" disabled=query_page.get() == 1 class=css::btn>
				Previous
			</button>
		</form>
	}
}

#[component]
pub fn PaginationNext(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	row_count: i64,
	hidden_fields: Vec<(String, String)>,
) -> impl IntoView {
	let max_pages = row_count as f64 / query_ipp.get() as f64;
	let is_last_page = query_page.get() as f64 >= max_pages;

	view! {
		<form action=action method="get">
			<FieldBuilder hidden_fields />
			<input
				type="hidden"
				name="page"
				value=move || {
					if !is_last_page {
						query_page.get() + 1
					} else {
						query_page.get()
					}
				}
			/>
			<input type="hidden" name="items_per_page" value=query_ipp.get() />
			<button type="submit" disabled=move || is_last_page class=css::btn>
				Next
			</button>
		</form>
	}
}

#[component]
pub fn ItemsPerPage(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	row_count: i64,
	hidden_fields: Vec<(String, String)>,
) -> impl IntoView {
	let mut to = query_page.get() as i64 * query_ipp.get() as i64;
	let from = to - query_ipp.get() as i64;
	if to > row_count {
		to = row_count;
	}

	view! {
		<form action=action method="get" class=css::ipp_form>
			<FieldBuilder hidden_fields />
			<input type="hidden" name="page" value=query_page.get() />
			<label>
				"Items per page: "
				<input
					class=css::ipp_input
					type="number"
					name="items_per_page"
					value=query_ipp.get()
					min="1"
					max="255"
				/>
			</label>
			<button type="submit" class=css::btn>
				Save
			</button>
			<span class=css::ipp_stats>{from}- {to}of {row_count}items</span>
		</form>
	}
}

#[component]
pub fn Pages(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	row_count: i64,
	hidden_fields: Vec<(String, String)>,
) -> impl IntoView {
	let pages = row_count / query_ipp.get() as i64;

	view! {
		<div class=css::pages>
			{get_page_range(pages, query_page.get() as i64)
				.map(move |page| {
					view! {
						<form
							action=action
							method="get"
							class=if page == query_page.get() as i64 {
								"is_current"
							} else {
								""
							}
						>
							<FieldBuilder hidden_fields=hidden_fields.clone() />
							<input
								type="hidden"
								name="items_per_page"
								value=query_ipp.get()
							/>
							<input type="hidden" name="page" value=page />
							<button
								type="submit"
								class=format!("{} input_shadow", css::btn)
							>
								{page}
							</button>
						</form>
					}
				})
				.collect_view()}
		</div>
	}
}

fn get_page_range(
	total_pages: i64,
	current_page: i64,
) -> std::ops::RangeInclusive<i64> {
	let range_size = 9;
	let half_range = range_size / 2;

	let start = if current_page <= half_range {
		1
	} else if current_page >= total_pages - half_range {
		total_pages.saturating_sub(range_size - 1).max(1)
	} else {
		current_page.saturating_sub(half_range).max(1)
	};

	let end = (start + range_size - 1).min(total_pages);

	start..=end
}

#[test]
fn get_page_range_test() {
	assert_eq!(get_page_range(5, 1), (1..=5));
	assert_eq!(get_page_range(5, 4), (1..=5));
	assert_eq!(get_page_range(5, 5), (1..=5));

	assert_eq!(get_page_range(200, 1), (1..=9));
	assert_eq!(get_page_range(200, 2), (1..=9));
	assert_eq!(get_page_range(200, 3), (1..=9));

	assert_eq!(get_page_range(200, 54), (50..=58));
	assert_eq!(get_page_range(200, 55), (51..=59));
	assert_eq!(get_page_range(200, 56), (52..=60));

	assert_eq!(get_page_range(200, 198), (192..=200));
	assert_eq!(get_page_range(200, 199), (192..=200));
	assert_eq!(get_page_range(200, 200), (192..=200));
}

#[component]
pub fn Pagination(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	row_count: i64,
	#[prop(optional)] hidden_fields: Vec<(String, String)>,
) -> impl IntoView {
	view! {
		<div class=css::pagination>
			<ItemsPerPage
				action
				query_page
				query_ipp
				row_count
				hidden_fields=hidden_fields.clone()
			/>
			<div class=css::pagination_group>
				<PaginationPrev
					action
					query_page
					query_ipp
					hidden_fields=hidden_fields.clone()
				/>
				<Pages
					action
					query_page
					query_ipp
					row_count
					hidden_fields=hidden_fields.clone()
				/>
				<PaginationNext
					action
					query_page
					query_ipp
					row_count
					hidden_fields
				/>
			</div>
		</div>
	}
}
