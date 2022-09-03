#[doc = "Register `SIDR` reader"]
pub struct R(crate::R<SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "IPCC Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](index.html) module"]
pub struct SIDR_SPEC;
impl crate::RegisterSpec for SIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sidr::R](R) reader structure"]
impl crate::Readable for SIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa3c5_dd01
    }
}
