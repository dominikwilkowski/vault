use floem::cosmic_text::{Attrs, AttrsList, FamilyOwned, Style, TextLayout};
use floem::peniko::Color;
use floem::views::{rich_text, RichText};
// use std::ops::Range;

const BANNER: &str = "
 __   __    _     _   _   _      _____
 \\ \\ / /   /_\\   | | | | | |    |_   _|
  \\ V /   / _ \\  | |_| | | |__    | |
   \\_/   /_/ \\_\\  \\___/  |____|   |_|
   ";

pub fn logo_text_layout() -> RichText {
	rich_text(move || {
		let attrs = Attrs::new()
			.color(Color::RED)
			.style(Style::Normal)
			.family(&[FamilyOwned::Monospace]);

		let attrs_list = AttrsList::new(attrs);
		//
		// attrs_list.add_span(
		// 	Range { start: 56, end: 73 },
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range { start: 78, end: 79 },
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 96,
		// 		end: 100,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 101,
		// 		end: 108,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 110,
		// 		end: 118,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 143,
		// 		end: 144,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 148,
		// 		end: 149,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 152,
		// 		end: 153,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );
		//
		// attrs_list.add_span(
		// 	Range {
		// 		start: 154,
		// 		end: 155,
		// 	},
		// 	Attrs::new()
		// 		.color(Color::RED)
		// 		.style(Style::Normal)
		// 		.family(&[FamilyOwned::Monospace]),
		// );

		let mut tl = TextLayout::new();
		tl.set_text(BANNER, attrs_list);
		tl
	})
}
