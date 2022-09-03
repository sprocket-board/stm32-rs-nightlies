#[doc = "Register `LTDC_L2CFBLNR` reader"]
pub struct R(crate::R<LTDC_L2CFBLNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2CFBLNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2CFBLNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2CFBLNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2CFBLNR` writer"]
pub struct W(crate::W<LTDC_L2CFBLNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2CFBLNR_SPEC>;
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
impl From<crate::W<LTDC_L2CFBLNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2CFBLNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBLNBR` reader - CFBLNBR"]
pub type CFBLNBR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CFBLNBR` writer - CFBLNBR"]
pub type CFBLNBR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2CFBLNR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - CFBLNBR"]
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - CFBLNBR"]
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W<0> {
        CFBLNBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the number of lines in the color frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfblnr](index.html) module"]
pub struct LTDC_L2CFBLNR_SPEC;
impl crate::RegisterSpec for LTDC_L2CFBLNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2cfblnr::R](R) reader structure"]
impl crate::Readable for LTDC_L2CFBLNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfblnr::W](W) writer structure"]
impl crate::Writable for LTDC_L2CFBLNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L2CFBLNR to value 0"]
impl crate::Resettable for LTDC_L2CFBLNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
