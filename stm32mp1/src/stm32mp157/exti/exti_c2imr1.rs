#[doc = "Register `EXTI_C2IMR1` reader"]
pub struct R(crate::R<EXTI_C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_C2IMR1` writer"]
pub struct W(crate::W<EXTI_C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_C2IMR1_SPEC>;
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
impl From<crate::W<EXTI_C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - IM0"]
pub type IM0_R = crate::BitReader<bool>;
#[doc = "Field `IM0` writer - IM0"]
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM1` reader - IM1"]
pub type IM1_R = crate::BitReader<bool>;
#[doc = "Field `IM1` writer - IM1"]
pub type IM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM2` reader - IM2"]
pub type IM2_R = crate::BitReader<bool>;
#[doc = "Field `IM2` writer - IM2"]
pub type IM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM3` reader - IM3"]
pub type IM3_R = crate::BitReader<bool>;
#[doc = "Field `IM3` writer - IM3"]
pub type IM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM4` reader - IM4"]
pub type IM4_R = crate::BitReader<bool>;
#[doc = "Field `IM4` writer - IM4"]
pub type IM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM5` reader - IM5"]
pub type IM5_R = crate::BitReader<bool>;
#[doc = "Field `IM5` writer - IM5"]
pub type IM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM6` reader - IM6"]
pub type IM6_R = crate::BitReader<bool>;
#[doc = "Field `IM6` writer - IM6"]
pub type IM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM7` reader - IM7"]
pub type IM7_R = crate::BitReader<bool>;
#[doc = "Field `IM7` writer - IM7"]
pub type IM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM8` reader - IM8"]
pub type IM8_R = crate::BitReader<bool>;
#[doc = "Field `IM8` writer - IM8"]
pub type IM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM9` reader - IM9"]
pub type IM9_R = crate::BitReader<bool>;
#[doc = "Field `IM9` writer - IM9"]
pub type IM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM10` reader - IM10"]
pub type IM10_R = crate::BitReader<bool>;
#[doc = "Field `IM10` writer - IM10"]
pub type IM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM11` reader - IM11"]
pub type IM11_R = crate::BitReader<bool>;
#[doc = "Field `IM11` writer - IM11"]
pub type IM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM12` reader - IM12"]
pub type IM12_R = crate::BitReader<bool>;
#[doc = "Field `IM12` writer - IM12"]
pub type IM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM13` reader - IM13"]
pub type IM13_R = crate::BitReader<bool>;
#[doc = "Field `IM13` writer - IM13"]
pub type IM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM14` reader - IM14"]
pub type IM14_R = crate::BitReader<bool>;
#[doc = "Field `IM14` writer - IM14"]
pub type IM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM15` reader - IM15"]
pub type IM15_R = crate::BitReader<bool>;
#[doc = "Field `IM15` writer - IM15"]
pub type IM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM16` reader - IM16"]
pub type IM16_R = crate::BitReader<bool>;
#[doc = "Field `IM16` writer - IM16"]
pub type IM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM17` reader - IM17"]
pub type IM17_R = crate::BitReader<bool>;
#[doc = "Field `IM17` writer - IM17"]
pub type IM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM18` reader - IM18"]
pub type IM18_R = crate::BitReader<bool>;
#[doc = "Field `IM18` writer - IM18"]
pub type IM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM19` reader - IM19"]
pub type IM19_R = crate::BitReader<bool>;
#[doc = "Field `IM19` writer - IM19"]
pub type IM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM20` reader - IM20"]
pub type IM20_R = crate::BitReader<bool>;
#[doc = "Field `IM20` writer - IM20"]
pub type IM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM21` reader - IM21"]
pub type IM21_R = crate::BitReader<bool>;
#[doc = "Field `IM21` writer - IM21"]
pub type IM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM22` reader - IM22"]
pub type IM22_R = crate::BitReader<bool>;
#[doc = "Field `IM22` writer - IM22"]
pub type IM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM23` reader - IM23"]
pub type IM23_R = crate::BitReader<bool>;
#[doc = "Field `IM23` writer - IM23"]
pub type IM23_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM24` reader - IM24"]
pub type IM24_R = crate::BitReader<bool>;
#[doc = "Field `IM24` writer - IM24"]
pub type IM24_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM25` reader - IM25"]
pub type IM25_R = crate::BitReader<bool>;
#[doc = "Field `IM25` writer - IM25"]
pub type IM25_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM26` reader - IM26"]
pub type IM26_R = crate::BitReader<bool>;
#[doc = "Field `IM26` writer - IM26"]
pub type IM26_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM27` reader - IM27"]
pub type IM27_R = crate::BitReader<bool>;
#[doc = "Field `IM27` writer - IM27"]
pub type IM27_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM28` reader - IM28"]
pub type IM28_R = crate::BitReader<bool>;
#[doc = "Field `IM28` writer - IM28"]
pub type IM28_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM29` reader - IM29"]
pub type IM29_R = crate::BitReader<bool>;
#[doc = "Field `IM29` writer - IM29"]
pub type IM29_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM30` reader - IM30"]
pub type IM30_R = crate::BitReader<bool>;
#[doc = "Field `IM30` writer - IM30"]
pub type IM30_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
#[doc = "Field `IM31` reader - IM31"]
pub type IM31_R = crate::BitReader<bool>;
#[doc = "Field `IM31` writer - IM31"]
pub type IM31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2IMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IM0"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IM1"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IM2"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IM3"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IM4"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IM5"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IM6"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IM7"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IM8"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IM9"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IM10"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - IM11"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IM12"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IM13"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - IM14"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IM15"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - IM16"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IM17"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IM18"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IM20"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IM21"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IM22"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IM24"]
    #[inline(always)]
    pub fn im24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IM26"]
    #[inline(always)]
    pub fn im26(&self) -> IM26_R {
        IM26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IM27"]
    #[inline(always)]
    pub fn im27(&self) -> IM27_R {
        IM27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IM28"]
    #[inline(always)]
    pub fn im28(&self) -> IM28_R {
        IM28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IM29"]
    #[inline(always)]
    pub fn im29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IM30"]
    #[inline(always)]
    pub fn im30(&self) -> IM30_R {
        IM30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IM31"]
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IM0"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    #[doc = "Bit 1 - IM1"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    #[doc = "Bit 2 - IM2"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    #[doc = "Bit 3 - IM3"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    #[doc = "Bit 4 - IM4"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    #[doc = "Bit 5 - IM5"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    #[doc = "Bit 6 - IM6"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    #[doc = "Bit 7 - IM7"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    #[doc = "Bit 8 - IM8"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    #[doc = "Bit 9 - IM9"]
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W<9> {
        IM9_W::new(self)
    }
    #[doc = "Bit 10 - IM10"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    #[doc = "Bit 11 - IM11"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    #[doc = "Bit 12 - IM12"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    #[doc = "Bit 13 - IM13"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    #[doc = "Bit 14 - IM14"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    #[doc = "Bit 15 - IM15"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    #[doc = "Bit 16 - IM16"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    #[doc = "Bit 17 - IM17"]
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W<17> {
        IM17_W::new(self)
    }
    #[doc = "Bit 18 - IM18"]
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W<18> {
        IM18_W::new(self)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    #[doc = "Bit 20 - IM20"]
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W<20> {
        IM20_W::new(self)
    }
    #[doc = "Bit 21 - IM21"]
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W<21> {
        IM21_W::new(self)
    }
    #[doc = "Bit 22 - IM22"]
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W<22> {
        IM22_W::new(self)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W<23> {
        IM23_W::new(self)
    }
    #[doc = "Bit 24 - IM24"]
    #[inline(always)]
    pub fn im24(&mut self) -> IM24_W<24> {
        IM24_W::new(self)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W<25> {
        IM25_W::new(self)
    }
    #[doc = "Bit 26 - IM26"]
    #[inline(always)]
    pub fn im26(&mut self) -> IM26_W<26> {
        IM26_W::new(self)
    }
    #[doc = "Bit 27 - IM27"]
    #[inline(always)]
    pub fn im27(&mut self) -> IM27_W<27> {
        IM27_W::new(self)
    }
    #[doc = "Bit 28 - IM28"]
    #[inline(always)]
    pub fn im28(&mut self) -> IM28_W<28> {
        IM28_W::new(self)
    }
    #[doc = "Bit 29 - IM29"]
    #[inline(always)]
    pub fn im29(&mut self) -> IM29_W<29> {
        IM29_W::new(self)
    }
    #[doc = "Bit 30 - IM30"]
    #[inline(always)]
    pub fn im30(&mut self) -> IM30_W<30> {
        IM30_W::new(self)
    }
    #[doc = "Bit 31 - IM31"]
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
#[doc = "Contains register bits for configurable events and Direct events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_c2imr1](index.html) module"]
pub struct EXTI_C2IMR1_SPEC;
impl crate::RegisterSpec for EXTI_C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_c2imr1::R](R) reader structure"]
impl crate::Readable for EXTI_C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_c2imr1::W](W) writer structure"]
impl crate::Writable for EXTI_C2IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_C2IMR1 to value 0xfffe_0000"]
impl crate::Resettable for EXTI_C2IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfffe_0000
    }
}
