#[doc = "Register `SWIER1` reader"]
pub struct R(crate::R<SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER1` writer"]
pub struct W(crate::W<SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER1_SPEC>;
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
impl From<crate::W<SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - Software interrupt on event"]
pub type SWI0_R = crate::BitReader<SWI0W_A>;
#[doc = "Software interrupt on event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI0W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI0W_A> for bool {
    #[inline(always)]
    fn from(variant: SWI0W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI0W_A> {
        match self.bits {
            true => Some(SWI0W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI0W_A::Pend
    }
}
#[doc = "Field `SWI0` writer - Software interrupt on event"]
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER1_SPEC, SWI0W_A, O>;
impl<'a, const O: u8> SWI0_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI0W_A::Pend)
    }
}
#[doc = "Field `SWI1` reader - Software interrupt on event"]
pub use SWI0_R as SWI1_R;
#[doc = "Field `SWI2` reader - Software interrupt on event"]
pub use SWI0_R as SWI2_R;
#[doc = "Field `SWI3` reader - Software interrupt on event"]
pub use SWI0_R as SWI3_R;
#[doc = "Field `SWI4` reader - Software interrupt on event"]
pub use SWI0_R as SWI4_R;
#[doc = "Field `SWI5` reader - Software interrupt on event"]
pub use SWI0_R as SWI5_R;
#[doc = "Field `SWI6` reader - Software interrupt on event"]
pub use SWI0_R as SWI6_R;
#[doc = "Field `SWI7` reader - Software interrupt on event"]
pub use SWI0_R as SWI7_R;
#[doc = "Field `SWI8` reader - Software interrupt on event"]
pub use SWI0_R as SWI8_R;
#[doc = "Field `SWI9` reader - Software interrupt on event"]
pub use SWI0_R as SWI9_R;
#[doc = "Field `SWI10` reader - Software interrupt on event"]
pub use SWI0_R as SWI10_R;
#[doc = "Field `SWI11` reader - Software interrupt on event"]
pub use SWI0_R as SWI11_R;
#[doc = "Field `SWI12` reader - Software interrupt on event"]
pub use SWI0_R as SWI12_R;
#[doc = "Field `SWI13` reader - Software interrupt on event"]
pub use SWI0_R as SWI13_R;
#[doc = "Field `SWI14` reader - Software interrupt on event"]
pub use SWI0_R as SWI14_R;
#[doc = "Field `SWI15` reader - Software interrupt on event"]
pub use SWI0_R as SWI15_R;
#[doc = "Field `SWI16` reader - Software interrupt on event"]
pub use SWI0_R as SWI16_R;
#[doc = "Field `SWI21` reader - Software interrupt on event"]
pub use SWI0_R as SWI21_R;
#[doc = "Field `SWI22` reader - Software interrupt on event"]
pub use SWI0_R as SWI22_R;
#[doc = "Field `SWI1` writer - Software interrupt on event"]
pub use SWI0_W as SWI1_W;
#[doc = "Field `SWI2` writer - Software interrupt on event"]
pub use SWI0_W as SWI2_W;
#[doc = "Field `SWI3` writer - Software interrupt on event"]
pub use SWI0_W as SWI3_W;
#[doc = "Field `SWI4` writer - Software interrupt on event"]
pub use SWI0_W as SWI4_W;
#[doc = "Field `SWI5` writer - Software interrupt on event"]
pub use SWI0_W as SWI5_W;
#[doc = "Field `SWI6` writer - Software interrupt on event"]
pub use SWI0_W as SWI6_W;
#[doc = "Field `SWI7` writer - Software interrupt on event"]
pub use SWI0_W as SWI7_W;
#[doc = "Field `SWI8` writer - Software interrupt on event"]
pub use SWI0_W as SWI8_W;
#[doc = "Field `SWI9` writer - Software interrupt on event"]
pub use SWI0_W as SWI9_W;
#[doc = "Field `SWI10` writer - Software interrupt on event"]
pub use SWI0_W as SWI10_W;
#[doc = "Field `SWI11` writer - Software interrupt on event"]
pub use SWI0_W as SWI11_W;
#[doc = "Field `SWI12` writer - Software interrupt on event"]
pub use SWI0_W as SWI12_W;
#[doc = "Field `SWI13` writer - Software interrupt on event"]
pub use SWI0_W as SWI13_W;
#[doc = "Field `SWI14` writer - Software interrupt on event"]
pub use SWI0_W as SWI14_W;
#[doc = "Field `SWI15` writer - Software interrupt on event"]
pub use SWI0_W as SWI15_W;
#[doc = "Field `SWI16` writer - Software interrupt on event"]
pub use SWI0_W as SWI16_W;
#[doc = "Field `SWI21` writer - Software interrupt on event"]
pub use SWI0_W as SWI21_W;
#[doc = "Field `SWI22` writer - Software interrupt on event"]
pub use SWI0_W as SWI22_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi22(&self) -> SWI22_R {
        SWI22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    #[doc = "Bit 1 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W<1> {
        SWI1_W::new(self)
    }
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W<2> {
        SWI2_W::new(self)
    }
    #[doc = "Bit 3 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W<3> {
        SWI3_W::new(self)
    }
    #[doc = "Bit 4 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W<4> {
        SWI4_W::new(self)
    }
    #[doc = "Bit 5 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W<5> {
        SWI5_W::new(self)
    }
    #[doc = "Bit 6 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W<6> {
        SWI6_W::new(self)
    }
    #[doc = "Bit 7 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W<7> {
        SWI7_W::new(self)
    }
    #[doc = "Bit 8 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W<8> {
        SWI8_W::new(self)
    }
    #[doc = "Bit 9 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W<9> {
        SWI9_W::new(self)
    }
    #[doc = "Bit 10 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W<10> {
        SWI10_W::new(self)
    }
    #[doc = "Bit 11 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W<11> {
        SWI11_W::new(self)
    }
    #[doc = "Bit 12 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W<12> {
        SWI12_W::new(self)
    }
    #[doc = "Bit 13 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W<13> {
        SWI13_W::new(self)
    }
    #[doc = "Bit 14 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W<14> {
        SWI14_W::new(self)
    }
    #[doc = "Bit 15 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W<15> {
        SWI15_W::new(self)
    }
    #[doc = "Bit 16 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W<16> {
        SWI16_W::new(self)
    }
    #[doc = "Bit 21 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi21(&mut self) -> SWI21_W<21> {
        SWI21_W::new(self)
    }
    #[doc = "Bit 22 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi22(&mut self) -> SWI22_W<22> {
        SWI22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](index.html) module"]
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier1::R](R) reader structure"]
impl crate::Readable for SWIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier1::W](W) writer structure"]
impl crate::Writable for SWIER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for SWIER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
