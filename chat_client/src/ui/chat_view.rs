use cursive::{
    View, view::{Nameable, Resizable}, views::{LinearLayout, ScrollView},
};

pub struct ChatView {
    pub name: String
}

impl ChatView {
    pub fn create_view(&self) -> Box<dyn View> {
        let layout = 
            LinearLayout::vertical()
            .with_name(&self.name);

        let scroll_view = 
            ScrollView::new(layout)
            .scroll_strategy(cursive::view::ScrollStrategy::StickToBottom)
            .full_width()
            .full_height();

        Box::new(scroll_view)
    }

}
