#[doc = "Register `IV1LR` reader"]
pub struct R(crate::R<IV1LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV1LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV1LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV1LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IV1LR` writer"]
pub struct W(crate::W<IV1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV1LR_SPEC>;
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
impl From<crate::W<IV1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV` reader - IV95"]
pub type IV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV` writer - IV95"]
pub type IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IV1LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV95"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<0> {
        IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iv1lr](index.html) module"]
pub struct IV1LR_SPEC;
impl crate::RegisterSpec for IV1LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iv1lr::R](R) reader structure"]
impl crate::Readable for IV1LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iv1lr::W](W) writer structure"]
impl crate::Writable for IV1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IV1LR to value 0"]
impl crate::Resettable for IV1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
