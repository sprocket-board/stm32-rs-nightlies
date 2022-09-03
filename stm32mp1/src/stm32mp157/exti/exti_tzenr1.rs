#[doc = "Register `EXTI_TZENR1` reader"]
pub struct R(crate::R<EXTI_TZENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TZENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TZENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TZENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TZENR1` writer"]
pub struct W(crate::W<EXTI_TZENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TZENR1_SPEC>;
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
impl From<crate::W<EXTI_TZENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TZENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZEN0` reader - TZEN0"]
pub type TZEN0_R = crate::BitReader<bool>;
#[doc = "Field `TZEN0` writer - TZEN0"]
pub type TZEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN1` reader - TZEN1"]
pub type TZEN1_R = crate::BitReader<bool>;
#[doc = "Field `TZEN1` writer - TZEN1"]
pub type TZEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN2` reader - TZEN2"]
pub type TZEN2_R = crate::BitReader<bool>;
#[doc = "Field `TZEN2` writer - TZEN2"]
pub type TZEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN3` reader - TZEN3"]
pub type TZEN3_R = crate::BitReader<bool>;
#[doc = "Field `TZEN3` writer - TZEN3"]
pub type TZEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN4` reader - TZEN4"]
pub type TZEN4_R = crate::BitReader<bool>;
#[doc = "Field `TZEN4` writer - TZEN4"]
pub type TZEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN5` reader - TZEN5"]
pub type TZEN5_R = crate::BitReader<bool>;
#[doc = "Field `TZEN5` writer - TZEN5"]
pub type TZEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN6` reader - TZEN6"]
pub type TZEN6_R = crate::BitReader<bool>;
#[doc = "Field `TZEN6` writer - TZEN6"]
pub type TZEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN7` reader - TZEN7"]
pub type TZEN7_R = crate::BitReader<bool>;
#[doc = "Field `TZEN7` writer - TZEN7"]
pub type TZEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN8` reader - TZEN8"]
pub type TZEN8_R = crate::BitReader<bool>;
#[doc = "Field `TZEN8` writer - TZEN8"]
pub type TZEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN9` reader - TZEN9"]
pub type TZEN9_R = crate::BitReader<bool>;
#[doc = "Field `TZEN9` writer - TZEN9"]
pub type TZEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN10` reader - TZEN10"]
pub type TZEN10_R = crate::BitReader<bool>;
#[doc = "Field `TZEN10` writer - TZEN10"]
pub type TZEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN11` reader - TZEN11"]
pub type TZEN11_R = crate::BitReader<bool>;
#[doc = "Field `TZEN11` writer - TZEN11"]
pub type TZEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN12` reader - TZEN12"]
pub type TZEN12_R = crate::BitReader<bool>;
#[doc = "Field `TZEN12` writer - TZEN12"]
pub type TZEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN13` reader - TZEN13"]
pub type TZEN13_R = crate::BitReader<bool>;
#[doc = "Field `TZEN13` writer - TZEN13"]
pub type TZEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN14` reader - TZEN14"]
pub type TZEN14_R = crate::BitReader<bool>;
#[doc = "Field `TZEN14` writer - TZEN14"]
pub type TZEN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN15` reader - TZEN15"]
pub type TZEN15_R = crate::BitReader<bool>;
#[doc = "Field `TZEN15` writer - TZEN15"]
pub type TZEN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN17` reader - TZEN17"]
pub type TZEN17_R = crate::BitReader<bool>;
#[doc = "Field `TZEN17` writer - TZEN17"]
pub type TZEN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN18` reader - TZEN18"]
pub type TZEN18_R = crate::BitReader<bool>;
#[doc = "Field `TZEN18` writer - TZEN18"]
pub type TZEN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN19` reader - TZEN19"]
pub type TZEN19_R = crate::BitReader<bool>;
#[doc = "Field `TZEN19` writer - TZEN19"]
pub type TZEN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN24` reader - TZEN24"]
pub type TZEN24_R = crate::BitReader<bool>;
#[doc = "Field `TZEN24` writer - TZEN24"]
pub type TZEN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
#[doc = "Field `TZEN26` reader - TZEN26"]
pub type TZEN26_R = crate::BitReader<bool>;
#[doc = "Field `TZEN26` writer - TZEN26"]
pub type TZEN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_TZENR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&self) -> TZEN0_R {
        TZEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&self) -> TZEN1_R {
        TZEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&self) -> TZEN2_R {
        TZEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&self) -> TZEN3_R {
        TZEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&self) -> TZEN4_R {
        TZEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&self) -> TZEN5_R {
        TZEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&self) -> TZEN6_R {
        TZEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&self) -> TZEN7_R {
        TZEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&self) -> TZEN8_R {
        TZEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&self) -> TZEN9_R {
        TZEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&self) -> TZEN10_R {
        TZEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&self) -> TZEN11_R {
        TZEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&self) -> TZEN12_R {
        TZEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&self) -> TZEN13_R {
        TZEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&self) -> TZEN14_R {
        TZEN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&self) -> TZEN15_R {
        TZEN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&self) -> TZEN17_R {
        TZEN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&self) -> TZEN18_R {
        TZEN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&self) -> TZEN19_R {
        TZEN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&self) -> TZEN24_R {
        TZEN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&self) -> TZEN26_R {
        TZEN26_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&mut self) -> TZEN0_W<0> {
        TZEN0_W::new(self)
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&mut self) -> TZEN1_W<1> {
        TZEN1_W::new(self)
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&mut self) -> TZEN2_W<2> {
        TZEN2_W::new(self)
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&mut self) -> TZEN3_W<3> {
        TZEN3_W::new(self)
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&mut self) -> TZEN4_W<4> {
        TZEN4_W::new(self)
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&mut self) -> TZEN5_W<5> {
        TZEN5_W::new(self)
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&mut self) -> TZEN6_W<6> {
        TZEN6_W::new(self)
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&mut self) -> TZEN7_W<7> {
        TZEN7_W::new(self)
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&mut self) -> TZEN8_W<8> {
        TZEN8_W::new(self)
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&mut self) -> TZEN9_W<9> {
        TZEN9_W::new(self)
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&mut self) -> TZEN10_W<10> {
        TZEN10_W::new(self)
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&mut self) -> TZEN11_W<11> {
        TZEN11_W::new(self)
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&mut self) -> TZEN12_W<12> {
        TZEN12_W::new(self)
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&mut self) -> TZEN13_W<13> {
        TZEN13_W::new(self)
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&mut self) -> TZEN14_W<14> {
        TZEN14_W::new(self)
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&mut self) -> TZEN15_W<15> {
        TZEN15_W::new(self)
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&mut self) -> TZEN17_W<17> {
        TZEN17_W::new(self)
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&mut self) -> TZEN18_W<18> {
        TZEN18_W::new(self)
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&mut self) -> TZEN19_W<19> {
        TZEN19_W::new(self)
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&mut self) -> TZEN24_W<24> {
        TZEN24_W::new(self)
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&mut self) -> TZEN26_W<26> {
        TZEN26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr1](index.html) module"]
pub struct EXTI_TZENR1_SPEC;
impl crate::RegisterSpec for EXTI_TZENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_tzenr1::R](R) reader structure"]
impl crate::Readable for EXTI_TZENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_tzenr1::W](W) writer structure"]
impl crate::Writable for EXTI_TZENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_TZENR1 to value 0"]
impl crate::Resettable for EXTI_TZENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
