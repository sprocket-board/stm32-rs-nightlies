#[doc = "Register `EP3R` reader"]
pub struct R(crate::R<EP3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP3R` writer"]
pub struct W(crate::W<EP3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP3R_SPEC>;
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
impl From<crate::W<EP3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EA` reader - EA"]
pub type EA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EA` writer - EA"]
pub type EA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, EP3R_SPEC, u8, u8, 4, O>;
#[doc = "Field `STAT_TX` reader - STAT_TX"]
pub type STAT_TX_R = crate::FieldReader<u8, STAT_TX_A>;
#[doc = "STAT_TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_TX_A {
    #[doc = "0: all transmission requests addressed to this endpoint are ignored"]
    Disabled = 0,
    #[doc = "1: the endpoint is stalled and all transmission requests result in a STALL handshake"]
    Stall = 1,
    #[doc = "2: the endpoint is naked and all transmission requests result in a NAK handshake"]
    Nak = 2,
    #[doc = "3: this endpoint is enabled for transmission"]
    Valid = 3,
}
impl From<STAT_TX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_TX_A) -> Self {
        variant as _
    }
}
impl STAT_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_TX_A {
        match self.bits {
            0 => STAT_TX_A::Disabled,
            1 => STAT_TX_A::Stall,
            2 => STAT_TX_A::Nak,
            3 => STAT_TX_A::Valid,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_TX_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Stall`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_TX_A::Stall
    }
    #[doc = "Checks if the value of the field is `Nak`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_TX_A::Nak
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_TX_A::Valid
    }
}
#[doc = "Field `STAT_TX` writer - STAT_TX"]
pub type STAT_TX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EP3R_SPEC, u8, STAT_TX_A, 2, O>;
impl<'a, const O: u8> STAT_TX_W<'a, O> {
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_TX_A::Disabled)
    }
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_TX_A::Stall)
    }
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_TX_A::Nak)
    }
    #[doc = "this endpoint is enabled for transmission"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_TX_A::Valid)
    }
}
#[doc = "Field `DTOG_TX` reader - DTOG_TX"]
pub type DTOG_TX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_TX` writer - DTOG_TX"]
pub type DTOG_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
#[doc = "Field `CTR_TX` reader - CTR_TX"]
pub type CTR_TX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_TX` writer - CTR_TX"]
pub type CTR_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
#[doc = "Field `EP_KIND` reader - EP_KIND"]
pub type EP_KIND_R = crate::BitReader<bool>;
#[doc = "Field `EP_KIND` writer - EP_KIND"]
pub type EP_KIND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
#[doc = "Field `EP_TYPE` reader - EPTYPE"]
pub type EP_TYPE_R = crate::FieldReader<u8, EP_TYPE_A>;
#[doc = "EPTYPE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EP_TYPE_A {
    #[doc = "0: Bulk endpoint"]
    Bulk = 0,
    #[doc = "1: Control endpoint"]
    Control = 1,
    #[doc = "2: Iso endpoint"]
    Iso = 2,
    #[doc = "3: Interrupt endpoint"]
    Interrupt = 3,
}
impl From<EP_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EP_TYPE_A) -> Self {
        variant as _
    }
}
impl EP_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_TYPE_A {
        match self.bits {
            0 => EP_TYPE_A::Bulk,
            1 => EP_TYPE_A::Control,
            2 => EP_TYPE_A::Iso,
            3 => EP_TYPE_A::Interrupt,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bulk`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPE_A::Bulk
    }
    #[doc = "Checks if the value of the field is `Control`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == EP_TYPE_A::Control
    }
    #[doc = "Checks if the value of the field is `Iso`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == EP_TYPE_A::Iso
    }
    #[doc = "Checks if the value of the field is `Interrupt`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == EP_TYPE_A::Interrupt
    }
}
#[doc = "Field `EP_TYPE` writer - EPTYPE"]
pub type EP_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EP3R_SPEC, u8, EP_TYPE_A, 2, O>;
impl<'a, const O: u8> EP_TYPE_W<'a, O> {
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EP_TYPE_A::Bulk)
    }
    #[doc = "Control endpoint"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(EP_TYPE_A::Control)
    }
    #[doc = "Iso endpoint"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(EP_TYPE_A::Iso)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EP_TYPE_A::Interrupt)
    }
}
#[doc = "Field `SETUP` reader - SETUP"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP"]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
#[doc = "Field `STAT_RX` reader - STAT_RX"]
pub type STAT_RX_R = crate::FieldReader<u8, STAT_RX_A>;
#[doc = "STAT_RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STAT_RX_A {
    #[doc = "0: all reception requests addressed to this endpoint are ignored"]
    Disabled = 0,
    #[doc = "1: the endpoint is stalled and all reception requests result in a STALL handshake"]
    Stall = 1,
    #[doc = "2: the endpoint is naked and all reception requests result in a NAK handshake"]
    Nak = 2,
    #[doc = "3: this endpoint is enabled for reception"]
    Valid = 3,
}
impl From<STAT_RX_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_RX_A) -> Self {
        variant as _
    }
}
impl STAT_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STAT_RX_A {
        match self.bits {
            0 => STAT_RX_A::Disabled,
            1 => STAT_RX_A::Stall,
            2 => STAT_RX_A::Nak,
            3 => STAT_RX_A::Valid,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_RX_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Stall`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == STAT_RX_A::Stall
    }
    #[doc = "Checks if the value of the field is `Nak`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == STAT_RX_A::Nak
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == STAT_RX_A::Valid
    }
}
#[doc = "Field `STAT_RX` writer - STAT_RX"]
pub type STAT_RX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EP3R_SPEC, u8, STAT_RX_A, 2, O>;
impl<'a, const O: u8> STAT_RX_W<'a, O> {
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_RX_A::Disabled)
    }
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_RX_A::Stall)
    }
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_RX_A::Nak)
    }
    #[doc = "this endpoint is enabled for reception"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_RX_A::Valid)
    }
}
#[doc = "Field `DTOG_RX` reader - DTOG_RX"]
pub type DTOG_RX_R = crate::BitReader<bool>;
#[doc = "Field `DTOG_RX` writer - DTOG_RX"]
pub type DTOG_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
#[doc = "Field `CTR_RX` reader - CTR_RX"]
pub type CTR_RX_R = crate::BitReader<bool>;
#[doc = "Field `CTR_RX` writer - CTR_RX"]
pub type CTR_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP3R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - EPTYPE"]
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - EA"]
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<0> {
        EA_W::new(self)
    }
    #[doc = "Bits 4:5 - STAT_TX"]
    #[inline(always)]
    pub fn stat_tx(&mut self) -> STAT_TX_W<4> {
        STAT_TX_W::new(self)
    }
    #[doc = "Bit 6 - DTOG_TX"]
    #[inline(always)]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<6> {
        DTOG_TX_W::new(self)
    }
    #[doc = "Bit 7 - CTR_TX"]
    #[inline(always)]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<7> {
        CTR_TX_W::new(self)
    }
    #[doc = "Bit 8 - EP_KIND"]
    #[inline(always)]
    pub fn ep_kind(&mut self) -> EP_KIND_W<8> {
        EP_KIND_W::new(self)
    }
    #[doc = "Bits 9:10 - EPTYPE"]
    #[inline(always)]
    pub fn ep_type(&mut self) -> EP_TYPE_W<9> {
        EP_TYPE_W::new(self)
    }
    #[doc = "Bit 11 - SETUP"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W<11> {
        SETUP_W::new(self)
    }
    #[doc = "Bits 12:13 - STAT_RX"]
    #[inline(always)]
    pub fn stat_rx(&mut self) -> STAT_RX_W<12> {
        STAT_RX_W::new(self)
    }
    #[doc = "Bit 14 - DTOG_RX"]
    #[inline(always)]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<14> {
        DTOG_RX_W::new(self)
    }
    #[doc = "Bit 15 - CTR_RX"]
    #[inline(always)]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<15> {
        CTR_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "endpoint register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep3r](index.html) module"]
pub struct EP3R_SPEC;
impl crate::RegisterSpec for EP3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep3r::R](R) reader structure"]
impl crate::Readable for EP3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep3r::W](W) writer structure"]
impl crate::Writable for EP3R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP3R to value 0"]
impl crate::Resettable for EP3R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
