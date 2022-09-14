pub mod types {
    tonic::include_proto!("types");
}

pub mod proto {
    pub mod user {
        pub mod oauth {
            tonic::include_proto!("user.oauth.v1");
        }
    }
}
