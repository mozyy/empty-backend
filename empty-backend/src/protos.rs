macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!(
            env!("OUT_DIR"),
            // "/workspaces/empty-backend/empty-backend/src/protos",
            concat!("/", $package, ".rs")
        ));
    };
}

pub mod types {
    include_proto!("types");
}

pub mod proto {
    pub mod user {
        pub mod oauth {
            include_proto!("user.oauth.v1");
        }
    }
}
