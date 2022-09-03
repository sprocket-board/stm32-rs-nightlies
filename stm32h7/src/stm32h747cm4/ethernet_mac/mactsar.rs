#[doc = "Register `MACTSAR` reader"]
pub struct R(crate::R<MACTSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSAR` writer"]
pub struct W(crate::W<MACTSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSAR_SPEC>;
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
impl From<crate::W<MACTSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TSAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTSAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W<0> {
        TSAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp addend register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsar](index.html) module"]
pub struct MACTSAR_SPEC;
impl crate::RegisterSpec for MACTSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactsar::R](R) reader structure"]
impl crate::Readable for MACTSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactsar::W](W) writer structure"]
impl crate::Writable for MACTSAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACTSAR to value 0"]
impl crate::Resettable for MACTSAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
