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
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub otg_hs_diepeachmsk1: OTG_HS_DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub otg_hs_doepeachmsk1: OTG_HS_DOEPEACHMSK1,
    _reserved15: [u8; 0x7c],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved16: [u8; 0x04],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved17: [u8; 0x04],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved20: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved21: [u8; 0x04],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved22: [u8; 0x04],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved25: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved26: [u8; 0x04],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved27: [u8; 0x04],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved30: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved31: [u8; 0x04],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved32: [u8; 0x04],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved35: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved36: [u8; 0x04],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved37: [u8; 0x04],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved40: [u8; 0x04],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_hs_diepctl5: OTG_HS_DIEPCTL5,
    _reserved41: [u8; 0x04],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub otg_hs_diepint5: OTG_HS_DIEPINT5,
    _reserved42: [u8; 0x04],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    _reserved43: [u8; 0x04],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved44: [u8; 0x04],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved45: [u8; 0x04],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved46: [u8; 0x14],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved47: [u8; 0x04],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved48: [u8; 0x0114],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved49: [u8; 0x04],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved50: [u8; 0x04],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    _reserved51: [u8; 0x0c],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved52: [u8; 0x04],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved53: [u8; 0x04],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    _reserved54: [u8; 0x0c],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved55: [u8; 0x04],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved56: [u8; 0x04],
    #[doc = "0x350 - OTG_HS device endpoint-3 transfer size register"]
    pub otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    _reserved57: [u8; 0x0c],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved58: [u8; 0x04],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved59: [u8; 0x04],
    #[doc = "0x370 - OTG_HS device endpoint-4 transfer size register"]
    pub otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    _reserved60: [u8; 0x14],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved61: [u8; 0x04],
    #[doc = "0x390 - OTG_HS device endpoint-5 transfer size register"]
    pub otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    _reserved62: [u8; 0x14],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved63: [u8; 0x1c],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved64: [u8; 0x1c],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub otg_hs_doepint7: OTG_HS_DOEPINT7,
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
#[doc = "OTG_HS_DIEPEACHMSK1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPEACHMSK1_SPEC>`"]
pub type OTG_HS_DIEPEACHMSK1 = crate::Reg<otg_hs_diepeachmsk1::OTG_HS_DIEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod otg_hs_diepeachmsk1;
#[doc = "OTG_HS_DOEPEACHMSK1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPEACHMSK1_SPEC>`"]
pub type OTG_HS_DOEPEACHMSK1 = crate::Reg<otg_hs_doepeachmsk1::OTG_HS_DOEPEACHMSK1_SPEC>;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod otg_hs_doepeachmsk1;
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
#[doc = "OTG_HS_DIEPDMA1 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA1_SPEC>`"]
pub type OTG_HS_DIEPDMA1 = crate::Reg<otg_hs_diepdma1::OTG_HS_DIEPDMA1_SPEC>;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod otg_hs_diepdma1;
#[doc = "OTG_HS_DIEPDMA2 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA2_SPEC>`"]
pub type OTG_HS_DIEPDMA2 = crate::Reg<otg_hs_diepdma2::OTG_HS_DIEPDMA2_SPEC>;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod otg_hs_diepdma2;
#[doc = "OTG_HS_DIEPDMA3 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA3_SPEC>`"]
pub type OTG_HS_DIEPDMA3 = crate::Reg<otg_hs_diepdma3::OTG_HS_DIEPDMA3_SPEC>;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod otg_hs_diepdma3;
#[doc = "OTG_HS_DIEPDMA4 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA4_SPEC>`"]
pub type OTG_HS_DIEPDMA4 = crate::Reg<otg_hs_diepdma4::OTG_HS_DIEPDMA4_SPEC>;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod otg_hs_diepdma4;
#[doc = "OTG_HS_DIEPDMA5 (rw) register accessor: an alias for `Reg<OTG_HS_DIEPDMA5_SPEC>`"]
pub type OTG_HS_DIEPDMA5 = crate::Reg<otg_hs_diepdma5::OTG_HS_DIEPDMA5_SPEC>;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod otg_hs_diepdma5;
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
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod otg_hs_doeptsiz0;
#[doc = "OTG_HS_DOEPTSIZ1 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ1_SPEC>`"]
pub type OTG_HS_DOEPTSIZ1 = crate::Reg<otg_hs_doeptsiz1::OTG_HS_DOEPTSIZ1_SPEC>;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod otg_hs_doeptsiz1;
#[doc = "OTG_HS_DOEPTSIZ2 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ2_SPEC>`"]
pub type OTG_HS_DOEPTSIZ2 = crate::Reg<otg_hs_doeptsiz2::OTG_HS_DOEPTSIZ2_SPEC>;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod otg_hs_doeptsiz2;
#[doc = "OTG_HS_DOEPTSIZ3 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ3_SPEC>`"]
pub type OTG_HS_DOEPTSIZ3 = crate::Reg<otg_hs_doeptsiz3::OTG_HS_DOEPTSIZ3_SPEC>;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod otg_hs_doeptsiz3;
#[doc = "OTG_HS_DOEPTSIZ4 (rw) register accessor: an alias for `Reg<OTG_HS_DOEPTSIZ4_SPEC>`"]
pub type OTG_HS_DOEPTSIZ4 = crate::Reg<otg_hs_doeptsiz4::OTG_HS_DOEPTSIZ4_SPEC>;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod otg_hs_doeptsiz4;
