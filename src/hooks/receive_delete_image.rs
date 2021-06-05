use crate::domain::entities::events::DeleteImageEvent;
use crate::domain::usecases::images::DeleteImageUseCase;

pub fn handler(message: &mut DeleteImageEvent) {
    DeleteImageUseCase::new().call(message).unwrap();
}