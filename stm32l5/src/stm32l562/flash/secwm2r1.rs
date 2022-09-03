#[doc = "Register `SECWM2R1` reader"]
pub struct R(crate::R<SECWM2R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECWM2R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECWM2R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECWM2R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECWM2R1` writer"]
pub struct W(crate::W<SECWM2R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECWM2R1_SPEC>;
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
impl From<crate::W<SECWM2R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECWM2R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECWM2_PSTRT` reader - SECWM2_PSTRT"]
pub type SECWM2_PSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM2_PSTRT` writer - SECWM2_PSTRT"]
pub type SECWM2_PSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM2R1_SPEC, u8, u8, 7, O>;
#[doc = "Field `SECWM2_PEND` reader - SECWM2_PEND"]
pub type SECWM2_PEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECWM2_PEND` writer - SECWM2_PEND"]
pub type SECWM2_PEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECWM2R1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - SECWM2_PSTRT"]
    #[inline(always)]
    pub fn secwm2_pstrt(&self) -> SECWM2_PSTRT_R {
        SECWM2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - SECWM2_PEND"]
    #[inline(always)]
    pub fn secwm2_pend(&self) -> SECWM2_PEND_R {
        SECWM2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - SECWM2_PSTRT"]
    #[inline(always)]
    pub fn secwm2_pstrt(&mut self) -> SECWM2_PSTRT_W<0> {
        SECWM2_PSTRT_W::new(self)
    }
    #[doc = "Bits 16:22 - SECWM2_PEND"]
    #[inline(always)]
    pub fn secwm2_pend(&mut self) -> SECWM2_PEND_W<16> {
        SECWM2_PEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash secure watermak2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secwm2r1](index.html) module"]
pub struct SECWM2R1_SPEC;
impl crate::RegisterSpec for SECWM2R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secwm2r1::R](R) reader structure"]
impl crate::Readable for SECWM2R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secwm2r1::W](W) writer structure"]
impl crate::Writable for SECWM2R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECWM2R1 to value 0xff00_ff00"]
impl crate::Resettable for SECWM2R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00_ff00
    }
}
