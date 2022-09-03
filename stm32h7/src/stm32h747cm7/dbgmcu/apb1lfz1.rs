#[doc = "Register `APB1LFZ1` reader"]
pub struct R(crate::R<APB1LFZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LFZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LFZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LFZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LFZ1` writer"]
pub struct W(crate::W<APB1LFZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LFZ1_SPEC>;
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
impl From<crate::W<APB1LFZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LFZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM2` reader - TIM2 stop in debug"]
pub type DBG_TIM2_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM2` writer - TIM2 stop in debug"]
pub type DBG_TIM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM3` reader - TIM3 stop in debug"]
pub type DBG_TIM3_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM3` writer - TIM3 stop in debug"]
pub type DBG_TIM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM4` reader - TIM4 stop in debug"]
pub type DBG_TIM4_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM4` writer - TIM4 stop in debug"]
pub type DBG_TIM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM5` reader - TIM5 stop in debug"]
pub type DBG_TIM5_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM5` writer - TIM5 stop in debug"]
pub type DBG_TIM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM6` reader - TIM6 stop in debug"]
pub type DBG_TIM6_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM6` writer - TIM6 stop in debug"]
pub type DBG_TIM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM7` reader - TIM7 stop in debug"]
pub type DBG_TIM7_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM7` writer - TIM7 stop in debug"]
pub type DBG_TIM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM12` reader - TIM12 stop in debug"]
pub type DBG_TIM12_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM12` writer - TIM12 stop in debug"]
pub type DBG_TIM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM13` reader - TIM13 stop in debug"]
pub type DBG_TIM13_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM13` writer - TIM13 stop in debug"]
pub type DBG_TIM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM14` reader - TIM14 stop in debug"]
pub type DBG_TIM14_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM14` writer - TIM14 stop in debug"]
pub type DBG_TIM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIM1` reader - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIM1` writer - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG2` reader - WWDG2 stop in debug"]
pub type DBG_WWDG2_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG2` writer - WWDG2 stop in debug"]
pub type DBG_WWDG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub type DBG_I2C1_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub type DBG_I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub type DBG_I2C2_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub type DBG_I2C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LFZ1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&self) -> DBG_TIM2_R {
        DBG_TIM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&self) -> DBG_TIM3_R {
        DBG_TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&self) -> DBG_TIM4_R {
        DBG_TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&self) -> DBG_TIM5_R {
        DBG_TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&self) -> DBG_TIM6_R {
        DBG_TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&self) -> DBG_TIM7_R {
        DBG_TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&self) -> DBG_TIM12_R {
        DBG_TIM12_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&self) -> DBG_TIM13_R {
        DBG_TIM13_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&self) -> DBG_TIM14_R {
        DBG_TIM14_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&self) -> DBG_LPTIM1_R {
        DBG_LPTIM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&self) -> DBG_WWDG2_R {
        DBG_WWDG2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&self) -> DBG_I2C1_R {
        DBG_I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&self) -> DBG_I2C2_R {
        DBG_I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&self) -> DBG_I2C3_R {
        DBG_I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&mut self) -> DBG_TIM2_W<0> {
        DBG_TIM2_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&mut self) -> DBG_TIM3_W<1> {
        DBG_TIM3_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&mut self) -> DBG_TIM4_W<2> {
        DBG_TIM4_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&mut self) -> DBG_TIM5_W<3> {
        DBG_TIM5_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&mut self) -> DBG_TIM6_W<4> {
        DBG_TIM6_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&mut self) -> DBG_TIM7_W<5> {
        DBG_TIM7_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&mut self) -> DBG_TIM12_W<6> {
        DBG_TIM12_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&mut self) -> DBG_TIM13_W<7> {
        DBG_TIM13_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&mut self) -> DBG_TIM14_W<8> {
        DBG_TIM14_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&mut self) -> DBG_LPTIM1_W<9> {
        DBG_LPTIM1_W::new(self)
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&mut self) -> DBG_WWDG2_W<11> {
        DBG_WWDG2_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&mut self) -> DBG_I2C1_W<21> {
        DBG_I2C1_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&mut self) -> DBG_I2C2_W<22> {
        DBG_I2C2_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&mut self) -> DBG_I2C3_W<23> {
        DBG_I2C3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz1](index.html) module"]
pub struct APB1LFZ1_SPEC;
impl crate::RegisterSpec for APB1LFZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lfz1::R](R) reader structure"]
impl crate::Readable for APB1LFZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lfz1::W](W) writer structure"]
impl crate::Writable for APB1LFZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LFZ1 to value 0"]
impl crate::Resettable for APB1LFZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
