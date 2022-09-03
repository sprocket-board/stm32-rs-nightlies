#[doc = "Register `CHSELR1` reader"]
pub struct R(crate::R<CHSELR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHSELR1` writer"]
pub struct W(crate::W<CHSELR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR1_SPEC>;
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
impl From<crate::W<CHSELR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ1` reader - SQ1"]
pub type SQ1_R = crate::FieldReader<u8, SQ1_A>;
#[doc = "SQ1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SQ1_A {
    #[doc = "0: Channel 0 selected for the Nth conversion"]
    Ch0 = 0,
    #[doc = "1: Channel 1 selected for the Nth conversion"]
    Ch1 = 1,
    #[doc = "2: Channel 2 selected for the Nth conversion"]
    Ch2 = 2,
    #[doc = "3: Channel 3 selected for the Nth conversion"]
    Ch3 = 3,
    #[doc = "4: Channel 4 selected for the Nth conversion"]
    Ch4 = 4,
    #[doc = "5: Channel 5 selected for the Nth conversion"]
    Ch5 = 5,
    #[doc = "6: Channel 6 selected for the Nth conversion"]
    Ch6 = 6,
    #[doc = "7: Channel 7 selected for the Nth conversion"]
    Ch7 = 7,
    #[doc = "8: Channel 8 selected for the Nth conversion"]
    Ch8 = 8,
    #[doc = "9: Channel 9 selected for the Nth conversion"]
    Ch9 = 9,
    #[doc = "10: Channel 10 selected for the Nth conversion"]
    Ch10 = 10,
    #[doc = "11: Channel 11 selected for the Nth conversion"]
    Ch11 = 11,
    #[doc = "12: Channel 12 selected for the Nth conversion"]
    Ch12 = 12,
    #[doc = "13: Channel 13 selected for the Nth conversion"]
    Ch13 = 13,
    #[doc = "14: Channel 14 selected for the Nth conversion"]
    Ch14 = 14,
    #[doc = "15: End of sequence"]
    Eos = 15,
}
impl From<SQ1_A> for u8 {
    #[inline(always)]
    fn from(variant: SQ1_A) -> Self {
        variant as _
    }
}
impl SQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQ1_A {
        match self.bits {
            0 => SQ1_A::Ch0,
            1 => SQ1_A::Ch1,
            2 => SQ1_A::Ch2,
            3 => SQ1_A::Ch3,
            4 => SQ1_A::Ch4,
            5 => SQ1_A::Ch5,
            6 => SQ1_A::Ch6,
            7 => SQ1_A::Ch7,
            8 => SQ1_A::Ch8,
            9 => SQ1_A::Ch9,
            10 => SQ1_A::Ch10,
            11 => SQ1_A::Ch11,
            12 => SQ1_A::Ch12,
            13 => SQ1_A::Ch13,
            14 => SQ1_A::Ch14,
            15 => SQ1_A::Eos,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Ch0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == SQ1_A::Ch0
    }
    #[doc = "Checks if the value of the field is `Ch1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == SQ1_A::Ch1
    }
    #[doc = "Checks if the value of the field is `Ch2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == SQ1_A::Ch2
    }
    #[doc = "Checks if the value of the field is `Ch3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == SQ1_A::Ch3
    }
    #[doc = "Checks if the value of the field is `Ch4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == SQ1_A::Ch4
    }
    #[doc = "Checks if the value of the field is `Ch5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == SQ1_A::Ch5
    }
    #[doc = "Checks if the value of the field is `Ch6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == SQ1_A::Ch6
    }
    #[doc = "Checks if the value of the field is `Ch7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == SQ1_A::Ch7
    }
    #[doc = "Checks if the value of the field is `Ch8`"]
    #[inline(always)]
    pub fn is_ch8(&self) -> bool {
        *self == SQ1_A::Ch8
    }
    #[doc = "Checks if the value of the field is `Ch9`"]
    #[inline(always)]
    pub fn is_ch9(&self) -> bool {
        *self == SQ1_A::Ch9
    }
    #[doc = "Checks if the value of the field is `Ch10`"]
    #[inline(always)]
    pub fn is_ch10(&self) -> bool {
        *self == SQ1_A::Ch10
    }
    #[doc = "Checks if the value of the field is `Ch11`"]
    #[inline(always)]
    pub fn is_ch11(&self) -> bool {
        *self == SQ1_A::Ch11
    }
    #[doc = "Checks if the value of the field is `Ch12`"]
    #[inline(always)]
    pub fn is_ch12(&self) -> bool {
        *self == SQ1_A::Ch12
    }
    #[doc = "Checks if the value of the field is `Ch13`"]
    #[inline(always)]
    pub fn is_ch13(&self) -> bool {
        *self == SQ1_A::Ch13
    }
    #[doc = "Checks if the value of the field is `Ch14`"]
    #[inline(always)]
    pub fn is_ch14(&self) -> bool {
        *self == SQ1_A::Ch14
    }
    #[doc = "Checks if the value of the field is `Eos`"]
    #[inline(always)]
    pub fn is_eos(&self) -> bool {
        *self == SQ1_A::Eos
    }
}
#[doc = "Field `SQ1` writer - SQ1"]
pub type SQ1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHSELR1_SPEC, u8, SQ1_A, 4, O>;
impl<'a, const O: u8> SQ1_W<'a, O> {
    #[doc = "Channel 0 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ1_A::Ch0)
    }
    #[doc = "Channel 1 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ1_A::Ch1)
    }
    #[doc = "Channel 2 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ1_A::Ch2)
    }
    #[doc = "Channel 3 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ1_A::Ch3)
    }
    #[doc = "Channel 4 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ1_A::Ch4)
    }
    #[doc = "Channel 5 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ1_A::Ch5)
    }
    #[doc = "Channel 6 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ1_A::Ch6)
    }
    #[doc = "Channel 7 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ1_A::Ch7)
    }
    #[doc = "Channel 8 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ1_A::Ch8)
    }
    #[doc = "Channel 9 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ1_A::Ch9)
    }
    #[doc = "Channel 10 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ1_A::Ch10)
    }
    #[doc = "Channel 11 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ1_A::Ch11)
    }
    #[doc = "Channel 12 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ1_A::Ch12)
    }
    #[doc = "Channel 13 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ1_A::Ch13)
    }
    #[doc = "Channel 14 selected for the Nth conversion"]
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ1_A::Ch14)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ1_A::Eos)
    }
}
#[doc = "Field `SQ2` reader - SQ2"]
pub use SQ1_R as SQ2_R;
#[doc = "Field `SQ3` reader - SQ3"]
pub use SQ1_R as SQ3_R;
#[doc = "Field `SQ4` reader - SQ4"]
pub use SQ1_R as SQ4_R;
#[doc = "Field `SQ5` reader - SQ5"]
pub use SQ1_R as SQ5_R;
#[doc = "Field `SQ6` reader - SQ6"]
pub use SQ1_R as SQ6_R;
#[doc = "Field `SQ7` reader - SQ7"]
pub use SQ1_R as SQ7_R;
#[doc = "Field `SQ8` reader - SQ8"]
pub use SQ1_R as SQ8_R;
#[doc = "Field `SQ2` writer - SQ2"]
pub use SQ1_W as SQ2_W;
#[doc = "Field `SQ3` writer - SQ3"]
pub use SQ1_W as SQ3_W;
#[doc = "Field `SQ4` writer - SQ4"]
pub use SQ1_W as SQ4_W;
#[doc = "Field `SQ5` writer - SQ5"]
pub use SQ1_W as SQ5_W;
#[doc = "Field `SQ6` writer - SQ6"]
pub use SQ1_W as SQ6_W;
#[doc = "Field `SQ7` writer - SQ7"]
pub use SQ1_W as SQ7_W;
#[doc = "Field `SQ8` writer - SQ8"]
pub use SQ1_W as SQ8_W;
impl R {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<0> {
        SQ1_W::new(self)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<4> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<8> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<12> {
        SQ4_W::new(self)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<16> {
        SQ5_W::new(self)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<20> {
        SQ6_W::new(self)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<24> {
        SQ7_W::new(self)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<28> {
        SQ8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chselr1](index.html) module"]
pub struct CHSELR1_SPEC;
impl crate::RegisterSpec for CHSELR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chselr1::R](R) reader structure"]
impl crate::Readable for CHSELR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chselr1::W](W) writer structure"]
impl crate::Writable for CHSELR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSELR1 to value 0"]
impl crate::Resettable for CHSELR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
