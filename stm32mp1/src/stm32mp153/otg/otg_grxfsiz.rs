#[doc = "Register `OTG_GRXFSIZ` reader"]
pub struct R(crate::R<OTG_GRXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GRXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GRXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GRXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GRXFSIZ` writer"]
pub struct W(crate::W<OTG_GRXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GRXFSIZ_SPEC>;
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
impl From<crate::W<OTG_GRXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GRXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFD` reader - RXFD"]
pub type RXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXFD` writer - RXFD"]
pub type RXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GRXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RXFD"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RXFD"]
    #[inline(always)]
    pub fn rxfd(&mut self) -> RXFD_W<0> {
        RXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grxfsiz](index.html) module"]
pub struct OTG_GRXFSIZ_SPEC;
impl crate::RegisterSpec for OTG_GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_grxfsiz::R](R) reader structure"]
impl crate::Readable for OTG_GRXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_grxfsiz::W](W) writer structure"]
impl crate::Writable for OTG_GRXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_GRXFSIZ to value 0x0400"]
impl crate::Resettable for OTG_GRXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
