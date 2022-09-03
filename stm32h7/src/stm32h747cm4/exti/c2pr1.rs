#[doc = "Register `C2PR1` reader"]
pub struct R(crate::R<C2PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2PR1` writer"]
pub struct W(crate::W<C2PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2PR1_SPEC>;
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
impl From<crate::W<C2PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - Configurable event input pending"]
pub type PR0_R = crate::BitReader<PR0R_A>;
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PR0R_A> for bool {
    #[inline(always)]
    fn from(variant: PR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl PR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR0R_A {
        match self.bits {
            false => PR0R_A::NotPending,
            true => PR0R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PR0R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PR0R_A::Pending
    }
}
#[doc = "Configurable event input pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR0W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: PR0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PR0` writer - Configurable event input pending"]
pub type PR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2PR1_SPEC, PR0W_AW, O>;
impl<'a, const O: u8> PR0_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PR0W_AW::Clear)
    }
}
#[doc = "Field `PR1` reader - Configurable event input pending"]
pub use PR0_R as PR1_R;
#[doc = "Field `PR2` reader - Configurable event input pending"]
pub use PR0_R as PR2_R;
#[doc = "Field `PR3` reader - Configurable event input pending"]
pub use PR0_R as PR3_R;
#[doc = "Field `PR4` reader - Configurable event input pending"]
pub use PR0_R as PR4_R;
#[doc = "Field `PR5` reader - Configurable event input pending"]
pub use PR0_R as PR5_R;
#[doc = "Field `PR6` reader - Configurable event input pending"]
pub use PR0_R as PR6_R;
#[doc = "Field `PR7` reader - Configurable event input pending"]
pub use PR0_R as PR7_R;
#[doc = "Field `PR8` reader - Configurable event input pending"]
pub use PR0_R as PR8_R;
#[doc = "Field `PR9` reader - Configurable event input pending"]
pub use PR0_R as PR9_R;
#[doc = "Field `PR10` reader - Configurable event input pending"]
pub use PR0_R as PR10_R;
#[doc = "Field `PR11` reader - Configurable event input pending"]
pub use PR0_R as PR11_R;
#[doc = "Field `PR12` reader - Configurable event input pending"]
pub use PR0_R as PR12_R;
#[doc = "Field `PR13` reader - Configurable event input pending"]
pub use PR0_R as PR13_R;
#[doc = "Field `PR14` reader - Configurable event input pending"]
pub use PR0_R as PR14_R;
#[doc = "Field `PR15` reader - Configurable event input pending"]
pub use PR0_R as PR15_R;
#[doc = "Field `PR16` reader - Configurable event input pending"]
pub use PR0_R as PR16_R;
#[doc = "Field `PR17` reader - Configurable event input pending"]
pub use PR0_R as PR17_R;
#[doc = "Field `PR18` reader - Configurable event input pending"]
pub use PR0_R as PR18_R;
#[doc = "Field `PR19` reader - Configurable event input pending"]
pub use PR0_R as PR19_R;
#[doc = "Field `PR20` reader - Configurable event input pending"]
pub use PR0_R as PR20_R;
#[doc = "Field `PR21` reader - Configurable event input pending"]
pub use PR0_R as PR21_R;
#[doc = "Field `PR1` writer - Configurable event input pending"]
pub use PR0_W as PR1_W;
#[doc = "Field `PR2` writer - Configurable event input pending"]
pub use PR0_W as PR2_W;
#[doc = "Field `PR3` writer - Configurable event input pending"]
pub use PR0_W as PR3_W;
#[doc = "Field `PR4` writer - Configurable event input pending"]
pub use PR0_W as PR4_W;
#[doc = "Field `PR5` writer - Configurable event input pending"]
pub use PR0_W as PR5_W;
#[doc = "Field `PR6` writer - Configurable event input pending"]
pub use PR0_W as PR6_W;
#[doc = "Field `PR7` writer - Configurable event input pending"]
pub use PR0_W as PR7_W;
#[doc = "Field `PR8` writer - Configurable event input pending"]
pub use PR0_W as PR8_W;
#[doc = "Field `PR9` writer - Configurable event input pending"]
pub use PR0_W as PR9_W;
#[doc = "Field `PR10` writer - Configurable event input pending"]
pub use PR0_W as PR10_W;
#[doc = "Field `PR11` writer - Configurable event input pending"]
pub use PR0_W as PR11_W;
#[doc = "Field `PR12` writer - Configurable event input pending"]
pub use PR0_W as PR12_W;
#[doc = "Field `PR13` writer - Configurable event input pending"]
pub use PR0_W as PR13_W;
#[doc = "Field `PR14` writer - Configurable event input pending"]
pub use PR0_W as PR14_W;
#[doc = "Field `PR15` writer - Configurable event input pending"]
pub use PR0_W as PR15_W;
#[doc = "Field `PR16` writer - Configurable event input pending"]
pub use PR0_W as PR16_W;
#[doc = "Field `PR17` writer - Configurable event input pending"]
pub use PR0_W as PR17_W;
#[doc = "Field `PR18` writer - Configurable event input pending"]
pub use PR0_W as PR18_W;
#[doc = "Field `PR19` writer - Configurable event input pending"]
pub use PR0_W as PR19_W;
#[doc = "Field `PR20` writer - Configurable event input pending"]
pub use PR0_W as PR20_W;
#[doc = "Field `PR21` writer - Configurable event input pending"]
pub use PR0_W as PR21_W;
impl R {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr0(&mut self) -> PR0_W<0> {
        PR0_W::new(self)
    }
    #[doc = "Bit 1 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W<1> {
        PR1_W::new(self)
    }
    #[doc = "Bit 2 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr2(&mut self) -> PR2_W<2> {
        PR2_W::new(self)
    }
    #[doc = "Bit 3 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W<3> {
        PR3_W::new(self)
    }
    #[doc = "Bit 4 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W<4> {
        PR4_W::new(self)
    }
    #[doc = "Bit 5 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W<5> {
        PR5_W::new(self)
    }
    #[doc = "Bit 6 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr6(&mut self) -> PR6_W<6> {
        PR6_W::new(self)
    }
    #[doc = "Bit 7 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr7(&mut self) -> PR7_W<7> {
        PR7_W::new(self)
    }
    #[doc = "Bit 8 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr8(&mut self) -> PR8_W<8> {
        PR8_W::new(self)
    }
    #[doc = "Bit 9 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr9(&mut self) -> PR9_W<9> {
        PR9_W::new(self)
    }
    #[doc = "Bit 10 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr10(&mut self) -> PR10_W<10> {
        PR10_W::new(self)
    }
    #[doc = "Bit 11 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr11(&mut self) -> PR11_W<11> {
        PR11_W::new(self)
    }
    #[doc = "Bit 12 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr12(&mut self) -> PR12_W<12> {
        PR12_W::new(self)
    }
    #[doc = "Bit 13 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr13(&mut self) -> PR13_W<13> {
        PR13_W::new(self)
    }
    #[doc = "Bit 14 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr14(&mut self) -> PR14_W<14> {
        PR14_W::new(self)
    }
    #[doc = "Bit 15 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr15(&mut self) -> PR15_W<15> {
        PR15_W::new(self)
    }
    #[doc = "Bit 16 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr16(&mut self) -> PR16_W<16> {
        PR16_W::new(self)
    }
    #[doc = "Bit 17 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr17(&mut self) -> PR17_W<17> {
        PR17_W::new(self)
    }
    #[doc = "Bit 18 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr18(&mut self) -> PR18_W<18> {
        PR18_W::new(self)
    }
    #[doc = "Bit 19 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr19(&mut self) -> PR19_W<19> {
        PR19_W::new(self)
    }
    #[doc = "Bit 20 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr20(&mut self) -> PR20_W<20> {
        PR20_W::new(self)
    }
    #[doc = "Bit 21 - Configurable event input pending"]
    #[inline(always)]
    pub fn pr21(&mut self) -> PR21_W<21> {
        PR21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2pr1](index.html) module"]
pub struct C2PR1_SPEC;
impl crate::RegisterSpec for C2PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2pr1::R](R) reader structure"]
impl crate::Readable for C2PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2pr1::W](W) writer structure"]
impl crate::Writable for C2PR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2PR1 to value 0"]
impl crate::Resettable for C2PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
