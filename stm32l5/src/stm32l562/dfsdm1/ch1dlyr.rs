#[doc = "Register `CH1DLYR` reader"]
pub struct R(crate::R<CH1DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1DLYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1DLYR` writer"]
pub struct W(crate::W<CH1DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1DLYR_SPEC>;
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
impl From<crate::W<CH1DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1DLYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PLSSKP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PLSSKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1DLYR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W<0> {
        PLSSKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1dlyr](index.html) module"]
pub struct CH1DLYR_SPEC;
impl crate::RegisterSpec for CH1DLYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1dlyr::R](R) reader structure"]
impl crate::Readable for CH1DLYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1dlyr::W](W) writer structure"]
impl crate::Writable for CH1DLYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH1DLYR to value 0"]
impl crate::Resettable for CH1DLYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
