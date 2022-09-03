#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
    pub i2c_cr1: I2C_CR1,
    #[doc = "0x04 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_cr2: I2C_CR2,
    #[doc = "0x08 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_oar1: I2C_OAR1,
    #[doc = "0x0c - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_oar2: I2C_OAR2,
    #[doc = "0x10 - Access: No wait states"]
    pub i2c_timingr: I2C_TIMINGR,
    #[doc = "0x14 - Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
    pub i2c_timeoutr: I2C_TIMEOUTR,
    #[doc = "0x18 - Access: No wait states"]
    pub i2c_isr: I2C_ISR,
    #[doc = "0x1c - Access: No wait states"]
    pub i2c_icr: I2C_ICR,
    #[doc = "0x20 - Access: No wait states"]
    pub i2c_pecr: I2C_PECR,
    #[doc = "0x24 - Access: No wait states"]
    pub i2c_rxdr: I2C_RXDR,
    #[doc = "0x28 - Access: No wait states"]
    pub i2c_txdr: I2C_TXDR,
    _reserved11: [u8; 0x03c4],
    #[doc = "0x3f0 - I2C hardware configuration register"]
    pub i2c_hwcfgr: I2C_HWCFGR,
    #[doc = "0x3f4 - I2C version register"]
    pub i2c_verr: I2C_VERR,
    #[doc = "0x3f8 - I2C identification register"]
    pub i2c_ipidr: I2C_IPIDR,
    #[doc = "0x3fc - I2C size identification register"]
    pub i2c_sidr: I2C_SIDR,
}
#[doc = "I2C_CR1 (rw) register accessor: an alias for `Reg<I2C_CR1_SPEC>`"]
pub type I2C_CR1 = crate::Reg<i2c_cr1::I2C_CR1_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2xi2c_pclk+6xi2c_ker_ck."]
pub mod i2c_cr1;
#[doc = "I2C_CR2 (rw) register accessor: an alias for `Reg<I2C_CR2_SPEC>`"]
pub type I2C_CR2 = crate::Reg<i2c_cr2::I2C_CR2_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_cr2;
#[doc = "I2C_OAR1 (rw) register accessor: an alias for `Reg<I2C_OAR1_SPEC>`"]
pub type I2C_OAR1 = crate::Reg<i2c_oar1::I2C_OAR1_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar1;
#[doc = "I2C_OAR2 (rw) register accessor: an alias for `Reg<I2C_OAR2_SPEC>`"]
pub type I2C_OAR2 = crate::Reg<i2c_oar2::I2C_OAR2_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_oar2;
#[doc = "I2C_TIMINGR (rw) register accessor: an alias for `Reg<I2C_TIMINGR_SPEC>`"]
pub type I2C_TIMINGR = crate::Reg<i2c_timingr::I2C_TIMINGR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_timingr;
#[doc = "I2C_TIMEOUTR (rw) register accessor: an alias for `Reg<I2C_TIMEOUTR_SPEC>`"]
pub type I2C_TIMEOUTR = crate::Reg<i2c_timeoutr::I2C_TIMEOUTR_SPEC>;
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck."]
pub mod i2c_timeoutr;
#[doc = "I2C_ISR (rw) register accessor: an alias for `Reg<I2C_ISR_SPEC>`"]
pub type I2C_ISR = crate::Reg<i2c_isr::I2C_ISR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_isr;
#[doc = "I2C_ICR (w) register accessor: an alias for `Reg<I2C_ICR_SPEC>`"]
pub type I2C_ICR = crate::Reg<i2c_icr::I2C_ICR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_icr;
#[doc = "I2C_PECR (r) register accessor: an alias for `Reg<I2C_PECR_SPEC>`"]
pub type I2C_PECR = crate::Reg<i2c_pecr::I2C_PECR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_pecr;
#[doc = "I2C_RXDR (r) register accessor: an alias for `Reg<I2C_RXDR_SPEC>`"]
pub type I2C_RXDR = crate::Reg<i2c_rxdr::I2C_RXDR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_rxdr;
#[doc = "I2C_TXDR (rw) register accessor: an alias for `Reg<I2C_TXDR_SPEC>`"]
pub type I2C_TXDR = crate::Reg<i2c_txdr::I2C_TXDR_SPEC>;
#[doc = "Access: No wait states"]
pub mod i2c_txdr;
#[doc = "I2C_HWCFGR (r) register accessor: an alias for `Reg<I2C_HWCFGR_SPEC>`"]
pub type I2C_HWCFGR = crate::Reg<i2c_hwcfgr::I2C_HWCFGR_SPEC>;
#[doc = "I2C hardware configuration register"]
pub mod i2c_hwcfgr;
#[doc = "I2C_VERR (r) register accessor: an alias for `Reg<I2C_VERR_SPEC>`"]
pub type I2C_VERR = crate::Reg<i2c_verr::I2C_VERR_SPEC>;
#[doc = "I2C version register"]
pub mod i2c_verr;
#[doc = "I2C_IPIDR (r) register accessor: an alias for `Reg<I2C_IPIDR_SPEC>`"]
pub type I2C_IPIDR = crate::Reg<i2c_ipidr::I2C_IPIDR_SPEC>;
#[doc = "I2C identification register"]
pub mod i2c_ipidr;
#[doc = "I2C_SIDR (r) register accessor: an alias for `Reg<I2C_SIDR_SPEC>`"]
pub type I2C_SIDR = crate::Reg<i2c_sidr::I2C_SIDR_SPEC>;
#[doc = "I2C size identification register"]
pub mod i2c_sidr;
