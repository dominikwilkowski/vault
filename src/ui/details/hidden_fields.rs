use floem::{
	reactive::{create_rw_signal, ReadSignal, RwSignal, WriteSignal},
	style::Display,
	view::View,
	views::{
		container, h_stack, label, v_stack, virtual_stack, Decorators,
		VirtualDirection, VirtualItemSize,
	},
};

use crate::env::Environment;
use crate::{
	db::DbFields,
	ui::{
		colors::*,
		details::list_item::{list_item, ListItem},
		primitives::{
			button::{icon_button, ButtonVariant, IconButton},
			tooltip::TooltipSignals,
		},
	},
	Que,
};

pub struct HiddeFields {
	pub id: usize,
	pub hidden_field_list: ReadSignal<im::Vector<DbFields>>,
	pub set_hidden_field_list: WriteSignal<im::Vector<DbFields>>,
	pub set_dyn_field_list: WriteSignal<im::Vector<DbFields>>,
	pub hidden_field_len: RwSignal<usize>,
	pub tooltip_signals: TooltipSignals,
	pub set_list: WriteSignal<im::Vector<(usize, &'static str, usize)>>,
	pub main_scroll_to: RwSignal<f32>,
	pub que: Que,
	pub env: Environment,
}

pub fn hidden_fields(param: HiddeFields) -> impl View {
	let HiddeFields {
		id,
		hidden_field_list,
		set_hidden_field_list,
		set_dyn_field_list,
		hidden_field_len,
		tooltip_signals,
		set_list,
		main_scroll_to,
		que,
		env,
	} = param;
	let is_expanded = create_rw_signal(false);

	let expand_icon = include_str!("../icons/expand.svg");
	let contract_icon = include_str!("../icons/contract.svg");

	v_stack((
		v_stack((
			container(label(|| "").style(|s| {
				s.border_top(1).border_color(C_BG_MAIN_BORDER).height(1).width(252)
			}))
			.style(|s| s.justify_center().margin_bottom(10)),
			virtual_stack(
				VirtualDirection::Vertical,
				VirtualItemSize::Fixed(Box::new(|| 35.0)),
				move || hidden_field_list.get(),
				move |item| *item,
				move |field| {
					list_item(ListItem {
						id,
						field,
						set_hidden_field_list,
						set_dyn_field_list,
						hidden_field_len,
						is_hidden: true,
						tooltip_signals,
						set_list,
						que,
						env: env.clone(),
					})
					.style(|s| s.padding_bottom(5))
				},
			)
			.style(|s| s.display(Display::Flex)),
		))
		.style(move |s| {
			s.display(Display::None)
				.margin_bottom(15)
				.margin_top(10)
				.apply_if(is_expanded.get(), |s| s.display(Display::Flex))
		}),
		h_stack((
			label(|| "").style(|s| {
				s.border_top(1).border_color(C_BG_MAIN_BORDER).height(1).width(120)
			}),
			icon_button(
				IconButton {
					variant: ButtonVariant::Tiny,
					icon: String::from(expand_icon),
					icon2: Some(String::from(contract_icon)),
					bubble: Some(hidden_field_len),
					tooltip: format!(
						"Show {} hidden field{}",
						hidden_field_len.get(),
						if hidden_field_len.get() > 1 { "s" } else { "" }
					),
					tooltip2: Some(String::from("Hide hidden field")),
					switch: Some(is_expanded),
					tooltip_signals,
				},
				move |_| {
					if is_expanded.get() {
						is_expanded.set(true);
						main_scroll_to.set(100.0);
					} else {
						is_expanded.set(false);
					}
				},
			),
			label(|| "").style(|s| {
				s.border_top(1).border_color(C_BG_MAIN_BORDER).height(1).width(120)
			}),
		))
		.style(|s| s.flex().items_center().justify_center().gap(4, 0)),
	))
	.style(move |s| {
		s.apply_if(hidden_field_len.get() < 1, |s| s.display(Display::None))
	})
}
