use floem::{
	event::{Event, EventListener},
	peniko::Color,
	reactive::RwSignal,
	style::{AlignItems, BoxShadowProp, CursorStyle, Display, Position},
	view::View,
	views::{label, svg, v_stack, Decorators},
	EventPropagation,
};

use crate::ui::{
	colors::*,
	primitives::{que::Que, styles, tooltip::TooltipSignals},
	settings::settings_view::Tabs,
};

pub fn tab_button(
	icon: String,
	this_tab: Tabs,
	tabs: RwSignal<im::Vector<Tabs>>,
	active_tab: RwSignal<usize>,
) -> impl View {
	let width = 75;
	v_stack((
		svg(move || icon.clone()).style(|s| s.width(30).height(30)),
		label(move || this_tab).style(|s| s.justify_center()),
		label(move || "").style(move |s| {
			s.position(Position::Absolute)
				.z_index(5)
				.width(width - 2)
				.height(3)
				.inset_left(0)
				.inset_top(55)
				.background(C_MAIN_BG)
				.display(Display::None)
				.apply_if(
					active_tab.get()
						== tabs
							.get_untracked()
							.iter()
							.position(|it| *it == this_tab)
							.unwrap(),
					|s| s.display(Display::Flex),
				)
		}),
	))
	.keyboard_navigatable()
	.on_click_stop(move |_| {
		active_tab.update(|v: &mut usize| {
			*v = tabs.get_untracked().iter().position(|it| *it == this_tab).unwrap();
		});
	})
	.style(move |s| {
		s.flex()
			.width(width)
			.height(52)
			.align_items(AlignItems::Center)
			.background(C_TOP_BG)
			.border_radius(6)
			.padding(3)
			.gap(0, 2.0)
			.border(1)
			.border_color(C_TOP_BG)
			.focus_visible(|s| s.outline(1).outline_color(C_FOCUS))
			.hover(|s| {
				s.background(C_MAIN_BG)
					.cursor(CursorStyle::Pointer)
					.border_color(C_MAIN_BG)
			})
			.apply_if(
				active_tab.get()
					== tabs
						.get_untracked()
						.iter()
						.position(|it| *it == this_tab)
						.unwrap(),
				|s| {
					s.background(C_MAIN_BG)
						.height(63)
						.padding_top(6)
						.padding_bottom(11)
						.inset_top(0)
						.border_color(C_TOP_BG_BORDER)
						.hover(|s| s.border_color(C_TOP_BG_BORDER))
				},
			)
	})
}

pub enum ButtonVariant {
	Default,
	Tiny,
}

pub struct IconButton {
	pub variant: ButtonVariant,
	pub icon: String,
	pub icon2: Option<String>,
	pub bubble: Option<RwSignal<usize>>,
	pub tooltip: String,
	pub tooltip2: Option<String>,
	pub switch: Option<RwSignal<bool>>,
	pub tooltip_signals: TooltipSignals,
}

impl Default for IconButton {
	fn default() -> Self {
		Self {
			variant: ButtonVariant::Default,
			icon: String::from(""),
			icon2: None,
			bubble: None,
			tooltip: String::from(""),
			tooltip2: None,
			switch: None,
			tooltip_signals: TooltipSignals::new(Que::default()),
		}
	}
}

pub fn icon_button(
	param: IconButton,
	on_click: impl Fn(&Event) + 'static,
) -> impl View {
	let IconButton {
		variant,
		icon,
		icon2,
		bubble,
		tooltip,
		tooltip2,
		switch,
		tooltip_signals,
	} = param;

	let tooltip_c = tooltip.clone();
	let tooltip2_c = tooltip2.clone();

	let is_tiny = matches!(&variant, &ButtonVariant::Tiny);

	let bubble_view = if bubble.is_some() {
		let notification_icon = include_str!("../icons/notification.svg");

		v_stack((v_stack((
			svg(move || String::from(notification_icon))
				.style(move |s| s.height(10).width(10)),
			label(move || {
				if bubble.unwrap().get() < 100 {
					format!("{}", bubble.unwrap().get())
				} else {
					String::from("x")
				}
			})
			.style(move |s| {
				let right = if bubble.unwrap().get() < 10 {
					-2.5
				} else if bubble.unwrap().get() < 100 {
					-0.5
				} else {
					-2.5
				};

				s.color(C_MAIN_TEXT)
					.height(8)
					.width(10)
					.font_size(8.0)
					.position(Position::Absolute)
					.inset_top(0)
					.inset_right(right)
			}),
		)),))
		.style(move |s| {
			s.position(Position::Absolute)
				.inset_top(0)
				.inset_right(0)
				.apply_if(is_tiny, |s| s.inset_top(-3).inset_right(-5))
		})
	} else {
		v_stack((label(|| "").style(|s| s.display(Display::None)),))
	};

	v_stack((
		svg(move || {
			if let (Some(icon2), Some(switch)) = (icon2.as_ref(), switch.as_ref()) {
				if switch.get() {
					icon2.clone()
				} else {
					icon.clone()
				}
			} else {
				icon.clone()
			}
		})
		.style(move |s| {
			s.height(17).width(17).apply_if(is_tiny, |s| s.width(12).height(12))
		}),
		bubble_view,
	))
	.keyboard_navigatable()
	.style(styles::button)
	.style(move |s| {
		s.margin_left(0)
			.margin_right(1.5)
			.hover(|s| s.apply_if(is_tiny, |s| s.background(Color::TRANSPARENT)))
			.apply_if(is_tiny, |s| s.border(0).set(BoxShadowProp, None))
	})
	.on_event(EventListener::PointerEnter, move |_| {
		if let (Some(tooltip2), Some(switch)) = (tooltip2.as_ref(), switch.as_ref())
		{
			if switch.get() {
				tooltip_signals.show(tooltip2.clone());
			} else {
				tooltip_signals.show(tooltip.clone());
			}
		} else {
			tooltip_signals.show(tooltip.clone());
		}
		EventPropagation::Continue
	})
	.on_event(EventListener::PointerLeave, move |_| {
		tooltip_signals.hide();
		EventPropagation::Continue
	})
	.on_click(move |event| {
		if let (Some(tooltip2_c), Some(switch)) =
			(tooltip2_c.as_ref(), switch.as_ref())
		{
			switch.set(!switch.get());

			if switch.get() {
				tooltip_signals.tooltip_text.set(tooltip2_c.clone());
			} else {
				tooltip_signals.tooltip_text.set(tooltip_c.clone());
			}
		}
		on_click(event);
		EventPropagation::Continue
	})
}

pub fn button(button_label: &'static str) -> impl View {
	label(move || String::from(button_label))
		.keyboard_navigatable()
		.style(styles::button)
}
