#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x04 - OPAMP2 control/status register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x08 - OPAMP3 control/status register"]
    pub opamp3_csr: OPAMP3_CSR,
    _reserved3: [u8; 0x0c],
    #[doc = "0x18 - OPAMP1 control/status register"]
    pub opamp1_tcmr: OPAMP1_TCMR,
    #[doc = "0x1c - OPAMP2 control/status register"]
    pub opamp2_tcmr: OPAMP2_TCMR,
    #[doc = "0x20 - OPAMP3 control/status register"]
    pub opamp3_tcmr: OPAMP3_TCMR,
}
#[doc = "OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: an alias for `Reg<OPAMP3_CSR_SPEC>`"]
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSR_SPEC>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_csr;
#[doc = "OPAMP1_TCMR (rw) register accessor: an alias for `Reg<OPAMP1_TCMR_SPEC>`"]
pub type OPAMP1_TCMR = crate::Reg<opamp1_tcmr::OPAMP1_TCMR_SPEC>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_tcmr;
#[doc = "OPAMP2_TCMR (rw) register accessor: an alias for `Reg<OPAMP2_TCMR_SPEC>`"]
pub type OPAMP2_TCMR = crate::Reg<opamp2_tcmr::OPAMP2_TCMR_SPEC>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_tcmr;
#[doc = "OPAMP3_TCMR (rw) register accessor: an alias for `Reg<OPAMP3_TCMR_SPEC>`"]
pub type OPAMP3_TCMR = crate::Reg<opamp3_tcmr::OPAMP3_TCMR_SPEC>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_tcmr;
