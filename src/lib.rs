//! Versioned gRPC contracts shared by cogate services.

pub mod cogate {
    pub mod cotab {
        pub mod v1 {
            tonic::include_proto!("cogate.cotab.v1");
        }
    }

    pub mod notify {
        pub mod v1 {
            tonic::include_proto!("cogate.notify.v1");
        }
    }
}

pub mod rusti2 {
    pub mod v1 {
        tonic::include_proto!("rusti2.v1");
    }
}
