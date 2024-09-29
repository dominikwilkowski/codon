use leptos::*;

stylance::import_style!(css, "icons.module.css");

#[component]
pub fn IncubationCabinetLogo() -> impl IntoView {
	view! {
		<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 870 870" class=css::logo>
			<path stroke="currentColor" stroke-width="15" d="M278 619h4v4h-4v-4Zm49 0h4v4h-4v-4Z" />
			<path
				stroke="currentColor"
				stroke-linecap="round"
				stroke-width="15"
				d="M50 820h770m-655.703-2.972V68.309c0-10.112-8.059-18.309-18-18.309h-17.138c-9.941 0-18 8.197-18 18.31v748.718m644.979 0V68.309c0-10.112-8.059-18.309-18-18.309H721c-9.941 0-18 8.197-18 18.31v748.718M165 680.647h538M165 342.946h538M413 680.647V560.621c0-5.056-4.029-9.155-9-9.155H243c-4.971 0-9 4.099-9 9.155v120.026m391-337.701V138.494c0-5.056-4.029-9.155-9-9.155H420c-4.971 0-9 4.099-9 9.155v204.452m89 324.478v-51.876c0-5.056-4.029-9.154-9-9.154h-69c-4.971 0-9 4.098-9 9.154v51.876m205.905-157.662H641m-22.095 0v63.118l13.683 23.62m-13.683-86.738H579.11m-23.11 0h23.11m0 0v63.898l-13.388 22.84m0 0-23.455 40.018c-3.851 6.753-9.756 14.546-3.595 23.507 6.162 8.961 12.453 7.273 23.364 7.273h81.132c10.655 0 14.634-2.858 17.972-10.65 3.338-7.792-3.851-17.533-7.702-24.156l-20.85-35.992m-66.866 0h66.866M350.806 311.13l-3.343 17.15m3.343-17.15 6.022-30.891m-6.022 30.891-44.306-8.936m53.824-39.894-3.496 17.939m0 0-47.939-9.668-41.585-38.111c-4.484-4.011-9.436-9.774-17.092-6.347-7.655 3.428-7.34 8.566-8.991 17.036l-11.39 58.43m70.162.563c1.57-.455 2.818-.692 3.459-.563l3.048.615m-6.507-.052c-5.893 1.707-16.332 6.484-16.332 6.484l-30.158 10.738c-5.552 1.988-13.947 6.094-19.288 2.324-5.341-3.77-6.883-7.291-5.271-15.562l.887-4.547m70.162.563 6.507.052m-6.507-.052-70.162-.563M547 129.339v82.392c0 10.111-8.059 18.309-18 18.309h-23c-9.941 0-18-8.198-18-18.309v-82.392"
			/>
		</svg>
	}
}

#[component]
pub fn IncubationCabinet() -> impl IntoView {
	view! {
		<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 300 300" class=css::icon>
			<path
				stroke="currentColor"
				stroke-linecap="round"
				stroke-width="10"
				d="M17 279h265M56.336 20v257.976m185.398 0V20M56.578 230.985h185.156M56.578 114.628h185.156m-90.823 116.357v-41.356c0-1.742-1.387-3.154-3.097-3.154h-55.41c-1.71 0-3.097 1.412-3.097 3.154v41.356M204.91 114.628V44.183c0-1.742-1.387-3.155-3.097-3.155h-67.455c-1.711 0-3.097 1.413-3.097 3.155v70.445m49.593 111.801v-17.874c0-1.742-1.387-3.154-3.098-3.154h-23.747c-1.71 0-3.097 1.412-3.097 3.154v17.874m27.154-185.4v28.388c0 3.484-2.774 6.308-6.195 6.308h-7.915c-3.422 0-6.195-2.824-6.195-6.308V41.028"
			/>
		</svg>
	}
}
