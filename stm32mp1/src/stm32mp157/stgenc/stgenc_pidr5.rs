#[doc = "Register `STGENC_PIDR5` reader"]
pub struct R(crate::R<STGENC_PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR5` reader - PIDR5"]
pub type PIDR5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR5"]
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
#[doc = "STGENC peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr5](index.html) module"]
pub struct STGENC_PIDR5_SPEC;
impl crate::RegisterSpec for STGENC_PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_pidr5::R](R) reader structure"]
impl crate::Readable for STGENC_PIDR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_PIDR5 to value 0"]
impl crate::Resettable for STGENC_PIDR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
