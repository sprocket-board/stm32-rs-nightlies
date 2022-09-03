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
#[doc = "Field `DBGSLPD1` reader - Allow D1 domain debug in Sleep mode"]
pub type DBGSLPD1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSLPD1` writer - Allow D1 domain debug in Sleep mode"]
pub type DBGSLPD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTPD1` reader - Allow D1 domain debug in Stop mode"]
pub type DBGSTPD1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTPD1` writer - Allow D1 domain debug in Stop mode"]
pub type DBGSTPD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBD1` reader - Allow D1 domain debug in Standby mode"]
pub type DBGSTBD1_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBD1` writer - Allow D1 domain debug in Standby mode"]
pub type DBGSTBD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSLPD2` reader - Allow D2 domain debug in Sleep mode"]
pub type DBGSLPD2_R = crate::BitReader<bool>;
#[doc = "Field `DBGSLPD2` writer - Allow D2 domain debug in Sleep mode"]
pub type DBGSLPD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTPD2` reader - Allow D2 domain debug in Stop mode"]
pub type DBGSTPD2_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTPD2` writer - Allow D2 domain debug in Stop mode"]
pub type DBGSTPD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBD2` reader - Allow D2 domain debug in Standby mode"]
pub type DBGSTBD2_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBD2` writer - Allow D2 domain debug in Standby mode"]
pub type DBGSTBD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTPD3` reader - Allow debug in D3 Stop mode"]
pub type DBGSTPD3_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTPD3` writer - Allow debug in D3 Stop mode"]
pub type DBGSTPD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DBGSTBD3` reader - Allow debug in D3 Standby mode"]
pub type DBGSTBD3_R = crate::BitReader<bool>;
#[doc = "Field `DBGSTBD3` writer - Allow debug in D3 Standby mode"]
pub type DBGSTBD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TRACECLKEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TRACECLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `D1DBGCKEN` reader - D1 debug clock enable"]
pub type D1DBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `D1DBGCKEN` writer - D1 debug clock enable"]
pub type D1DBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `D3DBGCKEN` reader - D3 debug clock enable"]
pub type D3DBGCKEN_R = crate::BitReader<bool>;
#[doc = "Field `D3DBGCKEN` writer - D3 debug clock enable"]
pub type D3DBGCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TRGOEN_R = crate::BitReader<bool>;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TRGOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd1(&self) -> DBGSLPD1_R {
        DBGSLPD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&self) -> DBGSTPD1_R {
        DBGSTPD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&self) -> DBGSTBD1_R {
        DBGSTBD1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&self) -> DBGSLPD2_R {
        DBGSLPD2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&self) -> DBGSTPD2_R {
        DBGSTPD2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&self) -> DBGSTBD2_R {
        DBGSTBD2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&self) -> DBGSTPD3_R {
        DBGSTPD3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&self) -> DBGSTBD3_R {
        DBGSTBD3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1DBGCKEN_R {
        D1DBGCKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3DBGCKEN_R {
        D3DBGCKEN_R::new(((self.bits >> 22) & 1) != 0)
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
    pub fn dbgslpd1(&mut self) -> DBGSLPD1_W<0> {
        DBGSLPD1_W::new(self)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&mut self) -> DBGSTPD1_W<1> {
        DBGSTPD1_W::new(self)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&mut self) -> DBGSTBD1_W<2> {
        DBGSTBD1_W::new(self)
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&mut self) -> DBGSLPD2_W<3> {
        DBGSLPD2_W::new(self)
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&mut self) -> DBGSTPD2_W<4> {
        DBGSTPD2_W::new(self)
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&mut self) -> DBGSTBD2_W<5> {
        DBGSTBD2_W::new(self)
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&mut self) -> DBGSTPD3_W<7> {
        DBGSTPD3_W::new(self)
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&mut self) -> DBGSTBD3_W<8> {
        DBGSTBD3_W::new(self)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<20> {
        TRACECLKEN_W::new(self)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1DBGCKEN_W<21> {
        D1DBGCKEN_W::new(self)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&mut self) -> D3DBGCKEN_W<22> {
        D3DBGCKEN_W::new(self)
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
