#[doc = "Register `R3NONCER1` reader"]
pub struct R(crate::R<R3NONCER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R3NONCER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R3NONCER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R3NONCER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R3NONCER1` writer"]
pub struct W(crate::W<R3NONCER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R3NONCER1_SPEC>;
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
impl From<crate::W<R3NONCER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R3NONCER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_NONCE` reader - REGx_NONCE"]
pub type REGX_NONCE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGx_NONCE` writer - REGx_NONCE"]
pub type REGX_NONCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, R3NONCER1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REGx_NONCE"]
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<0> {
        REGX_NONCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x nonce register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r3noncer1](index.html) module"]
pub struct R3NONCER1_SPEC;
impl crate::RegisterSpec for R3NONCER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r3noncer1::R](R) reader structure"]
impl crate::Readable for R3NONCER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r3noncer1::W](W) writer structure"]
impl crate::Writable for R3NONCER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R3NONCER1 to value 0"]
impl crate::Resettable for R3NONCER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
