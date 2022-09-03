#[doc = "Register `M1FDRL` reader"]
pub struct R(crate::R<M1FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1FDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FDATAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1fdrl](index.html) module"]
pub struct M1FDRL_SPEC;
impl crate::RegisterSpec for M1FDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m1fdrl::R](R) reader structure"]
impl crate::Readable for M1FDRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M1FDRL to value 0"]
impl crate::Resettable for M1FDRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}