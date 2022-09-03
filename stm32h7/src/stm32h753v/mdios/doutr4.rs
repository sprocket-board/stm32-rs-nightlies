#[doc = "Register `DOUTR4` reader"]
pub struct R(crate::R<DOUTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR4` writer"]
pub struct W(crate::W<DOUTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR4_SPEC>;
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
impl From<crate::W<DOUTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT4` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT4` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&self) -> DOUT4_R {
        DOUT4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&mut self) -> DOUT4_W<0> {
        DOUT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr4](index.html) module"]
pub struct DOUTR4_SPEC;
impl crate::RegisterSpec for DOUTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr4::R](R) reader structure"]
impl crate::Readable for DOUTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr4::W](W) writer structure"]
impl crate::Writable for DOUTR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOUTR4 to value 0"]
impl crate::Resettable for DOUTR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
