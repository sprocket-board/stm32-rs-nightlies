#[doc = "Register `C2AHB3SMENR` reader"]
pub struct R(crate::R<C2AHB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB3SMENR` writer"]
pub struct W(crate::W<C2AHB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB3SMENR_SPEC>;
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
impl From<crate::W<C2AHB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKASMEN` reader - PKA accelerator clock enable during CPU2 CSleep mode."]
pub type PKASMEN_R = crate::BitReader<bool>;
#[doc = "Field `PKASMEN` writer - PKA accelerator clock enable during CPU2 CSleep mode."]
pub type PKASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `AESSMEN` reader - AES accelerator clock enable during CPU2 CSleep mode."]
pub type AESSMEN_R = crate::BitReader<bool>;
#[doc = "Field `AESSMEN` writer - AES accelerator clock enable during CPU2 CSleep mode."]
pub type AESSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `RNGSMEN` reader - True RNG clock enable during CPU2 CSleep and CStop mode."]
pub type RNGSMEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGSMEN` writer - True RNG clock enable during CPU2 CSleep and CStop mode."]
pub type RNGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `SRAM1SMEN` reader - SRAM1 interface clock enable during CPU2 CSleep mode."]
pub type SRAM1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1SMEN` writer - SRAM1 interface clock enable during CPU2 CSleep mode."]
pub type SRAM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `SRAM2SMEN` reader - SRAM2 memory interface clock enable during CPU2 CSleep mode."]
pub type SRAM2SMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2SMEN` writer - SRAM2 memory interface clock enable during CPU2 CSleep mode."]
pub type SRAM2SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
#[doc = "Field `FLASHSMEN` reader - Flash interface clock enable during CPU2 CSleep mode."]
pub type FLASHSMEN_R = crate::BitReader<bool>;
#[doc = "Field `FLASHSMEN` writer - Flash interface clock enable during CPU2 CSleep mode."]
pub type FLASHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB3SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - True RNG clock enable during CPU2 CSleep and CStop mode."]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W<16> {
        PKASMEN_W::new(self)
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W<17> {
        AESSMEN_W::new(self)
    }
    #[doc = "Bit 18 - True RNG clock enable during CPU2 CSleep and CStop mode."]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W<18> {
        RNGSMEN_W::new(self)
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<23> {
        SRAM1SMEN_W::new(self)
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W<24> {
        SRAM2SMEN_W::new(self)
    }
    #[doc = "Bit 25 - Flash interface clock enable during CPU2 CSleep mode."]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<25> {
        FLASHSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB3 peripheral clocks enable in Sleep mode register \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb3smenr](index.html) module"]
pub struct C2AHB3SMENR_SPEC;
impl crate::RegisterSpec for C2AHB3SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb3smenr::R](R) reader structure"]
impl crate::Readable for C2AHB3SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb3smenr::W](W) writer structure"]
impl crate::Writable for C2AHB3SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB3SMENR to value 0x0387_0000"]
impl crate::Resettable for C2AHB3SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0387_0000
    }
}
