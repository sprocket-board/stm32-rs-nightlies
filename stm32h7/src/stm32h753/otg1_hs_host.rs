#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub hcfg: HCFG,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub hprt: HPRT,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub hcchar0: HCCHAR0,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub hcsplt0: HCSPLT0,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"]
    pub hcint0: HCINT0,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk0: HCINTMSK0,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz0: HCTSIZ0,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"]
    pub hcdma0: HCDMA0,
    _reserved13: [u8; 0x08],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub hcchar1: HCCHAR1,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub hcsplt1: HCSPLT1,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"]
    pub hcint1: HCINT1,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub hcintmsk1: HCINTMSK1,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub hctsiz1: HCTSIZ1,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"]
    pub hcdma1: HCDMA1,
    _reserved19: [u8; 0x08],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub hcchar2: HCCHAR2,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub hcsplt2: HCSPLT2,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"]
    pub hcint2: HCINT2,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub hcintmsk2: HCINTMSK2,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub hctsiz2: HCTSIZ2,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"]
    pub hcdma2: HCDMA2,
    _reserved25: [u8; 0x08],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub hcchar3: HCCHAR3,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub hcsplt3: HCSPLT3,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"]
    pub hcint3: HCINT3,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub hcintmsk3: HCINTMSK3,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub hctsiz3: HCTSIZ3,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"]
    pub hcdma3: HCDMA3,
    _reserved31: [u8; 0x08],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub hcchar4: HCCHAR4,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub hcsplt4: HCSPLT4,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"]
    pub hcint4: HCINT4,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub hcintmsk4: HCINTMSK4,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub hctsiz4: HCTSIZ4,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"]
    pub hcdma4: HCDMA4,
    _reserved37: [u8; 0x08],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub hcchar5: HCCHAR5,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub hcsplt5: HCSPLT5,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"]
    pub hcint5: HCINT5,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub hcintmsk5: HCINTMSK5,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub hctsiz5: HCTSIZ5,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"]
    pub hcdma5: HCDMA5,
    _reserved43: [u8; 0x08],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub hcchar6: HCCHAR6,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub hcsplt6: HCSPLT6,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"]
    pub hcint6: HCINT6,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub hcintmsk6: HCINTMSK6,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub hctsiz6: HCTSIZ6,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"]
    pub hcdma6: HCDMA6,
    _reserved49: [u8; 0x08],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub hcchar7: HCCHAR7,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub hcsplt7: HCSPLT7,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"]
    pub hcint7: HCINT7,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub hcintmsk7: HCINTMSK7,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub hctsiz7: HCTSIZ7,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"]
    pub hcdma7: HCDMA7,
    _reserved55: [u8; 0x08],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub hcchar8: HCCHAR8,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub hcsplt8: HCSPLT8,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"]
    pub hcint8: HCINT8,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub hcintmsk8: HCINTMSK8,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub hctsiz8: HCTSIZ8,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"]
    pub hcdma8: HCDMA8,
    _reserved61: [u8; 0x08],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub hcchar9: HCCHAR9,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub hcsplt9: HCSPLT9,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"]
    pub hcint9: HCINT9,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub hcintmsk9: HCINTMSK9,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub hctsiz9: HCTSIZ9,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"]
    pub hcdma9: HCDMA9,
    _reserved67: [u8; 0x08],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub hcchar10: HCCHAR10,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub hcsplt10: HCSPLT10,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"]
    pub hcint10: HCINT10,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub hcintmsk10: HCINTMSK10,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub hctsiz10: HCTSIZ10,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"]
    pub hcdma10: HCDMA10,
    _reserved73: [u8; 0x08],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub hcchar11: HCCHAR11,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub hcsplt11: HCSPLT11,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"]
    pub hcint11: HCINT11,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub hcintmsk11: HCINTMSK11,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub hctsiz11: HCTSIZ11,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"]
    pub hcdma11: HCDMA11,
    #[doc = "0x278 - OTG_HS host channel-12 characteristics register"]
    pub hcchar12: HCCHAR12,
    #[doc = "0x27c - OTG_HS host channel-12 split control register"]
    pub hcsplt12: HCSPLT12,
    #[doc = "0x280 - OTG_HS host channel-12 interrupt register"]
    pub hcint12: HCINT12,
    #[doc = "0x284 - OTG_HS host channel-12 interrupt mask register"]
    pub hcintmsk12: HCINTMSK12,
    #[doc = "0x288 - OTG_HS host channel-12 transfer size register"]
    pub hctsiz12: HCTSIZ12,
    #[doc = "0x28c - OTG_HS host channel-12 DMA address register"]
    pub hcdma12: HCDMA12,
    #[doc = "0x290 - OTG_HS host channel-13 characteristics register"]
    pub hcchar13: HCCHAR13,
    #[doc = "0x294 - OTG_HS host channel-13 split control register"]
    pub hcsplt13: HCSPLT13,
    #[doc = "0x298 - OTG_HS host channel-13 interrupt register"]
    pub hcint13: HCINT13,
    #[doc = "0x29c - OTG_HS host channel-13 interrupt mask register"]
    pub hcintmsk13: HCINTMSK13,
    #[doc = "0x2a0 - OTG_HS host channel-13 transfer size register"]
    pub hctsiz13: HCTSIZ13,
    #[doc = "0x2a4 - OTG_HS host channel-13 DMA address register"]
    pub hcdma13: HCDMA13,
    #[doc = "0x2a8 - OTG_HS host channel-14 characteristics register"]
    pub hcchar14: HCCHAR14,
    #[doc = "0x2ac - OTG_HS host channel-14 split control register"]
    pub hcsplt14: HCSPLT14,
    #[doc = "0x2b0 - OTG_HS host channel-14 interrupt register"]
    pub hcint14: HCINT14,
    #[doc = "0x2b4 - OTG_HS host channel-14 interrupt mask register"]
    pub hcintmsk14: HCINTMSK14,
    #[doc = "0x2b8 - OTG_HS host channel-14 transfer size register"]
    pub hctsiz14: HCTSIZ14,
    #[doc = "0x2bc - OTG_HS host channel-14 DMA address register"]
    pub hcdma14: HCDMA14,
    #[doc = "0x2c0 - OTG_HS host channel-15 characteristics register"]
    pub hcchar15: HCCHAR15,
    #[doc = "0x2c4 - OTG_HS host channel-15 split control register"]
    pub hcsplt15: HCSPLT15,
    #[doc = "0x2c8 - OTG_HS host channel-15 interrupt register"]
    pub hcint15: HCINT15,
    #[doc = "0x2cc - OTG_HS host channel-15 interrupt mask register"]
    pub hcintmsk15: HCINTMSK15,
    #[doc = "0x2d0 - OTG_HS host channel-15 transfer size register"]
    pub hctsiz15: HCTSIZ15,
    #[doc = "0x2d4 - OTG_HS host channel-15 DMA address register"]
    pub hcdma15: HCDMA15,
}
#[doc = "HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod hfnum;
#[doc = "HPTXSTS (rw) register accessor: an alias for `Reg<HPTXSTS_SPEC>`"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: an alias for `Reg<HCCHAR0_SPEC>`"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod hcchar0;
#[doc = "HCCHAR1 (rw) register accessor: an alias for `Reg<HCCHAR1_SPEC>`"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod hcchar1;
#[doc = "HCCHAR2 (rw) register accessor: an alias for `Reg<HCCHAR2_SPEC>`"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod hcchar2;
#[doc = "HCCHAR3 (rw) register accessor: an alias for `Reg<HCCHAR3_SPEC>`"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod hcchar3;
#[doc = "HCCHAR4 (rw) register accessor: an alias for `Reg<HCCHAR4_SPEC>`"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod hcchar4;
#[doc = "HCCHAR5 (rw) register accessor: an alias for `Reg<HCCHAR5_SPEC>`"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod hcchar5;
#[doc = "HCCHAR6 (rw) register accessor: an alias for `Reg<HCCHAR6_SPEC>`"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod hcchar6;
#[doc = "HCCHAR7 (rw) register accessor: an alias for `Reg<HCCHAR7_SPEC>`"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod hcchar7;
#[doc = "HCCHAR8 (rw) register accessor: an alias for `Reg<HCCHAR8_SPEC>`"]
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8_SPEC>;
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod hcchar8;
#[doc = "HCCHAR9 (rw) register accessor: an alias for `Reg<HCCHAR9_SPEC>`"]
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9_SPEC>;
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod hcchar9;
#[doc = "HCCHAR10 (rw) register accessor: an alias for `Reg<HCCHAR10_SPEC>`"]
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10_SPEC>;
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod hcchar10;
#[doc = "HCCHAR11 (rw) register accessor: an alias for `Reg<HCCHAR11_SPEC>`"]
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11_SPEC>;
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod hcchar11;
#[doc = "HCSPLT0 (rw) register accessor: an alias for `Reg<HCSPLT0_SPEC>`"]
pub type HCSPLT0 = crate::Reg<hcsplt0::HCSPLT0_SPEC>;
#[doc = "OTG_HS host channel-0 split control register"]
pub mod hcsplt0;
#[doc = "HCSPLT1 (rw) register accessor: an alias for `Reg<HCSPLT1_SPEC>`"]
pub type HCSPLT1 = crate::Reg<hcsplt1::HCSPLT1_SPEC>;
#[doc = "OTG_HS host channel-1 split control register"]
pub mod hcsplt1;
#[doc = "HCSPLT2 (rw) register accessor: an alias for `Reg<HCSPLT2_SPEC>`"]
pub type HCSPLT2 = crate::Reg<hcsplt2::HCSPLT2_SPEC>;
#[doc = "OTG_HS host channel-2 split control register"]
pub mod hcsplt2;
#[doc = "HCSPLT3 (rw) register accessor: an alias for `Reg<HCSPLT3_SPEC>`"]
pub type HCSPLT3 = crate::Reg<hcsplt3::HCSPLT3_SPEC>;
#[doc = "OTG_HS host channel-3 split control register"]
pub mod hcsplt3;
#[doc = "HCSPLT4 (rw) register accessor: an alias for `Reg<HCSPLT4_SPEC>`"]
pub type HCSPLT4 = crate::Reg<hcsplt4::HCSPLT4_SPEC>;
#[doc = "OTG_HS host channel-4 split control register"]
pub mod hcsplt4;
#[doc = "HCSPLT5 (rw) register accessor: an alias for `Reg<HCSPLT5_SPEC>`"]
pub type HCSPLT5 = crate::Reg<hcsplt5::HCSPLT5_SPEC>;
#[doc = "OTG_HS host channel-5 split control register"]
pub mod hcsplt5;
#[doc = "HCSPLT6 (rw) register accessor: an alias for `Reg<HCSPLT6_SPEC>`"]
pub type HCSPLT6 = crate::Reg<hcsplt6::HCSPLT6_SPEC>;
#[doc = "OTG_HS host channel-6 split control register"]
pub mod hcsplt6;
#[doc = "HCSPLT7 (rw) register accessor: an alias for `Reg<HCSPLT7_SPEC>`"]
pub type HCSPLT7 = crate::Reg<hcsplt7::HCSPLT7_SPEC>;
#[doc = "OTG_HS host channel-7 split control register"]
pub mod hcsplt7;
#[doc = "HCSPLT8 (rw) register accessor: an alias for `Reg<HCSPLT8_SPEC>`"]
pub type HCSPLT8 = crate::Reg<hcsplt8::HCSPLT8_SPEC>;
#[doc = "OTG_HS host channel-8 split control register"]
pub mod hcsplt8;
#[doc = "HCSPLT9 (rw) register accessor: an alias for `Reg<HCSPLT9_SPEC>`"]
pub type HCSPLT9 = crate::Reg<hcsplt9::HCSPLT9_SPEC>;
#[doc = "OTG_HS host channel-9 split control register"]
pub mod hcsplt9;
#[doc = "HCSPLT10 (rw) register accessor: an alias for `Reg<HCSPLT10_SPEC>`"]
pub type HCSPLT10 = crate::Reg<hcsplt10::HCSPLT10_SPEC>;
#[doc = "OTG_HS host channel-10 split control register"]
pub mod hcsplt10;
#[doc = "HCSPLT11 (rw) register accessor: an alias for `Reg<HCSPLT11_SPEC>`"]
pub type HCSPLT11 = crate::Reg<hcsplt11::HCSPLT11_SPEC>;
#[doc = "OTG_HS host channel-11 split control register"]
pub mod hcsplt11;
#[doc = "HCINT0 (rw) register accessor: an alias for `Reg<HCINT0_SPEC>`"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint0;
#[doc = "HCINT1 (rw) register accessor: an alias for `Reg<HCINT1_SPEC>`"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod hcint1;
#[doc = "HCINT2 (rw) register accessor: an alias for `Reg<HCINT2_SPEC>`"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod hcint2;
#[doc = "HCINT3 (rw) register accessor: an alias for `Reg<HCINT3_SPEC>`"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod hcint3;
#[doc = "HCINT4 (rw) register accessor: an alias for `Reg<HCINT4_SPEC>`"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod hcint4;
#[doc = "HCINT5 (rw) register accessor: an alias for `Reg<HCINT5_SPEC>`"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod hcint5;
#[doc = "HCINT6 (rw) register accessor: an alias for `Reg<HCINT6_SPEC>`"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod hcint6;
#[doc = "HCINT7 (rw) register accessor: an alias for `Reg<HCINT7_SPEC>`"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod hcint7;
#[doc = "HCINT8 (rw) register accessor: an alias for `Reg<HCINT8_SPEC>`"]
pub type HCINT8 = crate::Reg<hcint8::HCINT8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod hcint8;
#[doc = "HCINT9 (rw) register accessor: an alias for `Reg<HCINT9_SPEC>`"]
pub type HCINT9 = crate::Reg<hcint9::HCINT9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod hcint9;
#[doc = "HCINT10 (rw) register accessor: an alias for `Reg<HCINT10_SPEC>`"]
pub type HCINT10 = crate::Reg<hcint10::HCINT10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod hcint10;
#[doc = "HCINT11 (rw) register accessor: an alias for `Reg<HCINT11_SPEC>`"]
pub type HCINT11 = crate::Reg<hcint11::HCINT11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod hcint11;
#[doc = "HCINTMSK0 (rw) register accessor: an alias for `Reg<HCINTMSK0_SPEC>`"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk0;
#[doc = "HCINTMSK1 (rw) register accessor: an alias for `Reg<HCINTMSK1_SPEC>`"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod hcintmsk1;
#[doc = "HCINTMSK2 (rw) register accessor: an alias for `Reg<HCINTMSK2_SPEC>`"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod hcintmsk2;
#[doc = "HCINTMSK3 (rw) register accessor: an alias for `Reg<HCINTMSK3_SPEC>`"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod hcintmsk3;
#[doc = "HCINTMSK4 (rw) register accessor: an alias for `Reg<HCINTMSK4_SPEC>`"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod hcintmsk4;
#[doc = "HCINTMSK5 (rw) register accessor: an alias for `Reg<HCINTMSK5_SPEC>`"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod hcintmsk5;
#[doc = "HCINTMSK6 (rw) register accessor: an alias for `Reg<HCINTMSK6_SPEC>`"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod hcintmsk6;
#[doc = "HCINTMSK7 (rw) register accessor: an alias for `Reg<HCINTMSK7_SPEC>`"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod hcintmsk7;
#[doc = "HCINTMSK8 (rw) register accessor: an alias for `Reg<HCINTMSK8_SPEC>`"]
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod hcintmsk8;
#[doc = "HCINTMSK9 (rw) register accessor: an alias for `Reg<HCINTMSK9_SPEC>`"]
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod hcintmsk9;
#[doc = "HCINTMSK10 (rw) register accessor: an alias for `Reg<HCINTMSK10_SPEC>`"]
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod hcintmsk10;
#[doc = "HCINTMSK11 (rw) register accessor: an alias for `Reg<HCINTMSK11_SPEC>`"]
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod hcintmsk11;
#[doc = "HCTSIZ0 (rw) register accessor: an alias for `Reg<HCTSIZ0_SPEC>`"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz0;
#[doc = "HCTSIZ1 (rw) register accessor: an alias for `Reg<HCTSIZ1_SPEC>`"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCTSIZ2 (rw) register accessor: an alias for `Reg<HCTSIZ2_SPEC>`"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCTSIZ3 (rw) register accessor: an alias for `Reg<HCTSIZ3_SPEC>`"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCTSIZ4 (rw) register accessor: an alias for `Reg<HCTSIZ4_SPEC>`"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod hctsiz4;
#[doc = "HCTSIZ5 (rw) register accessor: an alias for `Reg<HCTSIZ5_SPEC>`"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCTSIZ6 (rw) register accessor: an alias for `Reg<HCTSIZ6_SPEC>`"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCTSIZ7 (rw) register accessor: an alias for `Reg<HCTSIZ7_SPEC>`"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod hctsiz7;
#[doc = "HCTSIZ8 (rw) register accessor: an alias for `Reg<HCTSIZ8_SPEC>`"]
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8_SPEC>;
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod hctsiz8;
#[doc = "HCTSIZ9 (rw) register accessor: an alias for `Reg<HCTSIZ9_SPEC>`"]
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9_SPEC>;
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod hctsiz9;
#[doc = "HCTSIZ10 (rw) register accessor: an alias for `Reg<HCTSIZ10_SPEC>`"]
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10_SPEC>;
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod hctsiz10;
#[doc = "HCTSIZ11 (rw) register accessor: an alias for `Reg<HCTSIZ11_SPEC>`"]
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod hctsiz11;
#[doc = "HCDMA0 (rw) register accessor: an alias for `Reg<HCDMA0_SPEC>`"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod hcdma0;
#[doc = "HCDMA1 (rw) register accessor: an alias for `Reg<HCDMA1_SPEC>`"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod hcdma1;
#[doc = "HCDMA2 (rw) register accessor: an alias for `Reg<HCDMA2_SPEC>`"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod hcdma2;
#[doc = "HCDMA3 (rw) register accessor: an alias for `Reg<HCDMA3_SPEC>`"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod hcdma3;
#[doc = "HCDMA4 (rw) register accessor: an alias for `Reg<HCDMA4_SPEC>`"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod hcdma4;
#[doc = "HCDMA5 (rw) register accessor: an alias for `Reg<HCDMA5_SPEC>`"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod hcdma5;
#[doc = "HCDMA6 (rw) register accessor: an alias for `Reg<HCDMA6_SPEC>`"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod hcdma6;
#[doc = "HCDMA7 (rw) register accessor: an alias for `Reg<HCDMA7_SPEC>`"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod hcdma7;
#[doc = "HCDMA8 (rw) register accessor: an alias for `Reg<HCDMA8_SPEC>`"]
pub type HCDMA8 = crate::Reg<hcdma8::HCDMA8_SPEC>;
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod hcdma8;
#[doc = "HCDMA9 (rw) register accessor: an alias for `Reg<HCDMA9_SPEC>`"]
pub type HCDMA9 = crate::Reg<hcdma9::HCDMA9_SPEC>;
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod hcdma9;
#[doc = "HCDMA10 (rw) register accessor: an alias for `Reg<HCDMA10_SPEC>`"]
pub type HCDMA10 = crate::Reg<hcdma10::HCDMA10_SPEC>;
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod hcdma10;
#[doc = "HCDMA11 (rw) register accessor: an alias for `Reg<HCDMA11_SPEC>`"]
pub type HCDMA11 = crate::Reg<hcdma11::HCDMA11_SPEC>;
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod hcdma11;
#[doc = "HCCHAR12 (rw) register accessor: an alias for `Reg<HCCHAR12_SPEC>`"]
pub type HCCHAR12 = crate::Reg<hcchar12::HCCHAR12_SPEC>;
#[doc = "OTG_HS host channel-12 characteristics register"]
pub mod hcchar12;
#[doc = "HCSPLT12 (rw) register accessor: an alias for `Reg<HCSPLT12_SPEC>`"]
pub type HCSPLT12 = crate::Reg<hcsplt12::HCSPLT12_SPEC>;
#[doc = "OTG_HS host channel-12 split control register"]
pub mod hcsplt12;
#[doc = "HCINT12 (rw) register accessor: an alias for `Reg<HCINT12_SPEC>`"]
pub type HCINT12 = crate::Reg<hcint12::HCINT12_SPEC>;
#[doc = "OTG_HS host channel-12 interrupt register"]
pub mod hcint12;
#[doc = "HCINTMSK12 (rw) register accessor: an alias for `Reg<HCINTMSK12_SPEC>`"]
pub type HCINTMSK12 = crate::Reg<hcintmsk12::HCINTMSK12_SPEC>;
#[doc = "OTG_HS host channel-12 interrupt mask register"]
pub mod hcintmsk12;
#[doc = "HCTSIZ12 (rw) register accessor: an alias for `Reg<HCTSIZ12_SPEC>`"]
pub type HCTSIZ12 = crate::Reg<hctsiz12::HCTSIZ12_SPEC>;
#[doc = "OTG_HS host channel-12 transfer size register"]
pub mod hctsiz12;
#[doc = "HCDMA12 (rw) register accessor: an alias for `Reg<HCDMA12_SPEC>`"]
pub type HCDMA12 = crate::Reg<hcdma12::HCDMA12_SPEC>;
#[doc = "OTG_HS host channel-12 DMA address register"]
pub mod hcdma12;
#[doc = "HCCHAR13 (rw) register accessor: an alias for `Reg<HCCHAR13_SPEC>`"]
pub type HCCHAR13 = crate::Reg<hcchar13::HCCHAR13_SPEC>;
#[doc = "OTG_HS host channel-13 characteristics register"]
pub mod hcchar13;
#[doc = "HCSPLT13 (rw) register accessor: an alias for `Reg<HCSPLT13_SPEC>`"]
pub type HCSPLT13 = crate::Reg<hcsplt13::HCSPLT13_SPEC>;
#[doc = "OTG_HS host channel-13 split control register"]
pub mod hcsplt13;
#[doc = "HCINT13 (rw) register accessor: an alias for `Reg<HCINT13_SPEC>`"]
pub type HCINT13 = crate::Reg<hcint13::HCINT13_SPEC>;
#[doc = "OTG_HS host channel-13 interrupt register"]
pub mod hcint13;
#[doc = "HCINTMSK13 (rw) register accessor: an alias for `Reg<HCINTMSK13_SPEC>`"]
pub type HCINTMSK13 = crate::Reg<hcintmsk13::HCINTMSK13_SPEC>;
#[doc = "OTG_HS host channel-13 interrupt mask register"]
pub mod hcintmsk13;
#[doc = "HCTSIZ13 (rw) register accessor: an alias for `Reg<HCTSIZ13_SPEC>`"]
pub type HCTSIZ13 = crate::Reg<hctsiz13::HCTSIZ13_SPEC>;
#[doc = "OTG_HS host channel-13 transfer size register"]
pub mod hctsiz13;
#[doc = "HCDMA13 (rw) register accessor: an alias for `Reg<HCDMA13_SPEC>`"]
pub type HCDMA13 = crate::Reg<hcdma13::HCDMA13_SPEC>;
#[doc = "OTG_HS host channel-13 DMA address register"]
pub mod hcdma13;
#[doc = "HCCHAR14 (rw) register accessor: an alias for `Reg<HCCHAR14_SPEC>`"]
pub type HCCHAR14 = crate::Reg<hcchar14::HCCHAR14_SPEC>;
#[doc = "OTG_HS host channel-14 characteristics register"]
pub mod hcchar14;
#[doc = "HCSPLT14 (rw) register accessor: an alias for `Reg<HCSPLT14_SPEC>`"]
pub type HCSPLT14 = crate::Reg<hcsplt14::HCSPLT14_SPEC>;
#[doc = "OTG_HS host channel-14 split control register"]
pub mod hcsplt14;
#[doc = "HCINT14 (rw) register accessor: an alias for `Reg<HCINT14_SPEC>`"]
pub type HCINT14 = crate::Reg<hcint14::HCINT14_SPEC>;
#[doc = "OTG_HS host channel-14 interrupt register"]
pub mod hcint14;
#[doc = "HCINTMSK14 (rw) register accessor: an alias for `Reg<HCINTMSK14_SPEC>`"]
pub type HCINTMSK14 = crate::Reg<hcintmsk14::HCINTMSK14_SPEC>;
#[doc = "OTG_HS host channel-14 interrupt mask register"]
pub mod hcintmsk14;
#[doc = "HCTSIZ14 (rw) register accessor: an alias for `Reg<HCTSIZ14_SPEC>`"]
pub type HCTSIZ14 = crate::Reg<hctsiz14::HCTSIZ14_SPEC>;
#[doc = "OTG_HS host channel-14 transfer size register"]
pub mod hctsiz14;
#[doc = "HCDMA14 (rw) register accessor: an alias for `Reg<HCDMA14_SPEC>`"]
pub type HCDMA14 = crate::Reg<hcdma14::HCDMA14_SPEC>;
#[doc = "OTG_HS host channel-14 DMA address register"]
pub mod hcdma14;
#[doc = "HCCHAR15 (rw) register accessor: an alias for `Reg<HCCHAR15_SPEC>`"]
pub type HCCHAR15 = crate::Reg<hcchar15::HCCHAR15_SPEC>;
#[doc = "OTG_HS host channel-15 characteristics register"]
pub mod hcchar15;
#[doc = "HCSPLT15 (rw) register accessor: an alias for `Reg<HCSPLT15_SPEC>`"]
pub type HCSPLT15 = crate::Reg<hcsplt15::HCSPLT15_SPEC>;
#[doc = "OTG_HS host channel-15 split control register"]
pub mod hcsplt15;
#[doc = "HCINT15 (rw) register accessor: an alias for `Reg<HCINT15_SPEC>`"]
pub type HCINT15 = crate::Reg<hcint15::HCINT15_SPEC>;
#[doc = "OTG_HS host channel-15 interrupt register"]
pub mod hcint15;
#[doc = "HCINTMSK15 (rw) register accessor: an alias for `Reg<HCINTMSK15_SPEC>`"]
pub type HCINTMSK15 = crate::Reg<hcintmsk15::HCINTMSK15_SPEC>;
#[doc = "OTG_HS host channel-15 interrupt mask register"]
pub mod hcintmsk15;
#[doc = "HCTSIZ15 (rw) register accessor: an alias for `Reg<HCTSIZ15_SPEC>`"]
pub type HCTSIZ15 = crate::Reg<hctsiz15::HCTSIZ15_SPEC>;
#[doc = "OTG_HS host channel-15 transfer size register"]
pub mod hctsiz15;
#[doc = "HCDMA15 (rw) register accessor: an alias for `Reg<HCDMA15_SPEC>`"]
pub type HCDMA15 = crate::Reg<hcdma15::HCDMA15_SPEC>;
#[doc = "OTG_HS host channel-15 DMA address register"]
pub mod hcdma15;
