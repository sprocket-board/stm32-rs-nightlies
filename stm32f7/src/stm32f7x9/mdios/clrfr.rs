#[doc = "Register `CLRFR` reader"]
pub struct R(crate::R<CLRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPERF` reader - Clear the preamble error flag"]
pub type CPERF_R = crate::BitReader<bool>;
#[doc = "Field `CPERF` writer - Clear the preamble error flag"]
pub type CPERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `CSERF` reader - Clear the start error flag"]
pub type CSERF_R = crate::BitReader<bool>;
#[doc = "Field `CSERF` writer - Clear the start error flag"]
pub type CSERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
#[doc = "Field `CTERF` reader - Clear the turnaround error flag"]
pub type CTERF_R = crate::BitReader<bool>;
#[doc = "Field `CTERF` writer - Clear the turnaround error flag"]
pub type CTERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&mut self) -> CPERF_W<0> {
        CPERF_W::new(self)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&mut self) -> CSERF_W<1> {
        CSERF_W::new(self)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&mut self) -> CTERF_W<2> {
        CTERF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrfr::R](R) reader structure"]
impl crate::Readable for CLRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
