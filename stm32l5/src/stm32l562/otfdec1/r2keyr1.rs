#[doc = "Register `R2KEYR1` writer"]
pub struct W(crate::W<R2KEYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R2KEYR1_SPEC>;
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
impl From<crate::W<R2KEYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R2KEYR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_KEY` writer - REGx_KEY"]
pub type REGX_KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R2KEYR1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - REGx_KEY"]
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W<0> {
        REGX_KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x key register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2keyr1](index.html) module"]
pub struct R2KEYR1_SPEC;
impl crate::RegisterSpec for R2KEYR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [r2keyr1::W](W) writer structure"]
impl crate::Writable for R2KEYR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R2KEYR1 to value 0"]
impl crate::Resettable for R2KEYR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}