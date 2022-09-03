#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub deachint: DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub deachintmsk: DEACHINTMSK,
    _reserved13: [u8; 0xc0],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub diepctl0: DIEPCTL0,
    _reserved14: [u8; 0x04],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub diepint0: DIEPINT0,
    _reserved15: [u8; 0x04],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub diepdma1: DIEPDMA1,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved18: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL1,
    _reserved19: [u8; 0x04],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub diepint1: DIEPINT1,
    _reserved20: [u8; 0x04],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz1: DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub diepdma2: DIEPDMA2,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS1,
    _reserved23: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub diepctl2: DIEPCTL2,
    _reserved24: [u8; 0x04],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub diepint2: DIEPINT2,
    _reserved25: [u8; 0x04],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz2: DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub diepdma3: DIEPDMA3,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS2,
    _reserved28: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub diepctl3: DIEPCTL3,
    _reserved29: [u8; 0x04],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub diepint3: DIEPINT3,
    _reserved30: [u8; 0x04],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz3: DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub diepdma4: DIEPDMA4,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS3,
    _reserved33: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub diepctl4: DIEPCTL4,
    _reserved34: [u8; 0x04],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub diepint4: DIEPINT4,
    _reserved35: [u8; 0x04],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz4: DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub diepdma5: DIEPDMA5,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS4,
    _reserved38: [u8; 0x04],
    _reserved_38_diepctl5: [u8; 0x04],
    #[doc = "0x1a4 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts6: DTXFSTS6,
    _reserved_40_diepint5: [u8; 0x04],
    #[doc = "0x1ac - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts7: DTXFSTS7,
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub dieptsiz5: DIEPTSIZ5,
    _reserved43: [u8; 0x04],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS5,
    _reserved44: [u8; 0x04],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub diepctl6: DIEPCTL6,
    _reserved45: [u8; 0x04],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub diepint6: DIEPINT6,
    _reserved46: [u8; 0x14],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub diepctl7: DIEPCTL7,
    _reserved47: [u8; 0x04],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub diepint7: DIEPINT7,
    _reserved48: [u8; 0x0114],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved49: [u8; 0x04],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved50: [u8; 0x04],
    #[doc = "0x310 - OTG_HS device endpoint-0 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved51: [u8; 0x0c],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub doepctl1: DOEPCTL1,
    _reserved52: [u8; 0x04],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub doepint1: DOEPINT1,
    _reserved53: [u8; 0x04],
    #[doc = "0x330 - OTG_HS device endpoint-1 transfer size register"]
    pub doeptsiz1: DOEPTSIZ1,
    _reserved54: [u8; 0x0c],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub doepctl2: DOEPCTL2,
    _reserved55: [u8; 0x04],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub doepint2: DOEPINT2,
    _reserved56: [u8; 0x04],
    #[doc = "0x350 - OTG_HS device endpoint-2 transfer size register"]
    pub doeptsiz2: DOEPTSIZ2,
    _reserved57: [u8; 0x0c],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub doepctl3: DOEPCTL3,
    _reserved58: [u8; 0x04],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub doepint3: DOEPINT3,
    _reserved59: [u8; 0x04],
    #[doc = "0x370 - OTG_HS device endpoint-3 transfer size register"]
    pub doeptsiz3: DOEPTSIZ3,
    _reserved60: [u8; 0x0c],
    #[doc = "0x380 - OTG device endpoint-4 control register"]
    pub doepctl4: DOEPCTL4,
    _reserved61: [u8; 0x04],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub doepint4: DOEPINT4,
    _reserved62: [u8; 0x04],
    #[doc = "0x390 - OTG_HS device endpoint-4 transfer size register"]
    pub doeptsiz4: DOEPTSIZ4,
    _reserved63: [u8; 0x0c],
    #[doc = "0x3a0 - OTG device endpoint-5 control register"]
    pub doepctl5: DOEPCTL5,
    _reserved64: [u8; 0x04],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub doepint5: DOEPINT5,
    _reserved65: [u8; 0x04],
    #[doc = "0x3b0 - OTG_HS device endpoint-5 transfer size register"]
    pub doeptsiz5: DOEPTSIZ5,
    _reserved66: [u8; 0x0c],
    #[doc = "0x3c0 - OTG device endpoint-6 control register"]
    pub doepctl6: DOEPCTL6,
    _reserved67: [u8; 0x04],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub doepint6: DOEPINT6,
    _reserved68: [u8; 0x04],
    #[doc = "0x3d0 - OTG_HS device endpoint-6 transfer size register"]
    pub doeptsiz6: DOEPTSIZ6,
    _reserved69: [u8; 0x0c],
    #[doc = "0x3e0 - OTG device endpoint-7 control register"]
    pub doepctl7: DOEPCTL7,
    _reserved70: [u8; 0x04],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub doepint7: DOEPINT7,
    _reserved71: [u8; 0x04],
    #[doc = "0x3f0 - OTG_HS device endpoint-7 transfer size register"]
    pub doeptsiz7: DOEPTSIZ7,
}
impl RegisterBlock {
    #[doc = "0x1a0 - OTG_HS device endpoint transfer size register"]
    #[inline(always)]
    pub fn dieptsiz6(&self) -> &DIEPTSIZ6 {
        unsafe { &*(((self as *const Self) as *const u8).add(416usize) as *const DIEPTSIZ6) }
    }
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    #[inline(always)]
    pub fn diepctl5(&self) -> &DIEPCTL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(416usize) as *const DIEPCTL5) }
    }
    #[doc = "0x1a8 - OTG_HS device endpoint transfer size register"]
    #[inline(always)]
    pub fn dieptsiz7(&self) -> &DIEPTSIZ7 {
        unsafe { &*(((self as *const Self) as *const u8).add(424usize) as *const DIEPTSIZ7) }
    }
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    #[inline(always)]
    pub fn diepint5(&self) -> &DIEPINT5 {
        unsafe { &*(((self as *const Self) as *const u8).add(424usize) as *const DIEPINT5) }
    }
}
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: an alias for `Reg<DTHRCTL_SPEC>`"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DEACHINT (rw) register accessor: an alias for `Reg<DEACHINT_SPEC>`"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod deachintmsk;
#[doc = "DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "OTG device endpoint-0 control register"]
pub mod diepctl0;
#[doc = "DIEPCTL1 (rw) register accessor: an alias for `Reg<DIEPCTL1_SPEC>`"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "DIEPCTL2 (rw) register accessor: an alias for `Reg<DIEPCTL2_SPEC>`"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "DIEPCTL3 (rw) register accessor: an alias for `Reg<DIEPCTL3_SPEC>`"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "DIEPCTL4 (rw) register accessor: an alias for `Reg<DIEPCTL4_SPEC>`"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod diepctl4;
#[doc = "DIEPCTL5 (rw) register accessor: an alias for `Reg<DIEPCTL5_SPEC>`"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod diepctl5;
#[doc = "DIEPCTL6 (rw) register accessor: an alias for `Reg<DIEPCTL6_SPEC>`"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = "OTG device endpoint-6 control register"]
pub mod diepctl6;
#[doc = "DIEPCTL7 (rw) register accessor: an alias for `Reg<DIEPCTL7_SPEC>`"]
pub type DIEPCTL7 = crate::Reg<diepctl7::DIEPCTL7_SPEC>;
#[doc = "OTG device endpoint-7 control register"]
pub mod diepctl7;
#[doc = "DIEPINT0 (rw) register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT1 (rw) register accessor: an alias for `Reg<DIEPINT1_SPEC>`"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "DIEPINT2 (rw) register accessor: an alias for `Reg<DIEPINT2_SPEC>`"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "DIEPINT3 (rw) register accessor: an alias for `Reg<DIEPINT3_SPEC>`"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "DIEPINT4 (rw) register accessor: an alias for `Reg<DIEPINT4_SPEC>`"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod diepint4;
#[doc = "DIEPINT5 (rw) register accessor: an alias for `Reg<DIEPINT5_SPEC>`"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod diepint5;
#[doc = "DIEPINT6 (rw) register accessor: an alias for `Reg<DIEPINT6_SPEC>`"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod diepint6;
#[doc = "DIEPINT7 (rw) register accessor: an alias for `Reg<DIEPINT7_SPEC>`"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod diepint7;
#[doc = "DIEPTSIZ0 (rw) register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DIEPDMA1 (rw) register accessor: an alias for `Reg<DIEPDMA1_SPEC>`"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod diepdma1;
#[doc = "DIEPDMA2 (rw) register accessor: an alias for `Reg<DIEPDMA2_SPEC>`"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod diepdma2;
#[doc = "DIEPDMA3 (rw) register accessor: an alias for `Reg<DIEPDMA3_SPEC>`"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod diepdma3;
#[doc = "DIEPDMA4 (rw) register accessor: an alias for `Reg<DIEPDMA4_SPEC>`"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod diepdma4;
#[doc = "DIEPDMA5 (rw) register accessor: an alias for `Reg<DIEPDMA5_SPEC>`"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod diepdma5;
#[doc = "DTXFSTS0 (r) register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS1 (r) register accessor: an alias for `Reg<DTXFSTS1_SPEC>`"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "DTXFSTS2 (r) register accessor: an alias for `Reg<DTXFSTS2_SPEC>`"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "DTXFSTS3 (r) register accessor: an alias for `Reg<DTXFSTS3_SPEC>`"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "DTXFSTS4 (r) register accessor: an alias for `Reg<DTXFSTS4_SPEC>`"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts4;
#[doc = "DTXFSTS5 (r) register accessor: an alias for `Reg<DTXFSTS5_SPEC>`"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts5;
#[doc = "DIEPTSIZ1 (rw) register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz1;
#[doc = "DIEPTSIZ2 (rw) register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz2;
#[doc = "DIEPTSIZ3 (rw) register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz3;
#[doc = "DIEPTSIZ4 (rw) register accessor: an alias for `Reg<DIEPTSIZ4_SPEC>`"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz4;
#[doc = "DIEPTSIZ5 (rw) register accessor: an alias for `Reg<DIEPTSIZ5_SPEC>`"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz5;
#[doc = "DOEPCTL0 (rw) register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL1 (rw) register accessor: an alias for `Reg<DOEPCTL1_SPEC>`"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "DOEPCTL2 (rw) register accessor: an alias for `Reg<DOEPCTL2_SPEC>`"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "DOEPCTL3 (rw) register accessor: an alias for `Reg<DOEPCTL3_SPEC>`"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "DOEPINT0 (rw) register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT1 (rw) register accessor: an alias for `Reg<DOEPINT1_SPEC>`"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "DOEPINT2 (rw) register accessor: an alias for `Reg<DOEPINT2_SPEC>`"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "DOEPINT3 (rw) register accessor: an alias for `Reg<DOEPINT3_SPEC>`"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "DOEPINT4 (rw) register accessor: an alias for `Reg<DOEPINT4_SPEC>`"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod doepint4;
#[doc = "DOEPINT5 (rw) register accessor: an alias for `Reg<DOEPINT5_SPEC>`"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod doepint5;
#[doc = "DOEPINT6 (rw) register accessor: an alias for `Reg<DOEPINT6_SPEC>`"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod doepint6;
#[doc = "DOEPINT7 (rw) register accessor: an alias for `Reg<DOEPINT7_SPEC>`"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod doepint7;
#[doc = "DOEPTSIZ0 (rw) register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "OTG_HS device endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DOEPTSIZ1 (rw) register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "DOEPTSIZ2 (rw) register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "DOEPTSIZ3 (rw) register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod doeptsiz3;
#[doc = "DOEPTSIZ4 (rw) register accessor: an alias for `Reg<DOEPTSIZ4_SPEC>`"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod doeptsiz4;
#[doc = "DIEPTSIZ6 (rw) register accessor: an alias for `Reg<DIEPTSIZ6_SPEC>`"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz6;
#[doc = "DTXFSTS6 (rw) register accessor: an alias for `Reg<DTXFSTS6_SPEC>`"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts6;
#[doc = "DIEPTSIZ7 (rw) register accessor: an alias for `Reg<DIEPTSIZ7_SPEC>`"]
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod dieptsiz7;
#[doc = "DTXFSTS7 (rw) register accessor: an alias for `Reg<DTXFSTS7_SPEC>`"]
pub type DTXFSTS7 = crate::Reg<dtxfsts7::DTXFSTS7_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts7;
#[doc = "DOEPCTL4 (rw) register accessor: an alias for `Reg<DOEPCTL4_SPEC>`"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod doepctl4;
#[doc = "DOEPCTL5 (rw) register accessor: an alias for `Reg<DOEPCTL5_SPEC>`"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod doepctl5;
#[doc = "DOEPCTL6 (rw) register accessor: an alias for `Reg<DOEPCTL6_SPEC>`"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = "OTG device endpoint-6 control register"]
pub mod doepctl6;
#[doc = "DOEPCTL7 (rw) register accessor: an alias for `Reg<DOEPCTL7_SPEC>`"]
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7_SPEC>;
#[doc = "OTG device endpoint-7 control register"]
pub mod doepctl7;
#[doc = "DOEPTSIZ5 (rw) register accessor: an alias for `Reg<DOEPTSIZ5_SPEC>`"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod doeptsiz5;
#[doc = "DOEPTSIZ6 (rw) register accessor: an alias for `Reg<DOEPTSIZ6_SPEC>`"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = "OTG_HS device endpoint-6 transfer size register"]
pub mod doeptsiz6;
#[doc = "DOEPTSIZ7 (rw) register accessor: an alias for `Reg<DOEPTSIZ7_SPEC>`"]
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7_SPEC>;
#[doc = "OTG_HS device endpoint-7 transfer size register"]
pub mod doeptsiz7;
