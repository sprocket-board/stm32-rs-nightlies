#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - software trigger register"]
    pub swtrgr: SWTRGR,
    #[doc = "0x08 - channel1 12-bit right-aligned data holding register"]
    pub dhr12r1: DHR12R1,
    #[doc = "0x0c - channel1 12-bit left aligned data holding register"]
    pub dhr12l1: DHR12L1,
    #[doc = "0x10 - channel1 8-bit right aligned data holding register"]
    pub dhr8r1: DHR8R1,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dhr12rd: DHR12RD,
    #[doc = "0x24 - Dual DAC 12-bit left aligned data holding register"]
    pub dhr12ld: DHR12LD,
    #[doc = "0x28 - Dual DAC 8-bit right aligned data holding register"]
    pub dhr8rd: DHR8RD,
    #[doc = "0x2c - DAC channel1 data output register"]
    pub dor1: DOR1,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - status register"]
    pub sr: SR,
    #[doc = "0x38 - calibration control register"]
    pub ccr: CCR,
    #[doc = "0x3c - mode control register"]
    pub mcr: MCR,
    #[doc = "0x40 - Sample and Hold sample time register 1"]
    pub shsr1: SHSR1,
    _reserved13: [u8; 0x04],
    #[doc = "0x48 - Sample and Hold hold time register"]
    pub shhr: SHHR,
    #[doc = "0x4c - Sample and Hold refresh time register"]
    pub shrr: SHRR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: an alias for `Reg<SWTRGR_SPEC>`"]
pub type SWTRGR = crate::Reg<swtrgr::SWTRGR_SPEC>;
#[doc = "software trigger register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: an alias for `Reg<DHR12R1_SPEC>`"]
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
#[doc = "channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: an alias for `Reg<DHR12L1_SPEC>`"]
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
#[doc = "channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: an alias for `Reg<DHR8R1_SPEC>`"]
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
#[doc = "channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12RD (rw) register accessor: an alias for `Reg<DHR12RD_SPEC>`"]
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: an alias for `Reg<DHR12LD_SPEC>`"]
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
#[doc = "Dual DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: an alias for `Reg<DHR8RD_SPEC>`"]
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
#[doc = "Dual DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: an alias for `Reg<DOR1_SPEC>`"]
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "calibration control register"]
pub mod ccr;
#[doc = "MCR (rw) register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "mode control register"]
pub mod mcr;
#[doc = "SHSR1 (rw) register accessor: an alias for `Reg<SHSR1_SPEC>`"]
pub type SHSR1 = crate::Reg<shsr1::SHSR1_SPEC>;
#[doc = "Sample and Hold sample time register 1"]
pub mod shsr1;
#[doc = "SHHR (rw) register accessor: an alias for `Reg<SHHR_SPEC>`"]
pub type SHHR = crate::Reg<shhr::SHHR_SPEC>;
#[doc = "Sample and Hold hold time register"]
pub mod shhr;
#[doc = "SHRR (rw) register accessor: an alias for `Reg<SHRR_SPEC>`"]
pub type SHRR = crate::Reg<shrr::SHRR_SPEC>;
#[doc = "Sample and Hold refresh time register"]
pub mod shrr;
