#[doc = "Register `AHB1RSTR` reader"]
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1RSTR` writer"]
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 block reset"]
pub type DMA1RST_R = crate::BitReader<DMA1RST_A>;
#[doc = "DMA1 block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA1RST_A> {
        match self.bits {
            true => Some(DMA1RST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST_A::Reset
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 block reset"]
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, DMA1RST_A, O>;
impl<'a, const O: u8> DMA1RST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::Reset)
    }
}
#[doc = "Field `DMA2RST` reader - DMA2 block reset"]
pub use DMA1RST_R as DMA2RST_R;
#[doc = "Field `ADC12RST` reader - ADC1&2 block reset"]
pub use DMA1RST_R as ADC12RST_R;
#[doc = "Field `ETH1MACRST` reader - ETH1MAC block reset"]
pub use DMA1RST_R as ETH1MACRST_R;
#[doc = "Field `USB1OTGRST` reader - USB1OTG block reset"]
pub use DMA1RST_R as USB1OTGRST_R;
#[doc = "Field `USB2OTGRST` reader - USB2OTG block reset"]
pub use DMA1RST_R as USB2OTGRST_R;
#[doc = "Field `DMA2RST` writer - DMA2 block reset"]
pub use DMA1RST_W as DMA2RST_W;
#[doc = "Field `ADC12RST` writer - ADC1&2 block reset"]
pub use DMA1RST_W as ADC12RST_W;
#[doc = "Field `ETH1MACRST` writer - ETH1MAC block reset"]
pub use DMA1RST_W as ETH1MACRST_W;
#[doc = "Field `USB1OTGRST` writer - USB1OTG block reset"]
pub use DMA1RST_W as USB1OTGRST_W;
#[doc = "Field `USB2OTGRST` writer - USB2OTG block reset"]
pub use DMA1RST_W as USB2OTGRST_W;
impl R {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&self) -> ETH1MACRST_R {
        ETH1MACRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&self) -> USB1OTGRST_R {
        USB1OTGRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&self) -> USB2OTGRST_R {
        USB2OTGRST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W<5> {
        ADC12RST_W::new(self)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&mut self) -> ETH1MACRST_W<15> {
        ETH1MACRST_W::new(self)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&mut self) -> USB1OTGRST_W<25> {
        USB1OTGRST_W::new(self)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&mut self) -> USB2OTGRST_W<27> {
        USB2OTGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB1 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1rstr](index.html) module"]
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1rstr::R](R) reader structure"]
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure"]
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for AHB1RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
