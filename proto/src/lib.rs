use empty_utils::errors::Error;
use std::ops::Deref;

#[allow(clippy::module_inception)]
pub mod pb {
    pub mod blog {
        pub mod blog {
            tonic::include_proto!("blog.blog");
        }
    }
    pub mod file {
        // tonic::include_proto!("file.v1");
    }
    pub mod lottery {
        pub mod lottery {
            tonic::include_proto!("lottery.lottery");
        }
        pub mod record {
            tonic::include_proto!("lottery.record");
        }
        pub mod template {
            tonic::include_proto!("lottery.template");
        }
        pub mod favorite {
            tonic::include_proto!("lottery.favorite");
        }
    }
    pub mod websocket {
        pub mod client {
            tonic::include_proto!("websocket.client");
        }
    }
    pub mod wx {
        pub mod wx {
            tonic::include_proto!("wx.wx");
        }
        pub mod user {
            tonic::include_proto!("wx.user");
        }
    }
    // pub mod oauth {
    //     pub mod oauth {
    //         tonic::include_proto!("oauth.oauth");
    //     }
    // }
    pub mod auth {
        pub mod auth {
            tonic::include_proto!("auth.auth");
        }
    }
    pub mod utils {
        pub mod paginate {
            tonic::include_proto!("utils.paginate");
        }
    }
    pub mod oss {
        pub mod oss {
            tonic::include_proto!("oss.oss");
        }
    }
}

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
}

pub mod model;
pub mod schema;
pub mod types;
pub mod utils;
