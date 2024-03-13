pub mod register {
    use xhartlet_application::common::error::ApplicationError;
    use xhartlet_application::use_cases::user::register as uc;

    pub type Error = ApplicationError;
    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = std::result::Result<Response, Error>;
}
