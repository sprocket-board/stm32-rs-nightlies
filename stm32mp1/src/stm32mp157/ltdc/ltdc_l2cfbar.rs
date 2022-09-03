#[doc = "Register `LTDC_L2CFBAR` reader"]
pub struct R(crate::R<LTDC_L2CFBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2CFBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2CFBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2CFBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2CFBAR` writer"]
pub struct W(crate::W<LTDC_L2CFBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2CFBAR_SPEC>;
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
impl From<crate::W<LTDC_L2CFBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2CFBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBADD` reader - CFBADD"]
pub type CFBADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFBADD` writer - CFBADD"]
pub type CFBADD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LTDC_L2CFBAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W<0> {
        CFBADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cfbar](index.html) module"]
pub struct LTDC_L2CFBAR_SPEC;
impl crate::RegisterSpec for LTDC_L2CFBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2cfbar::R](R) reader structure"]
impl crate::Readable for LTDC_L2CFBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cfbar::W](W) writer structure"]
impl crate::Writable for LTDC_L2CFBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L2CFBAR to value 0"]
impl crate::Resettable for LTDC_L2CFBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
