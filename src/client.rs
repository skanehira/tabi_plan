use crate::{
    error::AppError,
    routes::{Input, Output},
};

pub trait GoogleMapClient {
    fn routes(&self, input: Input) -> Result<Output, AppError>;
}
