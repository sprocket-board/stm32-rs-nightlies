#[doc = "Register `HASH_CSR52` reader"]
pub struct R(crate::R<HASH_CSR52_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR52_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR52_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR52_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR52` writer"]
pub struct W(crate::W<HASH_CSR52_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR52_SPEC>;
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
impl From<crate::W<HASH_CSR52_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR52_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS52` reader - CS52"]
pub type CS52_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CS52` writer - CS52"]
pub type CS52_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR52_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    pub fn cs52(&self) -> CS52_R {
        CS52_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS52"]
    #[inline(always)]
    pub fn cs52(&mut self) -> CS52_W<0> {
        CS52_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr52](index.html) module"]
pub struct HASH_CSR52_SPEC;
impl crate::RegisterSpec for HASH_CSR52_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr52::R](R) reader structure"]
impl crate::Readable for HASH_CSR52_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr52::W](W) writer structure"]
impl crate::Writable for HASH_CSR52_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_CSR52 to value 0"]
impl crate::Resettable for HASH_CSR52_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
