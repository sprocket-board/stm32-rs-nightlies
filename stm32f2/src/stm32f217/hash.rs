#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - data input register"]
    pub din: DIN,
    #[doc = "0x08 - start register"]
    pub str: STR,
    #[doc = "0x0c..0x20 - digest registers"]
    pub hr: [HR; 5],
    #[doc = "0x20 - interrupt enable register"]
    pub imr: IMR,
    #[doc = "0x24 - status register"]
    pub sr: SR,
    _reserved6: [u8; 0xd0],
    #[doc = "0xf8..0x1c4 - context swap registers"]
    pub csr: [CSR; 51],
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "STR (w) register accessor: an alias for `Reg<STR_SPEC>`"]
pub type STR = crate::Reg<str::STR_SPEC>;
#[doc = "start register"]
pub mod str;
#[doc = "HR (r) register accessor: an alias for `Reg<HR_SPEC>`"]
pub type HR = crate::Reg<hr::HR_SPEC>;
#[doc = "digest registers"]
pub mod hr;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "context swap registers"]
pub mod csr;
