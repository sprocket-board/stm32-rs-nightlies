#[doc = "Register `FDCAN_XIDAM` reader"]
pub struct R(crate::R<FDCAN_XIDAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_XIDAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_XIDAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_XIDAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_XIDAM` writer"]
pub struct W(crate::W<FDCAN_XIDAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_XIDAM_SPEC>;
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
impl From<crate::W<FDCAN_XIDAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_XIDAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIDM` reader - EIDM"]
pub type EIDM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EIDM` writer - EIDM"]
pub type EIDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_XIDAM_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - EIDM"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - EIDM"]
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W<0> {
        EIDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN extended ID and mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_xidam](index.html) module"]
pub struct FDCAN_XIDAM_SPEC;
impl crate::RegisterSpec for FDCAN_XIDAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_xidam::R](R) reader structure"]
impl crate::Readable for FDCAN_XIDAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_xidam::W](W) writer structure"]
impl crate::Writable for FDCAN_XIDAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_XIDAM to value 0x1fff_ffff"]
impl crate::Resettable for FDCAN_XIDAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1fff_ffff
    }
}
