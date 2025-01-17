#[doc = "Register `DAC_HWCFGR0` reader"]
pub struct R(crate::R<DAC_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DUAL` reader - DUAL"]
pub type DUAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFSR` reader - LFSR"]
pub type LFSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIANGLE` reader - TRIANGLE"]
pub type TRIANGLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE` reader - SAMPLE"]
pub type SAMPLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OR_CFG` reader - OR_CFG"]
pub type OR_CFG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - DUAL"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LFSR"]
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TRIANGLE"]
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SAMPLE"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - OR_CFG"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "DAC IP hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_hwcfgr0](index.html) module"]
pub struct DAC_HWCFGR0_SPEC;
impl crate::RegisterSpec for DAC_HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_hwcfgr0::R](R) reader structure"]
impl crate::Readable for DAC_HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAC_HWCFGR0 to value 0x1111"]
impl crate::Resettable for DAC_HWCFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1111
    }
}
