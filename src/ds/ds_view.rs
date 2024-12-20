use crate::{
	components::{
		button::{Button, ButtonVariant, MessageOptions, MessageVariant, use_message},
		checkbox::{Checkbox, CheckboxGroup, CheckboxItem},
		datepicker::DatePicker,
		dropdown::{Dropdown, DropdownItem, DropdownTrigger},
		file_input::FileInput,
		input::{Input, MoneyInput, TextArea},
		pagination::Pagination,
		radio::{Radio, RadioGroup, RadioItem},
		select::{MultiSelect, MultiSelectOption, Select, TagVariant},
		switch::Switch,
	},
	icons::{
		Culture, CultureLogo, Equipment, EquipmentLogo, Experiment, ExperimentLogo, Flask, FlaskLogo, IncubationCabinet,
		IncubationCabinetLogo, People, PeopleLogo, Vessel, VesselLogo,
	},
};

use chrono::prelude::*;
use leptos::*;

stylance::import_style!(css, "ds.module.css");

#[component]
pub fn Ds() -> impl IntoView {
	let message = use_message();

	view! {
		<h1>Codon Design System</h1>
		<section class=css::section>
			<h2>Logos</h2>
			<div class=css::grid>
				<CultureLogo />
				<FlaskLogo />
				<IncubationCabinetLogo />
				<VesselLogo />
				<EquipmentLogo />
				<ExperimentLogo />
				<PeopleLogo />
			</div>
		</section>

		<section class=css::section>
			<h2>Icons</h2>
			<div class=css::grid>
				<Culture />
				<Flask />
				<IncubationCabinet />
				<Vessel />
				<Equipment />
				<Experiment />
				<People />
			</div>
		</section>

		<section class=css::section>
			<h2>Pagination</h2>
			<Pagination
				action=String::from("/ds")
				page_key="page"
				ipp_key="items_per_page"
				query_page=create_rw_signal(1)
				query_ipp=create_rw_signal(25)
				row_count=200
				hidden_fields=vec![
					(String::from("field"), String::from("id")),
					(String::from("order"), String::from("asc")),
				]
			/>
		</section>

		<section class=css::section>
			<h2>Text Inputs</h2>
			<div class=css::stack_inline>
				<Input placeholder="Text input" value=create_rw_signal(String::from("")) />
				<Input value=create_rw_signal(String::from("Text input")) />
				<Input
					placeholder="Text input"
					value=create_rw_signal(String::from(""))
					disabled=create_rw_signal(true)
				/>
				<Input value=create_rw_signal(String::from("Text input")) disabled=create_rw_signal(true) />
			</div>
		</section>

		<section class=css::section>
			<h2>Money Inputs</h2>
			<div class=css::stack_inline>
				<MoneyInput value=create_rw_signal(String::from("")) />
				<MoneyInput value=create_rw_signal(String::from("1234.56")) />
				<MoneyInput value=create_rw_signal(String::from("")) disabled=create_rw_signal(true) />
				<MoneyInput value=create_rw_signal(String::from("1234.56")) disabled=create_rw_signal(true) />
			</div>
		</section>

		<section class=css::section>
			<h2>Multiline Inputs</h2>
			<div class=css::stack_inline>
				<TextArea value=create_rw_signal(String::from("")) placeholder="Textarea" />
				<TextArea value=create_rw_signal(String::from("Multiline Input")) placeholder="Textarea" />
				<TextArea
					value=create_rw_signal(String::from(""))
					placeholder="Textarea"
					disabled=create_rw_signal(true)
				/>
				<TextArea
					value=create_rw_signal(String::from("Multiline Input"))
					placeholder="Textarea"
					disabled=create_rw_signal(true)
				/>
			</div>
		</section>

		<section class=css::section>
			<h2>Buttons</h2>
			<div class=css::stack_inline>
				<Button on_click=|_| logging::log!("Hello World")>Button</Button>
				<Button on_click=|_| logging::log!("Hello World") kind="submit">
					Submit Button
				</Button>
				<Button on_click=|_| logging::log!("Hello World") disabled=create_rw_signal(true)>
					Disabled Button
				</Button>
				<Button on_click=|_| logging::log!("Hello World") loading=create_rw_signal(true)>
					Button
				</Button>
			</div>
		</section>

		<section class=css::section>
			<h2>Buttons outlined</h2>
			<div class=css::stack_inline>
				<Button variant=ButtonVariant::Outlined on_click=|_| logging::log!("Hello World")>
					Button
				</Button>
				<Button variant=ButtonVariant::Outlined kind="submit" on_click=|_| logging::log!("Hello World")>
					Submit Button
				</Button>
				<Button
					variant=ButtonVariant::Outlined
					disabled=create_rw_signal(true)
					on_click=|_| logging::log!("Hello World")
				>
					Disabled Button
				</Button>
				<Button
					variant=ButtonVariant::Outlined
					loading=create_rw_signal(true)
					on_click=|_| logging::log!("Hello World")
				>
					Button
				</Button>
			</div>
		</section>

		<section class=css::section>
			<h2>Buttons text</h2>
			<div class=css::stack_inline>
				<Button variant=ButtonVariant::Text on_click=|_| logging::log!("Hello World")>
					Button
				</Button>
				<Button variant=ButtonVariant::Text kind="submit" on_click=|_| logging::log!("Hello World")>
					Submit Button
				</Button>
				<Button
					variant=ButtonVariant::Text
					disabled=create_rw_signal(true)
					on_click=|_| logging::log!("Hello World")
				>
					Disabled Button
				</Button>
				<Button
					variant=ButtonVariant::Text
					loading=create_rw_signal(true)
					on_click=|_| logging::log!("Hello World")
				>
					Button
				</Button>
			</div>
		</section>

		<section class=css::section>
			<h2>Toasts</h2>
			<div class=css::stack_inline>
				<Button
					variant=ButtonVariant::Outlined
					on_click=move |_| {
						message
							.create(
								"Success message".into(),
								MessageVariant::Success,
								MessageOptions {
									closable: true,
									duration: std::time::Duration::from_secs(5),
								},
							);
					}
				>
					Success toast
				</Button>
				<Button
					variant=ButtonVariant::Outlined
					on_click=move |_| {
						message
							.create(
								"Warning message".into(),
								MessageVariant::Warning,
								MessageOptions {
									closable: true,
									duration: std::time::Duration::from_secs(5),
								},
							);
					}
				>
					Warning toast
				</Button>
				<Button
					variant=ButtonVariant::Outlined
					on_click=move |_| {
						message
							.create(
								"Error message".into(),
								MessageVariant::Error,
								MessageOptions {
									closable: true,
									duration: std::time::Duration::from_secs(5),
								},
							);
					}
				>
					Error toast
				</Button>
			</div>
		</section>

		<section class=css::section>
			<h2>FileInput</h2>
			<div class=css::stack_inline>
				<FileInput name="media1" value=create_rw_signal(String::new()) />
			</div>
		</section>

		<section class=css::section>
			<h2>Select</h2>
			<div class=css::stack_inline>
				<Select>
					<option value="a">Option A</option>
					<option value="b">Option B</option>
				</Select>

				<Select disabled=create_rw_signal(true)>
					<option value="a">Option A</option>
					<option value="b">Option B</option>
				</Select>

				<Select disabled=create_rw_signal(true)>
					<option value="a">Option A</option>
					<option value="b" selected>
						Option B
					</option>
				</Select>
			</div>
		</section>

		<section class=css::section>
			<h2>Multi Select</h2>

			<div class=css::stack_inline>
				<MultiSelect
					value=create_rw_signal(vec![])
					options=create_rw_signal(
						vec![
							MultiSelectOption::new("A", String::from("a")),
							MultiSelectOption::new("B", String::from("b")).with_variant(TagVariant::Success),
							MultiSelectOption::new("C", String::from("c")).with_variant(TagVariant::Warning),
							MultiSelectOption::new("D", String::from("d")).with_variant(TagVariant::Error),
						],
					)
				/>
			</div>
		</section>

		<section class=css::section>
			<h2>Datepicker</h2>
			<div class=css::stack_inline>
				<DatePicker value=create_rw_signal(Some(Local::now().date_naive())) />
			</div>
		</section>

		<section class=css::section>
			<h2>Switch</h2>
			<div class=css::stack_inline>
				<Switch value=create_rw_signal(false) />
			</div>
		</section>

		<section class=css::section>
			<h2>Checkbox</h2>
			<div class=css::stack_inline>
				<CheckboxGroup value=create_rw_signal(
					vec![String::from("b")].into_iter().collect::<std::collections::HashSet<String>>(),
				)>
					<CheckboxItem label="Option A" key=String::from("a") />
					<CheckboxItem label="Option B" key=String::from("b") />
					<CheckboxItem label="Option C" key=String::from("c") />
				</CheckboxGroup>
			</div>

			<div class=css::stack_inline>
				<Checkbox>Single Checkbox</Checkbox>
			</div>
		</section>

		<section class=css::section>
			<h2>Radio</h2>
			<div class=css::stack_inline>
				<RadioGroup value=create_rw_signal(Some(String::from("b")))>
					<RadioItem key="a">Radio A</RadioItem>
					<RadioItem key="b">Radio B</RadioItem>
				</RadioGroup>
			</div>

			<div class=css::stack_inline>
				<Radio>Single Radio</Radio>
			</div>
		</section>

		<section class=css::section>
			<h2>Dropdown</h2>
			<div class=css::stack_inline>
				<Dropdown on_select=move |_| {}>
					<DropdownTrigger slot>
						<Button variant=ButtonVariant::Outlined>"Click"</Button>
					</DropdownTrigger>
					<DropdownItem key="foo" label="Option A" />
					<DropdownItem key="bar" label="Option B" />
					<DropdownItem key="disabled" disabled=true label="Option C" />
				</Dropdown>
			</div>
		</section>

		<section class=css::section>
			<h2></h2>
			<div class=css::stack_inline></div>
		</section>
	}
}
