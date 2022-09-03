#[doc = "Register `KEYR6` reader"]
pub struct R(crate::R<KEYR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR6` writer"]
pub struct W(crate::W<KEYR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR6_SPEC>;
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
impl From<crate::W<KEYR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - Cryptographic key, bits \\[223:192\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[223:192\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[223:192\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[223:192\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
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
#[doc = "AES key register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr6](index.html) module"]
pub struct KEYR6_SPEC;
impl crate::RegisterSpec for KEYR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr6::R](R) reader structure"]
impl crate::Readable for KEYR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr6::W](W) writer structure"]
impl crate::Writable for KEYR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR6 to value 0"]
impl crate::Resettable for KEYR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}