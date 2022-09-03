#[doc = "Register `FDCAN_TTCSM` reader"]
pub struct R(crate::R<FDCAN_TTCSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTCSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTCSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTCSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSM` reader - CSM"]
pub type CSM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT cycle sync mark register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttcsm](index.html) module"]
pub struct FDCAN_TTCSM_SPEC;
impl crate::RegisterSpec for FDCAN_TTCSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttcsm::R](R) reader structure"]
impl crate::Readable for FDCAN_TTCSM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TTCSM to value 0"]
impl crate::Resettable for FDCAN_TTCSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
