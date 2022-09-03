#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader<RXNE_A>;
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNE_A {
    #[doc = "0: Rx buffer empty"]
    Empty = 0,
    #[doc = "1: Rx buffer not empty"]
    NotEmpty = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::Empty,
            true => RXNE_A::NotEmpty,
        }
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::Empty
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NotEmpty
    }
}
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXE_A {
    #[doc = "0: Tx buffer not empty"]
    NotEmpty = 0,
    #[doc = "1: Tx buffer empty"]
    Empty = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NotEmpty,
            true => TXE_A::Empty,
        }
    }
    #[doc = "Checks if the value of the field is `NotEmpty`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NotEmpty
    }
    #[doc = "Checks if the value of the field is `Empty`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::Empty
    }
}
#[doc = "Field `CHSIDE` reader - Channel side"]
pub type CHSIDE_R = crate::BitReader<bool>;
#[doc = "Field `UDR` reader - Underrun flag"]
pub type UDR_R = crate::BitReader<bool>;
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "CRC error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    Match = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NoMatch = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::Match,
            true => CRCERR_A::NoMatch,
        }
    }
    #[doc = "Checks if the value of the field is `Match`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERR_A::Match
    }
    #[doc = "Checks if the value of the field is `NoMatch`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERR_A::NoMatch
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub type CRCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, CRCERR_A, O>;
impl<'a, const O: u8> CRCERR_W<'a, O> {
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(CRCERR_A::Match)
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(CRCERR_A::NoMatch)
    }
}
#[doc = "Field `MODF` reader - Mode fault"]
pub type MODF_R = crate::BitReader<MODFR_A>;
#[doc = "Mode fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFR_A {
    #[doc = "0: No mode fault occurred"]
    NoFault = 0,
    #[doc = "1: Mode fault occurred"]
    Fault = 1,
}
impl From<MODFR_A> for bool {
    #[inline(always)]
    fn from(variant: MODFR_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFR_A {
        match self.bits {
            false => MODFR_A::NoFault,
            true => MODFR_A::Fault,
        }
    }
    #[doc = "Checks if the value of the field is `NoFault`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR_A::NoFault
    }
    #[doc = "Checks if the value of the field is `Fault`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODFR_A::Fault
    }
}
#[doc = "Field `OVR` reader - Overrun flag"]
pub type OVR_R = crate::BitReader<OVRR_A>;
#[doc = "Overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRR_A {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NoOverrun,
            true => OVRR_A::Overrun,
        }
    }
    #[doc = "Checks if the value of the field is `NoOverrun`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NoOverrun
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::Overrun
    }
}
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader<BSYR_A>;
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSYR_A {
    #[doc = "0: SPI not busy"]
    NotBusy = 0,
    #[doc = "1: SPI busy"]
    Busy = 1,
}
impl From<BSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYR_A {
        match self.bits {
            false => BSYR_A::NotBusy,
            true => BSYR_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR_A::Busy
    }
}
#[doc = "Field `FRE` reader - TI frame format error"]
pub type FRE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side"]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W<4> {
        CRCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
