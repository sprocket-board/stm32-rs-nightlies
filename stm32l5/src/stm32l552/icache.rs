#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ICACHE control register"]
    pub icache_cr: ICACHE_CR,
    #[doc = "0x04 - ICACHE status register"]
    pub icache_sr: ICACHE_SR,
    #[doc = "0x08 - ICACHE interrupt enable register"]
    pub icache_ier: ICACHE_IER,
    #[doc = "0x0c - ICACHE flag clear register"]
    pub icache_fcr: ICACHE_FCR,
    #[doc = "0x10 - ICACHE hit monitor register"]
    pub icache_hmonr: ICACHE_HMONR,
    #[doc = "0x14 - ICACHE miss monitor register"]
    pub icache_mmonr: ICACHE_MMONR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - ICACHE region configuration register"]
    pub icache_crr0: ICACHE_CRR0,
    #[doc = "0x24 - ICACHE region configuration register"]
    pub icache_crr1: ICACHE_CRR1,
    #[doc = "0x28 - ICACHE region configuration register"]
    pub icache_crr2: ICACHE_CRR2,
    #[doc = "0x2c - ICACHE region configuration register"]
    pub icache_crr3: ICACHE_CRR3,
}
#[doc = "ICACHE_CR (rw) register accessor: an alias for `Reg<ICACHE_CR_SPEC>`"]
pub type ICACHE_CR = crate::Reg<icache_cr::ICACHE_CR_SPEC>;
#[doc = "ICACHE control register"]
pub mod icache_cr;
#[doc = "ICACHE_SR (r) register accessor: an alias for `Reg<ICACHE_SR_SPEC>`"]
pub type ICACHE_SR = crate::Reg<icache_sr::ICACHE_SR_SPEC>;
#[doc = "ICACHE status register"]
pub mod icache_sr;
#[doc = "ICACHE_IER (rw) register accessor: an alias for `Reg<ICACHE_IER_SPEC>`"]
pub type ICACHE_IER = crate::Reg<icache_ier::ICACHE_IER_SPEC>;
#[doc = "ICACHE interrupt enable register"]
pub mod icache_ier;
#[doc = "ICACHE_FCR (w) register accessor: an alias for `Reg<ICACHE_FCR_SPEC>`"]
pub type ICACHE_FCR = crate::Reg<icache_fcr::ICACHE_FCR_SPEC>;
#[doc = "ICACHE flag clear register"]
pub mod icache_fcr;
#[doc = "ICACHE_HMONR (r) register accessor: an alias for `Reg<ICACHE_HMONR_SPEC>`"]
pub type ICACHE_HMONR = crate::Reg<icache_hmonr::ICACHE_HMONR_SPEC>;
#[doc = "ICACHE hit monitor register"]
pub mod icache_hmonr;
#[doc = "ICACHE_MMONR (r) register accessor: an alias for `Reg<ICACHE_MMONR_SPEC>`"]
pub type ICACHE_MMONR = crate::Reg<icache_mmonr::ICACHE_MMONR_SPEC>;
#[doc = "ICACHE miss monitor register"]
pub mod icache_mmonr;
#[doc = "ICACHE_CRR0 (rw) register accessor: an alias for `Reg<ICACHE_CRR0_SPEC>`"]
pub type ICACHE_CRR0 = crate::Reg<icache_crr0::ICACHE_CRR0_SPEC>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr0;
#[doc = "ICACHE_CRR1 (rw) register accessor: an alias for `Reg<ICACHE_CRR1_SPEC>`"]
pub type ICACHE_CRR1 = crate::Reg<icache_crr1::ICACHE_CRR1_SPEC>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr1;
#[doc = "ICACHE_CRR2 (rw) register accessor: an alias for `Reg<ICACHE_CRR2_SPEC>`"]
pub type ICACHE_CRR2 = crate::Reg<icache_crr2::ICACHE_CRR2_SPEC>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr2;
#[doc = "ICACHE_CRR3 (rw) register accessor: an alias for `Reg<ICACHE_CRR3_SPEC>`"]
pub type ICACHE_CRR3 = crate::Reg<icache_crr3::ICACHE_CRR3_SPEC>;
#[doc = "ICACHE region configuration register"]
pub mod icache_crr3;
