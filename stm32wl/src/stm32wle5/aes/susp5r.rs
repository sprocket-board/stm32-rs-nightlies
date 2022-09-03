#[doc = "Register `SUSP5R` reader"]
pub struct R(crate::R<SUSP5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUSP5R` writer"]
pub struct W(crate::W<SUSP5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP5R_SPEC>;
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
impl From<crate::W<SUSP5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSP` reader - AES suspend register 5"]
pub type SUSP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUSP` writer - AES suspend register 5"]
pub type SUSP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SUSP5R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 5"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 5"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<0> {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "AES suspend register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp5r](index.html) module"]
pub struct SUSP5R_SPEC;
impl crate::RegisterSpec for SUSP5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [susp5r::R](R) reader structure"]
impl crate::Readable for SUSP5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [susp5r::W](W) writer structure"]
impl crate::Writable for SUSP5R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUSP5R to value 0"]
impl crate::Resettable for SUSP5R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
