#[doc = "Register `CH6WDATR` reader"]
pub struct R(crate::R<CH6WDATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH6WDATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH6WDATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH6WDATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH6WDATR` writer"]
pub struct W(crate::W<CH6WDATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH6WDATR_SPEC>;
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
impl From<crate::W<CH6WDATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH6WDATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH6WDATR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH6WDATR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6wdatr](index.html) module"]
pub struct CH6WDATR_SPEC;
impl crate::RegisterSpec for CH6WDATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch6wdatr::R](R) reader structure"]
impl crate::Readable for CH6WDATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch6wdatr::W](W) writer structure"]
impl crate::Writable for CH6WDATR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH6WDATR to value 0"]
impl crate::Resettable for CH6WDATR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
