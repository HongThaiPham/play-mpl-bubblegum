use anchor_lang::{prelude::Pubkey, *};
#[derive(Clone)]
pub struct MplBubblegum;
impl Id for MplBubblegum {
    fn id() -> Pubkey {
        pubkey!("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY")
    }
}

#[derive(Clone)]
pub struct Noop;
impl Id for Noop {
    fn id() -> Pubkey {
        pubkey!("mnoopTCrg4p8ry25e4bcWA9XZjbNjMTfgYVGGEdRsf3")
    }
}

#[derive(Clone)]
pub struct SplAccountCompression;
impl Id for SplAccountCompression {
    fn id() -> Pubkey {
        pubkey!("mcmt6YrQEMKw8Mw43FmpRLmf7BqRnFMKmAcbxE3xkAW")
    }
}

#[derive(Clone)]
pub struct MplCore;
impl Id for MplCore {
    fn id() -> Pubkey {
        pubkey!("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")
    }
}
