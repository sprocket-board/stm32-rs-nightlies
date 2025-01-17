#[doc = "Register `CNTAR` reader"]
pub struct R(crate::R<CNTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTAR` writer"]
pub struct W(crate::W<CNTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTAR_SPEC>;
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
impl From<crate::W<CNTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub type CNTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub type CNTX_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CNTAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&mut self) -> CNTX_W<0> {
        CNTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntar](index.html) module"]
pub struct CNTAR_SPEC;
impl crate::RegisterSpec for CNTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntar::R](R) reader structure"]
impl crate::Readable for CNTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntar::W](W) writer structure"]
impl crate::Writable for CNTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTAR to value 0"]
impl crate::Resettable for CNTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
