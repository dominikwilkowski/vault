use std::rc::Rc;
use zeroize::Zeroize;

use floem::{
	event::EventListener,
	id::Id,
	reactive::{create_rw_signal, use_context, RwSignal},
	style::{AlignContent, AlignItems},
	view::View,
	views::{
		dyn_stack,
		editor::{
			core::{editor::EditType, selection::Selection},
			text::Document,
		},
		h_stack, label, svg, v_stack, Decorators,
	},
};

use crate::{
	db::DbFields,
	env::Environment,
	ui::{
		app_view::{PresetFieldSignal, SidebarList},
		details::{
			hidden_fields::{hidden_fields, HiddeFields},
			list_item::{list_item, ListItem},
			new_field::new_field,
		},
		primitives::tooltip::TooltipSignals,
	},
};

pub const SECRET_PLACEHOLDER: &str = "••••••••••••••••";
pub const SECRET_MULTILINE_PLACEHOLDER: &str =
	"•••••••••••\n•••••••••••••\n••••••\n•••••••••••••\n•••••••••\n•••••••••••\n••••••••••••••••\n••••••••••\n••••••\n••••••••••\n••••";
pub const INPUT_LINE_WIDTH: f64 = 250.0;
pub const LABEL_WIDTH: f64 = 142.0;
pub const LINE_WIDTH: f64 = 560.0;
pub const MULTILINE_HEIGHT: f64 = 165.0;
pub const DETAILS_MIN_WIDTH: f64 = 600.0;

pub struct SaveEdit {
	pub id: usize,
	pub field: DbFields,
	pub value: RwSignal<String>,
	pub doc: Rc<dyn Document>,
	pub dates: RwSignal<Vec<(usize, u64)>>,
	pub is_secret: bool,
	pub is_multiline: bool,
	pub input_id: Id,
}

pub fn save_edit(params: SaveEdit) {
	let SaveEdit {
		id,
		field,
		value,
		doc,
		dates,
		is_secret,
		is_multiline,
		input_id,
	} = params;

	let env = use_context::<Environment>().expect("No env context provider");
	let list_sidebar_signal = use_context::<SidebarList>()
		.expect("No list_sidebar_signal context provider");

	let field_value = if is_multiline {
		String::from(doc.text())
	} else {
		value.get()
	};

	let last_val = env.db.get_last_by_field(&id, &field);
	if last_val != field_value {
		env.db.edit_field(id, &field, field_value.clone());
		let _ = env.db.save();
		if field == DbFields::Title {
			let new_list = env.db.get_sidebar_list();
			list_sidebar_signal.update(
				|list: &mut im::Vector<(usize, String, usize)>| {
					*list = new_list;
				},
			);
		}

		dates.set(env.db.get_history_dates(&id, &field));
		input_id.request_focus();
	}

	if is_secret {
		match is_multiline {
			true => {
				value.set(String::from(SECRET_MULTILINE_PLACEHOLDER));
				doc.edit_single(
					Selection::region(0, doc.text().len()),
					SECRET_MULTILINE_PLACEHOLDER,
					EditType::DeleteSelection,
				);
			},
			false => {
				value.update(|field| field.zeroize());
				value.set(String::from(SECRET_PLACEHOLDER));
			},
		}
	} else if is_multiline {
		value.set(field_value);
	}
}

pub fn detail_view(id: usize, main_scroll_to: RwSignal<f32>) -> impl View {
	let env = use_context::<Environment>().expect("No env context provider");
	let tooltip_signals = use_context::<TooltipSignals>()
		.expect("No tooltip_signals context provider");
	let list_sidebar_signal = use_context::<SidebarList>()
		.expect("No list_sidebar_signal context provider");
	let field_presets = use_context::<PresetFieldSignal>()
		.expect("No field_presets context provider");

	let is_overflowing = create_rw_signal(false);

	let password_icon = include_str!("../icons/password.svg");

	let field_list: im::Vector<DbFields> = env.db.get_visible_fields(&id).into();
	let field_list = create_rw_signal(field_list);

	let hidden_field_list: im::Vector<DbFields> =
		env.db.get_hidden_fields(&id).into();
	let hidden_field_len = create_rw_signal(hidden_field_list.len());
	let hidden_field_list = create_rw_signal(hidden_field_list);

	v_stack((
		h_stack((
			svg(move || String::from(password_icon))
				.style(|s| s.width(24).height(24).min_width(24)),
			label(move || {
				list_sidebar_signal
					.get()
					.iter()
					.find(|item| item.0 == id)
					.unwrap_or(&(0, String::from("Details"), 0))
					.1.clone()
			})
			.on_text_overflow(move |is_overflown| {
				is_overflowing.set(is_overflown);
			})
			.on_event_cont(EventListener::PointerEnter, move |_| {
				if is_overflowing.get() {
					tooltip_signals.show(list_sidebar_signal
						.get()
						.iter()
						.find(|item| item.0 == id)
						.unwrap_or(&(0, String::from("Details"), 0))
						.1.clone()
						);
				}
			})
			.on_event_cont(EventListener::PointerLeave, move |_| {
				tooltip_signals.hide();
			})
			.style(|s| s.text_ellipsis().font_size(24.0).max_width_full()),
		))
		.style(|s| {
			s.flex()
				.flex_row()
				.align_items(AlignItems::Center)
				.max_width_pct(90.0)
				.gap(5, 0)
				.margin(5)
				.margin_right(20)
				.margin_top(15)
				.margin_bottom(20)
		}),
		v_stack((
			list_item(ListItem {
				id,
				field: DbFields::Title,
				hidden_field_list,
				field_list,
				hidden_field_len,
				is_hidden: false,
			}),
			dyn_stack(
				move || field_list.get(),
				move |item| *item,
				move |field| {
					list_item(ListItem {
						id,
						field,
						hidden_field_list,
						field_list,
						hidden_field_len,
						is_hidden: false,
					})
					.style(|s| s.padding_bottom(5))
				},
			)
			.style(|s| s.margin_bottom(10).flex_col()),
			hidden_fields(HiddeFields {
				id,
				hidden_field_list,
				field_list,
				hidden_field_len,
				main_scroll_to,
			})
			.style(|s| s.margin_bottom(10)),
			new_field(id, field_presets, field_list, main_scroll_to),
		))
		.style(|s| s.gap(0, 5)),
	))
	.style(|s| {
		s.padding(8.0)
			.width_full()
			.max_width_full()
			.justify_content(AlignContent::Center)
			.align_items(AlignItems::Center)
	})
}
