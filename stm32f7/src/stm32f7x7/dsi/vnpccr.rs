#[doc = "Register `VNPCCR` reader"]
pub struct R(crate::R<VNPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VNPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VNPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VNPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NPSIZE` reader - Null Packet Size"]
pub type NPSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host Video Null Packet Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpccr](index.html) module"]
pub struct VNPCCR_SPEC;
impl crate::RegisterSpec for VNPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vnpccr::R](R) reader structure"]
impl crate::Readable for VNPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VNPCCR to value 0"]
impl crate::Resettable for VNPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
