#[doc = "Register `OTG_FS_HAINTMSK` reader"]
pub struct R(crate::R<OTG_FS_HAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_HAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_HAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_HAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_HAINTMSK` writer"]
pub struct W(crate::W<OTG_FS_HAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_HAINTMSK_SPEC>;
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
impl From<crate::W<OTG_FS_HAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_HAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAINTM` reader - Channel interrupt mask"]
pub type HAINTM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HAINTM` writer - Channel interrupt mask"]
pub type HAINTM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_FS_HAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&self) -> HAINTM_R {
        HAINTM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel interrupt mask"]
    #[inline(always)]
    pub fn haintm(&mut self) -> HAINTM_W<0> {
        HAINTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_haintmsk](index.html) module"]
pub struct OTG_FS_HAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_FS_HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_haintmsk::R](R) reader structure"]
impl crate::Readable for OTG_FS_HAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_haintmsk::W](W) writer structure"]
impl crate::Writable for OTG_FS_HAINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_HAINTMSK to value 0"]
impl crate::Resettable for OTG_FS_HAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
