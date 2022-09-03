#[doc = "Register `GPIOB_HWCFGR2` reader"]
pub struct R(crate::R<GPIOB_HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOB_HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOB_HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOB_HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AFRL_RES` reader - AFRL_RES"]
pub type AFRL_RES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - AFRL_RES"]
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_hwcfgr2](index.html) module"]
pub struct GPIOB_HWCFGR2_SPEC;
impl crate::RegisterSpec for GPIOB_HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiob_hwcfgr2::R](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOB_HWCFGR2 to value 0"]
impl crate::Resettable for GPIOB_HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}