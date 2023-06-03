use crate::{error::AppError, input::routes::Input, output::routes::Output};

pub trait GoogleMapClient {
    fn routes(&self, input: Input) -> Result<Output, AppError>;
}
