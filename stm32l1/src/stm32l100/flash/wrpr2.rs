#[doc = "Register `WRPR2` reader"]
pub struct R(crate::R<WRPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPR2` writer"]
pub struct W(crate::W<WRPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPR2_SPEC>;
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
impl From<crate::W<WRPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRP2` reader - WRP2"]
pub type WRP2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRP2` writer - WRP2"]
pub type WRP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRPR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    pub fn wrp2(&self) -> WRP2_R {
        WRP2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRP2"]
    #[inline(always)]
    pub fn wrp2(&mut self) -> WRP2_W<0> {
        WRP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpr2](index.html) module"]
pub struct WRPR2_SPEC;
impl crate::RegisterSpec for WRPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpr2::R](R) reader structure"]
impl crate::Readable for WRPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpr2::W](W) writer structure"]
impl crate::Writable for WRPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRPR2 to value 0"]
impl crate::Resettable for WRPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
