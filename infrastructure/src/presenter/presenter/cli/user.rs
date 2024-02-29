use crate::presenter::{model::user::register::Result, presenter::Present};

#[derive(Debug, Default)]
pub struct Presenter;

impl Present<Result> for Presenter {
    type ViewModel = String;

    fn present(&self, result: Result) -> Self::ViewModel {
        match result {
            Ok(_) => "User created".to_string(),
            Err(e) => format!("Error: {:?}", e),
        }
    }
}
