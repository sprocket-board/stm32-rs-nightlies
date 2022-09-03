#[doc = "Register `CRYP_DMACR` reader"]
pub struct R(crate::R<CRYP_DMACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_DMACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_DMACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_DMACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_DMACR` writer"]
pub struct W(crate::W<CRYP_DMACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_DMACR_SPEC>;
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
impl From<crate::W<CRYP_DMACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_DMACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIEN` reader - DIEN"]
pub type DIEN_R = crate::BitReader<bool>;
#[doc = "Field `DIEN` writer - DIEN"]
pub type DIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_DMACR_SPEC, bool, O>;
#[doc = "Field `DOEN` reader - DOEN"]
pub type DOEN_R = crate::BitReader<bool>;
#[doc = "Field `DOEN` writer - DOEN"]
pub type DOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRYP_DMACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DIEN"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOEN"]
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIEN"]
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W<0> {
        DIEN_W::new(self)
    }
    #[doc = "Bit 1 - DOEN"]
    #[inline(always)]
    pub fn doen(&mut self) -> DOEN_W<1> {
        DOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRYP DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_dmacr](index.html) module"]
pub struct CRYP_DMACR_SPEC;
impl crate::RegisterSpec for CRYP_DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_dmacr::R](R) reader structure"]
impl crate::Readable for CRYP_DMACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_dmacr::W](W) writer structure"]
impl crate::Writable for CRYP_DMACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_DMACR to value 0"]
impl crate::Resettable for CRYP_DMACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
