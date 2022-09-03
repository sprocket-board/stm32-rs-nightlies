#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub din: DIN,
    #[doc = "0x0c - data output register"]
    pub dout: DOUT,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: DMACR,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x20 - key registers"]
    pub k0lr: K0LR,
    #[doc = "0x24 - key registers"]
    pub k0rr: K0RR,
    #[doc = "0x28 - key registers"]
    pub k1lr: K1LR,
    #[doc = "0x2c - key registers"]
    pub k1rr: K1RR,
    #[doc = "0x30 - key registers"]
    pub k2lr: K2LR,
    #[doc = "0x34 - key registers"]
    pub k2rr: K2RR,
    #[doc = "0x38 - key registers"]
    pub k3lr: K3LR,
    #[doc = "0x3c - key registers"]
    pub k3rr: K3RR,
    #[doc = "0x40 - Initialization vector register 0L"]
    pub iv0lr: IV0LR,
    #[doc = "0x44 - initialization vector register 0R"]
    pub iv0rr: IV0RR,
    #[doc = "0x48 - Initialization vector register 1L"]
    pub iv1lr: IV1LR,
    #[doc = "0x4c - Initialization vector register 1R"]
    pub iv1rr: IV1RR,
    #[doc = "0x50 - context swap register"]
    pub csgcmccm0r: CSGCMCCM0R,
    #[doc = "0x54 - context swap register"]
    pub csgcmccm1r: CSGCMCCM1R,
    #[doc = "0x58 - context swap register"]
    pub csgcmccm2r: CSGCMCCM2R,
    #[doc = "0x5c - context swap register"]
    pub csgcmccm3r: CSGCMCCM3R,
    #[doc = "0x60 - context swap register"]
    pub csgcmccm4r: CSGCMCCM4R,
    #[doc = "0x64 - context swap register"]
    pub csgcmccm5r: CSGCMCCM5R,
    #[doc = "0x68 - context swap register"]
    pub csgcmccm6r: CSGCMCCM6R,
    #[doc = "0x6c - context swap register"]
    pub csgcmccm7r: CSGCMCCM7R,
    #[doc = "0x70 - context swap register"]
    pub csgcm0r: CSGCM0R,
    #[doc = "0x74 - context swap register"]
    pub csgcm1r: CSGCM1R,
    #[doc = "0x78 - context swap register"]
    pub csgcm2r: CSGCM2R,
    #[doc = "0x7c - context swap register"]
    pub csgcm3r: CSGCM3R,
    #[doc = "0x80 - context swap register"]
    pub csgcm4r: CSGCM4R,
    #[doc = "0x84 - context swap register"]
    pub csgcm5r: CSGCM5R,
    #[doc = "0x88 - context swap register"]
    pub csgcm6r: CSGCM6R,
    #[doc = "0x8c - context swap register"]
    pub csgcm7r: CSGCM7R,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "data input register"]
pub mod din;
#[doc = "DOUT (r) register accessor: an alias for `Reg<DOUT_SPEC>`"]
pub type DOUT = crate::Reg<dout::DOUT_SPEC>;
#[doc = "data output register"]
pub mod dout;
#[doc = "DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "IMSCR (rw) register accessor: an alias for `Reg<IMSCR_SPEC>`"]
pub type IMSCR = crate::Reg<imscr::IMSCR_SPEC>;
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "RISR (r) register accessor: an alias for `Reg<RISR_SPEC>`"]
pub type RISR = crate::Reg<risr::RISR_SPEC>;
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "K0LR (w) register accessor: an alias for `Reg<K0LR_SPEC>`"]
pub type K0LR = crate::Reg<k0lr::K0LR_SPEC>;
#[doc = "key registers"]
pub mod k0lr;
#[doc = "K0RR (w) register accessor: an alias for `Reg<K0RR_SPEC>`"]
pub type K0RR = crate::Reg<k0rr::K0RR_SPEC>;
#[doc = "key registers"]
pub mod k0rr;
#[doc = "K1LR (w) register accessor: an alias for `Reg<K1LR_SPEC>`"]
pub type K1LR = crate::Reg<k1lr::K1LR_SPEC>;
#[doc = "key registers"]
pub mod k1lr;
#[doc = "K1RR (w) register accessor: an alias for `Reg<K1RR_SPEC>`"]
pub type K1RR = crate::Reg<k1rr::K1RR_SPEC>;
#[doc = "key registers"]
pub mod k1rr;
#[doc = "K2LR (w) register accessor: an alias for `Reg<K2LR_SPEC>`"]
pub type K2LR = crate::Reg<k2lr::K2LR_SPEC>;
#[doc = "key registers"]
pub mod k2lr;
#[doc = "K2RR (w) register accessor: an alias for `Reg<K2RR_SPEC>`"]
pub type K2RR = crate::Reg<k2rr::K2RR_SPEC>;
#[doc = "key registers"]
pub mod k2rr;
#[doc = "K3LR (w) register accessor: an alias for `Reg<K3LR_SPEC>`"]
pub type K3LR = crate::Reg<k3lr::K3LR_SPEC>;
#[doc = "key registers"]
pub mod k3lr;
#[doc = "K3RR (w) register accessor: an alias for `Reg<K3RR_SPEC>`"]
pub type K3RR = crate::Reg<k3rr::K3RR_SPEC>;
#[doc = "key registers"]
pub mod k3rr;
#[doc = "IV0LR (rw) register accessor: an alias for `Reg<IV0LR_SPEC>`"]
pub type IV0LR = crate::Reg<iv0lr::IV0LR_SPEC>;
#[doc = "Initialization vector register 0L"]
pub mod iv0lr;
#[doc = "IV0RR (rw) register accessor: an alias for `Reg<IV0RR_SPEC>`"]
pub type IV0RR = crate::Reg<iv0rr::IV0RR_SPEC>;
#[doc = "initialization vector register 0R"]
pub mod iv0rr;
#[doc = "IV1LR (rw) register accessor: an alias for `Reg<IV1LR_SPEC>`"]
pub type IV1LR = crate::Reg<iv1lr::IV1LR_SPEC>;
#[doc = "Initialization vector register 1L"]
pub mod iv1lr;
#[doc = "IV1RR (rw) register accessor: an alias for `Reg<IV1RR_SPEC>`"]
pub type IV1RR = crate::Reg<iv1rr::IV1RR_SPEC>;
#[doc = "Initialization vector register 1R"]
pub mod iv1rr;
#[doc = "CSGCMCCM0R (rw) register accessor: an alias for `Reg<CSGCMCCM0R_SPEC>`"]
pub type CSGCMCCM0R = crate::Reg<csgcmccm0r::CSGCMCCM0R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm0r;
#[doc = "CSGCMCCM1R (rw) register accessor: an alias for `Reg<CSGCMCCM1R_SPEC>`"]
pub type CSGCMCCM1R = crate::Reg<csgcmccm1r::CSGCMCCM1R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm1r;
#[doc = "CSGCMCCM2R (rw) register accessor: an alias for `Reg<CSGCMCCM2R_SPEC>`"]
pub type CSGCMCCM2R = crate::Reg<csgcmccm2r::CSGCMCCM2R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm2r;
#[doc = "CSGCMCCM3R (rw) register accessor: an alias for `Reg<CSGCMCCM3R_SPEC>`"]
pub type CSGCMCCM3R = crate::Reg<csgcmccm3r::CSGCMCCM3R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm3r;
#[doc = "CSGCMCCM4R (rw) register accessor: an alias for `Reg<CSGCMCCM4R_SPEC>`"]
pub type CSGCMCCM4R = crate::Reg<csgcmccm4r::CSGCMCCM4R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm4r;
#[doc = "CSGCMCCM5R (rw) register accessor: an alias for `Reg<CSGCMCCM5R_SPEC>`"]
pub type CSGCMCCM5R = crate::Reg<csgcmccm5r::CSGCMCCM5R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm5r;
#[doc = "CSGCMCCM6R (rw) register accessor: an alias for `Reg<CSGCMCCM6R_SPEC>`"]
pub type CSGCMCCM6R = crate::Reg<csgcmccm6r::CSGCMCCM6R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm6r;
#[doc = "CSGCMCCM7R (rw) register accessor: an alias for `Reg<CSGCMCCM7R_SPEC>`"]
pub type CSGCMCCM7R = crate::Reg<csgcmccm7r::CSGCMCCM7R_SPEC>;
#[doc = "context swap register"]
pub mod csgcmccm7r;
#[doc = "CSGCM0R (rw) register accessor: an alias for `Reg<CSGCM0R_SPEC>`"]
pub type CSGCM0R = crate::Reg<csgcm0r::CSGCM0R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm0r;
#[doc = "CSGCM1R (rw) register accessor: an alias for `Reg<CSGCM1R_SPEC>`"]
pub type CSGCM1R = crate::Reg<csgcm1r::CSGCM1R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm1r;
#[doc = "CSGCM2R (rw) register accessor: an alias for `Reg<CSGCM2R_SPEC>`"]
pub type CSGCM2R = crate::Reg<csgcm2r::CSGCM2R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm2r;
#[doc = "CSGCM3R (rw) register accessor: an alias for `Reg<CSGCM3R_SPEC>`"]
pub type CSGCM3R = crate::Reg<csgcm3r::CSGCM3R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm3r;
#[doc = "CSGCM4R (rw) register accessor: an alias for `Reg<CSGCM4R_SPEC>`"]
pub type CSGCM4R = crate::Reg<csgcm4r::CSGCM4R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm4r;
#[doc = "CSGCM5R (rw) register accessor: an alias for `Reg<CSGCM5R_SPEC>`"]
pub type CSGCM5R = crate::Reg<csgcm5r::CSGCM5R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm5r;
#[doc = "CSGCM6R (rw) register accessor: an alias for `Reg<CSGCM6R_SPEC>`"]
pub type CSGCM6R = crate::Reg<csgcm6r::CSGCM6R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm6r;
#[doc = "CSGCM7R (rw) register accessor: an alias for `Reg<CSGCM7R_SPEC>`"]
pub type CSGCM7R = crate::Reg<csgcm7r::CSGCM7R_SPEC>;
#[doc = "context swap register"]
pub mod csgcm7r;