#[doc = "Register `FDCAN_TXFQS` reader"]
pub struct R(crate::R<FDCAN_TXFQS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXFQS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXFQS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXFQS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFFL` reader - Tx FIFO Free Level"]
pub type TFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFGI` reader - TFGI"]
pub type TFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQPI` reader - Tx FIFO/Queue Put Index"]
pub type TFQPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQF` reader - Tx FIFO/Queue Full"]
pub type TFQF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TFFL_R {
        TFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - TFGI"]
    #[inline(always)]
    pub fn tfgi(&self) -> TFGI_R {
        TFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TFQPI_R {
        TFQPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TFQF_R {
        TFQF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "FDCAN Tx FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txfqs](index.html) module"]
pub struct FDCAN_TXFQS_SPEC;
impl crate::RegisterSpec for FDCAN_TXFQS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txfqs::R](R) reader structure"]
impl crate::Readable for FDCAN_TXFQS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TXFQS to value 0x03"]
impl crate::Resettable for FDCAN_TXFQS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
