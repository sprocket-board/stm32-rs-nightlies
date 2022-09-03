#[doc = "Register `MPCWM2_UPWMR` reader"]
pub struct R(crate::R<MPCWM2_UPWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM2_UPWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM2_UPWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM2_UPWMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCWM2_UPWMR` writer"]
pub struct W(crate::W<MPCWM2_UPWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM2_UPWMR_SPEC>;
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
impl From<crate::W<MPCWM2_UPWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM2_UPWMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LGTH` reader - LGTH"]
pub type LGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LGTH` writer - LGTH"]
pub type LGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPCWM2_UPWMR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W<16> {
        LGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unprivileged Water Mark 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcwm2_upwmr](index.html) module"]
pub struct MPCWM2_UPWMR_SPEC;
impl crate::RegisterSpec for MPCWM2_UPWMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcwm2_upwmr::R](R) reader structure"]
impl crate::Readable for MPCWM2_UPWMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcwm2_upwmr::W](W) writer structure"]
impl crate::Writable for MPCWM2_UPWMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCWM2_UPWMR to value 0x0fff_0000"]
impl crate::Resettable for MPCWM2_UPWMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
