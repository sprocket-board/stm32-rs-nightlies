#[doc = "Register `M5CR` reader"]
pub struct R(crate::R<M5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5CR` writer"]
pub struct W(crate::W<M5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5CR_SPEC>;
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
impl From<crate::W<M5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCSEIE` reader - ECC single error interrupt enable"]
pub type ECCSEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCSEIE` writer - ECC single error interrupt enable"]
pub type ECCSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5CR_SPEC, bool, O>;
#[doc = "Field `ECCDEIE` reader - ECC double error interrupt enable"]
pub type ECCDEIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCDEIE` writer - ECC double error interrupt enable"]
pub type ECCDEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5CR_SPEC, bool, O>;
#[doc = "Field `ECCDEBWIE` reader - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_R = crate::BitReader<bool>;
#[doc = "Field `ECCDEBWIE` writer - ECC double error on byte write (BW) interrupt enable"]
pub type ECCDEBWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5CR_SPEC, bool, O>;
#[doc = "Field `ECCELEN` reader - ECC error latching enable"]
pub type ECCELEN_R = crate::BitReader<bool>;
#[doc = "Field `ECCELEN` writer - ECC error latching enable"]
pub type ECCELEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, M5CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ECC single error interrupt enable"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<2> {
        ECCSEIE_W::new(self)
    }
    #[doc = "Bit 3 - ECC double error interrupt enable"]
    #[inline(always)]
    pub fn eccdeie(&mut self) -> ECCDEIE_W<3> {
        ECCDEIE_W::new(self)
    }
    #[doc = "Bit 4 - ECC double error on byte write (BW) interrupt enable"]
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W<4> {
        ECCDEBWIE_W::new(self)
    }
    #[doc = "Bit 5 - ECC error latching enable"]
    #[inline(always)]
    pub fn eccelen(&mut self) -> ECCELEN_W<5> {
        ECCELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5cr](index.html) module"]
pub struct M5CR_SPEC;
impl crate::RegisterSpec for M5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5cr::R](R) reader structure"]
impl crate::Readable for M5CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5cr::W](W) writer structure"]
impl crate::Writable for M5CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M5CR to value 0"]
impl crate::Resettable for M5CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
