use floem::{
	peniko::Color,
	reactive::{create_rw_signal, RwSignal},
	views::{container, dyn_stack, label, Decorators},
	IntoView,
};

fn sortable_item(
	view: impl IntoView + 'static,
	sortable_items: RwSignal<Vec<usize>>,
	dragger_id: RwSignal<usize>,
	item_id: usize,
) -> impl IntoView {
	container(view)
		.style(|s| s.background(Color::WHITE).selectable(false))
		.draggable()
		.on_event(floem::event::EventListener::DragStart, move |_| {
			dragger_id.set(item_id);
			floem::event::EventPropagation::Continue
		})
		.on_event(floem::event::EventListener::DragOver, move |_| {
			sort_items(sortable_items, dragger_id.get_untracked(), item_id);
			floem::event::EventPropagation::Continue
		})
}

fn sort_items(
	sortable_items: RwSignal<Vec<usize>>,
	dragger_id: usize,
	hover_id: usize,
) {
	if dragger_id != hover_id {
		let dragger_pos =
			sortable_items.get().iter().position(|id| *id == dragger_id).unwrap();
		let hover_pos =
			sortable_items.get().iter().position(|id| *id == hover_id).unwrap();

		sortable_items.update(|items| {
			items.remove(dragger_pos);
			items.insert(hover_pos, dragger_id);
		});
	}
}

#[test]
fn sort_items_test() {
	// 0,1,2,3,4
	let sortable_items = create_rw_signal((0..5).collect::<Vec<usize>>());
	sort_items(sortable_items, 2, 1);
	assert_eq!(sortable_items.get(), vec![0, 2, 1, 3, 4]);

	let sortable_items = create_rw_signal((0..5).collect::<Vec<usize>>());
	sort_items(sortable_items, 4, 1);
	assert_eq!(sortable_items.get(), vec![0, 4, 1, 2, 3]);

	let sortable_items = create_rw_signal((0..5).collect::<Vec<usize>>());
	sort_items(sortable_items, 0, 2);
	assert_eq!(sortable_items.get(), vec![1, 2, 0, 3, 4]);

	let sortable_items = create_rw_signal((0..5).collect::<Vec<usize>>());
	sort_items(sortable_items, 2, 2);
	assert_eq!(sortable_items.get(), vec![0, 1, 2, 3, 4]);
}

fn sortable<V: IntoView + 'static>(
	items: Vec<impl Fn() -> V + 'static>,
) -> impl IntoView {
	let sortable_items =
		create_rw_signal((0..items.len()).collect::<Vec<usize>>());
	let dragger_id = create_rw_signal(0);

	dyn_stack(
		move || sortable_items.get(),
		move |item_id| *item_id,
		move |item_id| {
			sortable_item(items[item_id](), sortable_items, dragger_id, item_id)
		},
	)
	.into_any()
}

//////

fn my_view(name: &str) -> impl IntoView {
	let name = String::from(name);
	label(move || name.clone()).style(|s| {
		s.padding(5).selectable(false).border(2).border_color(Color::RED)
	})
}

fn app_view() -> impl IntoView {
	sortable(vec![
		|| my_view("line zero"),
		|| my_view("line one"),
		|| my_view("line two"),
		|| my_view("line three"),
		|| my_view("line four"),
		|| my_view("line five"),
		|| my_view("line six"),
		|| my_view("line seven"),
		|| my_view("line eight"),
		|| my_view("line nine"),
		|| my_view("line ten"),
	])
	.style(|s| s.width_full().flex_col())
}

fn main() {
	floem::launch(app_view);
}
