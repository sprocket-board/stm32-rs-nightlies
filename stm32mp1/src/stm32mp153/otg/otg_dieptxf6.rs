#[doc = "Register `OTG_DIEPTXF6` reader"]
pub struct R(crate::R<OTG_DIEPTXF6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DIEPTXF6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DIEPTXF6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DIEPTXF6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DIEPTXF6` writer"]
pub struct W(crate::W<OTG_DIEPTXF6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DIEPTXF6_SPEC>;
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
impl From<crate::W<OTG_DIEPTXF6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DIEPTXF6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXSA` reader - INEPTXSA"]
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXSA` writer - INEPTXSA"]
pub type INEPTXSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DIEPTXF6_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPTXFD` reader - INEPTXFD"]
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXFD` writer - INEPTXFD"]
pub type INEPTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DIEPTXF6_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - INEPTXSA"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - INEPTXFD"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INEPTXSA"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - INEPTXFD"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<16> {
        INEPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device IN endpoint transmit FIFO 6 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf6](index.html) module"]
pub struct OTG_DIEPTXF6_SPEC;
impl crate::RegisterSpec for OTG_DIEPTXF6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dieptxf6::R](R) reader structure"]
impl crate::Readable for OTG_DIEPTXF6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf6::W](W) writer structure"]
impl crate::Writable for OTG_DIEPTXF6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DIEPTXF6 to value 0x0200_0400"]
impl crate::Resettable for OTG_DIEPTXF6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
