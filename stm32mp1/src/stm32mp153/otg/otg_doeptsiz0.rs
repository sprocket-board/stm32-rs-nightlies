#[doc = "Register `OTG_DOEPTSIZ0` reader"]
pub struct R(crate::R<OTG_DOEPTSIZ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPTSIZ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPTSIZ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPTSIZ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DOEPTSIZ0` writer"]
pub struct W(crate::W<OTG_DOEPTSIZ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPTSIZ0_SPEC>;
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
impl From<crate::W<OTG_DOEPTSIZ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPTSIZ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub type XFRSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub type XFRSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DOEPTSIZ0_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub type PKTCNT_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub type PKTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPTSIZ0_SPEC, bool, O>;
#[doc = "Field `STUPCNT` reader - STUPCNT"]
pub type STUPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STUPCNT` writer - STUPCNT"]
pub type STUPCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DOEPTSIZ0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<0> {
        XFRSIZ_W::new(self)
    }
    #[doc = "Bit 19 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<19> {
        PKTCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - STUPCNT"]
    #[inline(always)]
    pub fn stupcnt(&mut self) -> STUPCNT_W<29> {
        STUPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application must modify this register before enabling endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz0](index.html) module"]
pub struct OTG_DOEPTSIZ0_SPEC;
impl crate::RegisterSpec for OTG_DOEPTSIZ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_doeptsiz0::R](R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz0::W](W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DOEPTSIZ0 to value 0"]
impl crate::Resettable for OTG_DOEPTSIZ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
