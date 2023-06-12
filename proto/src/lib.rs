pub mod pb {
    pub mod blog {
        tonic::include_proto!("blog");
    }
    pub mod file {
        tonic::include_proto!("file.v1");
    }
    pub mod lottery {
        tonic::include_proto!("lottery");
    }
}

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
}
