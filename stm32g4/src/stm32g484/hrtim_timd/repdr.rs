#[doc = "Register `REPDR` reader"]
pub struct R(crate::R<REPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REPDR` writer"]
pub struct W(crate::W<REPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REPDR_SPEC>;
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
impl From<crate::W<REPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REPx` reader - Timerx Repetition counter value"]
pub type REPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPx` writer - Timerx Repetition counter value"]
pub type REPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REPDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&self) -> REPX_R {
        REPX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timerx Repetition counter value"]
    #[inline(always)]
    pub fn repx(&mut self) -> REPX_W<0> {
        REPX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repdr](index.html) module"]
pub struct REPDR_SPEC;
impl crate::RegisterSpec for REPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [repdr::R](R) reader structure"]
impl crate::Readable for REPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [repdr::W](W) writer structure"]
impl crate::Writable for REPDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REPDR to value 0"]
impl crate::Resettable for REPDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
