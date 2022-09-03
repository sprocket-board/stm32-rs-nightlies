#[doc = "Register `VVSACCR` reader"]
pub struct R(crate::R<VVSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVSACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VVSACCR` writer"]
pub struct W(crate::W<VVSACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVSACCR_SPEC>;
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
impl From<crate::W<VVSACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVSACCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSA` reader - Vertical synchronism active duration"]
pub type VSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VSA` writer - Vertical synchronism active duration"]
pub type VSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVSACCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Vertical synchronism active duration"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical synchronism active duration"]
    #[inline(always)]
    pub fn vsa(&mut self) -> VSA_W<0> {
        VSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video VSA current configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvsaccr](index.html) module"]
pub struct VVSACCR_SPEC;
impl crate::RegisterSpec for VVSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvsaccr::R](R) reader structure"]
impl crate::Readable for VVSACCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vvsaccr::W](W) writer structure"]
impl crate::Writable for VVSACCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VVSACCR to value 0"]
impl crate::Resettable for VVSACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
