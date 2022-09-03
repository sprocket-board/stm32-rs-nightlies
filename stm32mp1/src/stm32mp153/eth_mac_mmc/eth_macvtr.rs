#[doc = "Register `ETH_MACVTR` reader"]
pub struct R(crate::R<ETH_MACVTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACVTR` writer"]
pub struct W(crate::W<ETH_MACVTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACVTR_SPEC>;
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
impl From<crate::W<ETH_MACVTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACVTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VL` reader - VL"]
pub type VL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VL` writer - VL"]
pub type VL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACVTR_SPEC, u16, u16, 16, O>;
#[doc = "Field `ETV` reader - ETV"]
pub type ETV_R = crate::BitReader<bool>;
#[doc = "Field `ETV` writer - ETV"]
pub type ETV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `VTIM` reader - VTIM"]
pub type VTIM_R = crate::BitReader<bool>;
#[doc = "Field `VTIM` writer - VTIM"]
pub type VTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `ESVL` reader - ESVL"]
pub type ESVL_R = crate::BitReader<bool>;
#[doc = "Field `ESVL` writer - ESVL"]
pub type ESVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `ERSVLM` reader - ERSVLM"]
pub type ERSVLM_R = crate::BitReader<bool>;
#[doc = "Field `ERSVLM` writer - ERSVLM"]
pub type ERSVLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `DOVLTC` reader - DOVLTC"]
pub type DOVLTC_R = crate::BitReader<bool>;
#[doc = "Field `DOVLTC` writer - DOVLTC"]
pub type DOVLTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `EVLS` reader - EVLS"]
pub type EVLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EVLS` writer - EVLS"]
pub type EVLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACVTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EVLRXS` reader - EVLRXS"]
pub type EVLRXS_R = crate::BitReader<bool>;
#[doc = "Field `EVLRXS` writer - EVLRXS"]
pub type EVLRXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `VTHM` reader - VTHM"]
pub type VTHM_R = crate::BitReader<bool>;
#[doc = "Field `VTHM` writer - VTHM"]
pub type VTHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `EDVLP` reader - EDVLP"]
pub type EDVLP_R = crate::BitReader<bool>;
#[doc = "Field `EDVLP` writer - EDVLP"]
pub type EDVLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `ERIVLT` reader - ERIVLT"]
pub type ERIVLT_R = crate::BitReader<bool>;
#[doc = "Field `ERIVLT` writer - ERIVLT"]
pub type ERIVLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
#[doc = "Field `EIVLS` reader - EIVLS"]
pub type EIVLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIVLS` writer - EIVLS"]
pub type EIVLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACVTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EIVLRXS` reader - EIVLRXS"]
pub type EIVLRXS_R = crate::BitReader<bool>;
#[doc = "Field `EIVLRXS` writer - EIVLRXS"]
pub type EIVLRXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACVTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W<0> {
        VL_W::new(self)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W<16> {
        ETV_W::new(self)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W<17> {
        VTIM_W::new(self)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W<18> {
        ESVL_W::new(self)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W<19> {
        ERSVLM_W::new(self)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W<20> {
        DOVLTC_W::new(self)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&mut self) -> EVLS_W<21> {
        EVLS_W::new(self)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EVLRXS_W<24> {
        EVLRXS_W::new(self)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W<25> {
        VTHM_W::new(self)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&mut self) -> EDVLP_W<26> {
        EDVLP_W::new(self)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W<27> {
        ERIVLT_W::new(self)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&mut self) -> EIVLS_W<28> {
        EIVLS_W::new(self)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W<31> {
        EIVLRXS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The VLAN Tag register identifies the IEEE 802.1Q VLAN type packets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macvtr](index.html) module"]
pub struct ETH_MACVTR_SPEC;
impl crate::RegisterSpec for ETH_MACVTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macvtr::R](R) reader structure"]
impl crate::Readable for ETH_MACVTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macvtr::W](W) writer structure"]
impl crate::Writable for ETH_MACVTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACVTR to value 0"]
impl crate::Resettable for ETH_MACVTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
