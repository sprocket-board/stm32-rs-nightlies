#[doc = "Register `SDMMC_DTIMER` reader"]
pub struct R(crate::R<SDMMC_DTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_DTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_DTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_DTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_DTIMER` writer"]
pub struct W(crate::W<SDMMC_DTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_DTIMER_SPEC>;
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
impl From<crate::W<SDMMC_DTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_DTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATIME` reader - DATATIME"]
pub type DATATIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATATIME` writer - DATATIME"]
pub type DATATIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDMMC_DTIMER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DATATIME"]
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATATIME"]
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<0> {
        DATATIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_dtimer](index.html) module"]
pub struct SDMMC_DTIMER_SPEC;
impl crate::RegisterSpec for SDMMC_DTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_dtimer::R](R) reader structure"]
impl crate::Readable for SDMMC_DTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_dtimer::W](W) writer structure"]
impl crate::Writable for SDMMC_DTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_DTIMER to value 0"]
impl crate::Resettable for SDMMC_DTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
