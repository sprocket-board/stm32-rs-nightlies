#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - CPU wakeup with interrupt mask on event input"]
pub type IM0_R = crate::BitReader<IM0_A>;
#[doc = "CPU wakeup with interrupt mask on event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM0_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM0_A> for bool {
    #[inline(always)]
    fn from(variant: IM0_A) -> Self {
        variant as u8 != 0
    }
}
impl IM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM0_A {
        match self.bits {
            false => IM0_A::Masked,
            true => IM0_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM0_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM0_A::Unmasked
    }
}
#[doc = "Field `IM0` writer - CPU wakeup with interrupt mask on event input"]
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, IM0_A, O>;
impl<'a, const O: u8> IM0_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM0_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM0_A::Unmasked)
    }
}
#[doc = "Field `IM1` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM1_R;
#[doc = "Field `IM2` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM2_R;
#[doc = "Field `IM3` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM3_R;
#[doc = "Field `IM4` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM4_R;
#[doc = "Field `IM5` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM5_R;
#[doc = "Field `IM6` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM6_R;
#[doc = "Field `IM7` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM7_R;
#[doc = "Field `IM8` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM8_R;
#[doc = "Field `IM9` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM9_R;
#[doc = "Field `IM10` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM10_R;
#[doc = "Field `IM11` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM11_R;
#[doc = "Field `IM12` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM12_R;
#[doc = "Field `IM13` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM13_R;
#[doc = "Field `IM14` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM14_R;
#[doc = "Field `IM15` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM15_R;
#[doc = "Field `IM16` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM16_R;
#[doc = "Field `IM19` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM19_R;
#[doc = "Field `IM20` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM20_R;
#[doc = "Field `IM21` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM21_R;
#[doc = "Field `IM22` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM22_R;
#[doc = "Field `IM23` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM23_R;
#[doc = "Field `IM24` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM24_R;
#[doc = "Field `IM25` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM25_R;
#[doc = "Field `IM26` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM26_R;
#[doc = "Field `IM28` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM28_R;
#[doc = "Field `IM29` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM29_R;
#[doc = "Field `IM30` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM30_R;
#[doc = "Field `IM31` reader - CPU wakeup with interrupt mask on event input"]
pub use IM0_R as IM31_R;
#[doc = "Field `IM1` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM1_W;
#[doc = "Field `IM2` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM2_W;
#[doc = "Field `IM3` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM3_W;
#[doc = "Field `IM4` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM4_W;
#[doc = "Field `IM5` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM5_W;
#[doc = "Field `IM6` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM6_W;
#[doc = "Field `IM7` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM7_W;
#[doc = "Field `IM8` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM8_W;
#[doc = "Field `IM9` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM9_W;
#[doc = "Field `IM10` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM10_W;
#[doc = "Field `IM11` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM11_W;
#[doc = "Field `IM12` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM12_W;
#[doc = "Field `IM13` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM13_W;
#[doc = "Field `IM14` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM14_W;
#[doc = "Field `IM15` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM15_W;
#[doc = "Field `IM16` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM16_W;
#[doc = "Field `IM19` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM19_W;
#[doc = "Field `IM20` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM20_W;
#[doc = "Field `IM21` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM21_W;
#[doc = "Field `IM22` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM22_W;
#[doc = "Field `IM23` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM23_W;
#[doc = "Field `IM24` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM24_W;
#[doc = "Field `IM25` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM25_W;
#[doc = "Field `IM26` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM26_W;
#[doc = "Field `IM28` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM28_W;
#[doc = "Field `IM29` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM29_W;
#[doc = "Field `IM30` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM30_W;
#[doc = "Field `IM31` writer - CPU wakeup with interrupt mask on event input"]
pub use IM0_W as IM31_W;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im26(&self) -> IM26_R {
        IM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im28(&self) -> IM28_R {
        IM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im30(&self) -> IM30_R {
        IM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W<9> {
        IM9_W::new(self)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W<20> {
        IM20_W::new(self)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W<21> {
        IM21_W::new(self)
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W<22> {
        IM22_W::new(self)
    }
    #[doc = "Bit 23 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W<23> {
        IM23_W::new(self)
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im24(&mut self) -> IM24_W<24> {
        IM24_W::new(self)
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W<25> {
        IM25_W::new(self)
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im26(&mut self) -> IM26_W<26> {
        IM26_W::new(self)
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im28(&mut self) -> IM28_W<28> {
        IM28_W::new(self)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&mut self) -> IM29_W<29> {
        IM29_W::new(self)
    }
    #[doc = "Bit 30 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im30(&mut self) -> IM30_W<30> {
        IM30_W::new(self)
    }
    #[doc = "Bit 31 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im31(&mut self) -> IM31_W<31> {
        IM31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR1 to value 0xfff8_0000"]
impl crate::Resettable for IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfff8_0000
    }
}
