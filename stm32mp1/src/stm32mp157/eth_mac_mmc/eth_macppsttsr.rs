#[doc = "Register `ETH_MACPPSTTSR` reader"]
pub struct R(crate::R<ETH_MACPPSTTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSTTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSTTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSTTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSTTSR` writer"]
pub struct W(crate::W<ETH_MACPPSTTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSTTSR_SPEC>;
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
impl From<crate::W<ETH_MACPPSTTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSTTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSTRH0` reader - TSTRH0"]
pub type TSTRH0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSTRH0` writer - TSTRH0"]
pub type TSTRH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSTTSR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TSTRH0"]
    #[inline(always)]
    pub fn tstrh0(&self) -> TSTRH0_R {
        TSTRH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSTRH0"]
    #[inline(always)]
    pub fn tstrh0(&mut self) -> TSTRH0_W<0> {
        TSTRH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PPS Target Time Seconds register, along with PPS Target Time Nanoseconds register, is used to schedule an interrupt event \\[Bit 1 of ETH_MACTSSR\\]
when the system time exceeds the value programmed in these registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppsttsr](index.html) module"]
pub struct ETH_MACPPSTTSR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSTTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppsttsr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSTTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppsttsr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSTTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACPPSTTSR to value 0"]
impl crate::Resettable for ETH_MACPPSTTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
