use crate::state::{settings_editor::State, Message};
use iced::{Column, Element};

pub fn build(state: &mut State) -> Element<Message> {
	Column::new().into()
}
