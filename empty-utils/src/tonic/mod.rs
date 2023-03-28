pub type Resp<T> = core::result::Result<tonic::Response<T>, tonic::Status>;

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
// impl<T> std::ops::Try for Response<T> {
//     type Output = Resp<T>;

//     type Residual = Error;

//     fn from_output(output: Self::Output) -> Self {
//         todo!()
//     }

//     fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
//         todo!()
//     }
// }
