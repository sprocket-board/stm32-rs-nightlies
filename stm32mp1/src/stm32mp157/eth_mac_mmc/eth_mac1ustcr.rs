#[doc = "Register `ETH_MAC1USTCR` reader"]
pub struct R(crate::R<ETH_MAC1USTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MAC1USTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MAC1USTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MAC1USTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MAC1USTCR` writer"]
pub struct W(crate::W<ETH_MAC1USTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MAC1USTCR_SPEC>;
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
impl From<crate::W<ETH_MAC1USTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MAC1USTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIC_1US_CNTR` reader - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIC_1US_CNTR` writer - TIC_1US_CNTR"]
pub type TIC_1US_CNTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MAC1USTCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> TIC_1US_CNTR_R {
        TIC_1US_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&mut self) -> TIC_1US_CNTR_W<0> {
        TIC_1US_CNTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the generation of the Reference time (1-microsecond tick) for all the LPI timers. This timer has to be programmed by the software initially.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mac1ustcr](index.html) module"]
pub struct ETH_MAC1USTCR_SPEC;
impl crate::RegisterSpec for ETH_MAC1USTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mac1ustcr::R](R) reader structure"]
impl crate::Readable for ETH_MAC1USTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mac1ustcr::W](W) writer structure"]
impl crate::Writable for ETH_MAC1USTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MAC1USTCR to value 0"]
impl crate::Resettable for ETH_MAC1USTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
