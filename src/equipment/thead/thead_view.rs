use leptos::*;

stylance::import_style!(css, "thead.module.css");

#[component]
pub fn THead(
	action: &'static str,
	items: Vec<(String, String)>,
	query_field: RwSignal<String>,
	query_order: RwSignal<String>,
	field_filter: RwSignal<Vec<String>>,
	children: ChildrenFn,
) -> impl IntoView {
	let order = move |id: String| {
		if query_field.get() == id && query_order.get() == "asc" { "desc" } else { "asc" }
	};
	let svg_path = move |id: String| if query_field.get() == id {
		match order(id) {
			"desc" => {
				"M6.227 11h11.547c.862 0 1.32-1.02.747-1.665L12.748 2.84a.998.998 0 0 0-1.494 0L5.479 9.335C4.906 9.98 5.364 11 6.227 11z"
			}
			"asc" => {
				"M6.227 11m5.026 10.159a.998.998 0 0 0 1.494 0l5.773-6.495c.574-.644.116-1.664-.747-1.664H6.227c-.862 0-1.32 1.02-.747 1.665l5.773 6.494z"
			}
			_ => "",
		}
	} else {
		"M6.227 11h11.547c.862 0 1.32-1.02.747-1.665L12.748 2.84a.998.998 0 0 0-1.494 0L5.479 9.335C4.906 9.98 5.364 11 6.227 11zm5.026 10.159a.998.998 0 0 0 1.494 0l5.773-6.495c.574-.644.116-1.664-.747-1.664H6.227c-.862 0-1.32 1.02-.747 1.665l5.773 6.494z"
	};

	view! {
		{items
			.into_iter()
			.map(move |(id, name)| {
				let children = children.clone();
				let id_value = id.clone();
				view! {
					<Show when=move || field_filter.get().contains(&id)>
						<th class=format!("{} {}", id_value.clone(), css::th)>
							<div class=css::thead>
								{name.clone()} <form action=action method="get" class=css::sort_form>
									<input type="hidden" name="field" value=id_value.clone() />
									<input type="hidden" name="order" value=order(id_value.clone()) />
									{children()}
									<button type="submit" class=css::sort_btn>
										<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
											<path d=svg_path(id_value.clone()) />
										</svg>
									</button>
								</form>
							</div>
						</th>
					</Show>
				}
			})
			.collect_view()}
		<th colspan="1" class=format!("empty {}", css::th)></th>
	}
}
