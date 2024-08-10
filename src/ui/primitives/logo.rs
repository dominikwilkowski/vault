use std::ops::Range;

use floem::{
	peniko::Color,
	text::{Attrs, AttrsList, FamilyOwned, Style, TextLayout},
	views::{rich_text, RichText},
};

const BANNER: &str = r"
_    _     _     _   _   _      _____  •
\ \ / /   /_\   | | | | | |    |_   _| •
 \ V /   / _ \  | |_| | | |__    | |   •
  \_/   /_/ \_\  \___/  |____|   |_|";

pub fn logo() -> RichText {
	rich_text(move || {
		let attrs_list = AttrsList::new(
			Attrs::new()
				.style(Style::Normal)
				.family(&[FamilyOwned::Monospace])
				.font_size(8.0),
		);

		let mut text_layout = TextLayout::new();
		text_layout.set_text(&BANNER.replace('•', ""), attrs_list);
		text_layout
	})
}

pub fn _rainbow_logo() -> RichText {
	let colors = [
		Color::rgb8(255, 0, 0),
		Color::rgb8(255, 0, 40),
		Color::rgb8(255, 0, 80),
		Color::rgb8(255, 0, 120),
		Color::rgb8(255, 0, 161),
		Color::rgb8(255, 0, 201),
		Color::rgb8(255, 0, 241),
		Color::rgb8(228, 0, 255),
		Color::rgb8(187, 0, 255),
		Color::rgb8(147, 0, 255),
		Color::rgb8(107, 0, 255),
		Color::rgb8(67, 0, 255),
		Color::rgb8(26, 0, 255),
		Color::rgb8(0, 13, 255),
		Color::rgb8(0, 53, 255),
		Color::rgb8(0, 93, 255),
		Color::rgb8(0, 134, 255),
		Color::rgb8(0, 174, 255),
		Color::rgb8(0, 214, 255),
		Color::rgb8(0, 255, 255),
		Color::rgb8(0, 255, 214),
		Color::rgb8(0, 255, 174),
		Color::rgb8(0, 255, 134),
		Color::rgb8(0, 255, 93),
		Color::rgb8(0, 255, 53),
		Color::rgb8(0, 255, 13),
		Color::rgb8(26, 255, 0),
		Color::rgb8(67, 255, 0),
		Color::rgb8(107, 255, 0),
		Color::rgb8(147, 255, 0),
		Color::rgb8(187, 255, 0),
		Color::rgb8(228, 255, 0),
		Color::rgb8(255, 241, 0),
		Color::rgb8(255, 201, 0),
		Color::rgb8(255, 161, 0),
		Color::rgb8(255, 120, 0),
		Color::rgb8(255, 80, 0),
		Color::rgb8(255, 40, 0),
		Color::rgb8(255, 0, 0),
	];

	rich_text(move || {
		let attrs = Attrs::new()
			.style(Style::Normal)
			.family(&[FamilyOwned::Monospace])
			.font_size(8.0);

		let mut attrs_list = AttrsList::new(attrs);
		let banner = &BANNER.replace('•', "");
		let exploded_lines = banner.split('\n').collect::<Vec<&str>>();
		let line_length = exploded_lines[1].len();
		let lines = exploded_lines.len();

		for (c, color_at_pos) in colors.iter().enumerate().take(line_length) {
			for l in 0..lines {
				let pos = c + (l * 40);
				if let Some(char_at_pos) = banner.chars().nth(pos) {
					if char_at_pos != ' ' {
						attrs_list.add_span(
							Range {
								start: pos,
								end: pos + 1,
							},
							Attrs::new()
								.color(*color_at_pos)
								.style(Style::Normal)
								.family(&[FamilyOwned::Monospace])
								.font_size(8.0),
						);
					}
				}
			}
		}

		let mut text_layout = TextLayout::new();
		text_layout.set_text(banner, attrs_list);
		text_layout
	})
}
