#[doc = "Register `CALFACT` reader"]
pub struct R(crate::R<CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFACT` writer"]
pub struct W(crate::W<CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFACT_SPEC>;
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
impl From<crate::W<CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALFACT_S` reader - ADC calibration factor in single-ended mode"]
pub type CALFACT_S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALFACT_S` writer - ADC calibration factor in single-ended mode"]
pub type CALFACT_S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CALFACT_SPEC, u16, u16, 11, O>;
#[doc = "Field `CALFACT_D` reader - ADC calibration factor in differential mode"]
pub type CALFACT_D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALFACT_D` writer - ADC calibration factor in differential mode"]
pub type CALFACT_D_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CALFACT_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W<0> {
        CALFACT_S_W::new(self)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W<16> {
        CALFACT_D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factors register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact](index.html) module"]
pub struct CALFACT_SPEC;
impl crate::RegisterSpec for CALFACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfact::R](R) reader structure"]
impl crate::Readable for CALFACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfact::W](W) writer structure"]
impl crate::Writable for CALFACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CALFACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
