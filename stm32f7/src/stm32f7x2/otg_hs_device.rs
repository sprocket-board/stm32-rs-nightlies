#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub otg_hs_dcfg: OTG_HS_DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub otg_hs_dctl: OTG_HS_DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub otg_hs_dsts: OTG_HS_DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub otg_hs_diepmsk: OTG_HS_DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub otg_hs_doepmsk: OTG_HS_DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub otg_hs_daint: OTG_HS_DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub otg_hs_daintmsk: OTG_HS_DAINTMSK,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub otg_hs_dvbusdis: OTG_HS_DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub otg_hs_dvbuspulse: OTG_HS_DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub otg_hs_dthrctl: OTG_HS_DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_hs_diepempmsk: OTG_HS_DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub otg_hs_deachint: OTG_HS_DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub otg_hs_deachintmsk: OTG_HS_DEACHINTMSK,
    _reserved13: [u8; 0xc0],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved14: [u8; 0x04],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved15: [u8; 0x04],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub otg_hs_diepdma0: OTG_HS_DIEPDMA0,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved18: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved19: [u8; 0x04],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved20: [u8; 0x04],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved23: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved24: [u8; 0x04],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved25: [u8; 0x04],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved28: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved29: [u8; 0x04],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved30: [u8; 0x04],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved33: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved34: [u8; 0x04],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved35: [u8; 0x04],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved38: [u8; 0x04],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_hs_diepctl5: OTG_HS_DIEPCTL5,
    _reserved39: [u8; 0x04],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub otg_hs_diepint5: OTG_HS_DIEPINT5,
    _reserved40: [u8; 0x04],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    #[doc = "0x1b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved43: [u8; 0x04],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved44: [u8; 0x04],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved45: [u8; 0x04],
    #[doc = "0x1d0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz6: OTG_HS_DIEPTSIZ6,
    #[doc = "0x1d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma6: OTG_HS_DIEPDMA6,
    #[doc = "0x1d8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts6: OTG_HS_DTXFSTS6,
    _reserved48: [u8; 0x04],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved49: [u8; 0x04],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved50: [u8; 0x04],
    #[doc = "0x1f0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz7: OTG_HS_DIEPTSIZ7,
    #[doc = "0x1f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma7: OTG_HS_DIEPDMA7,
    #[doc = "0x1f8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts7: OTG_HS_DTXFSTS7,
    _reserved53: [u8; 0x18],
    #[doc = "0x214 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma8: OTG_HS_DIEPDMA8,
    _reserved54: [u8; 0x1c],
    #[doc = "0x234 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma9: OTG_HS_DIEPDMA9,
    _reserved55: [u8; 0x1c],
    #[doc = "0x254 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma10: OTG_HS_DIEPDMA10,
    _reserved56: [u8; 0x1c],
    #[doc = "0x274 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma11: OTG_HS_DIEPDMA11,
    _reserved57: [u8; 0x1c],
    #[doc = "0x294 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma12: OTG_HS_DIEPDMA12,
    _reserved58: [u8; 0x1c],
    #[doc = "0x2b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma13: OTG_HS_DIEPDMA13,
    _reserved59: [u8; 0x1c],
    #[doc = "0x2d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma14: OTG_HS_DIEPDMA14,
    _reserved60: [u8; 0x1c],
    #[doc = "0x2f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_diepdma15: OTG_HS_DIEPDMA15,
    _reserved61: [u8; 0x08],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved62: [u8; 0x04],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved63: [u8; 0x04],
    #[doc = "0x310 - OTG_HS device endpoint-0 transfer size register"]
    pub otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    #[doc = "0x314 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma0: OTG_HS_DOEPDMA0,
    _reserved65: [u8; 0x08],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved66: [u8; 0x04],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved67: [u8; 0x04],
    #[doc = "0x330 - OTG_HS device endpoint-1 transfer size register"]
    pub otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    #[doc = "0x334 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma1: OTG_HS_DOEPDMA1,
    _reserved69: [u8; 0x08],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved70: [u8; 0x04],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved71: [u8; 0x04],
    #[doc = "0x350 - OTG_HS device endpoint-2 transfer size register"]
    pub otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    #[doc = "0x354 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma2: OTG_HS_DOEPDMA2,
    _reserved73: [u8; 0x08],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved74: [u8; 0x04],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved75: [u8; 0x04],
    #[doc = "0x370 - OTG_HS device endpoint-3 transfer size register"]
    pub otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    #[doc = "0x374 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma3: OTG_HS_DOEPDMA3,
    _reserved77: [u8; 0x08],
    #[doc = "0x380 - OTG device endpoint-4 control register"]
    pub otg_hs_doepctl4: OTG_HS_DOEPCTL4,
    _reserved78: [u8; 0x04],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved79: [u8; 0x04],
    #[doc = "0x390 - OTG_HS device endpoint-4 transfer size register"]
    pub otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    #[doc = "0x394 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma4: OTG_HS_DOEPDMA4,
    _reserved81: [u8; 0x08],
    #[doc = "0x3a0 - OTG device endpoint-5 control register"]
    pub otg_hs_doepctl5: OTG_HS_DOEPCTL5,
    _reserved82: [u8; 0x04],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved83: [u8; 0x04],
    #[doc = "0x3b0 - OTG_HS device endpoint-5 transfer size register"]
    pub otg_hs_doeptsiz5: OTG_HS_DOEPTSIZ5,
    #[doc = "0x3b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma5: OTG_HS_DOEPDMA5,
    _reserved85: [u8; 0x08],
    #[doc = "0x3c0 - OTG device endpoint-6 control register"]
    pub otg_hs_doepctl6: OTG_HS_DOEPCTL6,
    _reserved86: [u8; 0x04],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved87: [u8; 0x04],
    #[doc = "0x3d0 - OTG_HS device endpoint-6 transfer size register"]
    pub otg_hs_doeptsiz6: OTG_HS_DOEPTSIZ6,
    #[doc = "0x3d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma6: OTG_HS_DOEPDMA6,
    _reserved89: [u8; 0x08],
    #[doc = "0x3e0 - OTG device endpoint-7 control register"]
    pub otg_hs_doepctl7: OTG_HS_DOEPCTL7,
    _reserved90: [u8; 0x04],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub otg_hs_doepint7: OTG_HS_DOEPINT7,
    _reserved91: [u8; 0x04],
    #[doc = "0x3f0 - OTG_HS device endpoint-7 transfer size register"]
    pub otg_hs_doeptsiz7: OTG_HS_DOEPTSIZ7,
    #[doc = "0x3f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma7: OTG_HS_DOEPDMA7,
    _reserved93: [u8; 0x1c],
    #[doc = "0x414 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma8: OTG_HS_DOEPDMA8,
    _reserved94: [u8; 0x1c],
    #[doc = "0x434 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma9: OTG_HS_DOEPDMA9,
    _reserved95: [u8; 0x1c],
    #[doc = "0x454 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma10: OTG_HS_DOEPDMA10,
    _reserved96: [u8; 0x1c],
    #[doc = "0x474 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma11: OTG_HS_DOEPDMA11,
    _reserved97: [u8; 0x1c],
    #[doc = "0x494 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma12: OTG_HS_DOEPDMA12,
    _reserved98: [u8; 0x1c],
    #[doc = "0x4b4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma13: OTG_HS_DOEPDMA13,
    _reserved99: [u8; 0x1c],
    #[doc = "0x4d4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma14: OTG_HS_DOEPDMA14,
    _reserved100: [u8; 0x1c],
    #[doc = "0x4f4 - OTG Device channel-x DMA address register"]
    pub otg_hs_doepdma15: OTG_HS_DOEPDMA15,
}
#[doc = "OTG_HS_DCFG (rw) register accessor: an alias for `Reg<OTG_HS_DCFG_SPEC>`"]
pub type OTG_HS_DCFG = crate::Reg<otg_hs_dcfg::OTG_HS_DCFG_SPEC>;
#[doc = "OTG_HS device configuration register"]
pub mod otg_hs_dcfg;
#[doc = "OTG_HS_DCTL (rw) register accessor: an alias for `Reg<OTG_HS_DCTL_SPEC>`"]
pub type OTG_HS_DCTL = crate::Reg<otg_hs_dctl::OTG_HS_DCTL_SPEC>;
#[doc = "OTG_HS device control register"]
pub mod otg_hs_dctl;
#[doc = "OTG_HS_DSTS (r) register accessor: an alias for `Reg<OTG_HS_DSTS_SPEC>`"]
pub type OTG_HS_DSTS = crate::Reg<otg_hs_dsts::OTG_HS_DSTS_SPEC>;
#[doc = "OTG_HS device status register"]
pub mod otg_hs_dsts;
#[doc = "OTG_HS_DIEPMSK (rw) register accessor: an alias for `Reg<OTG_HS_DIEPMSK_SPEC>`"]
pub type OTG_HS_DIEPMSK = crate::Reg<otg_hs_diepmsk::OTG_HS_DIEPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod otg_hs_diepmsk;
#[doc = "OTG_HS_DOEPMSK (rw) register accessor: an alias for `Reg<OTG_HS_DOEPMSK_SPEC>`"]
pub type OTG_HS_DOEPMSK = crate::Reg<otg_hs_doepmsk::OTG_HS_DOEPMSK_SPEC>;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod otg_hs_doepmsk;
#[doc = "OTG_HS_DAINT (r) register accessor: an alias for `Reg<OTG_HS_DAINT_SPEC>`"]
pub type OTG_HS_DAINT = crate::Reg<otg_hs_daint::OTG_HS_DAINT_SPEC>;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod otg_hs_daint;
#[doc = "OTG_HS_DAINTMSK (rw) register accessor: an alias for `Reg<OTG_HS_DAINTMSK_SPEC>`"]
pub type OTG_HS_DAINTMSK = crate::Reg<otg_hs_daintmsk::OTG_HS_DAINTMSK_SPEC>;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod otg_hs_daintmsk;
#[doc = "OTG_HS_DVBUSDIS (rw) register accessor: an alias for `Reg<OTG_HS_DVBUSDIS_SPEC>`"]
pub type OTG_HS_DVBUSDIS = crate::Reg<otg_hs_dvbusdis::OTG_HS_DVBUSDIS_SPEC>;
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod otg_hs_dvbusdis;
#[doc = "OTG_HS_DVBUSPULSE (rw) register accessor: an alias for `Reg<OTG_HS_DVBUSPULSE_SPEC>`"]
pub type OTG_HS_DVBUSPULSE = crate::Reg<otg_hs_dvbuspulse::OTG_HS_DVBUSPULSE_SPEC>;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod otg_hs_dvbuspulse;
#[doc = "OTG_HS_DTHRCTL (rw) register accessor: an alias for `Reg<OTG_HS_DTHRCTL_SPEC>`"]
pub type OTG_HS_DTHRCTL = crate::Reg<otg_hs_dthrctl::OTG_HS_DTHRCTL_SPEC>;
#[doc = "OTG_HS Device threshold control register"]
pub mod otg_hs_dthrctl;
#[doc = "OTG_HS_DIEPEMPMSK (rw) register accessor: an alias for `Reg<OTG_HS_DIEPEMPMSK_SPEC>`"]
pub type OTG_HS_DIEPEMPMSK = crate::Reg<otg_hs_diepempmsk::OTG_HS_DIEPEMPMSK_SPEC>;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_hs_diepempmsk;
#[doc = "OTG_HS_DEACHINT (rw) register accessor: an alias for `Reg<OTG_HS_DEACHINT_SPEC>`"]
pub type OTG_HS_DEACHINT = crate::Reg<otg_hs_deachint::OTG_HS_DEACHINT_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod otg_hs_deachint;
#[doc = "OTG_HS_DEACHINTMSK (rw) register accessor: an alias for `Reg<OTG_HS_DEACHINTMSK_SPEC>`"]
pub type OTG_HS_DEACHINTMSK = crate::Reg<otg_hs_deachintmsk::OTG_HS_DEACHINTMSK_SPEC>;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod otg_hs_deachintmsk;
#[doc = "OTG_HS_DIEPCTL0 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL0_SPEC>`"]
pub type OTG_HS_DIEPCTL0 = crate::Reg<otg_hs_diepctl0::OTG_HS_DIEPCTL0_SPEC>;
#[doc = "OTG device endpoint-0 control register"]
pub mod otg_hs_diepctl0;
#[doc = "OTG_HS_DIEPCTL1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL1_SPEC>`"]
pub type OTG_HS_DIEPCTL1 = crate::Reg<otg_hs_diepctl1::OTG_HS_DIEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_diepctl1;
#[doc = "OTG_HS_DIEPCTL2 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL2_SPEC>`"]
pub type OTG_HS_DIEPCTL2 = crate::Reg<otg_hs_diepctl2::OTG_HS_DIEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_diepctl2;
#[doc = "OTG_HS_DIEPCTL3 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL3_SPEC>`"]
pub type OTG_HS_DIEPCTL3 = crate::Reg<otg_hs_diepctl3::OTG_HS_DIEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_diepctl3;
#[doc = "OTG_HS_DIEPCTL4 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL4_SPEC>`"]
pub type OTG_HS_DIEPCTL4 = crate::Reg<otg_hs_diepctl4::OTG_HS_DIEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_diepctl4;
#[doc = "OTG_HS_DIEPCTL5 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL5_SPEC>`"]
pub type OTG_HS_DIEPCTL5 = crate::Reg<otg_hs_diepctl5::OTG_HS_DIEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_diepctl5;
#[doc = "OTG_HS_DIEPCTL6 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL6_SPEC>`"]
pub type OTG_HS_DIEPCTL6 = crate::Reg<otg_hs_diepctl6::OTG_HS_DIEPCTL6_SPEC>;
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_diepctl6;
#[doc = "OTG_HS_DIEPCTL7 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPCTL7_SPEC>`"]
pub type OTG_HS_DIEPCTL7 = crate::Reg<otg_hs_diepctl7::OTG_HS_DIEPCTL7_SPEC>;
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_diepctl7;
#[doc = "OTG_HS_DIEPINT0 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT0_SPEC>`"]
pub type OTG_HS_DIEPINT0 = crate::Reg<otg_hs_diepint0::OTG_HS_DIEPINT0_SPEC>;
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod otg_hs_diepint0;
#[doc = "OTG_HS_DIEPINT1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT1_SPEC>`"]
pub type OTG_HS_DIEPINT1 = crate::Reg<otg_hs_diepint1::OTG_HS_DIEPINT1_SPEC>;
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod otg_hs_diepint1;
#[doc = "OTG_HS_DIEPINT2 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT2_SPEC>`"]
pub type OTG_HS_DIEPINT2 = crate::Reg<otg_hs_diepint2::OTG_HS_DIEPINT2_SPEC>;
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod otg_hs_diepint2;
#[doc = "OTG_HS_DIEPINT3 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT3_SPEC>`"]
pub type OTG_HS_DIEPINT3 = crate::Reg<otg_hs_diepint3::OTG_HS_DIEPINT3_SPEC>;
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod otg_hs_diepint3;
#[doc = "OTG_HS_DIEPINT4 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT4_SPEC>`"]
pub type OTG_HS_DIEPINT4 = crate::Reg<otg_hs_diepint4::OTG_HS_DIEPINT4_SPEC>;
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod otg_hs_diepint4;
#[doc = "OTG_HS_DIEPINT5 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT5_SPEC>`"]
pub type OTG_HS_DIEPINT5 = crate::Reg<otg_hs_diepint5::OTG_HS_DIEPINT5_SPEC>;
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod otg_hs_diepint5;
#[doc = "OTG_HS_DIEPINT6 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT6_SPEC>`"]
pub type OTG_HS_DIEPINT6 = crate::Reg<otg_hs_diepint6::OTG_HS_DIEPINT6_SPEC>;
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod otg_hs_diepint6;
#[doc = "OTG_HS_DIEPINT7 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPINT7_SPEC>`"]
pub type OTG_HS_DIEPINT7 = crate::Reg<otg_hs_diepint7::OTG_HS_DIEPINT7_SPEC>;
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod otg_hs_diepint7;
#[doc = "OTG_HS_DIEPTSIZ0 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ0_SPEC>`"]
pub type OTG_HS_DIEPTSIZ0 = crate::Reg<otg_hs_dieptsiz0::OTG_HS_DIEPTSIZ0_SPEC>;
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod otg_hs_dieptsiz0;
#[doc = "OTG_HS_DIEPDMA0 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA0_SPEC>`"]
pub type OTG_HS_DIEPDMA0 = crate::Reg<otg_hs_diepdma0::OTG_HS_DIEPDMA0_SPEC>;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod otg_hs_diepdma0;
#[doc = "OTG_HS_DIEPDMA1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA1_SPEC>`"]
pub type OTG_HS_DIEPDMA1 = crate::Reg<otg_hs_diepdma1::OTG_HS_DIEPDMA1_SPEC>;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod otg_hs_diepdma1;
#[doc = "OTG_HS_DIEPDMA2 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA2_SPEC>`"]
pub type OTG_HS_DIEPDMA2 = crate::Reg<otg_hs_diepdma2::OTG_HS_DIEPDMA2_SPEC>;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod otg_hs_diepdma2;
#[doc = "OTG_HS_DIEPDMA3 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA3_SPEC>`"]
pub type OTG_HS_DIEPDMA3 = crate::Reg<otg_hs_diepdma3::OTG_HS_DIEPDMA3_SPEC>;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod otg_hs_diepdma3;
#[doc = "OTG_HS_DIEPDMA4 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA4_SPEC>`"]
pub type OTG_HS_DIEPDMA4 = crate::Reg<otg_hs_diepdma4::OTG_HS_DIEPDMA4_SPEC>;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod otg_hs_diepdma4;
#[doc = "OTG_HS_DTXFSTS0 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS0_SPEC>`"]
pub type OTG_HS_DTXFSTS0 = crate::Reg<otg_hs_dtxfsts0::OTG_HS_DTXFSTS0_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts0;
#[doc = "OTG_HS_DTXFSTS1 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS1_SPEC>`"]
pub type OTG_HS_DTXFSTS1 = crate::Reg<otg_hs_dtxfsts1::OTG_HS_DTXFSTS1_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts1;
#[doc = "OTG_HS_DTXFSTS2 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS2_SPEC>`"]
pub type OTG_HS_DTXFSTS2 = crate::Reg<otg_hs_dtxfsts2::OTG_HS_DTXFSTS2_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts2;
#[doc = "OTG_HS_DTXFSTS3 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS3_SPEC>`"]
pub type OTG_HS_DTXFSTS3 = crate::Reg<otg_hs_dtxfsts3::OTG_HS_DTXFSTS3_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts3;
#[doc = "OTG_HS_DTXFSTS4 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS4_SPEC>`"]
pub type OTG_HS_DTXFSTS4 = crate::Reg<otg_hs_dtxfsts4::OTG_HS_DTXFSTS4_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts4;
#[doc = "OTG_HS_DTXFSTS5 (r) register accessor: an alias for `Reg<OTG_HS_DTXFSTS5_SPEC>`"]
pub type OTG_HS_DTXFSTS5 = crate::Reg<otg_hs_dtxfsts5::OTG_HS_DTXFSTS5_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts5;
#[doc = "OTG_HS_DIEPTSIZ1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ1_SPEC>`"]
pub type OTG_HS_DIEPTSIZ1 = crate::Reg<otg_hs_dieptsiz1::OTG_HS_DIEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz1;
#[doc = "OTG_HS_DIEPTSIZ2 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ2_SPEC>`"]
pub type OTG_HS_DIEPTSIZ2 = crate::Reg<otg_hs_dieptsiz2::OTG_HS_DIEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz2;
#[doc = "OTG_HS_DIEPTSIZ3 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ3_SPEC>`"]
pub type OTG_HS_DIEPTSIZ3 = crate::Reg<otg_hs_dieptsiz3::OTG_HS_DIEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz3;
#[doc = "OTG_HS_DIEPTSIZ4 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ4_SPEC>`"]
pub type OTG_HS_DIEPTSIZ4 = crate::Reg<otg_hs_dieptsiz4::OTG_HS_DIEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz4;
#[doc = "OTG_HS_DIEPTSIZ5 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ5_SPEC>`"]
pub type OTG_HS_DIEPTSIZ5 = crate::Reg<otg_hs_dieptsiz5::OTG_HS_DIEPTSIZ5_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz5;
#[doc = "OTG_HS_DOEPCTL0 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL0_SPEC>`"]
pub type OTG_HS_DOEPCTL0 = crate::Reg<otg_hs_doepctl0::OTG_HS_DOEPCTL0_SPEC>;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod otg_hs_doepctl0;
#[doc = "OTG_HS_DOEPCTL1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL1_SPEC>`"]
pub type OTG_HS_DOEPCTL1 = crate::Reg<otg_hs_doepctl1::OTG_HS_DOEPCTL1_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_doepctl1;
#[doc = "OTG_HS_DOEPCTL2 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL2_SPEC>`"]
pub type OTG_HS_DOEPCTL2 = crate::Reg<otg_hs_doepctl2::OTG_HS_DOEPCTL2_SPEC>;
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_doepctl2;
#[doc = "OTG_HS_DOEPCTL3 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL3_SPEC>`"]
pub type OTG_HS_DOEPCTL3 = crate::Reg<otg_hs_doepctl3::OTG_HS_DOEPCTL3_SPEC>;
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_doepctl3;
#[doc = "OTG_HS_DOEPINT0 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT0_SPEC>`"]
pub type OTG_HS_DOEPINT0 = crate::Reg<otg_hs_doepint0::OTG_HS_DOEPINT0_SPEC>;
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod otg_hs_doepint0;
#[doc = "OTG_HS_DOEPINT1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT1_SPEC>`"]
pub type OTG_HS_DOEPINT1 = crate::Reg<otg_hs_doepint1::OTG_HS_DOEPINT1_SPEC>;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod otg_hs_doepint1;
#[doc = "OTG_HS_DOEPINT2 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT2_SPEC>`"]
pub type OTG_HS_DOEPINT2 = crate::Reg<otg_hs_doepint2::OTG_HS_DOEPINT2_SPEC>;
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod otg_hs_doepint2;
#[doc = "OTG_HS_DOEPINT3 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT3_SPEC>`"]
pub type OTG_HS_DOEPINT3 = crate::Reg<otg_hs_doepint3::OTG_HS_DOEPINT3_SPEC>;
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod otg_hs_doepint3;
#[doc = "OTG_HS_DOEPINT4 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT4_SPEC>`"]
pub type OTG_HS_DOEPINT4 = crate::Reg<otg_hs_doepint4::OTG_HS_DOEPINT4_SPEC>;
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod otg_hs_doepint4;
#[doc = "OTG_HS_DOEPINT5 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT5_SPEC>`"]
pub type OTG_HS_DOEPINT5 = crate::Reg<otg_hs_doepint5::OTG_HS_DOEPINT5_SPEC>;
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod otg_hs_doepint5;
#[doc = "OTG_HS_DOEPINT6 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT6_SPEC>`"]
pub type OTG_HS_DOEPINT6 = crate::Reg<otg_hs_doepint6::OTG_HS_DOEPINT6_SPEC>;
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod otg_hs_doepint6;
#[doc = "OTG_HS_DOEPINT7 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPINT7_SPEC>`"]
pub type OTG_HS_DOEPINT7 = crate::Reg<otg_hs_doepint7::OTG_HS_DOEPINT7_SPEC>;
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod otg_hs_doepint7;
#[doc = "OTG_HS_DOEPTSIZ0 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ0_SPEC>`"]
pub type OTG_HS_DOEPTSIZ0 = crate::Reg<otg_hs_doeptsiz0::OTG_HS_DOEPTSIZ0_SPEC>;
#[doc = "OTG_HS device endpoint-0 transfer size register"]
pub mod otg_hs_doeptsiz0;
#[doc = "OTG_HS_DOEPTSIZ1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ1_SPEC>`"]
pub type OTG_HS_DOEPTSIZ1 = crate::Reg<otg_hs_doeptsiz1::OTG_HS_DOEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod otg_hs_doeptsiz1;
#[doc = "OTG_HS_DOEPTSIZ2 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ2_SPEC>`"]
pub type OTG_HS_DOEPTSIZ2 = crate::Reg<otg_hs_doeptsiz2::OTG_HS_DOEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod otg_hs_doeptsiz2;
#[doc = "OTG_HS_DOEPTSIZ3 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ3_SPEC>`"]
pub type OTG_HS_DOEPTSIZ3 = crate::Reg<otg_hs_doeptsiz3::OTG_HS_DOEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod otg_hs_doeptsiz3;
#[doc = "OTG_HS_DOEPTSIZ4 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ4_SPEC>`"]
pub type OTG_HS_DOEPTSIZ4 = crate::Reg<otg_hs_doeptsiz4::OTG_HS_DOEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod otg_hs_doeptsiz4;
#[doc = "OTG_HS_DIEPTSIZ6 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ6_SPEC>`"]
pub type OTG_HS_DIEPTSIZ6 = crate::Reg<otg_hs_dieptsiz6::OTG_HS_DIEPTSIZ6_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz6;
#[doc = "OTG_HS_DTXFSTS6 (rw) register accessor: an alias for `Reg<OTG_HS_DTXFSTS6_SPEC>`"]
pub type OTG_HS_DTXFSTS6 = crate::Reg<otg_hs_dtxfsts6::OTG_HS_DTXFSTS6_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts6;
#[doc = "OTG_HS_DIEPTSIZ7 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPTSIZ7_SPEC>`"]
pub type OTG_HS_DIEPTSIZ7 = crate::Reg<otg_hs_dieptsiz7::OTG_HS_DIEPTSIZ7_SPEC>;
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz7;
#[doc = "OTG_HS_DTXFSTS7 (rw) register accessor: an alias for `Reg<OTG_HS_DTXFSTS7_SPEC>`"]
pub type OTG_HS_DTXFSTS7 = crate::Reg<otg_hs_dtxfsts7::OTG_HS_DTXFSTS7_SPEC>;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts7;
#[doc = "OTG_HS_DOEPCTL4 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL4_SPEC>`"]
pub type OTG_HS_DOEPCTL4 = crate::Reg<otg_hs_doepctl4::OTG_HS_DOEPCTL4_SPEC>;
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_doepctl4;
#[doc = "OTG_HS_DOEPCTL5 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL5_SPEC>`"]
pub type OTG_HS_DOEPCTL5 = crate::Reg<otg_hs_doepctl5::OTG_HS_DOEPCTL5_SPEC>;
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_doepctl5;
#[doc = "OTG_HS_DOEPCTL6 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL6_SPEC>`"]
pub type OTG_HS_DOEPCTL6 = crate::Reg<otg_hs_doepctl6::OTG_HS_DOEPCTL6_SPEC>;
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_doepctl6;
#[doc = "OTG_HS_DOEPCTL7 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPCTL7_SPEC>`"]
pub type OTG_HS_DOEPCTL7 = crate::Reg<otg_hs_doepctl7::OTG_HS_DOEPCTL7_SPEC>;
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_doepctl7;
#[doc = "OTG_HS_DOEPTSIZ5 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ5_SPEC>`"]
pub type OTG_HS_DOEPTSIZ5 = crate::Reg<otg_hs_doeptsiz5::OTG_HS_DOEPTSIZ5_SPEC>;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod otg_hs_doeptsiz5;
#[doc = "OTG_HS_DOEPTSIZ6 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ6_SPEC>`"]
pub type OTG_HS_DOEPTSIZ6 = crate::Reg<otg_hs_doeptsiz6::OTG_HS_DOEPTSIZ6_SPEC>;
#[doc = "OTG_HS device endpoint-6 transfer size register"]
pub mod otg_hs_doeptsiz6;
#[doc = "OTG_HS_DOEPTSIZ7 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ7_SPEC>`"]
pub type OTG_HS_DOEPTSIZ7 = crate::Reg<otg_hs_doeptsiz7::OTG_HS_DOEPTSIZ7_SPEC>;
#[doc = "OTG_HS device endpoint-7 transfer size register"]
pub mod otg_hs_doeptsiz7;
#[doc = "OTG_HS_DOEPDMA0 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA0_SPEC>`"]
pub type OTG_HS_DOEPDMA0 = crate::Reg<otg_hs_doepdma0::OTG_HS_DOEPDMA0_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma0;
#[doc = "OTG_HS_DOEPDMA1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA1_SPEC>`"]
pub type OTG_HS_DOEPDMA1 = crate::Reg<otg_hs_doepdma1::OTG_HS_DOEPDMA1_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma1;
#[doc = "OTG_HS_DOEPDMA2 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA2_SPEC>`"]
pub type OTG_HS_DOEPDMA2 = crate::Reg<otg_hs_doepdma2::OTG_HS_DOEPDMA2_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma2;
#[doc = "OTG_HS_DOEPDMA3 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA3_SPEC>`"]
pub type OTG_HS_DOEPDMA3 = crate::Reg<otg_hs_doepdma3::OTG_HS_DOEPDMA3_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma3;
#[doc = "OTG_HS_DOEPDMA4 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA4_SPEC>`"]
pub type OTG_HS_DOEPDMA4 = crate::Reg<otg_hs_doepdma4::OTG_HS_DOEPDMA4_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma4;
#[doc = "OTG_HS_DOEPDMA5 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA5_SPEC>`"]
pub type OTG_HS_DOEPDMA5 = crate::Reg<otg_hs_doepdma5::OTG_HS_DOEPDMA5_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma5;
#[doc = "OTG_HS_DOEPDMA6 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA6_SPEC>`"]
pub type OTG_HS_DOEPDMA6 = crate::Reg<otg_hs_doepdma6::OTG_HS_DOEPDMA6_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma6;
#[doc = "OTG_HS_DOEPDMA7 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA7_SPEC>`"]
pub type OTG_HS_DOEPDMA7 = crate::Reg<otg_hs_doepdma7::OTG_HS_DOEPDMA7_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma7;
#[doc = "OTG_HS_DOEPDMA8 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA8_SPEC>`"]
pub type OTG_HS_DOEPDMA8 = crate::Reg<otg_hs_doepdma8::OTG_HS_DOEPDMA8_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma8;
#[doc = "OTG_HS_DOEPDMA9 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA9_SPEC>`"]
pub type OTG_HS_DOEPDMA9 = crate::Reg<otg_hs_doepdma9::OTG_HS_DOEPDMA9_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma9;
#[doc = "OTG_HS_DOEPDMA10 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA10_SPEC>`"]
pub type OTG_HS_DOEPDMA10 = crate::Reg<otg_hs_doepdma10::OTG_HS_DOEPDMA10_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma10;
#[doc = "OTG_HS_DOEPDMA11 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA11_SPEC>`"]
pub type OTG_HS_DOEPDMA11 = crate::Reg<otg_hs_doepdma11::OTG_HS_DOEPDMA11_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma11;
#[doc = "OTG_HS_DOEPDMA12 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA12_SPEC>`"]
pub type OTG_HS_DOEPDMA12 = crate::Reg<otg_hs_doepdma12::OTG_HS_DOEPDMA12_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma12;
#[doc = "OTG_HS_DOEPDMA13 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA13_SPEC>`"]
pub type OTG_HS_DOEPDMA13 = crate::Reg<otg_hs_doepdma13::OTG_HS_DOEPDMA13_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma13;
#[doc = "OTG_HS_DOEPDMA14 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA14_SPEC>`"]
pub type OTG_HS_DOEPDMA14 = crate::Reg<otg_hs_doepdma14::OTG_HS_DOEPDMA14_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma14;
#[doc = "OTG_HS_DOEPDMA15 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPDMA15_SPEC>`"]
pub type OTG_HS_DOEPDMA15 = crate::Reg<otg_hs_doepdma15::OTG_HS_DOEPDMA15_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_doepdma15;
#[doc = "OTG_HS_DIEPDMA5 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA5_SPEC>`"]
pub type OTG_HS_DIEPDMA5 = crate::Reg<otg_hs_diepdma5::OTG_HS_DIEPDMA5_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma5;
#[doc = "OTG_HS_DIEPDMA6 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA6_SPEC>`"]
pub type OTG_HS_DIEPDMA6 = crate::Reg<otg_hs_diepdma6::OTG_HS_DIEPDMA6_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma6;
#[doc = "OTG_HS_DIEPDMA7 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA7_SPEC>`"]
pub type OTG_HS_DIEPDMA7 = crate::Reg<otg_hs_diepdma7::OTG_HS_DIEPDMA7_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma7;
#[doc = "OTG_HS_DIEPDMA8 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA8_SPEC>`"]
pub type OTG_HS_DIEPDMA8 = crate::Reg<otg_hs_diepdma8::OTG_HS_DIEPDMA8_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma8;
#[doc = "OTG_HS_DIEPDMA9 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA9_SPEC>`"]
pub type OTG_HS_DIEPDMA9 = crate::Reg<otg_hs_diepdma9::OTG_HS_DIEPDMA9_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma9;
#[doc = "OTG_HS_DIEPDMA10 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA10_SPEC>`"]
pub type OTG_HS_DIEPDMA10 = crate::Reg<otg_hs_diepdma10::OTG_HS_DIEPDMA10_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma10;
#[doc = "OTG_HS_DIEPDMA11 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA11_SPEC>`"]
pub type OTG_HS_DIEPDMA11 = crate::Reg<otg_hs_diepdma11::OTG_HS_DIEPDMA11_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma11;
#[doc = "OTG_HS_DIEPDMA12 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA12_SPEC>`"]
pub type OTG_HS_DIEPDMA12 = crate::Reg<otg_hs_diepdma12::OTG_HS_DIEPDMA12_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma12;
#[doc = "OTG_HS_DIEPDMA13 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA13_SPEC>`"]
pub type OTG_HS_DIEPDMA13 = crate::Reg<otg_hs_diepdma13::OTG_HS_DIEPDMA13_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma13;
#[doc = "OTG_HS_DIEPDMA14 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA14_SPEC>`"]
pub type OTG_HS_DIEPDMA14 = crate::Reg<otg_hs_diepdma14::OTG_HS_DIEPDMA14_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma14;
#[doc = "OTG_HS_DIEPDMA15 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA15_SPEC>`"]
pub type OTG_HS_DIEPDMA15 = crate::Reg<otg_hs_diepdma15::OTG_HS_DIEPDMA15_SPEC>;
#[doc = "OTG Device channel-x DMA address register"]
pub mod otg_hs_diepdma15;
