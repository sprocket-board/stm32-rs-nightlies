#[doc = "Register `K1RR` writer"]
pub struct W(crate::W<K1RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K1RR_SPEC>;
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
impl From<crate::W<K1RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K1RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `k1` writer - k128"]
pub type K1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, K1RR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - k128"]
    #[inline(always)]
    pub fn k1(&mut self) -> K1_W<0> {
        K1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1rr](index.html) module"]
pub struct K1RR_SPEC;
impl crate::RegisterSpec for K1RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k1rr::W](W) writer structure"]
impl crate::Writable for K1RR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K1RR to value 0"]
impl crate::Resettable for K1RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}