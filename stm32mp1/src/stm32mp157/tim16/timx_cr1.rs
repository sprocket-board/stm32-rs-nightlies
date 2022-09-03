#[doc = "Register `TIMx_CR1` reader"]
pub struct R(crate::R<TIMX_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CR1` writer"]
pub struct W(crate::W<TIMX_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CR1_SPEC>;
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
impl From<crate::W<TIMX_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - CEN"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - CEN"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
#[doc = "Field `UDIS` reader - UDIS"]
pub type UDIS_R = crate::BitReader<bool>;
#[doc = "Field `UDIS` writer - UDIS"]
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
#[doc = "Field `URS` reader - URS"]
pub type URS_R = crate::BitReader<bool>;
#[doc = "Field `URS` writer - URS"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
#[doc = "Field `OPM` reader - OPM"]
pub type OPM_R = crate::BitReader<bool>;
#[doc = "Field `OPM` writer - OPM"]
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
#[doc = "Field `ARPE` reader - ARPE"]
pub type ARPE_R = crate::BitReader<bool>;
#[doc = "Field `ARPE` writer - ARPE"]
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
#[doc = "Field `CKD` reader - CKD"]
pub type CKD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKD` writer - CKD"]
pub type CKD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIMX_CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `UIFREMAP` reader - UIFREMAP"]
pub type UIFREMAP_R = crate::BitReader<bool>;
#[doc = "Field `UIFREMAP` writer - UIFREMAP"]
pub type UIFREMAP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIMX_CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W<8> {
        CKD_W::new(self)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UIFREMAP_W<11> {
        UIFREMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr1](index.html) module"]
pub struct TIMX_CR1_SPEC;
impl crate::RegisterSpec for TIMX_CR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_cr1::R](R) reader structure"]
impl crate::Readable for TIMX_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_cr1::W](W) writer structure"]
impl crate::Writable for TIMX_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CR1 to value 0"]
impl crate::Resettable for TIMX_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
