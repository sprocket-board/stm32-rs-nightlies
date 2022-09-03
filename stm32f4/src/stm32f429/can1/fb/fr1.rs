#[doc = "Register `FR1` reader"]
pub struct R(crate::R<FR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR1` writer"]
pub struct W(crate::W<FR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR1_SPEC>;
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
impl From<crate::W<FR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB` reader - Filter bits"]
pub type FB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FB` writer - Filter bits"]
pub type FB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Filter bits"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<0> {
        FB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter bank 0 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr1](index.html) module"]
pub struct FR1_SPEC;
impl crate::RegisterSpec for FR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr1::R](R) reader structure"]
impl crate::Readable for FR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr1::W](W) writer structure"]
impl crate::Writable for FR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR1 to value 0"]
impl crate::Resettable for FR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}