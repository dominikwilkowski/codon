use crate::{
	components::{avatar::Avatar, multiline::MultiLine},
	equipment::NotesPerson,
};

use leptos::*;

stylance::import_style!(css, "notes_item.module.css");

#[component]
pub fn NotesItem(note: NotesPerson) -> impl IntoView {
	view! {
		<div class=css::notes_item>
			<Avatar data=note.person />
			<div>
				<small>
					{note
						.note
						.create_date
						.format("%d %b %Y %I:%M:%S %P")
						.to_string()}
				</small>
				<MultiLine text=note.note.notes />
				<div class=css::imgs>
					<NotesImg img=note.note.media1 />
					<NotesImg img=note.note.media2 />
					<NotesImg img=note.note.media3 />
					<NotesImg img=note.note.media4 />
					<NotesImg img=note.note.media5 />
					<NotesImg img=note.note.media6 />
					<NotesImg img=note.note.media7 />
					<NotesImg img=note.note.media8 />
					<NotesImg img=note.note.media9 />
					<NotesImg img=note.note.media10 />
				</div>
			</div>
		</div>
	}
}

#[component]
pub fn NotesImg(img: Option<String>) -> impl IntoView {
	view! {
		{if img.is_some() {
			let img = img.unwrap();
			view! {
				<a href=img.clone()>
					<img class=css::img src=img />
				</a>
			}
				.into_view()
		} else {
			view! {}.into_view()
		}}
	}
}
