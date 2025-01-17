#[doc = "Register `DOUTR15` reader"]
pub struct R(crate::R<DOUTR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR15` writer"]
pub struct W(crate::W<DOUTR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR15_SPEC>;
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
impl From<crate::W<DOUTR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT15` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT15` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR15_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&self) -> DOUT15_R {
        DOUT15_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&mut self) -> DOUT15_W<0> {
        DOUT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr15](index.html) module"]
pub struct DOUTR15_SPEC;
impl crate::RegisterSpec for DOUTR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr15::R](R) reader structure"]
impl crate::Readable for DOUTR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr15::W](W) writer structure"]
impl crate::Writable for DOUTR15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUTR15 to value 0"]
impl crate::Resettable for DOUTR15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
