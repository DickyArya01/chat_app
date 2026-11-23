use cursive::{
    traits::{Nameable, Resizable},
    views::{EditView, NamedView, ResizedView},
    Cursive,
};

pub fn get<F>(on_submit: F) -> ResizedView<ResizedView<NamedView<EditView>>>
where
    F: Fn(&mut Cursive, &str) + Send + Sync +'static,
{
    EditView::new()
        .on_submit(on_submit)
        .with_name("chat_input")
        .fixed_height(1)
        .full_width()
}