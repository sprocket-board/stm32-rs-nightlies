#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGSLEEP_CD` reader - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_CD_R = crate::BitReader<bool>;
#[doc = "Field `DBGSLEEP_CD` writer - Allow D1 domain debug in Sleep mode"]
pub type DBGSLEEP_CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTOP_CD` reader - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_CD_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTOP_CD` writer - Allow D1 domain debug in Stop mode"]
pub type DBGSTOP_CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBY_CD` reader - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_CD_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBY_CD` writer - Allow D1 domain debug in Standby mode"]
pub type DBGSTBY_CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTOP_SRD` reader - debug in SmartRun domain Stop mode"]
pub type DBGSTOP_SRD_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTOP_SRD` writer - debug in SmartRun domain Stop mode"]
pub type DBGSTOP_SRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBY_SRD` reader - debug in SmartRun domain Standby mode"]
pub type DBGSTBY_SRD_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBY_SRD` writer - debug in SmartRun domain Standby mode"]
pub type DBGSTBY_SRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TRACECLKEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TRACECLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CDDBGCKEN` reader - CPU domain debug clock enable"]
pub type CDDBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `CDDBGCKEN` writer - CPU domain debug clock enable"]
pub type CDDBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SRDDBGCKEN` reader - SmartRun domain debug clock enable"]
pub type SRDDBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `SRDDBGCKEN` writer - SmartRun domain debug clock enable"]
pub type SRDDBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TRGOEN_R = crate::BitReader<bool>;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TRGOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_cd(&self) -> DBGSLEEP_CD_R {
        DBGSLEEP_CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_cd(&self) -> DBGSTOP_CD_R {
        DBGSTOP_CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_cd(&self) -> DBGSTBY_CD_R {
        DBGSTBY_CD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    pub fn dbgstop_srd(&self) -> DBGSTOP_SRD_R {
        DBGSTOP_SRD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    pub fn dbgstby_srd(&self) -> DBGSTBY_SRD_R {
        DBGSTBY_SRD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    pub fn cddbgcken(&self) -> CDDBGCKEN_R {
        CDDBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    pub fn srddbgcken(&self) -> SRDDBGCKEN_R {
        SRDDBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TRGOEN_R {
        TRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgsleep_cd(&mut self) -> DBGSLEEP_CD_W<0> {
        DBGSLEEP_CD_W::new(self)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstop_cd(&mut self) -> DBGSTOP_CD_W<1> {
        DBGSTOP_CD_W::new(self)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstby_cd(&mut self) -> DBGSTBY_CD_W<2> {
        DBGSTBY_CD_W::new(self)
    }
    #[doc = "Bit 7 - debug in SmartRun domain Stop mode"]
    #[inline(always)]
    pub fn dbgstop_srd(&mut self) -> DBGSTOP_SRD_W<7> {
        DBGSTOP_SRD_W::new(self)
    }
    #[doc = "Bit 8 - debug in SmartRun domain Standby mode"]
    #[inline(always)]
    pub fn dbgstby_srd(&mut self) -> DBGSTBY_SRD_W<8> {
        DBGSTBY_SRD_W::new(self)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<20> {
        TRACECLKEN_W::new(self)
    }
    #[doc = "Bit 21 - CPU domain debug clock enable"]
    #[inline(always)]
    pub fn cddbgcken(&mut self) -> CDDBGCKEN_W<21> {
        CDDBGCKEN_W::new(self)
    }
    #[doc = "Bit 22 - SmartRun domain debug clock enable"]
    #[inline(always)]
    pub fn srddbgcken(&mut self) -> SRDDBGCKEN_W<22> {
        SRDDBGCKEN_W::new(self)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&mut self) -> TRGOEN_W<28> {
        TRGOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
