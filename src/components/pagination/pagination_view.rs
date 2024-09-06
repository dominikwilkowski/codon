use leptos::*;

stylance::import_style!(css, "pagination.module.css");

#[component]
pub fn PaginationPrev(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	children: Children,
) -> impl IntoView {
	view! {
		<form action=action method="get">
			{children()}
			<input
				type="hidden"
				name="page"
				value=if query_page.get() == 1 {
					1
				} else {
					query_page.get() - 1
				}
			/>
			<input
				type="hidden"
				name="items_per_page"
				value=query_ipp.get()
				min="1"
				max="255"
			/>
			<button type="submit" disabled=query_page.get() == 1>
				Previous Page
			</button>
		</form>
	}
}

#[component]
pub fn PaginationNext(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	items: RwSignal<usize>,
	children: Children,
) -> impl IntoView {
	view! {
		<form action=action method="get">
			{children()}
			<input
				type="hidden"
				name="page"
				value=move || {
					if items.get() == query_ipp.get() as usize {
						query_page.get() + 1
					} else {
						query_page.get()
					}
				}
			/>
			<input
				type="hidden"
				name="items_per_page"
				value=query_ipp.get()
				min="1"
				max="255"
			/>
			<button
				type="submit"
				disabled=move || items.get() < query_ipp.get() as usize
			>
				Next Page
			</button>
		</form>
	}
}

#[component]
pub fn ItemsPerPage(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	children: Children,
) -> impl IntoView {
	view! {
		<form action=action method="get">
			{children()}
			<input type="hidden" name="page" value=query_page.get() />
			<input
				type="number"
				name="items_per_page"
				value=query_ipp.get()
				min="1"
				max="255"
			/>
			<button type="submit">Save</button>
		</form>
	}
}

#[component]
pub fn Pagination(
	action: &'static str,
	query_page: RwSignal<u16>,
	query_ipp: RwSignal<u8>,
	items: RwSignal<usize>,
	children: ChildrenFn,
) -> impl IntoView {
	let children1 = children.clone();
	let children2 = children.clone();
	let children3 = children.clone();

	view! {
		<PaginationPrev action query_page query_ipp>
			{children1()}
		</PaginationPrev>
		<ItemsPerPage action query_page query_ipp>
			{children2()}
		</ItemsPerPage>
		<PaginationNext action query_page query_ipp items>
			{children3()}
		</PaginationNext>
	}
}
