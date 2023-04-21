pub mod pb {
    pub mod blog {
        tonic::include_proto!("blog.v1");
    }
    pub mod file {
        tonic::include_proto!("file.v1");
    }
}

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
}
