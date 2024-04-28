use crate::common::abstract_present::Present;
use crate::user::model::register::Result;

#[derive(Debug, Default)]
pub struct Presenter;

mod cli {
    use super::*;

    impl Present<Result> for Presenter {
        type ViewModel = String;

        fn present(&self, result: Result) -> Self::ViewModel {
            match result {
                Ok(_) => "User created".to_string(),
                Err(e) => format!("Error: {:?}", e),
            }
        }
    }
}
