#[doc = "Register `DCR4` reader"]
pub struct R(crate::R<DCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR4` writer"]
pub struct W(crate::W<DCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR4_SPEC>;
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
impl From<crate::W<DCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFRESH` reader - Refresh rate"]
pub type REFRESH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REFRESH` writer - Refresh rate"]
pub type REFRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Refresh rate"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Refresh rate"]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W<0> {
        REFRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr4](index.html) module"]
pub struct DCR4_SPEC;
impl crate::RegisterSpec for DCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr4::R](R) reader structure"]
impl crate::Readable for DCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr4::W](W) writer structure"]
impl crate::Writable for DCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR4 to value 0"]
impl crate::Resettable for DCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
