pub mod pb {
    pub mod blog {
        pub mod blog {
            // tonic::include_proto!("blog");
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
    }
    pub mod wx {
        pub mod wx {
            tonic::include_proto!("wx.wx");
        }
        pub mod user {
            tonic::include_proto!("wx.user");
        }
    }
    pub mod oauth {
        pub mod oauth {
            tonic::include_proto!("oauth.oauth");
        }
    }
    pub mod utils {
        pub mod paginate {
            tonic::include_proto!("utils.paginate");
        }
    }
}

pub mod model;
pub mod schema;
pub mod types;
pub mod utils;

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
}
