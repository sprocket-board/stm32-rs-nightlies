#[doc = "Register `CAN_TTGTP` reader"]
pub struct R(crate::R<CAN_TTGTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TTGTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_TTGTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_TTGTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_TTGTP` writer"]
pub struct W(crate::W<CAN_TTGTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_TTGTP_SPEC>;
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
impl From<crate::W<CAN_TTGTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_TTGTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - Time Preset"]
pub type NCL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NCL` writer - Time Preset"]
pub type NCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAN_TTGTP_SPEC, u16, u16, 16, O>;
#[doc = "Field `CTP` reader - Cycle Time Target Phase"]
pub type CTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTP` writer - Cycle Time Target Phase"]
pub type CTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAN_TTGTP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W<0> {
        NCL_W::new(self)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W<16> {
        CTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Global Time Preset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ttgtp](index.html) module"]
pub struct CAN_TTGTP_SPEC;
impl crate::RegisterSpec for CAN_TTGTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_ttgtp::R](R) reader structure"]
impl crate::Readable for CAN_TTGTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_ttgtp::W](W) writer structure"]
impl crate::Writable for CAN_TTGTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_TTGTP to value 0"]
impl crate::Resettable for CAN_TTGTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
