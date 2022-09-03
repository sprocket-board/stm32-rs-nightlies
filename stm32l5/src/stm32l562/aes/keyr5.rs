#[doc = "Register `KEYR5` writer"]
pub struct W(crate::W<KEYR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR5_SPEC>;
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
impl From<crate::W<KEYR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[191:160\\])"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR5_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[191:160\\])"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key register 5\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr5](index.html) module"]
pub struct KEYR5_SPEC;
impl crate::RegisterSpec for KEYR5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [keyr5::W](W) writer structure"]
impl crate::Writable for KEYR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR5 to value 0"]
impl crate::Resettable for KEYR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
