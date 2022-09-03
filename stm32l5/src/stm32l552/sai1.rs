#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    pub gcr: GCR,
    #[doc = "0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    pub ch: [CH; 2],
    #[doc = "0x44 - PDM control register"]
    pub pdmcr: PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub pdmdly: PDMDLY,
}
impl RegisterBlock {
    #[doc = "0x04..0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub fn cha(&self) -> &CH {
        &self.ch[0]
    }
    #[doc = "0x24..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
    #[inline(always)]
    pub fn chb(&self) -> &CH {
        &self.ch[1]
    }
}
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub use ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR"]
pub mod ch;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global configuration register"]
pub mod gcr;
#[doc = "PDMCR (rw) register accessor: an alias for `Reg<PDMCR_SPEC>`"]
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDMDLY (rw) register accessor: an alias for `Reg<PDMDLY_SPEC>`"]
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
#[doc = "PDM delay register"]
pub mod pdmdly;
