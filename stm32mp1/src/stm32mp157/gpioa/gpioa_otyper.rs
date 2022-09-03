#[doc = "Register `GPIOA_OTYPER` reader"]
pub struct R(crate::R<GPIOA_OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOA_OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOA_OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOA_OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOA_OTYPER` writer"]
pub struct W(crate::W<GPIOA_OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOA_OTYPER_SPEC>;
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
impl From<crate::W<GPIOA_OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOA_OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OT0` reader - OT0"]
pub type OT0_R = crate::BitReader<bool>;
#[doc = "Field `OT0` writer - OT0"]
pub type OT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT1` reader - OT1"]
pub type OT1_R = crate::BitReader<bool>;
#[doc = "Field `OT1` writer - OT1"]
pub type OT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT2` reader - OT2"]
pub type OT2_R = crate::BitReader<bool>;
#[doc = "Field `OT2` writer - OT2"]
pub type OT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT3` reader - OT3"]
pub type OT3_R = crate::BitReader<bool>;
#[doc = "Field `OT3` writer - OT3"]
pub type OT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT4` reader - OT4"]
pub type OT4_R = crate::BitReader<bool>;
#[doc = "Field `OT4` writer - OT4"]
pub type OT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT5` reader - OT5"]
pub type OT5_R = crate::BitReader<bool>;
#[doc = "Field `OT5` writer - OT5"]
pub type OT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT6` reader - OT6"]
pub type OT6_R = crate::BitReader<bool>;
#[doc = "Field `OT6` writer - OT6"]
pub type OT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT7` reader - OT7"]
pub type OT7_R = crate::BitReader<bool>;
#[doc = "Field `OT7` writer - OT7"]
pub type OT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT8` reader - OT8"]
pub type OT8_R = crate::BitReader<bool>;
#[doc = "Field `OT8` writer - OT8"]
pub type OT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT9` reader - OT9"]
pub type OT9_R = crate::BitReader<bool>;
#[doc = "Field `OT9` writer - OT9"]
pub type OT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT10` reader - OT10"]
pub type OT10_R = crate::BitReader<bool>;
#[doc = "Field `OT10` writer - OT10"]
pub type OT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT11` reader - OT11"]
pub type OT11_R = crate::BitReader<bool>;
#[doc = "Field `OT11` writer - OT11"]
pub type OT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT12` reader - OT12"]
pub type OT12_R = crate::BitReader<bool>;
#[doc = "Field `OT12` writer - OT12"]
pub type OT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT13` reader - OT13"]
pub type OT13_R = crate::BitReader<bool>;
#[doc = "Field `OT13` writer - OT13"]
pub type OT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT14` reader - OT14"]
pub type OT14_R = crate::BitReader<bool>;
#[doc = "Field `OT14` writer - OT14"]
pub type OT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
#[doc = "Field `OT15` reader - OT15"]
pub type OT15_R = crate::BitReader<bool>;
#[doc = "Field `OT15` writer - OT15"]
pub type OT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOA_OTYPER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OT0"]
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OT1"]
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OT2"]
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OT3"]
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OT4"]
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OT5"]
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OT6"]
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OT7"]
    #[inline(always)]
    pub fn ot7(&self) -> OT7_R {
        OT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OT8"]
    #[inline(always)]
    pub fn ot8(&self) -> OT8_R {
        OT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OT9"]
    #[inline(always)]
    pub fn ot9(&self) -> OT9_R {
        OT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OT10"]
    #[inline(always)]
    pub fn ot10(&self) -> OT10_R {
        OT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OT11"]
    #[inline(always)]
    pub fn ot11(&self) -> OT11_R {
        OT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OT12"]
    #[inline(always)]
    pub fn ot12(&self) -> OT12_R {
        OT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OT13"]
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OT14"]
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OT15"]
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OT0"]
    #[inline(always)]
    pub fn ot0(&mut self) -> OT0_W<0> {
        OT0_W::new(self)
    }
    #[doc = "Bit 1 - OT1"]
    #[inline(always)]
    pub fn ot1(&mut self) -> OT1_W<1> {
        OT1_W::new(self)
    }
    #[doc = "Bit 2 - OT2"]
    #[inline(always)]
    pub fn ot2(&mut self) -> OT2_W<2> {
        OT2_W::new(self)
    }
    #[doc = "Bit 3 - OT3"]
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W<3> {
        OT3_W::new(self)
    }
    #[doc = "Bit 4 - OT4"]
    #[inline(always)]
    pub fn ot4(&mut self) -> OT4_W<4> {
        OT4_W::new(self)
    }
    #[doc = "Bit 5 - OT5"]
    #[inline(always)]
    pub fn ot5(&mut self) -> OT5_W<5> {
        OT5_W::new(self)
    }
    #[doc = "Bit 6 - OT6"]
    #[inline(always)]
    pub fn ot6(&mut self) -> OT6_W<6> {
        OT6_W::new(self)
    }
    #[doc = "Bit 7 - OT7"]
    #[inline(always)]
    pub fn ot7(&mut self) -> OT7_W<7> {
        OT7_W::new(self)
    }
    #[doc = "Bit 8 - OT8"]
    #[inline(always)]
    pub fn ot8(&mut self) -> OT8_W<8> {
        OT8_W::new(self)
    }
    #[doc = "Bit 9 - OT9"]
    #[inline(always)]
    pub fn ot9(&mut self) -> OT9_W<9> {
        OT9_W::new(self)
    }
    #[doc = "Bit 10 - OT10"]
    #[inline(always)]
    pub fn ot10(&mut self) -> OT10_W<10> {
        OT10_W::new(self)
    }
    #[doc = "Bit 11 - OT11"]
    #[inline(always)]
    pub fn ot11(&mut self) -> OT11_W<11> {
        OT11_W::new(self)
    }
    #[doc = "Bit 12 - OT12"]
    #[inline(always)]
    pub fn ot12(&mut self) -> OT12_W<12> {
        OT12_W::new(self)
    }
    #[doc = "Bit 13 - OT13"]
    #[inline(always)]
    pub fn ot13(&mut self) -> OT13_W<13> {
        OT13_W::new(self)
    }
    #[doc = "Bit 14 - OT14"]
    #[inline(always)]
    pub fn ot14(&mut self) -> OT14_W<14> {
        OT14_W::new(self)
    }
    #[doc = "Bit 15 - OT15"]
    #[inline(always)]
    pub fn ot15(&mut self) -> OT15_W<15> {
        OT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_otyper](index.html) module"]
pub struct GPIOA_OTYPER_SPEC;
impl crate::RegisterSpec for GPIOA_OTYPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioa_otyper::R](R) reader structure"]
impl crate::Readable for GPIOA_OTYPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioa_otyper::W](W) writer structure"]
impl crate::Writable for GPIOA_OTYPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOA_OTYPER to value 0"]
impl crate::Resettable for GPIOA_OTYPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
