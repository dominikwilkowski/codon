use leptos::*;

stylance::import_style!(css, "file_input.module.css");

#[component]
pub fn FileInput(value: RwSignal<String>, name: &'static str) -> impl IntoView {
	let input_ref = create_node_ref::<html::Input>();

	view! {
		<label class=format!("input_shadow {}", css::file_input_wrapper)>
			<input
				ref=input_ref
				class=css::file_input
				type="file"
				accept="image/*,video/*"
				name=name
				on:change=move |_| {
					let input = input_ref.get().unwrap();
					match input.files().and_then(|files| files.item(0)) {
						Some(file) => {
							value.set(file.name());
						}
						_ => {
							value.set(String::new());
						}
					}
				}
			/>
			<span>
				{move || { if value.get().is_empty() { String::from("Add Photo/Video") } else { value.get() } }}
			</span>
		</label>
	}
}
