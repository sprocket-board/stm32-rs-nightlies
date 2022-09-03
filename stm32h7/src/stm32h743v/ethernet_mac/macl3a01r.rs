#[doc = "Register `MACL3A01R` reader"]
pub struct R(crate::R<MACL3A01R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL3A01R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL3A01R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL3A01R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACL3A01R` writer"]
pub struct W(crate::W<MACL3A01R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL3A01R_SPEC>;
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
impl From<crate::W<MACL3A01R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL3A01R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A01` reader - Layer 3 Address 0 Field"]
pub type L3A01_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L3A01` writer - Layer 3 Address 0 Field"]
pub type L3A01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL3A01R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 0 Field"]
    #[inline(always)]
    pub fn l3a01(&self) -> L3A01_R {
        L3A01_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 0 Field"]
    #[inline(always)]
    pub fn l3a01(&mut self) -> L3A01_W<0> {
        L3A01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer3 address 0 filter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a01r](index.html) module"]
pub struct MACL3A01R_SPEC;
impl crate::RegisterSpec for MACL3A01R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macl3a01r::R](R) reader structure"]
impl crate::Readable for MACL3A01R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macl3a01r::W](W) writer structure"]
impl crate::Writable for MACL3A01R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACL3A01R to value 0"]
impl crate::Resettable for MACL3A01R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}