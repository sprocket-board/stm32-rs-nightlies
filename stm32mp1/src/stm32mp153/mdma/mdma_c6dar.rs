#[doc = "Register `MDMA_C6DAR` reader"]
pub struct R(crate::R<MDMA_C6DAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C6DAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C6DAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C6DAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C6DAR` writer"]
pub struct W(crate::W<MDMA_C6DAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C6DAR_SPEC>;
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
impl From<crate::W<MDMA_C6DAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C6DAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAR` reader - DAR"]
pub type DAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAR` writer - DAR"]
pub type DAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C6DAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W<0> {
        DAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x0C). M\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c6dar](index.html) module"]
pub struct MDMA_C6DAR_SPEC;
impl crate::RegisterSpec for MDMA_C6DAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c6dar::R](R) reader structure"]
impl crate::Readable for MDMA_C6DAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c6dar::W](W) writer structure"]
impl crate::Writable for MDMA_C6DAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C6DAR to value 0"]
impl crate::Resettable for MDMA_C6DAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
