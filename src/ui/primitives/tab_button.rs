use floem::{
	reactive::{ReadSignal, WriteSignal},
	style::{AlignItems, CursorStyle},
	view::View,
	views::{label, svg, v_stack, Decorators},
};

use crate::ui::colors::*;
use crate::ui::settings_view::Tabs;

pub fn tab_button(
	icon: String,
	this_tab: Tabs,
	tabs: ReadSignal<im::Vector<Tabs>>,
	set_active_tab: WriteSignal<usize>,
	active_tab: ReadSignal<usize>,
) -> impl View {
	v_stack((
		svg(move || icon.clone()).style(|s| s.width(30).height(30)),
		label(move || this_tab).style(|s| s.justify_center().margin_top(2)),
	))
	.on_click_stop(move |_| {
		set_active_tab.update(|v: &mut usize| {
			*v = tabs.get_untracked().iter().position(|it| *it == this_tab).unwrap();
		});
	})
	.style(move |s| {
		s.flex()
			.width(58)
			.align_items(AlignItems::Center)
			.background(C_BG_SIDE_SELECTED.with_alpha_factor(0.2))
			.border_radius(3)
			.padding(3)
			.hover(|s| s.background(C_BG_MAIN).cursor(CursorStyle::Pointer))
			.apply_if(
				active_tab.get()
					== tabs
						.get_untracked()
						.iter()
						.position(|it| *it == this_tab)
						.unwrap(),
				|s| s.background(C_BG_MAIN),
			)
	})
}
