#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x04 - CEC own address register"]
    pub oar: OAR,
    #[doc = "0x08 - Rx Data Register"]
    pub pres: PRES,
    #[doc = "0x0c - CEC error status register"]
    pub esr: ESR,
    #[doc = "0x10 - CEC control and status register"]
    pub csr: CSR,
    #[doc = "0x14 - CEC Tx data register"]
    pub txd: TXD,
    #[doc = "0x18 - CEC Rx data register"]
    pub rxd: RXD,
}
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "OAR (rw) register accessor: an alias for `Reg<OAR_SPEC>`"]
pub type OAR = crate::Reg<oar::OAR_SPEC>;
#[doc = "CEC own address register"]
pub mod oar;
#[doc = "PRES (rw) register accessor: an alias for `Reg<PRES_SPEC>`"]
pub type PRES = crate::Reg<pres::PRES_SPEC>;
#[doc = "Rx Data Register"]
pub mod pres;
#[doc = "ESR (r) register accessor: an alias for `Reg<ESR_SPEC>`"]
pub type ESR = crate::Reg<esr::ESR_SPEC>;
#[doc = "CEC error status register"]
pub mod esr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "CEC control and status register"]
pub mod csr;
#[doc = "TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "CEC Tx data register"]
pub mod txd;
#[doc = "RXD (r) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "CEC Rx data register"]
pub mod rxd;
