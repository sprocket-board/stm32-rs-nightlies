#[doc = "Register `SDMMC_POWER` reader"]
pub struct R(crate::R<SDMMC_POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_POWER` writer"]
pub struct W(crate::W<SDMMC_POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDMMC_POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRCTRL` reader - PWRCTRL"]
pub type PWRCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRCTRL` writer - PWRCTRL"]
pub type PWRCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDMMC_POWER_SPEC, u8, u8, 2, O>;
#[doc = "Field `VSWITCH` reader - VSWITCH"]
pub type VSWITCH_R = crate::BitReader<bool>;
#[doc = "Field `VSWITCH` writer - VSWITCH"]
pub type VSWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_POWER_SPEC, bool, O>;
#[doc = "Field `VSWITCHEN` reader - VSWITCHEN"]
pub type VSWITCHEN_R = crate::BitReader<bool>;
#[doc = "Field `VSWITCHEN` writer - VSWITCHEN"]
pub type VSWITCHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_POWER_SPEC, bool, O>;
#[doc = "Field `DIRPOL` reader - DIRPOL"]
pub type DIRPOL_R = crate::BitReader<bool>;
#[doc = "Field `DIRPOL` writer - DIRPOL"]
pub type DIRPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_POWER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VSWITCH"]
    #[inline(always)]
    pub fn vswitch(&self) -> VSWITCH_R {
        VSWITCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSWITCHEN"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VSWITCHEN_R {
        VSWITCHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIRPOL"]
    #[inline(always)]
    pub fn dirpol(&self) -> DIRPOL_R {
        DIRPOL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<0> {
        PWRCTRL_W::new(self)
    }
    #[doc = "Bit 2 - VSWITCH"]
    #[inline(always)]
    pub fn vswitch(&mut self) -> VSWITCH_W<2> {
        VSWITCH_W::new(self)
    }
    #[doc = "Bit 3 - VSWITCHEN"]
    #[inline(always)]
    pub fn vswitchen(&mut self) -> VSWITCHEN_W<3> {
        VSWITCHEN_W::new(self)
    }
    #[doc = "Bit 4 - DIRPOL"]
    #[inline(always)]
    pub fn dirpol(&mut self) -> DIRPOL_W<4> {
        DIRPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMMC power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_power](index.html) module"]
pub struct SDMMC_POWER_SPEC;
impl crate::RegisterSpec for SDMMC_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_power::R](R) reader structure"]
impl crate::Readable for SDMMC_POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_power::W](W) writer structure"]
impl crate::Writable for SDMMC_POWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_POWER to value 0"]
impl crate::Resettable for SDMMC_POWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
