#[doc = "Register `EXTI_HWCFGR4` reader"]
pub struct R(crate::R<EXTI_HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EVENT_TRG` reader - EVENT_TRG"]
pub type EVENT_TRG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EVENT_TRG"]
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
#[doc = "EXTI hardware configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr4](index.html) module"]
pub struct EXTI_HWCFGR4_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr4::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR4 to value 0x0001_ffff"]
impl crate::Resettable for EXTI_HWCFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_ffff
    }
}