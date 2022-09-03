#[doc = "Register `CNDTR2` reader"]
pub struct R(crate::R<CNDTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR2` writer"]
pub struct W(crate::W<CNDTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR2_SPEC>;
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
impl From<crate::W<CNDTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNDTR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<0> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr2](index.html) module"]
pub struct CNDTR2_SPEC;
impl crate::RegisterSpec for CNDTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr2::R](R) reader structure"]
impl crate::Readable for CNDTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr2::W](W) writer structure"]
impl crate::Writable for CNDTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDTR2 to value 0"]
impl crate::Resettable for CNDTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
