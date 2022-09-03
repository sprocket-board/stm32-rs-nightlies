#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sr: SR,
    #[doc = "0x04 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x08 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x0c - sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x10 - sample time register 2"]
    pub smpr2: SMPR2,
    #[doc = "0x14..0x24 - injected channel data offset register x"]
    pub jofr: [JOFR; 4],
    #[doc = "0x24 - watchdog higher threshold register"]
    pub htr: HTR,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub ltr: LTR,
    #[doc = "0x2c - regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x30 - regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x34 - regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x38 - injected sequence register"]
    pub jsqr: JSQR,
    #[doc = "0x3c..0x4c - injected data register x"]
    pub jdr: [JDR; 4],
    #[doc = "0x4c - regular data register"]
    pub dr: DR,
}
impl RegisterBlock {
    #[doc = "0x14 - injected channel data offset register x"]
    #[inline(always)]
    pub fn jofr1(&self) -> &JOFR {
        &self.jofr[0]
    }
    #[doc = "0x18 - injected channel data offset register x"]
    #[inline(always)]
    pub fn jofr2(&self) -> &JOFR {
        &self.jofr[1]
    }
    #[doc = "0x1c - injected channel data offset register x"]
    #[inline(always)]
    pub fn jofr3(&self) -> &JOFR {
        &self.jofr[2]
    }
    #[doc = "0x20 - injected channel data offset register x"]
    #[inline(always)]
    pub fn jofr4(&self) -> &JOFR {
        &self.jofr[3]
    }
    #[doc = "0x3c - injected data register x"]
    #[inline(always)]
    pub fn jdr1(&self) -> &JDR {
        &self.jdr[0]
    }
    #[doc = "0x40 - injected data register x"]
    #[inline(always)]
    pub fn jdr2(&self) -> &JDR {
        &self.jdr[1]
    }
    #[doc = "0x44 - injected data register x"]
    #[inline(always)]
    pub fn jdr3(&self) -> &JDR {
        &self.jdr[2]
    }
    #[doc = "0x48 - injected data register x"]
    #[inline(always)]
    pub fn jdr4(&self) -> &JDR {
        &self.jdr[3]
    }
}
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "JOFR (rw) register accessor: an alias for `Reg<JOFR_SPEC>`"]
pub type JOFR = crate::Reg<jofr::JOFR_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr;
#[doc = "HTR (rw) register accessor: an alias for `Reg<HTR_SPEC>`"]
pub type HTR = crate::Reg<htr::HTR_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "LTR (rw) register accessor: an alias for `Reg<LTR_SPEC>`"]
pub type LTR = crate::Reg<ltr::LTR_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "SQR1 (rw) register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "JDR (r) register accessor: an alias for `Reg<JDR_SPEC>`"]
pub type JDR = crate::Reg<jdr::JDR_SPEC>;
#[doc = "injected data register x"]
pub mod jdr;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "regular data register"]
pub mod dr;
