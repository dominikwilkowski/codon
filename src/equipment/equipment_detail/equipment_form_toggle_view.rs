use crate::{
	app::UserSignal,
	components::button::{Button, ButtonVariant},
	equipment::{EquipmentCell, EquipmentCellView},
	permission::Permissions,
};

use leptos::*;
use leptos_router::*;

#[component]
pub fn EquipmentFormToggle<T: EquipmentCellView + Clone + 'static>(
	user_signal: UserSignal,
	user_id: i32,
	id: i32,
	item: T,
	children: ChildrenFn,
) -> impl IntoView {
	let toggle = create_rw_signal(false);
	view! {
		<Show when=move || toggle.get() fallback=move || view! { <EquipmentCell cell=item.clone() /> }>
			{children()}
		</Show>

		<Suspense fallback=move || {
			view! { <A href="/login">"Login"</A> }
		}>
			{move || {
				match user_signal.get() {
					None => view! { <span /> }.into_view(),
					Some(user) => {
						let Permissions::All { read: _, write: perm, create: _ } = user.permission_equipment;
						view! {
							<Show when=move || perm.has_permission("write", id, user_id)>
								<Button
									variant=ButtonVariant::Text
									on_click=move |_| toggle.update(|toggle| *toggle = !*toggle)
								>
									{move || if toggle.get() { "Cancel" } else { "Edit" }}
								</Button>
							</Show>
						}
							.into_view()
					}
				}
			}}
		</Suspense>
	}
}
