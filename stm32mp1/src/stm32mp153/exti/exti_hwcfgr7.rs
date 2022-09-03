#[doc = "Register `EXTI_HWCFGR7` reader"]
pub struct R(crate::R<EXTI_HWCFGR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_HWCFGR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_HWCFGR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_HWCFGR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPUEVENT` reader - CPUEVENT"]
pub type CPUEVENT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CPUEVENT"]
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
#[doc = "EXTI hardware configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_hwcfgr7](index.html) module"]
pub struct EXTI_HWCFGR7_SPEC;
impl crate::RegisterSpec for EXTI_HWCFGR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_hwcfgr7::R](R) reader structure"]
impl crate::Readable for EXTI_HWCFGR7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXTI_HWCFGR7 to value 0x000e_ffff"]
impl crate::Resettable for EXTI_HWCFGR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000e_ffff
    }
}