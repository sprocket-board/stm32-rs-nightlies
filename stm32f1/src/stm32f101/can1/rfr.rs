#[doc = "Register `RF%sR` reader"]
pub struct R(crate::R<RFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF%sR` writer"]
pub struct W(crate::W<RFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFR_SPEC>;
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
impl From<crate::W<RFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMP` reader - FMP0"]
pub type FMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FULL` reader - FULL0"]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `FULL` writer - FULL0"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, bool, O>;
#[doc = "Field `FOVR` reader - FOVR0"]
pub type FOVR_R = crate::BitReader<bool>;
#[doc = "Field `FOVR` writer - FOVR0"]
pub type FOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, bool, O>;
#[doc = "Field `RFOM` reader - RFOM0"]
pub type RFOM_R = crate::BitReader<bool>;
#[doc = "Field `RFOM` writer - RFOM0"]
pub type RFOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W<3> {
        FULL_W::new(self)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&mut self) -> FOVR_W<4> {
        FOVR_W::new(self)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&mut self) -> RFOM_W<5> {
        RFOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO %s register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](index.html) module"]
pub struct RFR_SPEC;
impl crate::RegisterSpec for RFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfr::R](R) reader structure"]
impl crate::Readable for RFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfr::W](W) writer structure"]
impl crate::Writable for RFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF%sR to value 0"]
impl crate::Resettable for RFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
