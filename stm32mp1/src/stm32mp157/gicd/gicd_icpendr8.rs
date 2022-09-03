#[doc = "Register `GICD_ICPENDR8` reader"]
pub struct R(crate::R<GICD_ICPENDR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICPENDR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICPENDR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICPENDR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICPENDR8` writer"]
pub struct W(crate::W<GICD_ICPENDR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICPENDR8_SPEC>;
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
impl From<crate::W<GICD_ICPENDR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICPENDR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICPENDR8` reader - ICPENDR8"]
pub type ICPENDR8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICPENDR8` writer - ICPENDR8"]
pub type ICPENDR8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICPENDR8_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    pub fn icpendr8(&self) -> ICPENDR8_R {
        ICPENDR8_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    pub fn icpendr8(&mut self) -> ICPENDR8_W<0> {
        ICPENDR8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icpendr8](index.html) module"]
pub struct GICD_ICPENDR8_SPEC;
impl crate::RegisterSpec for GICD_ICPENDR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icpendr8::R](R) reader structure"]
impl crate::Readable for GICD_ICPENDR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icpendr8::W](W) writer structure"]
impl crate::Writable for GICD_ICPENDR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICPENDR8 to value 0"]
impl crate::Resettable for GICD_ICPENDR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
