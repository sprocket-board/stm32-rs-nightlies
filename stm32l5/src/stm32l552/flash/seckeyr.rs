#[doc = "Register `SECKEYR` writer"]
pub struct W(crate::W<SECKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECKEYR_SPEC>;
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
impl From<crate::W<SECKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECKEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECKEYR` writer - SECKEYR"]
pub type SECKEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECKEYR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - SECKEYR"]
    #[inline(always)]
    pub fn seckeyr(&mut self) -> SECKEYR_W<0> {
        SECKEYR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure key register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seckeyr](index.html) module"]
pub struct SECKEYR_SPEC;
impl crate::RegisterSpec for SECKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [seckeyr::W](W) writer structure"]
impl crate::Writable for SECKEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECKEYR to value 0"]
impl crate::Resettable for SECKEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
