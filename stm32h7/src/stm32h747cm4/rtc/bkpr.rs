#[doc = "Register `BKP%sR` reader"]
pub struct R(crate::R<BKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP%sR` writer"]
pub struct W(crate::W<BKPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKPR_SPEC>;
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
impl From<crate::W<BKPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
pub type BKP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC backup registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkpr](index.html) module"]
pub struct BKPR_SPEC;
impl crate::RegisterSpec for BKPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkpr::R](R) reader structure"]
impl crate::Readable for BKPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkpr::W](W) writer structure"]
impl crate::Writable for BKPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP%sR to value 0"]
impl crate::Resettable for BKPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}