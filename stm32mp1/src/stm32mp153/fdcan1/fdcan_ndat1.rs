#[doc = "Register `FDCAN_NDAT1` reader"]
pub struct R(crate::R<FDCAN_NDAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_NDAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_NDAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_NDAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_NDAT1` writer"]
pub struct W(crate::W<FDCAN_NDAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_NDAT1_SPEC>;
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
impl From<crate::W<FDCAN_NDAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_NDAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND0` reader - ND0"]
pub type ND0_R = crate::BitReader<bool>;
#[doc = "Field `ND0` writer - ND0"]
pub type ND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND1` reader - ND1"]
pub type ND1_R = crate::BitReader<bool>;
#[doc = "Field `ND1` writer - ND1"]
pub type ND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND2` reader - ND2"]
pub type ND2_R = crate::BitReader<bool>;
#[doc = "Field `ND2` writer - ND2"]
pub type ND2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND3` reader - ND3"]
pub type ND3_R = crate::BitReader<bool>;
#[doc = "Field `ND3` writer - ND3"]
pub type ND3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND4` reader - ND4"]
pub type ND4_R = crate::BitReader<bool>;
#[doc = "Field `ND4` writer - ND4"]
pub type ND4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND5` reader - ND5"]
pub type ND5_R = crate::BitReader<bool>;
#[doc = "Field `ND5` writer - ND5"]
pub type ND5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND6` reader - ND6"]
pub type ND6_R = crate::BitReader<bool>;
#[doc = "Field `ND6` writer - ND6"]
pub type ND6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND7` reader - ND7"]
pub type ND7_R = crate::BitReader<bool>;
#[doc = "Field `ND7` writer - ND7"]
pub type ND7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND8` reader - ND8"]
pub type ND8_R = crate::BitReader<bool>;
#[doc = "Field `ND8` writer - ND8"]
pub type ND8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND9` reader - ND9"]
pub type ND9_R = crate::BitReader<bool>;
#[doc = "Field `ND9` writer - ND9"]
pub type ND9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND10` reader - ND10"]
pub type ND10_R = crate::BitReader<bool>;
#[doc = "Field `ND10` writer - ND10"]
pub type ND10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND11` reader - ND11"]
pub type ND11_R = crate::BitReader<bool>;
#[doc = "Field `ND11` writer - ND11"]
pub type ND11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND12` reader - ND12"]
pub type ND12_R = crate::BitReader<bool>;
#[doc = "Field `ND12` writer - ND12"]
pub type ND12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND13` reader - ND13"]
pub type ND13_R = crate::BitReader<bool>;
#[doc = "Field `ND13` writer - ND13"]
pub type ND13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND14` reader - ND14"]
pub type ND14_R = crate::BitReader<bool>;
#[doc = "Field `ND14` writer - ND14"]
pub type ND14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND15` reader - ND15"]
pub type ND15_R = crate::BitReader<bool>;
#[doc = "Field `ND15` writer - ND15"]
pub type ND15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND16` reader - ND16"]
pub type ND16_R = crate::BitReader<bool>;
#[doc = "Field `ND16` writer - ND16"]
pub type ND16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND17` reader - ND17"]
pub type ND17_R = crate::BitReader<bool>;
#[doc = "Field `ND17` writer - ND17"]
pub type ND17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND18` reader - ND18"]
pub type ND18_R = crate::BitReader<bool>;
#[doc = "Field `ND18` writer - ND18"]
pub type ND18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND19` reader - ND19"]
pub type ND19_R = crate::BitReader<bool>;
#[doc = "Field `ND19` writer - ND19"]
pub type ND19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND20` reader - ND20"]
pub type ND20_R = crate::BitReader<bool>;
#[doc = "Field `ND20` writer - ND20"]
pub type ND20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND21` reader - ND21"]
pub type ND21_R = crate::BitReader<bool>;
#[doc = "Field `ND21` writer - ND21"]
pub type ND21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND22` reader - ND22"]
pub type ND22_R = crate::BitReader<bool>;
#[doc = "Field `ND22` writer - ND22"]
pub type ND22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND23` reader - ND23"]
pub type ND23_R = crate::BitReader<bool>;
#[doc = "Field `ND23` writer - ND23"]
pub type ND23_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND24` reader - ND24"]
pub type ND24_R = crate::BitReader<bool>;
#[doc = "Field `ND24` writer - ND24"]
pub type ND24_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND25` reader - ND25"]
pub type ND25_R = crate::BitReader<bool>;
#[doc = "Field `ND25` writer - ND25"]
pub type ND25_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND26` reader - ND26"]
pub type ND26_R = crate::BitReader<bool>;
#[doc = "Field `ND26` writer - ND26"]
pub type ND26_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND27` reader - ND27"]
pub type ND27_R = crate::BitReader<bool>;
#[doc = "Field `ND27` writer - ND27"]
pub type ND27_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND28` reader - ND28"]
pub type ND28_R = crate::BitReader<bool>;
#[doc = "Field `ND28` writer - ND28"]
pub type ND28_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND29` reader - ND29"]
pub type ND29_R = crate::BitReader<bool>;
#[doc = "Field `ND29` writer - ND29"]
pub type ND29_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND30` reader - ND30"]
pub type ND30_R = crate::BitReader<bool>;
#[doc = "Field `ND30` writer - ND30"]
pub type ND30_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
#[doc = "Field `ND31` reader - ND31"]
pub type ND31_R = crate::BitReader<bool>;
#[doc = "Field `ND31` writer - ND31"]
pub type ND31_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_NDAT1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ND0"]
    #[inline(always)]
    pub fn nd0(&self) -> ND0_R {
        ND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ND1"]
    #[inline(always)]
    pub fn nd1(&self) -> ND1_R {
        ND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ND2"]
    #[inline(always)]
    pub fn nd2(&self) -> ND2_R {
        ND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ND3"]
    #[inline(always)]
    pub fn nd3(&self) -> ND3_R {
        ND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ND4"]
    #[inline(always)]
    pub fn nd4(&self) -> ND4_R {
        ND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ND5"]
    #[inline(always)]
    pub fn nd5(&self) -> ND5_R {
        ND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ND6"]
    #[inline(always)]
    pub fn nd6(&self) -> ND6_R {
        ND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ND7"]
    #[inline(always)]
    pub fn nd7(&self) -> ND7_R {
        ND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ND8"]
    #[inline(always)]
    pub fn nd8(&self) -> ND8_R {
        ND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ND9"]
    #[inline(always)]
    pub fn nd9(&self) -> ND9_R {
        ND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ND10"]
    #[inline(always)]
    pub fn nd10(&self) -> ND10_R {
        ND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ND11"]
    #[inline(always)]
    pub fn nd11(&self) -> ND11_R {
        ND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ND12"]
    #[inline(always)]
    pub fn nd12(&self) -> ND12_R {
        ND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ND13"]
    #[inline(always)]
    pub fn nd13(&self) -> ND13_R {
        ND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ND14"]
    #[inline(always)]
    pub fn nd14(&self) -> ND14_R {
        ND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ND15"]
    #[inline(always)]
    pub fn nd15(&self) -> ND15_R {
        ND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ND16"]
    #[inline(always)]
    pub fn nd16(&self) -> ND16_R {
        ND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ND17"]
    #[inline(always)]
    pub fn nd17(&self) -> ND17_R {
        ND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ND18"]
    #[inline(always)]
    pub fn nd18(&self) -> ND18_R {
        ND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ND19"]
    #[inline(always)]
    pub fn nd19(&self) -> ND19_R {
        ND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ND20"]
    #[inline(always)]
    pub fn nd20(&self) -> ND20_R {
        ND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ND21"]
    #[inline(always)]
    pub fn nd21(&self) -> ND21_R {
        ND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ND22"]
    #[inline(always)]
    pub fn nd22(&self) -> ND22_R {
        ND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ND23"]
    #[inline(always)]
    pub fn nd23(&self) -> ND23_R {
        ND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ND24"]
    #[inline(always)]
    pub fn nd24(&self) -> ND24_R {
        ND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ND25"]
    #[inline(always)]
    pub fn nd25(&self) -> ND25_R {
        ND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ND26"]
    #[inline(always)]
    pub fn nd26(&self) -> ND26_R {
        ND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ND27"]
    #[inline(always)]
    pub fn nd27(&self) -> ND27_R {
        ND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ND28"]
    #[inline(always)]
    pub fn nd28(&self) -> ND28_R {
        ND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ND29"]
    #[inline(always)]
    pub fn nd29(&self) -> ND29_R {
        ND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ND30"]
    #[inline(always)]
    pub fn nd30(&self) -> ND30_R {
        ND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ND31"]
    #[inline(always)]
    pub fn nd31(&self) -> ND31_R {
        ND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ND0"]
    #[inline(always)]
    pub fn nd0(&mut self) -> ND0_W<0> {
        ND0_W::new(self)
    }
    #[doc = "Bit 1 - ND1"]
    #[inline(always)]
    pub fn nd1(&mut self) -> ND1_W<1> {
        ND1_W::new(self)
    }
    #[doc = "Bit 2 - ND2"]
    #[inline(always)]
    pub fn nd2(&mut self) -> ND2_W<2> {
        ND2_W::new(self)
    }
    #[doc = "Bit 3 - ND3"]
    #[inline(always)]
    pub fn nd3(&mut self) -> ND3_W<3> {
        ND3_W::new(self)
    }
    #[doc = "Bit 4 - ND4"]
    #[inline(always)]
    pub fn nd4(&mut self) -> ND4_W<4> {
        ND4_W::new(self)
    }
    #[doc = "Bit 5 - ND5"]
    #[inline(always)]
    pub fn nd5(&mut self) -> ND5_W<5> {
        ND5_W::new(self)
    }
    #[doc = "Bit 6 - ND6"]
    #[inline(always)]
    pub fn nd6(&mut self) -> ND6_W<6> {
        ND6_W::new(self)
    }
    #[doc = "Bit 7 - ND7"]
    #[inline(always)]
    pub fn nd7(&mut self) -> ND7_W<7> {
        ND7_W::new(self)
    }
    #[doc = "Bit 8 - ND8"]
    #[inline(always)]
    pub fn nd8(&mut self) -> ND8_W<8> {
        ND8_W::new(self)
    }
    #[doc = "Bit 9 - ND9"]
    #[inline(always)]
    pub fn nd9(&mut self) -> ND9_W<9> {
        ND9_W::new(self)
    }
    #[doc = "Bit 10 - ND10"]
    #[inline(always)]
    pub fn nd10(&mut self) -> ND10_W<10> {
        ND10_W::new(self)
    }
    #[doc = "Bit 11 - ND11"]
    #[inline(always)]
    pub fn nd11(&mut self) -> ND11_W<11> {
        ND11_W::new(self)
    }
    #[doc = "Bit 12 - ND12"]
    #[inline(always)]
    pub fn nd12(&mut self) -> ND12_W<12> {
        ND12_W::new(self)
    }
    #[doc = "Bit 13 - ND13"]
    #[inline(always)]
    pub fn nd13(&mut self) -> ND13_W<13> {
        ND13_W::new(self)
    }
    #[doc = "Bit 14 - ND14"]
    #[inline(always)]
    pub fn nd14(&mut self) -> ND14_W<14> {
        ND14_W::new(self)
    }
    #[doc = "Bit 15 - ND15"]
    #[inline(always)]
    pub fn nd15(&mut self) -> ND15_W<15> {
        ND15_W::new(self)
    }
    #[doc = "Bit 16 - ND16"]
    #[inline(always)]
    pub fn nd16(&mut self) -> ND16_W<16> {
        ND16_W::new(self)
    }
    #[doc = "Bit 17 - ND17"]
    #[inline(always)]
    pub fn nd17(&mut self) -> ND17_W<17> {
        ND17_W::new(self)
    }
    #[doc = "Bit 18 - ND18"]
    #[inline(always)]
    pub fn nd18(&mut self) -> ND18_W<18> {
        ND18_W::new(self)
    }
    #[doc = "Bit 19 - ND19"]
    #[inline(always)]
    pub fn nd19(&mut self) -> ND19_W<19> {
        ND19_W::new(self)
    }
    #[doc = "Bit 20 - ND20"]
    #[inline(always)]
    pub fn nd20(&mut self) -> ND20_W<20> {
        ND20_W::new(self)
    }
    #[doc = "Bit 21 - ND21"]
    #[inline(always)]
    pub fn nd21(&mut self) -> ND21_W<21> {
        ND21_W::new(self)
    }
    #[doc = "Bit 22 - ND22"]
    #[inline(always)]
    pub fn nd22(&mut self) -> ND22_W<22> {
        ND22_W::new(self)
    }
    #[doc = "Bit 23 - ND23"]
    #[inline(always)]
    pub fn nd23(&mut self) -> ND23_W<23> {
        ND23_W::new(self)
    }
    #[doc = "Bit 24 - ND24"]
    #[inline(always)]
    pub fn nd24(&mut self) -> ND24_W<24> {
        ND24_W::new(self)
    }
    #[doc = "Bit 25 - ND25"]
    #[inline(always)]
    pub fn nd25(&mut self) -> ND25_W<25> {
        ND25_W::new(self)
    }
    #[doc = "Bit 26 - ND26"]
    #[inline(always)]
    pub fn nd26(&mut self) -> ND26_W<26> {
        ND26_W::new(self)
    }
    #[doc = "Bit 27 - ND27"]
    #[inline(always)]
    pub fn nd27(&mut self) -> ND27_W<27> {
        ND27_W::new(self)
    }
    #[doc = "Bit 28 - ND28"]
    #[inline(always)]
    pub fn nd28(&mut self) -> ND28_W<28> {
        ND28_W::new(self)
    }
    #[doc = "Bit 29 - ND29"]
    #[inline(always)]
    pub fn nd29(&mut self) -> ND29_W<29> {
        ND29_W::new(self)
    }
    #[doc = "Bit 30 - ND30"]
    #[inline(always)]
    pub fn nd30(&mut self) -> ND30_W<30> {
        ND30_W::new(self)
    }
    #[doc = "Bit 31 - ND31"]
    #[inline(always)]
    pub fn nd31(&mut self) -> ND31_W<31> {
        ND31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN new data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ndat1](index.html) module"]
pub struct FDCAN_NDAT1_SPEC;
impl crate::RegisterSpec for FDCAN_NDAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ndat1::R](R) reader structure"]
impl crate::Readable for FDCAN_NDAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ndat1::W](W) writer structure"]
impl crate::Writable for FDCAN_NDAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_NDAT1 to value 0"]
impl crate::Resettable for FDCAN_NDAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
