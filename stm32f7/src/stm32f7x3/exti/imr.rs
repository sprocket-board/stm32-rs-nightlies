#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - Interrupt Mask on line 0"]
pub type IM0_R = crate::BitReader<IM0_A>;
#[doc = "Interrupt Mask on line 0\n\nValue on reset: 0"]
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
#[doc = "Field `IM0` writer - Interrupt Mask on line 0"]
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, IM0_A, O>;
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
#[doc = "Field `IM1` reader - Interrupt Mask on line 1"]
pub use IM0_R as IM1_R;
#[doc = "Field `IM2` reader - Interrupt Mask on line 2"]
pub use IM0_R as IM2_R;
#[doc = "Field `IM3` reader - Interrupt Mask on line 3"]
pub use IM0_R as IM3_R;
#[doc = "Field `IM4` reader - Interrupt Mask on line 4"]
pub use IM0_R as IM4_R;
#[doc = "Field `IM5` reader - Interrupt Mask on line 5"]
pub use IM0_R as IM5_R;
#[doc = "Field `IM6` reader - Interrupt Mask on line 6"]
pub use IM0_R as IM6_R;
#[doc = "Field `IM7` reader - Interrupt Mask on line 7"]
pub use IM0_R as IM7_R;
#[doc = "Field `IM8` reader - Interrupt Mask on line 8"]
pub use IM0_R as IM8_R;
#[doc = "Field `IM1` writer - Interrupt Mask on line 1"]
pub use IM0_W as IM1_W;
#[doc = "Field `IM2` writer - Interrupt Mask on line 2"]
pub use IM0_W as IM2_W;
#[doc = "Field `IM3` writer - Interrupt Mask on line 3"]
pub use IM0_W as IM3_W;
#[doc = "Field `IM4` writer - Interrupt Mask on line 4"]
pub use IM0_W as IM4_W;
#[doc = "Field `IM5` writer - Interrupt Mask on line 5"]
pub use IM0_W as IM5_W;
#[doc = "Field `IM6` writer - Interrupt Mask on line 6"]
pub use IM0_W as IM6_W;
#[doc = "Field `IM7` writer - Interrupt Mask on line 7"]
pub use IM0_W as IM7_W;
#[doc = "Field `IM8` writer - Interrupt Mask on line 8"]
pub use IM0_W as IM8_W;
#[doc = "Field `MI9` reader - Interrupt Mask on line 9"]
pub type MI9_R = crate::BitReader<bool>;
#[doc = "Field `MI9` writer - Interrupt Mask on line 9"]
pub type MI9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IM10` reader - Interrupt Mask on line 10"]
pub use IM0_R as IM10_R;
#[doc = "Field `IM11` reader - Interrupt Mask on line 11"]
pub use IM0_R as IM11_R;
#[doc = "Field `IM12` reader - Interrupt Mask on line 12"]
pub use IM0_R as IM12_R;
#[doc = "Field `IM13` reader - Interrupt Mask on line 13"]
pub use IM0_R as IM13_R;
#[doc = "Field `IM14` reader - Interrupt Mask on line 14"]
pub use IM0_R as IM14_R;
#[doc = "Field `IM15` reader - Interrupt Mask on line 15"]
pub use IM0_R as IM15_R;
#[doc = "Field `IM16` reader - Interrupt Mask on line 16"]
pub use IM0_R as IM16_R;
#[doc = "Field `IM17` reader - Interrupt Mask on line 17"]
pub use IM0_R as IM17_R;
#[doc = "Field `IM18` reader - Interrupt Mask on line 18"]
pub use IM0_R as IM18_R;
#[doc = "Field `IM19` reader - Interrupt Mask on line 19"]
pub use IM0_R as IM19_R;
#[doc = "Field `IM20` reader - Interrupt Mask on line 20"]
pub use IM0_R as IM20_R;
#[doc = "Field `IM21` reader - Interrupt Mask on line 21"]
pub use IM0_R as IM21_R;
#[doc = "Field `IM22` reader - Interrupt Mask on line 22"]
pub use IM0_R as IM22_R;
#[doc = "Field `IM23` reader - Interrupt Mask on line 23"]
pub use IM0_R as IM23_R;
#[doc = "Field `IM10` writer - Interrupt Mask on line 10"]
pub use IM0_W as IM10_W;
#[doc = "Field `IM11` writer - Interrupt Mask on line 11"]
pub use IM0_W as IM11_W;
#[doc = "Field `IM12` writer - Interrupt Mask on line 12"]
pub use IM0_W as IM12_W;
#[doc = "Field `IM13` writer - Interrupt Mask on line 13"]
pub use IM0_W as IM13_W;
#[doc = "Field `IM14` writer - Interrupt Mask on line 14"]
pub use IM0_W as IM14_W;
#[doc = "Field `IM15` writer - Interrupt Mask on line 15"]
pub use IM0_W as IM15_W;
#[doc = "Field `IM16` writer - Interrupt Mask on line 16"]
pub use IM0_W as IM16_W;
#[doc = "Field `IM17` writer - Interrupt Mask on line 17"]
pub use IM0_W as IM17_W;
#[doc = "Field `IM18` writer - Interrupt Mask on line 18"]
pub use IM0_W as IM18_W;
#[doc = "Field `IM19` writer - Interrupt Mask on line 19"]
pub use IM0_W as IM19_W;
#[doc = "Field `IM20` writer - Interrupt Mask on line 20"]
pub use IM0_W as IM20_W;
#[doc = "Field `IM21` writer - Interrupt Mask on line 21"]
pub use IM0_W as IM21_W;
#[doc = "Field `IM22` writer - Interrupt Mask on line 22"]
pub use IM0_W as IM22_W;
#[doc = "Field `IM23` writer - Interrupt Mask on line 23"]
pub use IM0_W as IM23_W;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn mi9(&self) -> MI9_R {
        MI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Mask on line 23"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn mi9(&mut self) -> MI9_W<9> {
        MI9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W<17> {
        IM17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W<18> {
        IM18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W<20> {
        IM20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W<21> {
        IM21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W<22> {
        IM22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt Mask on line 23"]
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W<23> {
        IM23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register (EXTI_IMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
