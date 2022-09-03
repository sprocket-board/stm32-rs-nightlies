#[doc = "Register `RSTE2R` reader"]
pub struct R(crate::R<RSTE2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTE2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTE2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTE2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTE2R` writer"]
pub struct W(crate::W<RSTE2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTE2R_SPEC>;
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
impl From<crate::W<RSTE2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTE2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRT` reader - SRT"]
pub type SRT_R = crate::BitReader<bool>;
#[doc = "Field `SRT` writer - SRT"]
pub type SRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `RESYNC` reader - RESYNC"]
pub type RESYNC_R = crate::BitReader<bool>;
#[doc = "Field `RESYNC` writer - RESYNC"]
pub type RESYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `PER` reader - PER"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `PER` writer - PER"]
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `CMP1` reader - CMP1"]
pub type CMP1_R = crate::BitReader<bool>;
#[doc = "Field `CMP1` writer - CMP1"]
pub type CMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `CMP2` reader - CMP2"]
pub type CMP2_R = crate::BitReader<bool>;
#[doc = "Field `CMP2` writer - CMP2"]
pub type CMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `CMP3` reader - CMP3"]
pub type CMP3_R = crate::BitReader<bool>;
#[doc = "Field `CMP3` writer - CMP3"]
pub type CMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `CMP4` reader - CMP4"]
pub type CMP4_R = crate::BitReader<bool>;
#[doc = "Field `CMP4` writer - CMP4"]
pub type CMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `MSTPER` reader - MSTPER"]
pub type MSTPER_R = crate::BitReader<bool>;
#[doc = "Field `MSTPER` writer - MSTPER"]
pub type MSTPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `MSTCMP1` reader - MSTCMP1"]
pub type MSTCMP1_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP1` writer - MSTCMP1"]
pub type MSTCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `MSTCMP2` reader - MSTCMP2"]
pub type MSTCMP2_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP2` writer - MSTCMP2"]
pub type MSTCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `MSTCMP3` reader - MSTCMP3"]
pub type MSTCMP3_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP3` writer - MSTCMP3"]
pub type MSTCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `MSTCMP4` reader - MSTCMP4"]
pub type MSTCMP4_R = crate::BitReader<bool>;
#[doc = "Field `MSTCMP4` writer - MSTCMP4"]
pub type MSTCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT1` reader - TIMEVNT1"]
pub type TIMEVNT1_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT1` writer - TIMEVNT1"]
pub type TIMEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT2` reader - TIMEVNT2"]
pub type TIMEVNT2_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT2` writer - TIMEVNT2"]
pub type TIMEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT3` reader - TIMEVNT3"]
pub type TIMEVNT3_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT3` writer - TIMEVNT3"]
pub type TIMEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT4` reader - TIMEVNT4"]
pub type TIMEVNT4_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT4` writer - TIMEVNT4"]
pub type TIMEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT5` reader - TIMEVNT5"]
pub type TIMEVNT5_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT5` writer - TIMEVNT5"]
pub type TIMEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT6` reader - TIMEVNT6"]
pub type TIMEVNT6_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT6` writer - TIMEVNT6"]
pub type TIMEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT7` reader - TIMEVNT7"]
pub type TIMEVNT7_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT7` writer - TIMEVNT7"]
pub type TIMEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT8` reader - TIMEVNT8"]
pub type TIMEVNT8_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT8` writer - TIMEVNT8"]
pub type TIMEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `TIMEVNT9` reader - TIMEVNT9"]
pub type TIMEVNT9_R = crate::BitReader<bool>;
#[doc = "Field `TIMEVNT9` writer - TIMEVNT9"]
pub type TIMEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT1` reader - EXTEVNT1"]
pub type EXTEVNT1_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT1` writer - EXTEVNT1"]
pub type EXTEVNT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT2` reader - EXTEVNT2"]
pub type EXTEVNT2_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT2` writer - EXTEVNT2"]
pub type EXTEVNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT3` reader - EXTEVNT3"]
pub type EXTEVNT3_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT3` writer - EXTEVNT3"]
pub type EXTEVNT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT4` reader - EXTEVNT4"]
pub type EXTEVNT4_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT4` writer - EXTEVNT4"]
pub type EXTEVNT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT5` reader - EXTEVNT5"]
pub type EXTEVNT5_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT5` writer - EXTEVNT5"]
pub type EXTEVNT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT6` reader - EXTEVNT6"]
pub type EXTEVNT6_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT6` writer - EXTEVNT6"]
pub type EXTEVNT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT7` reader - EXTEVNT7"]
pub type EXTEVNT7_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT7` writer - EXTEVNT7"]
pub type EXTEVNT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT8` reader - EXTEVNT8"]
pub type EXTEVNT8_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT8` writer - EXTEVNT8"]
pub type EXTEVNT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT9` reader - EXTEVNT9"]
pub type EXTEVNT9_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT9` writer - EXTEVNT9"]
pub type EXTEVNT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `EXTEVNT10` reader - EXTEVNT10"]
pub type EXTEVNT10_R = crate::BitReader<bool>;
#[doc = "Field `EXTEVNT10` writer - EXTEVNT10"]
pub type EXTEVNT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
#[doc = "Field `UPDATE` reader - UPDATE"]
pub type UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE` writer - UPDATE"]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTE2R_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    pub fn srt(&self) -> SRT_R {
        SRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    pub fn resync(&self) -> RESYNC_R {
        RESYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    pub fn timevnt1(&self) -> TIMEVNT1_R {
        TIMEVNT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    pub fn timevnt2(&self) -> TIMEVNT2_R {
        TIMEVNT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    pub fn timevnt3(&self) -> TIMEVNT3_R {
        TIMEVNT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    pub fn timevnt4(&self) -> TIMEVNT4_R {
        TIMEVNT4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    pub fn timevnt5(&self) -> TIMEVNT5_R {
        TIMEVNT5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    pub fn timevnt6(&self) -> TIMEVNT6_R {
        TIMEVNT6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    pub fn timevnt7(&self) -> TIMEVNT7_R {
        TIMEVNT7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    pub fn timevnt8(&self) -> TIMEVNT8_R {
        TIMEVNT8_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    pub fn timevnt9(&self) -> TIMEVNT9_R {
        TIMEVNT9_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRT"]
    #[inline(always)]
    pub fn srt(&mut self) -> SRT_W<0> {
        SRT_W::new(self)
    }
    #[doc = "Bit 1 - RESYNC"]
    #[inline(always)]
    pub fn resync(&mut self) -> RESYNC_W<1> {
        RESYNC_W::new(self)
    }
    #[doc = "Bit 2 - PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<2> {
        PER_W::new(self)
    }
    #[doc = "Bit 3 - CMP1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> CMP1_W<3> {
        CMP1_W::new(self)
    }
    #[doc = "Bit 4 - CMP2"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W<4> {
        CMP2_W::new(self)
    }
    #[doc = "Bit 5 - CMP3"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> CMP3_W<5> {
        CMP3_W::new(self)
    }
    #[doc = "Bit 6 - CMP4"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W<6> {
        CMP4_W::new(self)
    }
    #[doc = "Bit 7 - MSTPER"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W<7> {
        MSTPER_W::new(self)
    }
    #[doc = "Bit 8 - MSTCMP1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W<8> {
        MSTCMP1_W::new(self)
    }
    #[doc = "Bit 9 - MSTCMP2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W<9> {
        MSTCMP2_W::new(self)
    }
    #[doc = "Bit 10 - MSTCMP3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W<10> {
        MSTCMP3_W::new(self)
    }
    #[doc = "Bit 11 - MSTCMP4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W<11> {
        MSTCMP4_W::new(self)
    }
    #[doc = "Bit 12 - TIMEVNT1"]
    #[inline(always)]
    pub fn timevnt1(&mut self) -> TIMEVNT1_W<12> {
        TIMEVNT1_W::new(self)
    }
    #[doc = "Bit 13 - TIMEVNT2"]
    #[inline(always)]
    pub fn timevnt2(&mut self) -> TIMEVNT2_W<13> {
        TIMEVNT2_W::new(self)
    }
    #[doc = "Bit 14 - TIMEVNT3"]
    #[inline(always)]
    pub fn timevnt3(&mut self) -> TIMEVNT3_W<14> {
        TIMEVNT3_W::new(self)
    }
    #[doc = "Bit 15 - TIMEVNT4"]
    #[inline(always)]
    pub fn timevnt4(&mut self) -> TIMEVNT4_W<15> {
        TIMEVNT4_W::new(self)
    }
    #[doc = "Bit 16 - TIMEVNT5"]
    #[inline(always)]
    pub fn timevnt5(&mut self) -> TIMEVNT5_W<16> {
        TIMEVNT5_W::new(self)
    }
    #[doc = "Bit 17 - TIMEVNT6"]
    #[inline(always)]
    pub fn timevnt6(&mut self) -> TIMEVNT6_W<17> {
        TIMEVNT6_W::new(self)
    }
    #[doc = "Bit 18 - TIMEVNT7"]
    #[inline(always)]
    pub fn timevnt7(&mut self) -> TIMEVNT7_W<18> {
        TIMEVNT7_W::new(self)
    }
    #[doc = "Bit 19 - TIMEVNT8"]
    #[inline(always)]
    pub fn timevnt8(&mut self) -> TIMEVNT8_W<19> {
        TIMEVNT8_W::new(self)
    }
    #[doc = "Bit 20 - TIMEVNT9"]
    #[inline(always)]
    pub fn timevnt9(&mut self) -> TIMEVNT9_W<20> {
        TIMEVNT9_W::new(self)
    }
    #[doc = "Bit 21 - EXTEVNT1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W<21> {
        EXTEVNT1_W::new(self)
    }
    #[doc = "Bit 22 - EXTEVNT2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W<22> {
        EXTEVNT2_W::new(self)
    }
    #[doc = "Bit 23 - EXTEVNT3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W<23> {
        EXTEVNT3_W::new(self)
    }
    #[doc = "Bit 24 - EXTEVNT4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W<24> {
        EXTEVNT4_W::new(self)
    }
    #[doc = "Bit 25 - EXTEVNT5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W<25> {
        EXTEVNT5_W::new(self)
    }
    #[doc = "Bit 26 - EXTEVNT6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W<26> {
        EXTEVNT6_W::new(self)
    }
    #[doc = "Bit 27 - EXTEVNT7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W<27> {
        EXTEVNT7_W::new(self)
    }
    #[doc = "Bit 28 - EXTEVNT8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W<28> {
        EXTEVNT8_W::new(self)
    }
    #[doc = "Bit 29 - EXTEVNT9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W<29> {
        EXTEVNT9_W::new(self)
    }
    #[doc = "Bit 30 - EXTEVNT10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W<30> {
        EXTEVNT10_W::new(self)
    }
    #[doc = "Bit 31 - UPDATE"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<31> {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rste2r](index.html) module"]
pub struct RSTE2R_SPEC;
impl crate::RegisterSpec for RSTE2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rste2r::R](R) reader structure"]
impl crate::Readable for RSTE2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rste2r::W](W) writer structure"]
impl crate::Writable for RSTE2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTE2R to value 0"]
impl crate::Resettable for RSTE2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
