#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_AW, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::Update)
    }
}
#[doc = "Capture/Compare 1 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1GW_AW {
    #[doc = "1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    Trigger = 1,
}
impl From<CC1GW_AW> for bool {
    #[inline(always)]
    fn from(variant: CC1GW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1G` writer - Capture/Compare 1 generation"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, CC1GW_AW, O>;
impl<'a, const O: u8> CC1G_W<'a, O> {
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC1GW_AW::Trigger)
    }
}
#[doc = "Field `CC2G` writer - Capture/Compare 2 generation"]
pub use CC1G_W as CC2G_W;
#[doc = "Field `CC3G` writer - Capture/Compare 3 generation"]
pub use CC1G_W as CC3G_W;
#[doc = "Field `CC4G` writer - Capture/Compare 4 generation"]
pub use CC1G_W as CC4G_W;
#[doc = "Capture/Compare control update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMGW_AW {
    #[doc = "1: When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    Trigger = 1,
}
impl From<COMGW_AW> for bool {
    #[inline(always)]
    fn from(variant: COMGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, COMGW_AW, O>;
impl<'a, const O: u8> COMG_W<'a, O> {
    #[doc = "When CCPC bit is set, it allows CCxE, CCxNE and OCxM bits to be updated"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(COMGW_AW::Trigger)
    }
}
#[doc = "Trigger generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TGW_AW {
    #[doc = "1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
    Trigger = 1,
}
impl From<TGW_AW> for bool {
    #[inline(always)]
    fn from(variant: TGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, TGW_AW, O>;
impl<'a, const O: u8> TG_W<'a, O> {
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TGW_AW::Trigger)
    }
}
#[doc = "Break generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BGW_AW {
    #[doc = "1: A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    Trigger = 1,
}
impl From<BGW_AW> for bool {
    #[inline(always)]
    fn from(variant: BGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, BGW_AW, O>;
impl<'a, const O: u8> BG_W<'a, O> {
    #[doc = "A break event is generated. MOE bit is cleared and BIF flag is set. Related interrupt or DMA transfer can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(BGW_AW::Trigger)
    }
}
#[doc = "Break 2 generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B2GW_AW {
    #[doc = "1: A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled"]
    Trigger = 1,
}
impl From<B2GW_AW> for bool {
    #[inline(always)]
    fn from(variant: B2GW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B2G` writer - Break 2 generation"]
pub type B2G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, B2GW_AW, O>;
impl<'a, const O: u8> B2G_W<'a, O> {
    #[doc = "A break 2 event is generated. MOE bit is cleared and B2IF flag is set. Related interrupt can occur if enabled"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(B2GW_AW::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 2 - Capture/Compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> CC3G_W<3> {
        CC3G_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> CC4G_W<4> {
        CC4G_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<7> {
        BG_W::new(self)
    }
    #[doc = "Bit 8 - Break 2 generation"]
    #[inline(always)]
    pub fn b2g(&mut self) -> B2G_W<8> {
        B2G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
