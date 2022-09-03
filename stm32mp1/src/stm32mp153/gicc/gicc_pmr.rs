#[doc = "Register `GICC_PMR` reader"]
pub struct R(crate::R<GICC_PMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_PMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_PMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_PMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_PMR` writer"]
pub struct W(crate::W<GICC_PMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_PMR_SPEC>;
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
impl From<crate::W<GICC_PMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_PMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIORITY` writer - PRIORITY"]
pub type PRIORITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICC_PMR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W<3> {
        PRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC input priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_pmr](index.html) module"]
pub struct GICC_PMR_SPEC;
impl crate::RegisterSpec for GICC_PMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_pmr::R](R) reader structure"]
impl crate::Readable for GICC_PMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_pmr::W](W) writer structure"]
impl crate::Writable for GICC_PMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICC_PMR to value 0"]
impl crate::Resettable for GICC_PMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
