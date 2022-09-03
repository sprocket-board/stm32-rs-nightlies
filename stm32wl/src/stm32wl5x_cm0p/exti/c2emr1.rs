#[doc = "Register `C2EMR1` reader"]
pub struct R(crate::R<C2EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2EMR1` writer"]
pub struct W(crate::W<C2EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2EMR1_SPEC>;
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
impl From<crate::W<C2EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - Wakeup with event generation Mask on Event input"]
pub type EM0_R = crate::BitReader<EM0_A>;
#[doc = "Wakeup with event generation Mask on Event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM0_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<EM0_A> for bool {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        variant as u8 != 0
    }
}
impl EM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EM0_A {
        match self.bits {
            false => EM0_A::Masked,
            true => EM0_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM0_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM0_A::Unmasked
    }
}
#[doc = "Field `EM0` writer - Wakeup with event generation Mask on Event input"]
pub type EM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2EMR1_SPEC, EM0_A, O>;
impl<'a, const O: u8> EM0_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM0_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM0_A::Unmasked)
    }
}
#[doc = "Field `EM1` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM1_R;
#[doc = "Field `EM2` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM2_R;
#[doc = "Field `EM3` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM3_R;
#[doc = "Field `EM4` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM4_R;
#[doc = "Field `EM5` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM5_R;
#[doc = "Field `EM6` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM6_R;
#[doc = "Field `EM7` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM7_R;
#[doc = "Field `EM8` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM8_R;
#[doc = "Field `EM9` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM9_R;
#[doc = "Field `EM10` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM10_R;
#[doc = "Field `EM11` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM11_R;
#[doc = "Field `EM12` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM12_R;
#[doc = "Field `EM13` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM13_R;
#[doc = "Field `EM14` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM14_R;
#[doc = "Field `EM15` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM15_R;
#[doc = "Field `EM17` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM17_R;
#[doc = "Field `EM18` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM18_R;
#[doc = "Field `EM19` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM19_R;
#[doc = "Field `EM20` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM20_R;
#[doc = "Field `EM21` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM21_R;
#[doc = "Field `EM22` reader - Wakeup with event generation Mask on Event input"]
pub use EM0_R as EM22_R;
#[doc = "Field `EM1` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM1_W;
#[doc = "Field `EM2` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM2_W;
#[doc = "Field `EM3` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM3_W;
#[doc = "Field `EM4` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM4_W;
#[doc = "Field `EM5` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM5_W;
#[doc = "Field `EM6` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM6_W;
#[doc = "Field `EM7` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM7_W;
#[doc = "Field `EM8` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM8_W;
#[doc = "Field `EM9` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM9_W;
#[doc = "Field `EM10` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM10_W;
#[doc = "Field `EM11` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM11_W;
#[doc = "Field `EM12` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM12_W;
#[doc = "Field `EM13` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM13_W;
#[doc = "Field `EM14` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM14_W;
#[doc = "Field `EM15` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM15_W;
#[doc = "Field `EM17` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM17_W;
#[doc = "Field `EM18` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM18_W;
#[doc = "Field `EM19` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM19_W;
#[doc = "Field `EM20` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM20_W;
#[doc = "Field `EM21` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM21_W;
#[doc = "Field `EM22` writer - Wakeup with event generation Mask on Event input"]
pub use EM0_W as EM22_W;
impl R {
    #[doc = "Bit 0 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W<0> {
        EM0_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W<1> {
        EM1_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W<2> {
        EM2_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W<3> {
        EM3_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em4(&mut self) -> EM4_W<4> {
        EM4_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em5(&mut self) -> EM5_W<5> {
        EM5_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em6(&mut self) -> EM6_W<6> {
        EM6_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em7(&mut self) -> EM7_W<7> {
        EM7_W::new(self)
    }
    #[doc = "Bit 8 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em8(&mut self) -> EM8_W<8> {
        EM8_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em9(&mut self) -> EM9_W<9> {
        EM9_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em10(&mut self) -> EM10_W<10> {
        EM10_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em11(&mut self) -> EM11_W<11> {
        EM11_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em12(&mut self) -> EM12_W<12> {
        EM12_W::new(self)
    }
    #[doc = "Bit 13 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em13(&mut self) -> EM13_W<13> {
        EM13_W::new(self)
    }
    #[doc = "Bit 14 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em14(&mut self) -> EM14_W<14> {
        EM14_W::new(self)
    }
    #[doc = "Bit 15 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em15(&mut self) -> EM15_W<15> {
        EM15_W::new(self)
    }
    #[doc = "Bit 17 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em17(&mut self) -> EM17_W<17> {
        EM17_W::new(self)
    }
    #[doc = "Bit 18 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em18(&mut self) -> EM18_W<18> {
        EM18_W::new(self)
    }
    #[doc = "Bit 19 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em19(&mut self) -> EM19_W<19> {
        EM19_W::new(self)
    }
    #[doc = "Bit 20 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em20(&mut self) -> EM20_W<20> {
        EM20_W::new(self)
    }
    #[doc = "Bit 21 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em21(&mut self) -> EM21_W<21> {
        EM21_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup with event generation Mask on Event input"]
    #[inline(always)]
    pub fn em22(&mut self) -> EM22_W<22> {
        EM22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr1](index.html) module"]
pub struct C2EMR1_SPEC;
impl crate::RegisterSpec for C2EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2emr1::R](R) reader structure"]
impl crate::Readable for C2EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2emr1::W](W) writer structure"]
impl crate::Writable for C2EMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2EMR1 to value 0"]
impl crate::Resettable for C2EMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
