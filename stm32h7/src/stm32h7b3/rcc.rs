#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub cr: CR,
    #[doc = "0x04 - RCC HSI calibration register"]
    pub hsicfgr: HSICFGR,
    #[doc = "0x08 - RCC clock recovery RC register"]
    pub crrcr: CRRCR,
    #[doc = "0x0c - RCC CSI calibration register"]
    pub csicfgr: CSICFGR,
    #[doc = "0x10 - "]
    pub cfgr: CFGR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - "]
    pub cdcfgr1: CDCFGR1,
    #[doc = "0x1c - "]
    pub cdcfgr2: CDCFGR2,
    #[doc = "0x20 - "]
    pub srdcfgr: SRDCFGR,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - "]
    pub pllckselr: PLLCKSELR,
    #[doc = "0x2c - "]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x30 - "]
    pub pll1divr: PLL1DIVR,
    #[doc = "0x34 - "]
    pub pll1fracr: PLL1FRACR,
    #[doc = "0x38 - "]
    pub pll2divr: PLL2DIVR,
    #[doc = "0x3c - "]
    pub pll2fracr: PLL2FRACR,
    #[doc = "0x40 - "]
    pub pll3divr: PLL3DIVR,
    #[doc = "0x44 - "]
    pub pll3fracr: PLL3FRACR,
    _reserved16: [u8; 0x04],
    #[doc = "0x4c - RCC CPU domain kernel clock configuration register"]
    pub cdccipr: CDCCIPR,
    #[doc = "0x50 - RCC CPU domain kernel clock configuration register"]
    pub cdccip1r: CDCCIP1R,
    #[doc = "0x54 - RCC CPU domain kernel clock configuration register"]
    pub cdccip2r: CDCCIP2R,
    #[doc = "0x58 - RCC SmartRun domain kernel clock configuration register"]
    pub srdccipr: SRDCCIPR,
    _reserved20: [u8; 0x04],
    #[doc = "0x60 - "]
    pub cier: CIER,
    #[doc = "0x64 - "]
    pub cifr: CIFR,
    #[doc = "0x68 - "]
    pub cicr: CICR,
    _reserved23: [u8; 0x04],
    #[doc = "0x70 - RCC Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - RCC clock control and status register"]
    pub csr: CSR,
    _reserved25: [u8; 0x04],
    #[doc = "0x7c - "]
    pub ahb3rstr: AHB3RSTR,
    #[doc = "0x80 - "]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x84 - "]
    pub ahb2rstr: AHB2RSTR,
    #[doc = "0x88 - "]
    pub ahb4rstr: AHB4RSTR,
    #[doc = "0x8c - "]
    pub apb3rstr: APB3RSTR,
    #[doc = "0x90 - "]
    pub apb1lrstr: APB1LRSTR,
    #[doc = "0x94 - "]
    pub apb1hrstr: APB1HRSTR,
    #[doc = "0x98 - "]
    pub apb2rstr: APB2RSTR,
    #[doc = "0x9c - "]
    pub apb4rstr: APB4RSTR,
    #[doc = "0xa0 - Global Control Register"]
    pub gcr: GCR,
    _reserved35: [u8; 0x04],
    #[doc = "0xa8 - RCC SmartRun domain Autonomous mode register"]
    pub srdamr: SRDAMR,
    _reserved36: [u8; 0x04],
    #[doc = "0xb0 - RCC AXI clocks gating enable register"]
    pub ckgaenr: CKGAENR,
    _reserved37: [u8; 0x7c],
    #[doc = "0x130 - RCC reset status register"]
    pub rsr: RSR,
    #[doc = "0x134 - "]
    pub ahb3enr: AHB3ENR,
    #[doc = "0x138 - "]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x13c - "]
    pub ahb2enr: AHB2ENR,
    #[doc = "0x140 - "]
    pub ahb4enr: AHB4ENR,
    #[doc = "0x144 - "]
    pub apb3enr: APB3ENR,
    #[doc = "0x148 - "]
    pub apb1lenr: APB1LENR,
    #[doc = "0x14c - "]
    pub apb1henr: APB1HENR,
    #[doc = "0x150 - "]
    pub apb2enr: APB2ENR,
    #[doc = "0x154 - "]
    pub apb4enr: APB4ENR,
    _reserved47: [u8; 0x04],
    #[doc = "0x15c - "]
    pub ahb3lpenr: AHB3LPENR,
    #[doc = "0x160 - "]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x164 - "]
    pub ahb2lpenr: AHB2LPENR,
    #[doc = "0x168 - "]
    pub ahb4lpenr: AHB4LPENR,
    #[doc = "0x16c - "]
    pub apb3lpenr: APB3LPENR,
    #[doc = "0x170 - "]
    pub apb1llpenr: APB1LLPENR,
    #[doc = "0x174 - "]
    pub apb1hlpenr: APB1HLPENR,
    #[doc = "0x178 - "]
    pub apb2lpenr: APB2LPENR,
    #[doc = "0x17c - "]
    pub apb4lpenr: APB4LPENR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = ""]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: an alias for `Reg<HSICFGR_SPEC>`"]
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
#[doc = "RCC HSI calibration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: an alias for `Reg<CRRCR_SPEC>`"]
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
#[doc = "RCC clock recovery RC register"]
pub mod crrcr;
#[doc = "CSICFGR (rw) register accessor: an alias for `Reg<CSICFGR_SPEC>`"]
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
#[doc = "RCC CSI calibration register"]
pub mod csicfgr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = ""]
pub mod cfgr;
#[doc = "CDCFGR1 (rw) register accessor: an alias for `Reg<CDCFGR1_SPEC>`"]
pub type CDCFGR1 = crate::Reg<cdcfgr1::CDCFGR1_SPEC>;
#[doc = ""]
pub mod cdcfgr1;
#[doc = "CDCFGR2 (rw) register accessor: an alias for `Reg<CDCFGR2_SPEC>`"]
pub type CDCFGR2 = crate::Reg<cdcfgr2::CDCFGR2_SPEC>;
#[doc = ""]
pub mod cdcfgr2;
#[doc = "SRDCFGR (rw) register accessor: an alias for `Reg<SRDCFGR_SPEC>`"]
pub type SRDCFGR = crate::Reg<srdcfgr::SRDCFGR_SPEC>;
#[doc = ""]
pub mod srdcfgr;
#[doc = "PLLCKSELR (rw) register accessor: an alias for `Reg<PLLCKSELR_SPEC>`"]
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELR_SPEC>;
#[doc = ""]
pub mod pllckselr;
#[doc = "PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = ""]
pub mod pllcfgr;
#[doc = "PLL1DIVR (rw) register accessor: an alias for `Reg<PLL1DIVR_SPEC>`"]
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
#[doc = ""]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: an alias for `Reg<PLL1FRACR_SPEC>`"]
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
#[doc = ""]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: an alias for `Reg<PLL2DIVR_SPEC>`"]
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
#[doc = ""]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: an alias for `Reg<PLL2FRACR_SPEC>`"]
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
#[doc = ""]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: an alias for `Reg<PLL3DIVR_SPEC>`"]
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVR_SPEC>;
#[doc = ""]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: an alias for `Reg<PLL3FRACR_SPEC>`"]
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACR_SPEC>;
#[doc = ""]
pub mod pll3fracr;
#[doc = "CDCCIPR (rw) register accessor: an alias for `Reg<CDCCIPR_SPEC>`"]
pub type CDCCIPR = crate::Reg<cdccipr::CDCCIPR_SPEC>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccipr;
#[doc = "CDCCIP1R (rw) register accessor: an alias for `Reg<CDCCIP1R_SPEC>`"]
pub type CDCCIP1R = crate::Reg<cdccip1r::CDCCIP1R_SPEC>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccip1r;
#[doc = "CDCCIP2R (rw) register accessor: an alias for `Reg<CDCCIP2R_SPEC>`"]
pub type CDCCIP2R = crate::Reg<cdccip2r::CDCCIP2R_SPEC>;
#[doc = "RCC CPU domain kernel clock configuration register"]
pub mod cdccip2r;
#[doc = "SRDCCIPR (rw) register accessor: an alias for `Reg<SRDCCIPR_SPEC>`"]
pub type SRDCCIPR = crate::Reg<srdccipr::SRDCCIPR_SPEC>;
#[doc = "RCC SmartRun domain kernel clock configuration register"]
pub mod srdccipr;
#[doc = "CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`"]
pub type CIER = crate::Reg<cier::CIER_SPEC>;
#[doc = ""]
pub mod cier;
#[doc = "CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`"]
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
#[doc = ""]
pub mod cifr;
#[doc = "CICR (rw) register accessor: an alias for `Reg<CICR_SPEC>`"]
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
#[doc = ""]
pub mod cicr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "RCC Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "RCC clock control and status register"]
pub mod csr;
#[doc = "AHB3RSTR (rw) register accessor: an alias for `Reg<AHB3RSTR_SPEC>`"]
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
#[doc = ""]
pub mod ahb3rstr;
#[doc = "AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = ""]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = ""]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: an alias for `Reg<AHB4RSTR_SPEC>`"]
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTR_SPEC>;
#[doc = ""]
pub mod ahb4rstr;
#[doc = "APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`"]
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
#[doc = ""]
pub mod apb3rstr;
#[doc = "APB1LRSTR (rw) register accessor: an alias for `Reg<APB1LRSTR_SPEC>`"]
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
#[doc = ""]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: an alias for `Reg<APB1HRSTR_SPEC>`"]
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
#[doc = ""]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = ""]
pub mod apb2rstr;
#[doc = "APB4RSTR (rw) register accessor: an alias for `Reg<APB4RSTR_SPEC>`"]
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTR_SPEC>;
#[doc = ""]
pub mod apb4rstr;
#[doc = "SRDAMR (rw) register accessor: an alias for `Reg<SRDAMR_SPEC>`"]
pub type SRDAMR = crate::Reg<srdamr::SRDAMR_SPEC>;
#[doc = "RCC SmartRun domain Autonomous mode register"]
pub mod srdamr;
#[doc = "CKGAENR (rw) register accessor: an alias for `Reg<CKGAENR_SPEC>`"]
pub type CKGAENR = crate::Reg<ckgaenr::CKGAENR_SPEC>;
#[doc = "RCC AXI clocks gating enable register"]
pub mod ckgaenr;
#[doc = "RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "RCC reset status register"]
pub mod rsr;
#[doc = "AHB3ENR (rw) register accessor: an alias for `Reg<AHB3ENR_SPEC>`"]
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
#[doc = ""]
pub mod ahb3enr;
#[doc = "AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = ""]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = ""]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: an alias for `Reg<AHB4ENR_SPEC>`"]
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENR_SPEC>;
#[doc = ""]
pub mod ahb4enr;
#[doc = "APB3ENR (rw) register accessor: an alias for `Reg<APB3ENR_SPEC>`"]
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
#[doc = ""]
pub mod apb3enr;
#[doc = "APB1LENR (rw) register accessor: an alias for `Reg<APB1LENR_SPEC>`"]
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
#[doc = ""]
pub mod apb1lenr;
#[doc = "APB1HENR (rw) register accessor: an alias for `Reg<APB1HENR_SPEC>`"]
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
#[doc = ""]
pub mod apb1henr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = ""]
pub mod apb2enr;
#[doc = "APB4ENR (rw) register accessor: an alias for `Reg<APB4ENR_SPEC>`"]
pub type APB4ENR = crate::Reg<apb4enr::APB4ENR_SPEC>;
#[doc = ""]
pub mod apb4enr;
#[doc = "AHB3LPENR (rw) register accessor: an alias for `Reg<AHB3LPENR_SPEC>`"]
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>;
#[doc = ""]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = ""]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: an alias for `Reg<AHB2LPENR_SPEC>`"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = ""]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: an alias for `Reg<AHB4LPENR_SPEC>`"]
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>;
#[doc = ""]
pub mod ahb4lpenr;
#[doc = "APB3LPENR (rw) register accessor: an alias for `Reg<APB3LPENR_SPEC>`"]
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
#[doc = ""]
pub mod apb3lpenr;
#[doc = "APB1LLPENR (rw) register accessor: an alias for `Reg<APB1LLPENR_SPEC>`"]
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
#[doc = ""]
pub mod apb1llpenr;
#[doc = "APB1HLPENR (rw) register accessor: an alias for `Reg<APB1HLPENR_SPEC>`"]
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
#[doc = ""]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = ""]
pub mod apb2lpenr;
#[doc = "APB4LPENR (rw) register accessor: an alias for `Reg<APB4LPENR_SPEC>`"]
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENR_SPEC>;
#[doc = ""]
pub mod apb4lpenr;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global Control Register"]
pub mod gcr;
