#[doc = "Register `TIM3_BDTR` reader"]
pub struct R(crate::R<TIM3_BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_BDTR` writer"]
pub struct W(crate::W<TIM3_BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_BDTR_SPEC>;
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
impl From<crate::W<TIM3_BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTG` reader - DTG"]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - DTG"]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_BDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_BDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSSI` reader - OSSI"]
pub type OSSI_R = crate::BitReader<bool>;
#[doc = "Field `OSSI` writer - OSSI"]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `OSSR` reader - OSSR"]
pub type OSSR_R = crate::BitReader<bool>;
#[doc = "Field `OSSR` writer - OSSR"]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - BKE"]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - BKE"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `AOE` reader - AOE"]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - AOE"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `MOE` reader - MOE"]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - MOE"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BKF` reader - BKF"]
pub type BKF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKF` writer - BKF"]
pub type BKF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_BDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BK2F` reader - BK2F"]
pub type BK2F_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BK2F` writer - BK2F"]
pub type BK2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_BDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BK2E` reader - BK2E"]
pub type BK2E_R = crate::BitReader<bool>;
#[doc = "Field `BK2E` writer - BK2E"]
pub type BK2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BK2P` reader - BK2P"]
pub type BK2P_R = crate::BitReader<bool>;
#[doc = "Field `BK2P` writer - BK2P"]
pub type BK2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BKDSRM_R = crate::BitReader<bool>;
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BKDSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BK2DSRM` reader - BK2DSRM"]
pub type BK2DSRM_R = crate::BitReader<bool>;
#[doc = "Field `BK2DSRM` writer - BK2DSRM"]
pub type BK2DSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BKBID` reader - BKBID"]
pub type BKBID_R = crate::BitReader<bool>;
#[doc = "Field `BKBID` writer - BKBID"]
pub type BKBID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
#[doc = "Field `BK2BID` reader - BK2BID"]
pub type BK2BID_R = crate::BitReader<bool>;
#[doc = "Field `BK2BID` writer - BK2BID"]
pub type BK2BID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_BDTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - BK2F"]
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BK2E"]
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BK2P"]
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BK2DSRM"]
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BK2BID"]
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    pub fn bkf(&mut self) -> BKF_W<16> {
        BKF_W::new(self)
    }
    #[doc = "Bits 20:23 - BK2F"]
    #[inline(always)]
    pub fn bk2f(&mut self) -> BK2F_W<20> {
        BK2F_W::new(self)
    }
    #[doc = "Bit 24 - BK2E"]
    #[inline(always)]
    pub fn bk2e(&mut self) -> BK2E_W<24> {
        BK2E_W::new(self)
    }
    #[doc = "Bit 25 - BK2P"]
    #[inline(always)]
    pub fn bk2p(&mut self) -> BK2P_W<25> {
        BK2P_W::new(self)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<26> {
        BKDSRM_W::new(self)
    }
    #[doc = "Bit 27 - BK2DSRM"]
    #[inline(always)]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<27> {
        BK2DSRM_W::new(self)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&mut self) -> BKBID_W<28> {
        BKBID_W::new(self)
    }
    #[doc = "Bit 29 - BK2BID"]
    #[inline(always)]
    pub fn bk2bid(&mut self) -> BK2BID_W<29> {
        BK2BID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_bdtr](index.html) module"]
pub struct TIM3_BDTR_SPEC;
impl crate::RegisterSpec for TIM3_BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim3_bdtr::R](R) reader structure"]
impl crate::Readable for TIM3_BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_bdtr::W](W) writer structure"]
impl crate::Writable for TIM3_BDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM3_BDTR to value 0"]
impl crate::Resettable for TIM3_BDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
