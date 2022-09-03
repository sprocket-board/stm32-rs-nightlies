#[doc = "Register `CMCR` reader"]
pub struct R(crate::R<CMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMCR` writer"]
pub struct W(crate::W<CMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCR_SPEC>;
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
impl From<crate::W<CMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEARE` reader - TEARE"]
pub type TEARE_R = crate::BitReader<bool>;
#[doc = "Field `TEARE` writer - TEARE"]
pub type TEARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `ARE` reader - ARE"]
pub type ARE_R = crate::BitReader<bool>;
#[doc = "Field `ARE` writer - ARE"]
pub type ARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSW0TX` reader - GSW0TX"]
pub type GSW0TX_R = crate::BitReader<bool>;
#[doc = "Field `GSW0TX` writer - GSW0TX"]
pub type GSW0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSW1TX` reader - GSW1TX"]
pub type GSW1TX_R = crate::BitReader<bool>;
#[doc = "Field `GSW1TX` writer - GSW1TX"]
pub type GSW1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSW2TX` reader - GSW2TX"]
pub type GSW2TX_R = crate::BitReader<bool>;
#[doc = "Field `GSW2TX` writer - GSW2TX"]
pub type GSW2TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSR0TX` reader - GSR0TX"]
pub type GSR0TX_R = crate::BitReader<bool>;
#[doc = "Field `GSR0TX` writer - GSR0TX"]
pub type GSR0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSR1TX` reader - GSR1TX"]
pub type GSR1TX_R = crate::BitReader<bool>;
#[doc = "Field `GSR1TX` writer - GSR1TX"]
pub type GSR1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GSR2TX` reader - GSR2TX"]
pub type GSR2TX_R = crate::BitReader<bool>;
#[doc = "Field `GSR2TX` writer - GSR2TX"]
pub type GSR2TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `GLWTX` reader - GLWTX"]
pub type GLWTX_R = crate::BitReader<bool>;
#[doc = "Field `GLWTX` writer - GLWTX"]
pub type GLWTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `DSW0TX` reader - DSW0TX"]
pub type DSW0TX_R = crate::BitReader<bool>;
#[doc = "Field `DSW0TX` writer - DSW0TX"]
pub type DSW0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `DSW1TX` reader - DSW1TX"]
pub type DSW1TX_R = crate::BitReader<bool>;
#[doc = "Field `DSW1TX` writer - DSW1TX"]
pub type DSW1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `DSR0TX` reader - DSR0TX"]
pub type DSR0TX_R = crate::BitReader<bool>;
#[doc = "Field `DSR0TX` writer - DSR0TX"]
pub type DSR0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `DLWTX` reader - DLWTX"]
pub type DLWTX_R = crate::BitReader<bool>;
#[doc = "Field `DLWTX` writer - DLWTX"]
pub type DLWTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
#[doc = "Field `MRDPS` reader - MRDPS"]
pub type MRDPS_R = crate::BitReader<bool>;
#[doc = "Field `MRDPS` writer - MRDPS"]
pub type MRDPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&mut self) -> TEARE_W<0> {
        TEARE_W::new(self)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&mut self) -> ARE_W<1> {
        ARE_W::new(self)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<8> {
        GSW0TX_W::new(self)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<9> {
        GSW1TX_W::new(self)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<10> {
        GSW2TX_W::new(self)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<11> {
        GSR0TX_W::new(self)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<12> {
        GSR1TX_W::new(self)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<13> {
        GSR2TX_W::new(self)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&mut self) -> GLWTX_W<14> {
        GLWTX_W::new(self)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<16> {
        DSW0TX_W::new(self)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<17> {
        DSW1TX_W::new(self)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<18> {
        DSR0TX_W::new(self)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DLWTX_W<19> {
        DLWTX_W::new(self)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&mut self) -> MRDPS_W<24> {
        MRDPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host command mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcr](index.html) module"]
pub struct CMCR_SPEC;
impl crate::RegisterSpec for CMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmcr::R](R) reader structure"]
impl crate::Readable for CMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmcr::W](W) writer structure"]
impl crate::Writable for CMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMCR to value 0"]
impl crate::Resettable for CMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
