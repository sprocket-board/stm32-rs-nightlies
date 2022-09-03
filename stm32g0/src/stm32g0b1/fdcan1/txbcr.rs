#[doc = "Register `TXBCR` reader"]
pub struct R(crate::R<TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCR` writer"]
pub struct W(crate::W<TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCR_SPEC>;
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
impl From<crate::W<TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation request Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit; writing a 0 has no impact. This enables the Host to set cancellation requests for multiple Tx buffers with one write to TXBCR. The bits remain set until the corresponding TXBRP bit is reset."]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer cancellation request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcr](index.html) module"]
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcr::R](R) reader structure"]
impl crate::Readable for TXBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcr::W](W) writer structure"]
impl crate::Writable for TXBCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TXBCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
