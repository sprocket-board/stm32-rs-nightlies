#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub daint: DAINT,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 0x04],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 0xc8],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub diepctl0: DIEPCTL0,
    _reserved11: [u8; 0x04],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub diepint0: DIEPINT0,
    _reserved12: [u8; 0x04],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    _reserved13: [u8; 0x04],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved14: [u8; 0x04],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL,
    _reserved15: [u8; 0x04],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diepint1: DIEPINT,
    _reserved16: [u8; 0x04],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub dieptsiz1: DIEPTSIZ,
    _reserved17: [u8; 0x04],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS,
    _reserved18: [u8; 0x04],
    #[doc = "0x140 - OTG device endpoint-1 control register"]
    pub diepctl2: DIEPCTL,
    _reserved19: [u8; 0x04],
    #[doc = "0x148 - device endpoint-1 interrupt register"]
    pub diepint2: DIEPINT,
    _reserved20: [u8; 0x04],
    #[doc = "0x150 - device endpoint-1 transfer size register"]
    pub dieptsiz2: DIEPTSIZ,
    _reserved21: [u8; 0x04],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS,
    _reserved22: [u8; 0x04],
    #[doc = "0x160 - OTG device endpoint-1 control register"]
    pub diepctl3: DIEPCTL,
    _reserved23: [u8; 0x04],
    #[doc = "0x168 - device endpoint-1 interrupt register"]
    pub diepint3: DIEPINT,
    _reserved24: [u8; 0x04],
    #[doc = "0x170 - device endpoint-1 transfer size register"]
    pub dieptsiz3: DIEPTSIZ,
    _reserved25: [u8; 0x04],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS,
    _reserved26: [u8; 0x04],
    #[doc = "0x180 - OTG device endpoint-1 control register"]
    pub diepctl4: DIEPCTL,
    _reserved27: [u8; 0x04],
    #[doc = "0x188 - device endpoint-1 interrupt register"]
    pub diepint4: DIEPINT,
    _reserved28: [u8; 0x04],
    #[doc = "0x190 - device endpoint-1 transfer size register"]
    pub dieptsiz4: DIEPTSIZ,
    _reserved29: [u8; 0x04],
    #[doc = "0x198 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts4: DTXFSTS,
    _reserved30: [u8; 0x04],
    #[doc = "0x1a0 - OTG device endpoint-1 control register"]
    pub diepctl5: DIEPCTL,
    _reserved31: [u8; 0x04],
    #[doc = "0x1a8 - device endpoint-1 interrupt register"]
    pub diepint5: DIEPINT,
    _reserved32: [u8; 0x04],
    #[doc = "0x1b0 - device endpoint-1 transfer size register"]
    pub dieptsiz5: DIEPTSIZ,
    _reserved33: [u8; 0x04],
    #[doc = "0x1b8 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts5: DTXFSTS,
    _reserved34: [u8; 0x0144],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved35: [u8; 0x04],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved36: [u8; 0x04],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved37: [u8; 0x0c],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doepctl1: DOEPCTL,
    _reserved38: [u8; 0x04],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub doepint1: DOEPINT,
    _reserved39: [u8; 0x04],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz1: DOEPTSIZ,
    _reserved40: [u8; 0x0c],
    #[doc = "0x340 - device endpoint-1 control register"]
    pub doepctl2: DOEPCTL,
    _reserved41: [u8; 0x04],
    #[doc = "0x348 - device endpoint-1 interrupt register"]
    pub doepint2: DOEPINT,
    _reserved42: [u8; 0x04],
    #[doc = "0x350 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz2: DOEPTSIZ,
    _reserved43: [u8; 0x0c],
    #[doc = "0x360 - device endpoint-1 control register"]
    pub doepctl3: DOEPCTL,
    _reserved44: [u8; 0x04],
    #[doc = "0x368 - device endpoint-1 interrupt register"]
    pub doepint3: DOEPINT,
    _reserved45: [u8; 0x04],
    #[doc = "0x370 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz3: DOEPTSIZ,
    _reserved46: [u8; 0x0c],
    #[doc = "0x380 - device endpoint-1 control register"]
    pub doepctl4: DOEPCTL,
    _reserved47: [u8; 0x04],
    #[doc = "0x388 - device endpoint-1 interrupt register"]
    pub doepint4: DOEPINT,
    _reserved48: [u8; 0x04],
    #[doc = "0x390 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz4: DOEPTSIZ,
    _reserved49: [u8; 0x0c],
    #[doc = "0x3a0 - device endpoint-1 control register"]
    pub doepctl5: DOEPCTL,
    _reserved50: [u8; 0x04],
    #[doc = "0x3a8 - device endpoint-1 interrupt register"]
    pub doepint5: DOEPINT,
    _reserved51: [u8; 0x04],
    #[doc = "0x3b0 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz5: DOEPTSIZ,
}
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod diepctl0;
#[doc = "DIEPCTL (rw) register accessor: an alias for `Reg<DIEPCTL_SPEC>`"]
pub type DIEPCTL = crate::Reg<diepctl::DIEPCTL_SPEC>;
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl;
#[doc = "DOEPCTL0 (rw) register accessor: an alias for `Reg<DOEPCTL0_SPEC>`"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "device endpoint-0 control register"]
pub mod doepctl0;
#[doc = "DOEPCTL (rw) register accessor: an alias for `Reg<DOEPCTL_SPEC>`"]
pub type DOEPCTL = crate::Reg<doepctl::DOEPCTL_SPEC>;
#[doc = "device endpoint-1 control register"]
pub mod doepctl;
#[doc = "DIEPINT0 (rw) register accessor: an alias for `Reg<DIEPINT0_SPEC>`"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "device endpoint-x interrupt register"]
pub mod diepint0;
#[doc = "DIEPINT (rw) register accessor: an alias for `Reg<DIEPINT_SPEC>`"]
pub type DIEPINT = crate::Reg<diepint::DIEPINT_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod diepint;
#[doc = "DOEPINT0 (rw) register accessor: an alias for `Reg<DOEPINT0_SPEC>`"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "DOEPINT (rw) register accessor: an alias for `Reg<DOEPINT_SPEC>`"]
pub type DOEPINT = crate::Reg<doepint::DOEPINT_SPEC>;
#[doc = "device endpoint-1 interrupt register"]
pub mod doepint;
#[doc = "DIEPTSIZ0 (rw) register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "device endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "DOEPTSIZ0 (rw) register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "DIEPTSIZ (rw) register accessor: an alias for `Reg<DIEPTSIZ_SPEC>`"]
pub type DIEPTSIZ = crate::Reg<dieptsiz::DIEPTSIZ_SPEC>;
#[doc = "device endpoint-1 transfer size register"]
pub mod dieptsiz;
#[doc = "DTXFSTS0 (r) register accessor: an alias for `Reg<DTXFSTS0_SPEC>`"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "DTXFSTS (r) register accessor: an alias for `Reg<DTXFSTS_SPEC>`"]
pub type DTXFSTS = crate::Reg<dtxfsts::DTXFSTS_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts;
#[doc = "DOEPTSIZ (rw) register accessor: an alias for `Reg<DOEPTSIZ_SPEC>`"]
pub type DOEPTSIZ = crate::Reg<doeptsiz::DOEPTSIZ_SPEC>;
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod doeptsiz;
