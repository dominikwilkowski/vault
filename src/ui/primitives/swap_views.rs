use floem::{
	context,
	reactive::{as_child_of_current_scope, create_updater, Scope},
	IntoView, View, ViewId,
};

type ChildFn<T> = dyn Fn(T) -> (Box<dyn View>, Scope);

pub struct SwapViews<T: 'static> {
	id: ViewId,
	child_scope: Scope,
	child_fn: Box<ChildFn<T>>,
}

pub fn swap_views<CF: Fn(T) -> Box<dyn View> + 'static, T: 'static>(
	update_view: impl Fn() -> T + 'static,
	child_fn: CF,
) -> SwapViews<T> {
	let id = ViewId::new();

	let initial =
		create_updater(update_view, move |new_state| id.update_state(new_state));

	let child_fn =
		Box::new(as_child_of_current_scope(move |e| child_fn(e).into_any()));
	let (child, child_scope) = child_fn(initial);
	id.set_children(vec![child]);
	SwapViews {
		id,
		child_scope,
		child_fn,
	}
}

impl<T: 'static> View for SwapViews<T> {
	fn id(&self) -> ViewId {
		self.id
	}

	fn update(
		&mut self,
		cx: &mut context::UpdateCx,
		state: Box<dyn std::any::Any>,
	) {
		if let Ok(val) = state.downcast::<T>() {
			let old_child_scope = self.child_scope;
			for child in self.id.children() {
				cx.app_state_mut().remove_view(child);
			}
			let (child, child_scope) = (self.child_fn)(*val);
			self.child_scope = child_scope;
			self.id.set_children(vec![child]);
			old_child_scope.dispose();
			self.id.request_all();
		}
	}
}
