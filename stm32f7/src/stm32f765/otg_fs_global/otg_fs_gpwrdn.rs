#[doc = "Register `OTG_FS_GPWRDN` reader"]
pub struct R(crate::R<OTG_FS_GPWRDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GPWRDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GPWRDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GPWRDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GPWRDN` writer"]
pub struct W(crate::W<OTG_FS_GPWRDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GPWRDN_SPEC>;
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
impl From<crate::W<OTG_FS_GPWRDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GPWRDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADPMEN` reader - ADP module enable"]
pub type ADPMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADPMEN` writer - ADP module enable"]
pub type ADPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_FS_GPWRDN_SPEC, bool, O>;
#[doc = "Field `ADPIF` reader - ADP interrupt flag"]
pub type ADPIF_R = crate::BitReader<bool>;
#[doc = "Field `ADPIF` writer - ADP interrupt flag"]
pub type ADPIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_FS_GPWRDN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&self) -> ADPMEN_R {
        ADPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&self) -> ADPIF_R {
        ADPIF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADP module enable"]
    #[inline(always)]
    pub fn adpmen(&mut self) -> ADPMEN_W<0> {
        ADPMEN_W::new(self)
    }
    #[doc = "Bit 23 - ADP interrupt flag"]
    #[inline(always)]
    pub fn adpif(&mut self) -> ADPIF_W<23> {
        ADPIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG power down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gpwrdn](index.html) module"]
pub struct OTG_FS_GPWRDN_SPEC;
impl crate::RegisterSpec for OTG_FS_GPWRDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gpwrdn::R](R) reader structure"]
impl crate::Readable for OTG_FS_GPWRDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gpwrdn::W](W) writer structure"]
impl crate::Writable for OTG_FS_GPWRDN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GPWRDN to value 0x0200_0400"]
impl crate::Resettable for OTG_FS_GPWRDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}
