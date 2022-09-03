#[doc = "Register `RG1CR` reader"]
pub struct R(crate::R<RG1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RG1CR` writer"]
pub struct W(crate::W<RG1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG1CR_SPEC>;
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
impl From<crate::W<RG1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SIG_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG1CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OIE_R = crate::BitReader<bool>;
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG1CR_SPEC, bool, O>;
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GE_R = crate::BitReader<bool>;
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG1CR_SPEC, bool, O>;
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG1CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG1CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg1cr](index.html) module"]
pub struct RG1CR_SPEC;
impl crate::RegisterSpec for RG1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rg1cr::R](R) reader structure"]
impl crate::Readable for RG1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rg1cr::W](W) writer structure"]
impl crate::Writable for RG1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RG1CR to value 0"]
impl crate::Resettable for RG1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
