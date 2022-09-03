#[doc = "Register `SECBB1R1` reader"]
pub struct R(crate::R<SECBB1R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBB1R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBB1R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBB1R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECBB1R1` writer"]
pub struct W(crate::W<SECBB1R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBB1R1_SPEC>;
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
impl From<crate::W<SECBB1R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBB1R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECBB1` reader - SECBB1"]
pub type SECBB1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SECBB1` writer - SECBB1"]
pub type SECBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECBB1R1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SECBB1"]
    #[inline(always)]
    pub fn secbb1(&mut self) -> SECBB1_W<0> {
        SECBB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH secure block based bank 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secbb1r1](index.html) module"]
pub struct SECBB1R1_SPEC;
impl crate::RegisterSpec for SECBB1R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secbb1r1::R](R) reader structure"]
impl crate::Readable for SECBB1R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secbb1r1::W](W) writer structure"]
impl crate::Writable for SECBB1R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECBB1R1 to value 0"]
impl crate::Resettable for SECBB1R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
