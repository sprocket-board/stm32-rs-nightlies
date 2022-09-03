#[doc = "Register `DMA_HWCFGR2` reader"]
pub struct R(crate::R<DMA_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO_SIZE` reader - FIFO_SIZE"]
pub type FIFO_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITE_BUFFERABLE` reader - WRITE_BUFFERABLE"]
pub type WRITE_BUFFERABLE_R = crate::BitReader<bool>;
#[doc = "Field `CHSEL_WIDTH` reader - CHSEL_WIDTH"]
pub type CHSEL_WIDTH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - FIFO_SIZE"]
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - WRITE_BUFFERABLE"]
    #[inline(always)]
    pub fn write_bufferable(&self) -> WRITE_BUFFERABLE_R {
        WRITE_BUFFERABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - CHSEL_WIDTH"]
    #[inline(always)]
    pub fn chsel_width(&self) -> CHSEL_WIDTH_R {
        CHSEL_WIDTH_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "DMA hardware configuration 2register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hwcfgr2](index.html) module"]
pub struct DMA_HWCFGR2_SPEC;
impl crate::RegisterSpec for DMA_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_hwcfgr2::R](R) reader structure"]
impl crate::Readable for DMA_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_HWCFGR2 to value 0x01"]
impl crate::Resettable for DMA_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
