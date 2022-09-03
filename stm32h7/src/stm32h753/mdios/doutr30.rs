#[doc = "Register `DOUTR30` reader"]
pub struct R(crate::R<DOUTR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR30` writer"]
pub struct W(crate::W<DOUTR30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR30_SPEC>;
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
impl From<crate::W<DOUTR30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT30` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT30_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT30` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR30_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&self) -> DOUT30_R {
        DOUT30_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&mut self) -> DOUT30_W<0> {
        DOUT30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr30](index.html) module"]
pub struct DOUTR30_SPEC;
impl crate::RegisterSpec for DOUTR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr30::R](R) reader structure"]
impl crate::Readable for DOUTR30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr30::W](W) writer structure"]
impl crate::Writable for DOUTR30_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUTR30 to value 0"]
impl crate::Resettable for DOUTR30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}