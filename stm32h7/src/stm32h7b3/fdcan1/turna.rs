#[doc = "Register `TURNA` reader"]
pub struct R(crate::R<TURNA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TURNA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TURNA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TURNA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NAV` reader - Numerator Actual Value"]
pub type NAV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NAV_R {
        NAV_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
#[doc = "FDCAN TUR Numerator Actual Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [turna](index.html) module"]
pub struct TURNA_SPEC;
impl crate::RegisterSpec for TURNA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [turna::R](R) reader structure"]
impl crate::Readable for TURNA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TURNA to value 0"]
impl crate::Resettable for TURNA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
