use leptos::*;

stylance::import_style!(css, "table.module.css");

#[component]
pub fn TableHead(
	action: &'static str,
	items: Vec<String>,
	query_field: RwSignal<String>,
	query_order: RwSignal<String>,
	children: ChildrenFn,
) -> impl IntoView {
	items
		.iter()
		.map(move |name| {
			let label = if query_field.get() == *name {
				"*↕️"
			} else {
				"↕️"
			};
			let order = if query_field.get() == *name && query_order.get() == "asc" {
				"desc"
			} else {
				"asc"
			};
			view! {
				<th>
					{name} <form action=action method="get">
						<input type="hidden" name="field" value=name />
						<input type="hidden" name="order" value=order />
						{children()}
						<button type="submit">{label}</button>
					</form>
				</th>
			}
		})
		.collect_view()
}
