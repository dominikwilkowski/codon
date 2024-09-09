use crate::{
	components::input::{Input, MoneyInput},
	icons::{
		Culture, CultureLogo, Equipment, EquipmentLogo, Experiment, ExperimentLogo,
		Flask, FlaskLogo, IncubationCabinet, IncubationCabinetLogo, People,
		PeopleLogo, Vessel, VesselLogo,
	},
};

// use chrono::prelude::*;
use leptos::*;
// use thaw::*;

stylance::import_style!(css, "ds.module.css");

#[component]
pub fn Ds() -> impl IntoView {
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
			<h2>Inputs</h2>
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

				<div class=css::stack_inline>
					<MoneyInput
						placeholder="Money input"
						value=create_rw_signal(String::from(""))
					/>
					<MoneyInput value=create_rw_signal(
						String::from("1234.56"),
					) />
					<MoneyInput
						placeholder="Money input"
						value=create_rw_signal(String::from(""))
						disabled=create_rw_signal(true)
					/>
					<MoneyInput
						value=create_rw_signal(String::from("1234.56"))
						disabled=create_rw_signal(true)
					/>
				</div>

				<div class=css::stack_inline>
					<input type="text" placeholder="Text input placeholder" />
					<input type="text" value="Text input" />
					<input
						type="text"
						placeholder="Disabled text input placeholder"
						disabled
					/>
					<input type="text" value="Disabled text input" disabled />
				</div>
				<div class=css::stack_inline>
					<input type="number" placeholder="Text input placeholder" />
					<input type="number" value="12345.67" />
					<input
						type="number"
						placeholder="Disabled text input placeholder"
						disabled
					/>
					<input type="number" value="12345.67" disabled />
				</div>
				<div class=css::stack_inline>
					<textarea placeholder="Textarea placeholder"></textarea>
					<textarea>Textarea</textarea>
					<textarea
						placeholder="Textarea placeholder"
						disabled
					></textarea>
					<textarea disabled>Textarea</textarea>
				</div>
			</div>
		</section>
		// <section class=css::section>
		// <h2>Thaw components</h2>
		// <div class=css::stack_block>
		// <Input
		// class="codon-thaw-input"
		// placeholder="Thaw input"
		// value=create_rw_signal(String::from(""))
		// />
		// <InputNumber
		// class="codon-thaw-input"
		// placeholder="Thaw number input"
		// attr:_type="number"
		// value=create_rw_signal(0.0)
		// step=1.0
		// />
		// <TextArea value=create_rw_signal(String::from("")) placeholder="Textarea"/>
		// <DatePicker
		// class="codon-thaw-input"
		// value=create_rw_signal(Some(Local::now().date_naive()))
		// />
		// <div class=css::stack_inline>
		// <Button
		// variant=ButtonVariant::Primary
		// class="codon-thaw-btn"
		// >
		// Button
		// </Button>
		// <Button
		// variant=ButtonVariant::Outlined
		// class="codon-thaw-btn"
		// >
		// Outlined
		// </Button>
		// <Button
		// variant=ButtonVariant::Primary
		// disabled=true
		// class="codon-thaw-btn"
		// >
		// Disabled Button
		// </Button>
		// </div>
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

		// <Switch class="input_shadow" value=create_rw_signal(false) />

		// <Select
		// class="input_shadow"
		// value=create_rw_signal(None)
		// options=vec![
		// SelectOption::new("Option A", String::from("option_a")),
		// SelectOption::new("Option B", String::from("option_b")),
		// ]
		// />

		// <div class=css::stack_inline>
		// <Button
		// class="codon-thaw-btn"
		// variant=ButtonVariant::Outlined
		// on_click=move |_| {
		// message
		// .create(
		// "Success message".into(),
		// MessageVariant::Success,
		// MessageOptions {
		// closable: true,
		// duration: std::time::Duration::from_secs(5),
		// },
		// );
		// }
		// >
		// Success toast
		// </Button>
		// <Button
		// class="codon-thaw-btn"
		// variant=ButtonVariant::Outlined
		// on_click=move |_| {
		// message
		// .create(
		// "Warning message".into(),
		// MessageVariant::Warning,
		// MessageOptions {
		// closable: true,
		// duration: std::time::Duration::from_secs(5),
		// },
		// );
		// }
		// >
		// Warning toast
		// </Button>
		// <Button
		// class="codon-thaw-btn"
		// variant=ButtonVariant::Outlined
		// on_click=move |_| {
		// message
		// .create(
		// "Error message".into(),
		// MessageVariant::Error,
		// MessageOptions {
		// closable: true,
		// duration: std::time::Duration::from_secs(5),
		// },
		// );
		// }
		// >
		// Error toast
		// </Button>
		// </div>
		// </div>
		// </section>

		<section class=css::section>
			<h2></h2>
		</section>
	}
}
