#[doc = "Register `OTG_FS_DTXFSTS55` reader"]
pub struct R(crate::R<OTG_FS_DTXFSTS55_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DTXFSTS55_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DTXFSTS55_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DTXFSTS55_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_DTXFSTS55` writer"]
pub struct W(crate::W<OTG_FS_DTXFSTS55_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_DTXFSTS55_SPEC>;
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
impl From<crate::W<OTG_FS_DTXFSTS55_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_DTXFSTS55_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTFSAV` writer - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_FS_DTXFSTS55_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&mut self) -> INEPTFSAV_W<0> {
        INEPTFSAV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts55](index.html) module"]
pub struct OTG_FS_DTXFSTS55_SPEC;
impl crate::RegisterSpec for OTG_FS_DTXFSTS55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_dtxfsts55::R](R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS55_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_dtxfsts55::W](W) writer structure"]
impl crate::Writable for OTG_FS_DTXFSTS55_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_DTXFSTS55 to value 0"]
impl crate::Resettable for OTG_FS_DTXFSTS55_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
