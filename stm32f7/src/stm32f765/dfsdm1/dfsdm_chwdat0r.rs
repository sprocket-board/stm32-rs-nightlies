#[doc = "Register `DFSDM_CHWDAT0R` reader"]
pub struct R(crate::R<DFSDM_CHWDAT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHWDAT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHWDAT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHWDAT0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDATA` reader - Input channel y watchdog data"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input channel y watchdog data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DFSDM channel watchdog filter data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chwdat0r](index.html) module"]
pub struct DFSDM_CHWDAT0R_SPEC;
impl crate::RegisterSpec for DFSDM_CHWDAT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_chwdat0r::R](R) reader structure"]
impl crate::Readable for DFSDM_CHWDAT0R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_CHWDAT0R to value 0"]
impl crate::Resettable for DFSDM_CHWDAT0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}