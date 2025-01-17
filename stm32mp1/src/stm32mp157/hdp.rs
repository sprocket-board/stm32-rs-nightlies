#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HDP Control"]
    pub hdp_ctrl: HDP_CTRL,
    #[doc = "0x04 - HDP multiplexing"]
    pub hdp_mux: HDP_MUX,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - HDP value"]
    pub hdp_val: HDP_VAL,
    #[doc = "0x14 - HDP GPO set"]
    pub hdp_gposet: HDP_GPOSET,
    #[doc = "0x18 - HDP GPO clear"]
    pub hdp_gpoclr: HDP_GPOCLR,
    #[doc = "0x1c - HDP GPO value"]
    pub hdp_gpoval: HDP_GPOVAL,
    _reserved6: [u8; 0x03d4],
    #[doc = "0x3f4 - HDP version register"]
    pub hdp_verr: HDP_VERR,
    #[doc = "0x3f8 - HDP IP identification register"]
    pub hdp_ipidr: HDP_IPIDR,
    #[doc = "0x3fc - HDP size identification register"]
    pub hdp_sidr: HDP_SIDR,
}
#[doc = "HDP_CTRL (rw) register accessor: an alias for `Reg<HDP_CTRL_SPEC>`"]
pub type HDP_CTRL = crate::Reg<hdp_ctrl::HDP_CTRL_SPEC>;
#[doc = "HDP Control"]
pub mod hdp_ctrl;
#[doc = "HDP_MUX (rw) register accessor: an alias for `Reg<HDP_MUX_SPEC>`"]
pub type HDP_MUX = crate::Reg<hdp_mux::HDP_MUX_SPEC>;
#[doc = "HDP multiplexing"]
pub mod hdp_mux;
#[doc = "HDP_VAL (r) register accessor: an alias for `Reg<HDP_VAL_SPEC>`"]
pub type HDP_VAL = crate::Reg<hdp_val::HDP_VAL_SPEC>;
#[doc = "HDP value"]
pub mod hdp_val;
#[doc = "HDP_GPOSET (w) register accessor: an alias for `Reg<HDP_GPOSET_SPEC>`"]
pub type HDP_GPOSET = crate::Reg<hdp_gposet::HDP_GPOSET_SPEC>;
#[doc = "HDP GPO set"]
pub mod hdp_gposet;
#[doc = "HDP_GPOCLR (w) register accessor: an alias for `Reg<HDP_GPOCLR_SPEC>`"]
pub type HDP_GPOCLR = crate::Reg<hdp_gpoclr::HDP_GPOCLR_SPEC>;
#[doc = "HDP GPO clear"]
pub mod hdp_gpoclr;
#[doc = "HDP_GPOVAL (rw) register accessor: an alias for `Reg<HDP_GPOVAL_SPEC>`"]
pub type HDP_GPOVAL = crate::Reg<hdp_gpoval::HDP_GPOVAL_SPEC>;
#[doc = "HDP GPO value"]
pub mod hdp_gpoval;
#[doc = "HDP_VERR (r) register accessor: an alias for `Reg<HDP_VERR_SPEC>`"]
pub type HDP_VERR = crate::Reg<hdp_verr::HDP_VERR_SPEC>;
#[doc = "HDP version register"]
pub mod hdp_verr;
#[doc = "HDP_IPIDR (r) register accessor: an alias for `Reg<HDP_IPIDR_SPEC>`"]
pub type HDP_IPIDR = crate::Reg<hdp_ipidr::HDP_IPIDR_SPEC>;
#[doc = "HDP IP identification register"]
pub mod hdp_ipidr;
#[doc = "HDP_SIDR (r) register accessor: an alias for `Reg<HDP_SIDR_SPEC>`"]
pub type HDP_SIDR = crate::Reg<hdp_sidr::HDP_SIDR_SPEC>;
#[doc = "HDP size identification register"]
pub mod hdp_sidr;
