#[doc = "Register `DTCMCR` reader"]
pub struct R(crate::R<DTCMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCMCR` writer"]
pub struct W(crate::W<DTCMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCMCR_SPEC>;
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
impl From<crate::W<DTCMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCMCR_SPEC, bool, O>;
#[doc = "Field `RMW` reader - RMW"]
pub type RMW_R = crate::BitReader<bool>;
#[doc = "Field `RMW` writer - RMW"]
pub type RMW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCMCR_SPEC, bool, O>;
#[doc = "Field `RETEN` reader - RETEN"]
pub type RETEN_R = crate::BitReader<bool>;
#[doc = "Field `RETEN` writer - RETEN"]
pub type RETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTCMCR_SPEC, bool, O>;
#[doc = "Field `SZ` reader - SZ"]
pub type SZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SZ` writer - SZ"]
pub type SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTCMCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - RMW"]
    #[inline(always)]
    pub fn rmw(&mut self) -> RMW_W<1> {
        RMW_W::new(self)
    }
    #[doc = "Bit 2 - RETEN"]
    #[inline(always)]
    pub fn reten(&mut self) -> RETEN_W<2> {
        RETEN_W::new(self)
    }
    #[doc = "Bits 3:6 - SZ"]
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W<3> {
        SZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcmcr](index.html) module"]
pub struct DTCMCR_SPEC;
impl crate::RegisterSpec for DTCMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtcmcr::R](R) reader structure"]
impl crate::Readable for DTCMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtcmcr::W](W) writer structure"]
impl crate::Writable for DTCMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTCMCR to value 0"]
impl crate::Resettable for DTCMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
