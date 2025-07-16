// Generated protobuf modules

// Regen modules
pub mod regen {
    pub mod data {
        pub mod v1 {
            include!("regen.data.v1.rs");
        }
        pub mod v2 {
            include!("regen.data.v2.rs");
        }
    }

    pub mod ecocredit {
        pub mod v1 {
            include!("regen.ecocredit.v1.rs");
        }
        pub mod v1alpha1 {
            include!("regen.ecocredit.v1alpha1.rs");
        }
        pub mod basket {
            pub mod v1 {
                include!("regen.ecocredit.basket.v1.rs");
            }
        }
        pub mod marketplace {
            pub mod v1 {
                include!("regen.ecocredit.marketplace.v1.rs");
            }
        }
        pub mod orderbook {
            pub mod v1alpha1 {
                include!("regen.ecocredit.orderbook.v1alpha1.rs");
            }
        }
    }

    pub mod intertx {
        pub mod v1 {
            include!("regen.intertx.v1.rs");
        }
    }
}

// Re-export for convenience
pub use regen::*;
