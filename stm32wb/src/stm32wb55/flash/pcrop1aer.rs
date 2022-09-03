#[doc = "Register `PCROP1AER` reader"]
pub struct R(crate::R<PCROP1AER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1AER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1AER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1AER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1AER` writer"]
pub struct W(crate::W<PCROP1AER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1AER_SPEC>;
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
impl From<crate::W<PCROP1AER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1AER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1A_END` reader - Bank 1 PCROP area end offset"]
pub type PCROP1A_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP1A_END` writer - Bank 1 PCROP area end offset"]
pub type PCROP1A_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP1AER_SPEC, u16, u16, 9, O>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader<bool>;
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCROP1AER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&self) -> PCROP1A_END_R {
        PCROP1A_END_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1a_end(&mut self) -> PCROP1A_END_W<0> {
        PCROP1A_END_W::new(self)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<31> {
        PCROP_RDP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP End address zone A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1aer](index.html) module"]
pub struct PCROP1AER_SPEC;
impl crate::RegisterSpec for PCROP1AER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1aer::R](R) reader structure"]
impl crate::Readable for PCROP1AER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1aer::W](W) writer structure"]
impl crate::Writable for PCROP1AER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP1AER to value 0x7fff_fe00"]
impl crate::Resettable for PCROP1AER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_fe00
    }
}
