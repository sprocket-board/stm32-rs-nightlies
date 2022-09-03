#[doc = "Register `SUSP3R` reader"]
pub struct R(crate::R<SUSP3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUSP3R` writer"]
pub struct W(crate::W<SUSP3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP3R_SPEC>;
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
impl From<crate::W<SUSP3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSP` reader - AES suspend register 3"]
pub type SUSP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SUSP` writer - AES suspend register 3"]
pub type SUSP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SUSP3R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - AES suspend register 3"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 3"]
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
#[doc = "AES suspend register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [susp3r](index.html) module"]
pub struct SUSP3R_SPEC;
impl crate::RegisterSpec for SUSP3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [susp3r::R](R) reader structure"]
impl crate::Readable for SUSP3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [susp3r::W](W) writer structure"]
impl crate::Writable for SUSP3R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUSP3R to value 0"]
impl crate::Resettable for SUSP3R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
