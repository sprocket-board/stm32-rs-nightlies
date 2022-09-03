#[doc = "Register `DTXFSTS7` reader"]
pub struct R(crate::R<DTXFSTS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTXFSTS7` writer"]
pub struct W(crate::W<DTXFSTS7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTXFSTS7_SPEC>;
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
impl From<crate::W<DTXFSTS7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTXFSTS7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTFSAV` writer - IN endpoint TxFIFO space avail"]
pub type INEPTFSAV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTXFSTS7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
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
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts7](index.html) module"]
pub struct DTXFSTS7_SPEC;
impl crate::RegisterSpec for DTXFSTS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts7::R](R) reader structure"]
impl crate::Readable for DTXFSTS7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtxfsts7::W](W) writer structure"]
impl crate::Writable for DTXFSTS7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTXFSTS7 to value 0"]
impl crate::Resettable for DTXFSTS7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}