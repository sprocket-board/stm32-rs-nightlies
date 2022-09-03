#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DFSDM channel configuration 0 register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x04 - DFSDM channel configuration 0 register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x08 - DFSDM analog watchdog and short-circuit detector register"]
    pub awscdr: AWSCDR,
    #[doc = "0x0c - DFSDM channel watchdog filter data register"]
    pub wdatr: WDATR,
    #[doc = "0x10 - DFSDM channel data input register"]
    pub datinr: DATINR,
}
#[doc = "CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "DFSDM channel configuration 0 register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "DFSDM channel configuration 0 register 2"]
pub mod cfgr2;
#[doc = "AWSCDR (rw) register accessor: an alias for `Reg<AWSCDR_SPEC>`"]
pub type AWSCDR = crate::Reg<awscdr::AWSCDR_SPEC>;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod awscdr;
#[doc = "WDATR (r) register accessor: an alias for `Reg<WDATR_SPEC>`"]
pub type WDATR = crate::Reg<wdatr::WDATR_SPEC>;
#[doc = "DFSDM channel watchdog filter data register"]
pub mod wdatr;
#[doc = "DATINR (rw) register accessor: an alias for `Reg<DATINR_SPEC>`"]
pub type DATINR = crate::Reg<datinr::DATINR_SPEC>;
#[doc = "DFSDM channel data input register"]
pub mod datinr;
