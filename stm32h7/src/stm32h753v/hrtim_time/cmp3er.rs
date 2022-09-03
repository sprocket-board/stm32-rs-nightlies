#[doc = "Register `CMP3ER` reader"]
pub struct R(crate::R<CMP3ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP3ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP3ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP3ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP3ER` writer"]
pub struct W(crate::W<CMP3ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP3ER_SPEC>;
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
impl From<crate::W<CMP3ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP3ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP3x` reader - Timerx Compare 3 value"]
pub type CMP3X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CMP3x` writer - Timerx Compare 3 value"]
pub type CMP3X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMP3ER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&mut self) -> CMP3X_W<0> {
        CMP3X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3er](index.html) module"]
pub struct CMP3ER_SPEC;
impl crate::RegisterSpec for CMP3ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp3er::R](R) reader structure"]
impl crate::Readable for CMP3ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp3er::W](W) writer structure"]
impl crate::Writable for CMP3ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP3ER to value 0"]
impl crate::Resettable for CMP3ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
