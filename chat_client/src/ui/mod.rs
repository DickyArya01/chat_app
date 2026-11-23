pub mod chat_input;
pub mod chat_view;
pub mod exit_dialog;

use cursive::{
    Cursive,
    event::{Event, Key::Esc},
    views::{Dialog, EditView, LinearLayout, TextView},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::ui::chat_view::ChatView;

pub struct ChatInterface {}

impl ChatInterface {
    pub fn init_chat(&self, tx: UnboundedSender<String>) -> Cursive {
        let mut c = Cursive::default();

        let mut root_layout = LinearLayout::vertical();
        let chat_view = ChatView {
            name: String::from("chat_view"),
        };

        root_layout.add_child(chat_view.create_view());

        root_layout.add_child(chat_input::get(move |c: &mut Cursive, text: &str| {
            on_submit(c, text, &tx);
        }));

        c.add_fullscreen_layer(root_layout);

        c.set_global_callback(Event::Key(Esc), |s: &mut Cursive| {
            if s.find_name::<Dialog>("exit_dialog")
                .is_none()
            {
                s.add_layer(exit_dialog::get());
            }
        });

        c
    }
}

pub fn on_submit(c: &mut Cursive, text: &str, tx: &UnboundedSender<String>) {
    match text.trim() {
        "" => {}
        "/quit" => c.quit(),
        "/help" => write_msg_view(c, "/quit: Close chat\n/help: Show help"),
        msg => {
            write_msg_view(c, msg);

            if tx.send(msg.to_string()).is_err() {
                write_msg_view(c, "not connected");
            }
        }
    }

    c.call_on_name("chat_input", |v: &mut EditView| {
        v.set_content("");
    });
}

pub fn write_msg_view(c: &mut Cursive, text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }

    c.call_on_name("chat_view", |v: &mut LinearLayout| {
        v.add_child(TextView::new(text));
    });
}
