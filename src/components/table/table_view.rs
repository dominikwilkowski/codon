use convert_case::{Case, Casing};
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

			let name = match name.as_str() {
				"cost_in_cent" => &String::from("Cost"),
				_ => name,
			};

			view! {
				<th class=name>
					{name.to_case(Case::Title)}
					<form action=action method="get">
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
