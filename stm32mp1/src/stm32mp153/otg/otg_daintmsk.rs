#[doc = "Register `OTG_DAINTMSK` reader"]
pub struct R(crate::R<OTG_DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DAINTMSK` writer"]
pub struct W(crate::W<OTG_DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DAINTMSK_SPEC>;
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
impl From<crate::W<OTG_DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEPM` reader - IEPM"]
pub type IEPM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEPM` writer - IEPM"]
pub type IEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DAINTMSK_SPEC, u16, u16, 16, O>;
#[doc = "Field `OEPM` reader - OEPM"]
pub type OEPM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OEPM` writer - OEPM"]
pub type OEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IEPM"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OEPM"]
    #[inline(always)]
    pub fn oepm(&self) -> OEPM_R {
        OEPM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IEPM"]
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<0> {
        IEPM_W::new(self)
    }
    #[doc = "Bits 16:31 - OEPM"]
    #[inline(always)]
    pub fn oepm(&mut self) -> OEPM_W<16> {
        OEPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_daintmsk](index.html) module"]
pub struct OTG_DAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_daintmsk::R](R) reader structure"]
impl crate::Readable for OTG_DAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_daintmsk::W](W) writer structure"]
impl crate::Writable for OTG_DAINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DAINTMSK to value 0"]
impl crate::Resettable for OTG_DAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
