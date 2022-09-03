#[doc = "Register `DCMI_ICR` writer"]
pub struct W(crate::W<DCMI_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_ICR_SPEC>;
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
impl From<crate::W<DCMI_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_ISC` writer - FRAME_ISC"]
pub type FRAME_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `OVR_ISC` writer - OVR_ISC"]
pub type OVR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `ERR_ISC` writer - ERR_ISC"]
pub type ERR_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `VSYNC_ISC` writer - VSYNC_ISC"]
pub type VSYNC_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
#[doc = "Field `LINE_ISC` writer - LINE_ISC"]
pub type LINE_ISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCMI_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - FRAME_ISC"]
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<0> {
        FRAME_ISC_W::new(self)
    }
    #[doc = "Bit 1 - OVR_ISC"]
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<1> {
        OVR_ISC_W::new(self)
    }
    #[doc = "Bit 2 - ERR_ISC"]
    #[inline(always)]
    pub fn err_isc(&mut self) -> ERR_ISC_W<2> {
        ERR_ISC_W::new(self)
    }
    #[doc = "Bit 3 - VSYNC_ISC"]
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<3> {
        VSYNC_ISC_W::new(self)
    }
    #[doc = "Bit 4 - LINE_ISC"]
    #[inline(always)]
    pub fn line_isc(&mut self) -> LINE_ISC_W<4> {
        LINE_ISC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DCMI_ICR register is write-only.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_icr](index.html) module"]
pub struct DCMI_ICR_SPEC;
impl crate::RegisterSpec for DCMI_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcmi_icr::W](W) writer structure"]
impl crate::Writable for DCMI_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCMI_ICR to value 0"]
impl crate::Resettable for DCMI_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
