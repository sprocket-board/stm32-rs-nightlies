#[doc = "Register `BMCR` reader"]
pub struct R(crate::R<BMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMCR` writer"]
pub struct W(crate::W<BMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCR_SPEC>;
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
impl From<crate::W<BMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BME` reader - Burst Mode enable"]
pub type BME_R = crate::BitReader<bool>;
#[doc = "Field `BME` writer - Burst Mode enable"]
pub type BME_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub type BMOM_R = crate::BitReader<bool>;
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub type BMOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub type BMCLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub type BMCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub type BMPRSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub type BMPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub type BMPREN_R = crate::BitReader<bool>;
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub type BMPREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub type MTBM_R = crate::BitReader<bool>;
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub type MTBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub type TABM_R = crate::BitReader<bool>;
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub type TABM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub type TBBM_R = crate::BitReader<bool>;
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub type TBBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub type TCBM_R = crate::BitReader<bool>;
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub type TCBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub type TDBM_R = crate::BitReader<bool>;
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub type TDBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub type TEBM_R = crate::BitReader<bool>;
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub type TEBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub type BMSTAT_R = crate::BitReader<bool>;
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub type BMSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&mut self) -> BME_W<0> {
        BME_W::new(self)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&mut self) -> BMOM_W<1> {
        BMOM_W::new(self)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&mut self) -> BMCLK_W<2> {
        BMCLK_W::new(self)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&mut self) -> BMPRSC_W<6> {
        BMPRSC_W::new(self)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&mut self) -> BMPREN_W<10> {
        BMPREN_W::new(self)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&mut self) -> MTBM_W<16> {
        MTBM_W::new(self)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&mut self) -> TABM_W<17> {
        TABM_W::new(self)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&mut self) -> TBBM_W<18> {
        TBBM_W::new(self)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&mut self) -> TCBM_W<19> {
        TCBM_W::new(self)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&mut self) -> TDBM_W<20> {
        TDBM_W::new(self)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&mut self) -> TEBM_W<21> {
        TEBM_W::new(self)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&mut self) -> BMSTAT_W<31> {
        BMSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr](index.html) module"]
pub struct BMCR_SPEC;
impl crate::RegisterSpec for BMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmcr::R](R) reader structure"]
impl crate::Readable for BMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmcr::W](W) writer structure"]
impl crate::Writable for BMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
