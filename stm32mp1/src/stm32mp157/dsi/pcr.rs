#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETTXE` reader - ETTXE"]
pub type ETTXE_R = crate::BitReader<bool>;
#[doc = "Field `ETTXE` writer - ETTXE"]
pub type ETTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `ETRXE` reader - ETRXE"]
pub type ETRXE_R = crate::BitReader<bool>;
#[doc = "Field `ETRXE` writer - ETRXE"]
pub type ETRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `BTAE` reader - BTAE"]
pub type BTAE_R = crate::BitReader<bool>;
#[doc = "Field `BTAE` writer - BTAE"]
pub type BTAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `ECCRXE` reader - ECCRXE"]
pub type ECCRXE_R = crate::BitReader<bool>;
#[doc = "Field `ECCRXE` writer - ECCRXE"]
pub type ECCRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
#[doc = "Field `CRCRXE` reader - CRCRXE"]
pub type CRCRXE_R = crate::BitReader<bool>;
#[doc = "Field `CRCRXE` writer - CRCRXE"]
pub type CRCRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ETTXE"]
    #[inline(always)]
    pub fn ettxe(&mut self) -> ETTXE_W<0> {
        ETTXE_W::new(self)
    }
    #[doc = "Bit 1 - ETRXE"]
    #[inline(always)]
    pub fn etrxe(&mut self) -> ETRXE_W<1> {
        ETRXE_W::new(self)
    }
    #[doc = "Bit 2 - BTAE"]
    #[inline(always)]
    pub fn btae(&mut self) -> BTAE_W<2> {
        BTAE_W::new(self)
    }
    #[doc = "Bit 3 - ECCRXE"]
    #[inline(always)]
    pub fn eccrxe(&mut self) -> ECCRXE_W<3> {
        ECCRXE_W::new(self)
    }
    #[doc = "Bit 4 - CRCRXE"]
    #[inline(always)]
    pub fn crcrxe(&mut self) -> CRCRXE_W<4> {
        CRCRXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host protocol configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
