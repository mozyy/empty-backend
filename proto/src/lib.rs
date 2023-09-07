use empty_utils::errors::Error;
use std::ops::Deref;

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
    pub mod oss {
        pub mod oss {
            tonic::include_proto!("oss.oss");
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

#[derive(Clone)]
pub struct UserId(pb::oauth::oauth::ResourceResponse);

impl<T> TryFrom<&tonic::Request<T>> for UserId {
    type Error = Error;

    fn try_from(value: &tonic::Request<T>) -> Result<Self, Self::Error> {
        let user_id = value
            .extensions()
            .get::<UserId>()
            .ok_or_else(|| Error::StatusError(tonic::Status::unauthenticated("no auth")))?;
        Ok(user_id.to_owned())
    }
}

impl UserId {
    pub fn new(res: pb::oauth::oauth::ResourceResponse) -> Self {
        Self(res)
    }
}

impl Deref for UserId {
    type Target = pb::oauth::oauth::ResourceResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.owner_id.clone()
    }
}
