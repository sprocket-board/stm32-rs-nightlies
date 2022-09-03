#[doc = "Register `GICD_ICPENDR1` reader"]
pub struct R(crate::R<GICD_ICPENDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICPENDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICPENDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICPENDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICPENDR1` writer"]
pub struct W(crate::W<GICD_ICPENDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICPENDR1_SPEC>;
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
impl From<crate::W<GICD_ICPENDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICPENDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICPENDR1` reader - ICPENDR1"]
pub type ICPENDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICPENDR1` writer - ICPENDR1"]
pub type ICPENDR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICPENDR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR1"]
    #[inline(always)]
    pub fn icpendr1(&self) -> ICPENDR1_R {
        ICPENDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR1"]
    #[inline(always)]
    pub fn icpendr1(&mut self) -> ICPENDR1_W<0> {
        ICPENDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr1](index.html) module"]
pub struct GICD_ICPENDR1_SPEC;
impl crate::RegisterSpec for GICD_ICPENDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icpendr1::R](R) reader structure"]
impl crate::Readable for GICD_ICPENDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr1::W](W) writer structure"]
impl crate::Writable for GICD_ICPENDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICPENDR1 to value 0"]
impl crate::Resettable for GICD_ICPENDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
