#[doc = "Register `LAR` reader"]
pub struct R(crate::R<LAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAR` writer"]
pub struct W(crate::W<LAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAR_SPEC>;
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
impl From<crate::W<LAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAR` reader - Link address register"]
pub type LAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LAR` writer - Link address register"]
pub type LAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&self) -> LAR_R {
        LAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&mut self) -> LAR_W<0> {
        LAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Link Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](index.html) module"]
pub struct LAR_SPEC;
impl crate::RegisterSpec for LAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lar::R](R) reader structure"]
impl crate::Readable for LAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lar::W](W) writer structure"]
impl crate::Writable for LAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
