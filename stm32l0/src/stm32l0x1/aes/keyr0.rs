#[doc = "Register `KEYR0` reader"]
pub struct R(crate::R<KEYR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR0` writer"]
pub struct W(crate::W<KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR0_SPEC>;
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
impl From<crate::W<KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - Data Output Register (LSB key \\[31:0\\])"]
pub type KEY0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY0` writer - Data Output Register (LSB key \\[31:0\\])"]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Output Register (LSB key \\[31:0\\])"]
    #[inline(always)]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "key register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr0](index.html) module"]
pub struct KEYR0_SPEC;
impl crate::RegisterSpec for KEYR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr0::R](R) reader structure"]
impl crate::Readable for KEYR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr0::W](W) writer structure"]
impl crate::Writable for KEYR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR0 to value 0"]
impl crate::Resettable for KEYR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
