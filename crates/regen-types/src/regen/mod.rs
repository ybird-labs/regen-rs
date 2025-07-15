pub mod data_client;
pub mod eco_credit_client;

pub mod data {
    pub mod v2 {
        include!("../gen/regen.data.v2.rs");
    }
}

pub mod ecocredit {
    pub mod v1 {
        include!("../gen/regen.ecocredit.v1.rs");
    }
}
