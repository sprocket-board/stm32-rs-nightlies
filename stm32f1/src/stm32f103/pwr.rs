#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CR)"]
    pub cr: CR,
    #[doc = "0x04 - Power control register (PWR_CR)"]
    pub csr: CSR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Power control register (PWR_CR)"]
pub mod cr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Power control register (PWR_CR)"]
pub mod csr;
