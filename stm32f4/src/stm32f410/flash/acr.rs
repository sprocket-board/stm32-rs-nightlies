#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::FieldReader<u8, LATENCY_A>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states"]
    Ws0 = 0,
    #[doc = "1: 1 wait states"]
    Ws1 = 1,
    #[doc = "2: 2 wait states"]
    Ws2 = 2,
    #[doc = "3: 3 wait states"]
    Ws3 = 3,
    #[doc = "4: 4 wait states"]
    Ws4 = 4,
    #[doc = "5: 5 wait states"]
    Ws5 = 5,
    #[doc = "6: 6 wait states"]
    Ws6 = 6,
    #[doc = "7: 7 wait states"]
    Ws7 = 7,
    #[doc = "8: 8 wait states"]
    Ws8 = 8,
    #[doc = "9: 9 wait states"]
    Ws9 = 9,
    #[doc = "10: 10 wait states"]
    Ws10 = 10,
    #[doc = "11: 11 wait states"]
    Ws11 = 11,
    #[doc = "12: 12 wait states"]
    Ws12 = 12,
    #[doc = "13: 13 wait states"]
    Ws13 = 13,
    #[doc = "14: 14 wait states"]
    Ws14 = 14,
    #[doc = "15: 15 wait states"]
    Ws15 = 15,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATENCY_A {
        match self.bits {
            0 => LATENCY_A::Ws0,
            1 => LATENCY_A::Ws1,
            2 => LATENCY_A::Ws2,
            3 => LATENCY_A::Ws3,
            4 => LATENCY_A::Ws4,
            5 => LATENCY_A::Ws5,
            6 => LATENCY_A::Ws6,
            7 => LATENCY_A::Ws7,
            8 => LATENCY_A::Ws8,
            9 => LATENCY_A::Ws9,
            10 => LATENCY_A::Ws10,
            11 => LATENCY_A::Ws11,
            12 => LATENCY_A::Ws12,
            13 => LATENCY_A::Ws13,
            14 => LATENCY_A::Ws14,
            15 => LATENCY_A::Ws15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ws0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::Ws0
    }
    #[doc = "Checks if the value of the field is `Ws1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::Ws1
    }
    #[doc = "Checks if the value of the field is `Ws2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::Ws2
    }
    #[doc = "Checks if the value of the field is `Ws3`"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY_A::Ws3
    }
    #[doc = "Checks if the value of the field is `Ws4`"]
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY_A::Ws4
    }
    #[doc = "Checks if the value of the field is `Ws5`"]
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY_A::Ws5
    }
    #[doc = "Checks if the value of the field is `Ws6`"]
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY_A::Ws6
    }
    #[doc = "Checks if the value of the field is `Ws7`"]
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY_A::Ws7
    }
    #[doc = "Checks if the value of the field is `Ws8`"]
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY_A::Ws8
    }
    #[doc = "Checks if the value of the field is `Ws9`"]
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY_A::Ws9
    }
    #[doc = "Checks if the value of the field is `Ws10`"]
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY_A::Ws10
    }
    #[doc = "Checks if the value of the field is `Ws11`"]
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY_A::Ws11
    }
    #[doc = "Checks if the value of the field is `Ws12`"]
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY_A::Ws12
    }
    #[doc = "Checks if the value of the field is `Ws13`"]
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY_A::Ws13
    }
    #[doc = "Checks if the value of the field is `Ws14`"]
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY_A::Ws14
    }
    #[doc = "Checks if the value of the field is `Ws15`"]
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY_A::Ws15
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ACR_SPEC, u8, LATENCY_A, 4, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn ws4(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn ws5(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn ws6(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn ws7(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn ws8(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn ws9(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn ws10(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn ws11(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn ws12(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn ws13(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn ws14(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn ws15(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws15)
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN_A>;
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTEN_A {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTEN_A {
        match self.bits {
            false => PRFTEN_A::Disabled,
            true => PRFTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN_A::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRFTEN_A, O>;
impl<'a, const O: u8> PRFTEN_W<'a, O> {
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTEN_A::Enabled)
    }
}
#[doc = "Field `ICEN` reader - Instruction cache enable"]
pub type ICEN_R = crate::BitReader<ICEN_A>;
#[doc = "Instruction cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICEN_A {
    #[doc = "0: Instruction cache is disabled"]
    Disabled = 0,
    #[doc = "1: Instruction cache is enabled"]
    Enabled = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::Disabled,
            true => ICEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ICEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ICEN_A::Enabled
    }
}
#[doc = "Field `ICEN` writer - Instruction cache enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ICEN_A, O>;
impl<'a, const O: u8> ICEN_W<'a, O> {
    #[doc = "Instruction cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ICEN_A::Disabled)
    }
    #[doc = "Instruction cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ICEN_A::Enabled)
    }
}
#[doc = "Field `DCEN` reader - Data cache enable"]
pub type DCEN_R = crate::BitReader<DCEN_A>;
#[doc = "Data cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCEN_A {
    #[doc = "0: Data cache is disabled"]
    Disabled = 0,
    #[doc = "1: Data cache is enabled"]
    Enabled = 1,
}
impl From<DCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEN_A {
        match self.bits {
            false => DCEN_A::Disabled,
            true => DCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCEN_A::Enabled
    }
}
#[doc = "Field `DCEN` writer - Data cache enable"]
pub type DCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, DCEN_A, O>;
impl<'a, const O: u8> DCEN_W<'a, O> {
    #[doc = "Data cache is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCEN_A::Disabled)
    }
    #[doc = "Data cache is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCEN_A::Enabled)
    }
}
#[doc = "Instruction cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRST_AW {
    #[doc = "0: Instruction cache is not reset"]
    NotReset = 0,
    #[doc = "1: Instruction cache is reset"]
    Reset = 1,
}
impl From<ICRST_AW> for bool {
    #[inline(always)]
    fn from(variant: ICRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICRST` writer - Instruction cache reset"]
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ICRST_AW, O>;
impl<'a, const O: u8> ICRST_W<'a, O> {
    #[doc = "Instruction cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(ICRST_AW::NotReset)
    }
    #[doc = "Instruction cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ICRST_AW::Reset)
    }
}
#[doc = "Field `DCRST` reader - Data cache reset"]
pub type DCRST_R = crate::BitReader<DCRST_A>;
#[doc = "Data cache reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCRST_A {
    #[doc = "0: Data cache is not reset"]
    NotReset = 0,
    #[doc = "1: Data cache is reset"]
    Reset = 1,
}
impl From<DCRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCRST_A {
        match self.bits {
            false => DCRST_A::NotReset,
            true => DCRST_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NotReset`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == DCRST_A::NotReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCRST_A::Reset
    }
}
#[doc = "Field `DCRST` writer - Data cache reset"]
pub type DCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, DCRST_A, O>;
impl<'a, const O: u8> DCRST_W<'a, O> {
    #[doc = "Data cache is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(DCRST_A::NotReset)
    }
    #[doc = "Data cache is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCRST_A::Reset)
    }
}
impl R {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline(always)]
    pub fn icen(&mut self) -> ICEN_W<9> {
        ICEN_W::new(self)
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline(always)]
    pub fn dcen(&mut self) -> DCEN_W<10> {
        DCEN_W::new(self)
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline(always)]
    pub fn icrst(&mut self) -> ICRST_W<11> {
        ICRST_W::new(self)
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline(always)]
    pub fn dcrst(&mut self) -> DCRST_W<12> {
        DCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
