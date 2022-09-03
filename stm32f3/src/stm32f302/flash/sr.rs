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
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<BSYR_A>;
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSYR_A {
    #[doc = "0: No write/erase operation is in progress"]
    Inactive = 0,
    #[doc = "1: No write/erase operation is in progress"]
    Active = 1,
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
            false => BSYR_A::Inactive,
            true => BSYR_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `Inactive`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSYR_A::Inactive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSYR_A::Active
    }
}
#[doc = "Field `PGERR` reader - Programming error"]
pub type PGERR_R = crate::BitReader<PGERRR_A>;
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERRR_A {
    #[doc = "0: No programming error occurred"]
    NoError = 0,
    #[doc = "1: A programming error occurred"]
    Error = 1,
}
impl From<PGERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGERRR_A {
        match self.bits {
            false => PGERRR_A::NoError,
            true => PGERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGERRR_A::Error
    }
}
#[doc = "Programming error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGERRW_AW {
    #[doc = "1: Reset programming error"]
    Reset = 1,
}
impl From<PGERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGERR` writer - Programming error"]
pub type PGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PGERRW_AW, O>;
impl<'a, const O: u8> PGERR_W<'a, O> {
    #[doc = "Reset programming error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PGERRW_AW::Reset)
    }
}
#[doc = "Field `WRPRTERR` reader - Write protection error"]
pub type WRPRTERR_R = crate::BitReader<WRPRTERRR_A>;
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPRTERRR_A {
    #[doc = "0: No write protection error occurred"]
    NoError = 0,
    #[doc = "1: A write protection error occurred"]
    Error = 1,
}
impl From<WRPRTERRR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPRTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRPRTERRR_A {
        match self.bits {
            false => WRPRTERRR_A::NoError,
            true => WRPRTERRR_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPRTERRR_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPRTERRR_A::Error
    }
}
#[doc = "Write protection error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRPRTERRW_AW {
    #[doc = "1: Reset write protection error"]
    Reset = 1,
}
impl From<WRPRTERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPRTERRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRPRTERR` writer - Write protection error"]
pub type WRPRTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, WRPRTERRW_AW, O>;
impl<'a, const O: u8> WRPRTERR_W<'a, O> {
    #[doc = "Reset write protection error"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WRPRTERRW_AW::Reset)
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<EOPR_A>;
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPR_A {
    #[doc = "0: No EOP event occurred"]
    NoEvent = 0,
    #[doc = "1: An EOP event occurred"]
    Event = 1,
}
impl From<EOPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOPR_A {
        match self.bits {
            false => EOPR_A::NoEvent,
            true => EOPR_A::Event,
        }
    }
    #[doc = "Checks if the value of the field is `NoEvent`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR_A::NoEvent
    }
    #[doc = "Checks if the value of the field is `Event`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR_A::Event
    }
}
#[doc = "End of operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOPW_AW {
    #[doc = "1: Reset EOP event"]
    Reset = 1,
}
impl From<EOPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EOPW_AW, O>;
impl<'a, const O: u8> EOP_W<'a, O> {
    #[doc = "Reset EOP event"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(EOPW_AW::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&self) -> WRPRTERR_R {
        WRPRTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programming error"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W<2> {
        PGERR_W::new(self)
    }
    #[doc = "Bit 4 - Write protection error"]
    #[inline(always)]
    pub fn wrprterr(&mut self) -> WRPRTERR_W<4> {
        WRPRTERR_W::new(self)
    }
    #[doc = "Bit 5 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<5> {
        EOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
