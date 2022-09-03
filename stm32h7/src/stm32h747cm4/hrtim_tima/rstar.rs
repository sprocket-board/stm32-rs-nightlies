#[doc = "Register `RSTAR` reader"]
pub struct R(crate::R<RSTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTAR` writer"]
pub struct W(crate::W<RSTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTAR_SPEC>;
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
impl From<crate::W<RSTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDT` reader - Timer A Update reset"]
pub type UPDT_R = crate::BitReader<bool>;
#[doc = "Field `UPDT` writer - Timer A Update reset"]
pub type UPDT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `CMP2` reader - Timer A compare 2 reset"]
pub type CMP2_R = crate::BitReader<bool>;
#[doc = "Field `CMP2` writer - Timer A compare 2 reset"]
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `CMP4` reader - Timer A compare 4 reset"]
pub type CMP4_R = crate::BitReader<bool>;
#[doc = "Field `CMP4` writer - Timer A compare 4 reset"]
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `MSTPER` reader - Master timer Period"]
pub type MSTPER_R = crate::BitReader<bool>;
#[doc = "Field `MSTPER` writer - Master timer Period"]
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `MSTCMP1` reader - Master compare 1"]
pub type MSTCMP1_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP1` writer - Master compare 1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `MSTCMP2` reader - Master compare 2"]
pub type MSTCMP2_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP2` writer - Master compare 2"]
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `MSTCMP3` reader - Master compare 3"]
pub type MSTCMP3_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP3` writer - Master compare 3"]
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `MSTCMP4` reader - Master compare 4"]
pub type MSTCMP4_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP4` writer - Master compare 4"]
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub type EXTEVNT1_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub type EXTEVNT2_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub type EXTEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub type EXTEVNT3_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub type EXTEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub type EXTEVNT4_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub type EXTEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub type EXTEVNT5_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub type EXTEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub type EXTEVNT6_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub type EXTEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub type EXTEVNT7_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub type EXTEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub type EXTEVNT8_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub type EXTEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub type EXTEVNT9_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub type EXTEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub type EXTEVNT10_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub type EXTEVNT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMBCMP1` reader - Timer B Compare 1"]
pub type TIMBCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMBCMP1` writer - Timer B Compare 1"]
pub type TIMBCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMBCMP2` reader - Timer B Compare 2"]
pub type TIMBCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMBCMP2` writer - Timer B Compare 2"]
pub type TIMBCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMBCMP4` reader - Timer B Compare 4"]
pub type TIMBCMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMBCMP4` writer - Timer B Compare 4"]
pub type TIMBCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMCCMP1` reader - Timer C Compare 1"]
pub type TIMCCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMCCMP1` writer - Timer C Compare 1"]
pub type TIMCCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMCCMP2` reader - Timer C Compare 2"]
pub type TIMCCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMCCMP2` writer - Timer C Compare 2"]
pub type TIMCCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMCCMP4` reader - Timer C Compare 4"]
pub type TIMCCMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMCCMP4` writer - Timer C Compare 4"]
pub type TIMCCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMDCMP1` reader - Timer D Compare 1"]
pub type TIMDCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMDCMP1` writer - Timer D Compare 1"]
pub type TIMDCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMDCMP2` reader - Timer D Compare 2"]
pub type TIMDCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMDCMP2` writer - Timer D Compare 2"]
pub type TIMDCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMDCMP4` reader - Timer D Compare 4"]
pub type TIMDCMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMDCMP4` writer - Timer D Compare 4"]
pub type TIMDCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMECMP1` reader - Timer E Compare 1"]
pub type TIMECMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMECMP1` writer - Timer E Compare 1"]
pub type TIMECMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMECMP2` reader - Timer E Compare 2"]
pub type TIMECMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMECMP2` writer - Timer E Compare 2"]
pub type TIMECMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
#[doc = "Field `TIMECMP4` reader - Timer E Compare 4"]
pub type TIMECMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMECMP4` writer - Timer E Compare 4"]
pub type TIMECMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn timccmp1(&self) -> TIMCCMP1_R {
        TIMCCMP1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    pub fn timccmp4(&self) -> TIMCCMP4_R {
        TIMCCMP4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&mut self) -> UPDT_W<1> {
        UPDT_W::new(self)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W<2> {
        CMP2_W::new(self)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W<3> {
        CMP4_W::new(self)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W<4> {
        MSTPER_W::new(self)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<5> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<6> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<7> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<8> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<9> {
        EXTEVNT1_W::new(self)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<10> {
        EXTEVNT2_W::new(self)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<11> {
        EXTEVNT3_W::new(self)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<12> {
        EXTEVNT4_W::new(self)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<13> {
        EXTEVNT5_W::new(self)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<14> {
        EXTEVNT6_W::new(self)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<15> {
        EXTEVNT7_W::new(self)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<16> {
        EXTEVNT8_W::new(self)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<17> {
        EXTEVNT9_W::new(self)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<18> {
        EXTEVNT10_W::new(self)
    }
    #[doc = "Bit 19 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W<19> {
        TIMBCMP1_W::new(self)
    }
    #[doc = "Bit 20 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W<20> {
        TIMBCMP2_W::new(self)
    }
    #[doc = "Bit 21 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W<21> {
        TIMBCMP4_W::new(self)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn timccmp1(&mut self) -> TIMCCMP1_W<22> {
        TIMCCMP1_W::new(self)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W<23> {
        TIMCCMP2_W::new(self)
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    pub fn timccmp4(&mut self) -> TIMCCMP4_W<24> {
        TIMCCMP4_W::new(self)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W<25> {
        TIMDCMP1_W::new(self)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W<26> {
        TIMDCMP2_W::new(self)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W<27> {
        TIMDCMP4_W::new(self)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&mut self) -> TIMECMP1_W<28> {
        TIMECMP1_W::new(self)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&mut self) -> TIMECMP2_W<29> {
        TIMECMP2_W::new(self)
    }
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&mut self) -> TIMECMP4_W<30> {
        TIMECMP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstar](index.html) module"]
pub struct RSTAR_SPEC;
impl crate::RegisterSpec for RSTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstar::R](R) reader structure"]
impl crate::Readable for RSTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstar::W](W) writer structure"]
impl crate::Writable for RSTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTAR to value 0"]
impl crate::Resettable for RSTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
