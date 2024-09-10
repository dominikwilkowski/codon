use crate::{
	components::{
		button::{use_message, Button, MessageOptions, MessageVariant},
		datepicker::DatePicker,
		input::{Input, MoneyInput, TextArea},
		select::{Select, SelectOption},
		switch::Switch,
	},
	icons::{
		Culture, CultureLogo, Equipment, EquipmentLogo, Experiment, ExperimentLogo,
		Flask, FlaskLogo, IncubationCabinet, IncubationCabinetLogo, People,
		PeopleLogo, Vessel, VesselLogo,
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
			<h2>Text Inputs</h2>
			<div class=css::stack_block>
				<div class=css::stack_inline>
					<Input
						placeholder="Text input"
						value=create_rw_signal(String::from(""))
					/>
					<Input value=create_rw_signal(String::from("Text input")) />
					<Input
						placeholder="Text input"
						value=create_rw_signal(String::from(""))
						disabled=create_rw_signal(true)
					/>
					<Input
						value=create_rw_signal(String::from("Text input"))
						disabled=create_rw_signal(true)
					/>
				</div>

				<h2>Money Inputs</h2>
				<div class=css::stack_inline>
					<MoneyInput value=create_rw_signal(String::from("")) />
					<MoneyInput value=create_rw_signal(
						String::from("1234.56"),
					) />
					<MoneyInput
						value=create_rw_signal(String::from(""))
						disabled=create_rw_signal(true)
					/>
					<MoneyInput
						value=create_rw_signal(String::from("1234.56"))
						disabled=create_rw_signal(true)
					/>
				</div>

				<h2>Multiline Inputs</h2>
				<div class=css::stack_inline>
					<TextArea
						value=create_rw_signal(String::from(""))
						placeholder="Textarea"
					/>
					<TextArea
						value=create_rw_signal(String::from("Multiline Input"))
						placeholder="Textarea"
					/>
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

				<h2>Buttons</h2>
				<div class=css::stack_inline>
					<Button>Button</Button>
					<Button disabled=create_rw_signal(
						true,
					)>Disabled Button</Button>
					<Button loading=create_rw_signal(true)>Button</Button>
				</div>

				<h2>Buttons outlined</h2>
				<div class=css::stack_inline>
					<Button outlined=true>Button</Button>
					<Button outlined=true disabled=create_rw_signal(true)>
						Disabled Button
					</Button>
					<Button outlined=true loading=create_rw_signal(true)>
						Button
					</Button>
				</div>

				<h2>Toasts</h2>
				<div class=css::stack_inline>
					<Button
						outlined=true
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
						outlined=true
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
						outlined=true
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

				<h2>Select</h2>
				<div class=css::stack_inline>
					<Select
						value=create_rw_signal(None)
						options=vec![
							SelectOption::new("Option A", String::from("option_a")),
							SelectOption::new("Option B", String::from("option_b")),
						]
					/>

					<select>
						<option value="a">Option A</option>
						<option value="b">Option B</option>
					</select>
				</div>

				<h2>Datepicker</h2>
				<div class=css::stack_inline>
					<DatePicker value=create_rw_signal(
						Some(Local::now().date_naive()),
					) />
				</div>

				<h2>Switch</h2>
				<div class=css::stack_inline>
					<Switch value=create_rw_signal(false) />
				</div>

			</div>
		</section>

		// <CheckboxGroup value=create_rw_signal(
		// vec![String::from("b")]
		// .into_iter()
		// .collect::<std::collections::HashSet<String>>(),
		// )>
		// <CheckboxItem label="Option A" key=String::from("a") />
		// <CheckboxItem label="Option B" key=String::from("b") />
		// <CheckboxItem label="Option C" key=String::from("c") />
		// </CheckboxGroup>

		// <RadioGroup value=create_rw_signal(Some(String::from("b")))>
		// <RadioItem key="a">Radio A</RadioItem>
		// <RadioItem key="b">Radio B</RadioItem>
		// </RadioGroup>

		<section class=css::section>
			<h2></h2>
		</section>
	}
}
