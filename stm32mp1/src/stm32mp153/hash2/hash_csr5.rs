#[doc = "Register `HASH_CSR5` reader"]
pub struct R(crate::R<HASH_CSR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR5` writer"]
pub struct W(crate::W<HASH_CSR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR5_SPEC>;
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
impl From<crate::W<HASH_CSR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS5` reader - CS5"]
pub type CS5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CS5` writer - CS5"]
pub type CS5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH_CSR5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CS5"]
    #[inline(always)]
    pub fn cs5(&self) -> CS5_R {
        CS5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS5"]
    #[inline(always)]
    pub fn cs5(&mut self) -> CS5_W<0> {
        CS5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr5](index.html) module"]
pub struct HASH_CSR5_SPEC;
impl crate::RegisterSpec for HASH_CSR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr5::R](R) reader structure"]
impl crate::Readable for HASH_CSR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr5::W](W) writer structure"]
impl crate::Writable for HASH_CSR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_CSR5 to value 0"]
impl crate::Resettable for HASH_CSR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
