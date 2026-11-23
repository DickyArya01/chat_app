use cursive::{Cursive, View, view::{Nameable, Resizable}, views::{Dialog, TextView}};

pub fn get() -> Box<dyn View> {
    let mut dialog = Dialog::new()
        .title("Exit App")
        .dismiss_button("No")
        .button("Yes", |c: &mut Cursive| {
            c.quit();
        })
        .content(TextView::new("Are you sure you want to exit"))
        .with_name("exit_dialog")
        .full_width();

    dialog.set_height(cursive::view::SizeConstraint::AtLeast(10));

    Box::new(dialog)
}
