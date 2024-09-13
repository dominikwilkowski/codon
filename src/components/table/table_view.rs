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
	view! {
		{items
			.iter()
			.map(move |name| {
				let order = if query_field.get() == *name
					&& query_order.get() == "asc"
				{
					"desc"
				} else {
					"asc"
				};
				let svg_path = if query_field.get() == *name {
					match order {
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
				let name = match name.as_str() {
					"cost_in_cent" => &String::from("Cost"),
					"equipment_type" => &String::from("Type"),
					"create_date" => &String::from("Created"),
					"purchase_date" => &String::from("Purchased"),
					_ => name,
				};
				view! {
					<th class=format!("{name} {}", css::th)>
						<div class=css::thead>
							{name.to_case(Case::Title)}
							<form action=action method="get" class=css::sort_form>
								<input type="hidden" name="field" value=name />
								<input type="hidden" name="order" value=order />
								{children()}
								<button type="submit" class=css::sort_btn>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										viewBox="0 0 24 24"
										fill="currentColor"
									>
										<path d=svg_path />
									</svg>
								</button>
							</form>
						</div>
					</th>
				}
			})
			.collect_view()}
		<th colspan="3" class=format!("empty {}", css::th)></th>
	}
}
