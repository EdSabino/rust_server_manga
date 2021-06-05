use crate::eventbus::Event;

pub struct DeleteImageEvent {
    pub name: String
}

impl DeleteImageEvent {
    pub fn new(name: String) -> Self {
        DeleteImageEvent {
            name: name
        }
    }
}

impl Event for DeleteImageEvent {}