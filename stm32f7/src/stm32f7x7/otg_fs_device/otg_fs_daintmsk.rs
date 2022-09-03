#[doc = "Register `OTG_FS_DAINTMSK` reader"]
pub struct R(crate::R<OTG_FS_DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_DAINTMSK` writer"]
pub struct W(crate::W<OTG_FS_DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_DAINTMSK_SPEC>;
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
impl From<crate::W<OTG_FS_DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEPM` reader - IN EP interrupt mask bits"]
pub type IEPM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEPM` writer - IN EP interrupt mask bits"]
pub type IEPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_FS_DAINTMSK_SPEC, u16, u16, 16, O>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OEPINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OEPINT` writer - OUT endpoint interrupt bits"]
pub type OEPINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_FS_DAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP interrupt mask bits"]
    #[inline(always)]
    pub fn iepm(&mut self) -> IEPM_W<0> {
        IEPM_W::new(self)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&mut self) -> OEPINT_W<16> {
        OEPINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_daintmsk](index.html) module"]
pub struct OTG_FS_DAINTMSK_SPEC;
impl crate::RegisterSpec for OTG_FS_DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_daintmsk::R](R) reader structure"]
impl crate::Readable for OTG_FS_DAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_daintmsk::W](W) writer structure"]
impl crate::Writable for OTG_FS_DAINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_DAINTMSK to value 0"]
impl crate::Resettable for OTG_FS_DAINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
