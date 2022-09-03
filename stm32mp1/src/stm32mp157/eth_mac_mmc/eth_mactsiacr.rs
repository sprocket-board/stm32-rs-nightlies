#[doc = "Register `ETH_MACTSIACR` reader"]
pub struct R(crate::R<ETH_MACTSIACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSIACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSIACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSIACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSIACR` writer"]
pub struct W(crate::W<ETH_MACTSIACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSIACR_SPEC>;
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
impl From<crate::W<ETH_MACTSIACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSIACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTIAC` reader - OSTIAC"]
pub type OSTIAC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OSTIAC` writer - OSTIAC"]
pub type OSTIAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACTSIACR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&mut self) -> OSTIAC_W<0> {
        OSTIAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MAC Timestamp Ingress Asymmetry Correction register contains the Ingress Asymmetry Correction value to be used while updating correction field in PDelay_Resp PTP messages.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsiacr](index.html) module"]
pub struct ETH_MACTSIACR_SPEC;
impl crate::RegisterSpec for ETH_MACTSIACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactsiacr::R](R) reader structure"]
impl crate::Readable for ETH_MACTSIACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactsiacr::W](W) writer structure"]
impl crate::Writable for ETH_MACTSIACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACTSIACR to value 0"]
impl crate::Resettable for ETH_MACTSIACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
