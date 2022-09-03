#[doc = "Register `WPIR` reader"]
pub struct R(crate::R<WPIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPIR` writer"]
pub struct W(crate::W<WPIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPIR_SPEC>;
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
impl From<crate::W<WPIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRUCTION` reader - INSTRUCTION"]
pub type INSTRUCTION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INSTRUCTION` writer - INSTRUCTION"]
pub type INSTRUCTION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPIR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<0> {
        INSTRUCTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wrap instruction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpir](index.html) module"]
pub struct WPIR_SPEC;
impl crate::RegisterSpec for WPIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpir::R](R) reader structure"]
impl crate::Readable for WPIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpir::W](W) writer structure"]
impl crate::Writable for WPIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPIR to value 0"]
impl crate::Resettable for WPIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
