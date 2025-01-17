#[doc = "Register `TIM15_DCR` reader"]
pub struct R(crate::R<TIM15_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_DCR` writer"]
pub struct W(crate::W<TIM15_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_DCR_SPEC>;
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
impl From<crate::W<TIM15_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBA` reader - DBA"]
pub type DBA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBA` writer - DBA"]
pub type DBA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM15_DCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DBL` reader - DBL"]
pub type DBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBL` writer - DBL"]
pub type DBL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM15_DCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<0> {
        DBA_W::new(self)
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<8> {
        DBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_dcr](index.html) module"]
pub struct TIM15_DCR_SPEC;
impl crate::RegisterSpec for TIM15_DCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim15_dcr::R](R) reader structure"]
impl crate::Readable for TIM15_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_dcr::W](W) writer structure"]
impl crate::Writable for TIM15_DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM15_DCR to value 0"]
impl crate::Resettable for TIM15_DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
