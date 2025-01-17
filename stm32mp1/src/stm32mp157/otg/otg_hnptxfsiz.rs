#[doc = "Register `OTG_HNPTXFSIZ` reader"]
pub struct R(crate::R<OTG_HNPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HNPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HNPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HNPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HNPTXFSIZ` writer"]
pub struct W(crate::W<OTG_HNPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HNPTXFSIZ_SPEC>;
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
impl From<crate::W<OTG_HNPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HNPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPTXFSA` reader - NPTXFSA"]
pub type NPTXFSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXFSA` writer - NPTXFSA"]
pub type NPTXFSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HNPTXFSIZ_SPEC, u16, u16, 16, O>;
#[doc = "Field `NPTXFD` reader - NPTXFD"]
pub type NPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPTXFD` writer - NPTXFD"]
pub type NPTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HNPTXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<0> {
        NPTXFSA_W::new(self)
    }
    #[doc = "Bits 16:31 - NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W<16> {
        NPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hnptxfsiz](index.html) module"]
pub struct OTG_HNPTXFSIZ_SPEC;
impl crate::RegisterSpec for OTG_HNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hnptxfsiz::R](R) reader structure"]
impl crate::Readable for OTG_HNPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hnptxfsiz::W](W) writer structure"]
impl crate::Writable for OTG_HNPTXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HNPTXFSIZ to value 0x0200_0200"]
impl crate::Resettable for OTG_HNPTXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0200
    }
}
