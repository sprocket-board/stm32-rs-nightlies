#[doc = "Register `M4FECR` reader"]
pub struct R(crate::R<M4FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4FECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fecr](index.html) module"]
pub struct M4FECR_SPEC;
impl crate::RegisterSpec for M4FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4fecr::R](R) reader structure"]
impl crate::Readable for M4FECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4FECR to value 0"]
impl crate::Resettable for M4FECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}