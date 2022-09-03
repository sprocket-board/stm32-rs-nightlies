#[doc = "Register `MPCBB2_LCKVTR1` reader"]
pub struct R(crate::R<MPCBB2_LCKVTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_LCKVTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_LCKVTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_LCKVTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_LCKVTR1` writer"]
pub struct W(crate::W<MPCBB2_LCKVTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_LCKVTR1_SPEC>;
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
impl From<crate::W<MPCBB2_LCKVTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_LCKVTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCKSB0` reader - LCKSB0"]
pub type LCKSB0_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB0` writer - LCKSB0"]
pub type LCKSB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB1` reader - LCKSB1"]
pub type LCKSB1_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB1` writer - LCKSB1"]
pub type LCKSB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB2` reader - LCKSB2"]
pub type LCKSB2_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB2` writer - LCKSB2"]
pub type LCKSB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB3` reader - LCKSB3"]
pub type LCKSB3_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB3` writer - LCKSB3"]
pub type LCKSB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB4` reader - LCKSB4"]
pub type LCKSB4_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB4` writer - LCKSB4"]
pub type LCKSB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB5` reader - LCKSB5"]
pub type LCKSB5_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB5` writer - LCKSB5"]
pub type LCKSB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB6` reader - LCKSB6"]
pub type LCKSB6_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB6` writer - LCKSB6"]
pub type LCKSB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB7` reader - LCKSB7"]
pub type LCKSB7_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB7` writer - LCKSB7"]
pub type LCKSB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB8` reader - LCKSB8"]
pub type LCKSB8_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB8` writer - LCKSB8"]
pub type LCKSB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB9` reader - LCKSB9"]
pub type LCKSB9_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB9` writer - LCKSB9"]
pub type LCKSB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB10` reader - LCKSB10"]
pub type LCKSB10_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB10` writer - LCKSB10"]
pub type LCKSB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB11` reader - LCKSB11"]
pub type LCKSB11_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB11` writer - LCKSB11"]
pub type LCKSB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB12` reader - LCKSB12"]
pub type LCKSB12_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB12` writer - LCKSB12"]
pub type LCKSB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB13` reader - LCKSB13"]
pub type LCKSB13_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB13` writer - LCKSB13"]
pub type LCKSB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB14` reader - LCKSB14"]
pub type LCKSB14_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB14` writer - LCKSB14"]
pub type LCKSB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB15` reader - LCKSB15"]
pub type LCKSB15_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB15` writer - LCKSB15"]
pub type LCKSB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB16` reader - LCKSB16"]
pub type LCKSB16_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB16` writer - LCKSB16"]
pub type LCKSB16_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB17` reader - LCKSB17"]
pub type LCKSB17_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB17` writer - LCKSB17"]
pub type LCKSB17_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB18` reader - LCKSB18"]
pub type LCKSB18_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB18` writer - LCKSB18"]
pub type LCKSB18_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB19` reader - LCKSB19"]
pub type LCKSB19_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB19` writer - LCKSB19"]
pub type LCKSB19_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB20` reader - LCKSB20"]
pub type LCKSB20_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB20` writer - LCKSB20"]
pub type LCKSB20_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB21` reader - LCKSB21"]
pub type LCKSB21_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB21` writer - LCKSB21"]
pub type LCKSB21_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB22` reader - LCKSB22"]
pub type LCKSB22_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB22` writer - LCKSB22"]
pub type LCKSB22_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB23` reader - LCKSB23"]
pub type LCKSB23_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB23` writer - LCKSB23"]
pub type LCKSB23_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB24` reader - LCKSB24"]
pub type LCKSB24_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB24` writer - LCKSB24"]
pub type LCKSB24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB25` reader - LCKSB25"]
pub type LCKSB25_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB25` writer - LCKSB25"]
pub type LCKSB25_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB26` reader - LCKSB26"]
pub type LCKSB26_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB26` writer - LCKSB26"]
pub type LCKSB26_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB27` reader - LCKSB27"]
pub type LCKSB27_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB27` writer - LCKSB27"]
pub type LCKSB27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB28` reader - LCKSB28"]
pub type LCKSB28_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB28` writer - LCKSB28"]
pub type LCKSB28_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB29` reader - LCKSB29"]
pub type LCKSB29_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB29` writer - LCKSB29"]
pub type LCKSB29_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB30` reader - LCKSB30"]
pub type LCKSB30_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB30` writer - LCKSB30"]
pub type LCKSB30_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
#[doc = "Field `LCKSB31` reader - LCKSB31"]
pub type LCKSB31_R = crate::BitReader<bool>;
#[doc = "Field `LCKSB31` writer - LCKSB31"]
pub type LCKSB31_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPCBB2_LCKVTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LCKSB0"]
    #[inline(always)]
    pub fn lcksb0(&self) -> LCKSB0_R {
        LCKSB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCKSB1"]
    #[inline(always)]
    pub fn lcksb1(&self) -> LCKSB1_R {
        LCKSB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCKSB2"]
    #[inline(always)]
    pub fn lcksb2(&self) -> LCKSB2_R {
        LCKSB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCKSB3"]
    #[inline(always)]
    pub fn lcksb3(&self) -> LCKSB3_R {
        LCKSB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCKSB4"]
    #[inline(always)]
    pub fn lcksb4(&self) -> LCKSB4_R {
        LCKSB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCKSB5"]
    #[inline(always)]
    pub fn lcksb5(&self) -> LCKSB5_R {
        LCKSB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCKSB6"]
    #[inline(always)]
    pub fn lcksb6(&self) -> LCKSB6_R {
        LCKSB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCKSB7"]
    #[inline(always)]
    pub fn lcksb7(&self) -> LCKSB7_R {
        LCKSB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCKSB8"]
    #[inline(always)]
    pub fn lcksb8(&self) -> LCKSB8_R {
        LCKSB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCKSB9"]
    #[inline(always)]
    pub fn lcksb9(&self) -> LCKSB9_R {
        LCKSB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCKSB10"]
    #[inline(always)]
    pub fn lcksb10(&self) -> LCKSB10_R {
        LCKSB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCKSB11"]
    #[inline(always)]
    pub fn lcksb11(&self) -> LCKSB11_R {
        LCKSB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCKSB12"]
    #[inline(always)]
    pub fn lcksb12(&self) -> LCKSB12_R {
        LCKSB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCKSB13"]
    #[inline(always)]
    pub fn lcksb13(&self) -> LCKSB13_R {
        LCKSB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCKSB14"]
    #[inline(always)]
    pub fn lcksb14(&self) -> LCKSB14_R {
        LCKSB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCKSB15"]
    #[inline(always)]
    pub fn lcksb15(&self) -> LCKSB15_R {
        LCKSB15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LCKSB16"]
    #[inline(always)]
    pub fn lcksb16(&self) -> LCKSB16_R {
        LCKSB16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LCKSB17"]
    #[inline(always)]
    pub fn lcksb17(&self) -> LCKSB17_R {
        LCKSB17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LCKSB18"]
    #[inline(always)]
    pub fn lcksb18(&self) -> LCKSB18_R {
        LCKSB18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LCKSB19"]
    #[inline(always)]
    pub fn lcksb19(&self) -> LCKSB19_R {
        LCKSB19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LCKSB20"]
    #[inline(always)]
    pub fn lcksb20(&self) -> LCKSB20_R {
        LCKSB20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LCKSB21"]
    #[inline(always)]
    pub fn lcksb21(&self) -> LCKSB21_R {
        LCKSB21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LCKSB22"]
    #[inline(always)]
    pub fn lcksb22(&self) -> LCKSB22_R {
        LCKSB22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LCKSB23"]
    #[inline(always)]
    pub fn lcksb23(&self) -> LCKSB23_R {
        LCKSB23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LCKSB24"]
    #[inline(always)]
    pub fn lcksb24(&self) -> LCKSB24_R {
        LCKSB24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LCKSB25"]
    #[inline(always)]
    pub fn lcksb25(&self) -> LCKSB25_R {
        LCKSB25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LCKSB26"]
    #[inline(always)]
    pub fn lcksb26(&self) -> LCKSB26_R {
        LCKSB26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCKSB27"]
    #[inline(always)]
    pub fn lcksb27(&self) -> LCKSB27_R {
        LCKSB27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LCKSB28"]
    #[inline(always)]
    pub fn lcksb28(&self) -> LCKSB28_R {
        LCKSB28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LCKSB29"]
    #[inline(always)]
    pub fn lcksb29(&self) -> LCKSB29_R {
        LCKSB29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LCKSB30"]
    #[inline(always)]
    pub fn lcksb30(&self) -> LCKSB30_R {
        LCKSB30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LCKSB31"]
    #[inline(always)]
    pub fn lcksb31(&self) -> LCKSB31_R {
        LCKSB31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCKSB0"]
    #[inline(always)]
    pub fn lcksb0(&mut self) -> LCKSB0_W<0> {
        LCKSB0_W::new(self)
    }
    #[doc = "Bit 1 - LCKSB1"]
    #[inline(always)]
    pub fn lcksb1(&mut self) -> LCKSB1_W<1> {
        LCKSB1_W::new(self)
    }
    #[doc = "Bit 2 - LCKSB2"]
    #[inline(always)]
    pub fn lcksb2(&mut self) -> LCKSB2_W<2> {
        LCKSB2_W::new(self)
    }
    #[doc = "Bit 3 - LCKSB3"]
    #[inline(always)]
    pub fn lcksb3(&mut self) -> LCKSB3_W<3> {
        LCKSB3_W::new(self)
    }
    #[doc = "Bit 4 - LCKSB4"]
    #[inline(always)]
    pub fn lcksb4(&mut self) -> LCKSB4_W<4> {
        LCKSB4_W::new(self)
    }
    #[doc = "Bit 5 - LCKSB5"]
    #[inline(always)]
    pub fn lcksb5(&mut self) -> LCKSB5_W<5> {
        LCKSB5_W::new(self)
    }
    #[doc = "Bit 6 - LCKSB6"]
    #[inline(always)]
    pub fn lcksb6(&mut self) -> LCKSB6_W<6> {
        LCKSB6_W::new(self)
    }
    #[doc = "Bit 7 - LCKSB7"]
    #[inline(always)]
    pub fn lcksb7(&mut self) -> LCKSB7_W<7> {
        LCKSB7_W::new(self)
    }
    #[doc = "Bit 8 - LCKSB8"]
    #[inline(always)]
    pub fn lcksb8(&mut self) -> LCKSB8_W<8> {
        LCKSB8_W::new(self)
    }
    #[doc = "Bit 9 - LCKSB9"]
    #[inline(always)]
    pub fn lcksb9(&mut self) -> LCKSB9_W<9> {
        LCKSB9_W::new(self)
    }
    #[doc = "Bit 10 - LCKSB10"]
    #[inline(always)]
    pub fn lcksb10(&mut self) -> LCKSB10_W<10> {
        LCKSB10_W::new(self)
    }
    #[doc = "Bit 11 - LCKSB11"]
    #[inline(always)]
    pub fn lcksb11(&mut self) -> LCKSB11_W<11> {
        LCKSB11_W::new(self)
    }
    #[doc = "Bit 12 - LCKSB12"]
    #[inline(always)]
    pub fn lcksb12(&mut self) -> LCKSB12_W<12> {
        LCKSB12_W::new(self)
    }
    #[doc = "Bit 13 - LCKSB13"]
    #[inline(always)]
    pub fn lcksb13(&mut self) -> LCKSB13_W<13> {
        LCKSB13_W::new(self)
    }
    #[doc = "Bit 14 - LCKSB14"]
    #[inline(always)]
    pub fn lcksb14(&mut self) -> LCKSB14_W<14> {
        LCKSB14_W::new(self)
    }
    #[doc = "Bit 15 - LCKSB15"]
    #[inline(always)]
    pub fn lcksb15(&mut self) -> LCKSB15_W<15> {
        LCKSB15_W::new(self)
    }
    #[doc = "Bit 16 - LCKSB16"]
    #[inline(always)]
    pub fn lcksb16(&mut self) -> LCKSB16_W<16> {
        LCKSB16_W::new(self)
    }
    #[doc = "Bit 17 - LCKSB17"]
    #[inline(always)]
    pub fn lcksb17(&mut self) -> LCKSB17_W<17> {
        LCKSB17_W::new(self)
    }
    #[doc = "Bit 18 - LCKSB18"]
    #[inline(always)]
    pub fn lcksb18(&mut self) -> LCKSB18_W<18> {
        LCKSB18_W::new(self)
    }
    #[doc = "Bit 19 - LCKSB19"]
    #[inline(always)]
    pub fn lcksb19(&mut self) -> LCKSB19_W<19> {
        LCKSB19_W::new(self)
    }
    #[doc = "Bit 20 - LCKSB20"]
    #[inline(always)]
    pub fn lcksb20(&mut self) -> LCKSB20_W<20> {
        LCKSB20_W::new(self)
    }
    #[doc = "Bit 21 - LCKSB21"]
    #[inline(always)]
    pub fn lcksb21(&mut self) -> LCKSB21_W<21> {
        LCKSB21_W::new(self)
    }
    #[doc = "Bit 22 - LCKSB22"]
    #[inline(always)]
    pub fn lcksb22(&mut self) -> LCKSB22_W<22> {
        LCKSB22_W::new(self)
    }
    #[doc = "Bit 23 - LCKSB23"]
    #[inline(always)]
    pub fn lcksb23(&mut self) -> LCKSB23_W<23> {
        LCKSB23_W::new(self)
    }
    #[doc = "Bit 24 - LCKSB24"]
    #[inline(always)]
    pub fn lcksb24(&mut self) -> LCKSB24_W<24> {
        LCKSB24_W::new(self)
    }
    #[doc = "Bit 25 - LCKSB25"]
    #[inline(always)]
    pub fn lcksb25(&mut self) -> LCKSB25_W<25> {
        LCKSB25_W::new(self)
    }
    #[doc = "Bit 26 - LCKSB26"]
    #[inline(always)]
    pub fn lcksb26(&mut self) -> LCKSB26_W<26> {
        LCKSB26_W::new(self)
    }
    #[doc = "Bit 27 - LCKSB27"]
    #[inline(always)]
    pub fn lcksb27(&mut self) -> LCKSB27_W<27> {
        LCKSB27_W::new(self)
    }
    #[doc = "Bit 28 - LCKSB28"]
    #[inline(always)]
    pub fn lcksb28(&mut self) -> LCKSB28_W<28> {
        LCKSB28_W::new(self)
    }
    #[doc = "Bit 29 - LCKSB29"]
    #[inline(always)]
    pub fn lcksb29(&mut self) -> LCKSB29_W<29> {
        LCKSB29_W::new(self)
    }
    #[doc = "Bit 30 - LCKSB30"]
    #[inline(always)]
    pub fn lcksb30(&mut self) -> LCKSB30_W<30> {
        LCKSB30_W::new(self)
    }
    #[doc = "Bit 31 - LCKSB31"]
    #[inline(always)]
    pub fn lcksb31(&mut self) -> LCKSB31_W<31> {
        LCKSB31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_lckvtr1](index.html) module"]
pub struct MPCBB2_LCKVTR1_SPEC;
impl crate::RegisterSpec for MPCBB2_LCKVTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_lckvtr1::R](R) reader structure"]
impl crate::Readable for MPCBB2_LCKVTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_lckvtr1::W](W) writer structure"]
impl crate::Writable for MPCBB2_LCKVTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_LCKVTR1 to value 0"]
impl crate::Resettable for MPCBB2_LCKVTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
