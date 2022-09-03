#[doc = "Register `OTG_HCSPLT0` reader"]
pub struct R(crate::R<OTG_HCSPLT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCSPLT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCSPLT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCSPLT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCSPLT0` writer"]
pub struct W(crate::W<OTG_HCSPLT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCSPLT0_SPEC>;
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
impl From<crate::W<OTG_HCSPLT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCSPLT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTADDR` reader - PRTADDR"]
pub type PRTADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRTADDR` writer - PRTADDR"]
pub type PRTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT0_SPEC, u8, u8, 7, O>;
#[doc = "Field `HUBADDR` reader - HUBADDR"]
pub type HUBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HUBADDR` writer - HUBADDR"]
pub type HUBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT0_SPEC, u8, u8, 7, O>;
#[doc = "Field `XACTPOS` reader - XACTPOS"]
pub type XACTPOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XACTPOS` writer - XACTPOS"]
pub type XACTPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT0_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMPLSPLT` reader - COMPLSPLT"]
pub type COMPLSPLT_R = crate::BitReader<bool>;
#[doc = "Field `COMPLSPLT` writer - COMPLSPLT"]
pub type COMPLSPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCSPLT0_SPEC, bool, O>;
#[doc = "Field `SPLITEN` reader - SPLITEN"]
pub type SPLITEN_R = crate::BitReader<bool>;
#[doc = "Field `SPLITEN` writer - SPLITEN"]
pub type SPLITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCSPLT0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - PRTADDR"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - HUBADDR"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - COMPLSPLT"]
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - SPLITEN"]
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - PRTADDR"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PRTADDR_W<0> {
        PRTADDR_W::new(self)
    }
    #[doc = "Bits 7:13 - HUBADDR"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HUBADDR_W<7> {
        HUBADDR_W::new(self)
    }
    #[doc = "Bits 14:15 - XACTPOS"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XACTPOS_W<14> {
        XACTPOS_W::new(self)
    }
    #[doc = "Bit 16 - COMPLSPLT"]
    #[inline(always)]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<16> {
        COMPLSPLT_W::new(self)
    }
    #[doc = "Bit 31 - SPLITEN"]
    #[inline(always)]
    pub fn spliten(&mut self) -> SPLITEN_W<31> {
        SPLITEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG host channel 0 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt0](index.html) module"]
pub struct OTG_HCSPLT0_SPEC;
impl crate::RegisterSpec for OTG_HCSPLT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcsplt0::R](R) reader structure"]
impl crate::Readable for OTG_HCSPLT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt0::W](W) writer structure"]
impl crate::Writable for OTG_HCSPLT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HCSPLT0 to value 0"]
impl crate::Resettable for OTG_HCSPLT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
