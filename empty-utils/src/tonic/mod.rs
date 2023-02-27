use tonic::Status;

pub type Resp<T> = core::result::Result<tonic::Response<T>, Status>;

pub struct Response<T>(pub T);

impl<T> Response<T> {
    pub fn new(value: T) -> Self {
        Response(value)
    }
}

impl<T> From<Response<T>> for Resp<T> {
    fn from(value: Response<T>) -> Self {
        Ok(tonic::Response::new(value.0))
    }
}
