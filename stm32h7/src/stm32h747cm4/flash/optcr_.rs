#[doc = "Register `OPTCR_` reader"]
pub struct R(crate::R<OPTCR__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTCR_` writer"]
pub struct W(crate::W<OPTCR__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR__SPEC>;
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
impl From<crate::W<OPTCR__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPTLOCK` reader - FLASH"]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - FLASH"]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
#[doc = "Field `OPTSTART` reader - Option byte start change option configuration bit"]
pub type OPTSTART_R = crate::BitReader<bool>;
#[doc = "Field `OPTSTART` writer - Option byte start change option configuration bit"]
pub type OPTSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
#[doc = "Field `MER` reader - mass erase request"]
pub type MER_R = crate::BitReader<bool>;
#[doc = "Field `MER` writer - mass erase request"]
pub type MER_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_R = crate::BitReader<bool>;
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit"]
pub type OPTCHANGEERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping option configuration bit"]
pub type SWAP_BANK_R = crate::BitReader<bool>;
#[doc = "Field `SWAP_BANK` writer - Bank swapping option configuration bit"]
pub type SWAP_BANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR__SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&self) -> OPTSTART_R {
        OPTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - mass erase request"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OPTCHANGEERRIE_R {
        OPTCHANGEERRIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SWAP_BANK_R {
        SWAP_BANK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<0> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&mut self) -> OPTSTART_W<1> {
        OPTSTART_W::new(self)
    }
    #[doc = "Bit 4 - mass erase request"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<4> {
        MER_W::new(self)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&mut self) -> OPTCHANGEERRIE_W<30> {
        OPTCHANGEERRIE_W::new(self)
    }
    #[doc = "Bit 31 - Bank swapping option configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SWAP_BANK_W<31> {
        SWAP_BANK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH option control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optcr_](index.html) module"]
pub struct OPTCR__SPEC;
impl crate::RegisterSpec for OPTCR__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optcr_::R](R) reader structure"]
impl crate::Readable for OPTCR__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optcr_::W](W) writer structure"]
impl crate::Writable for OPTCR__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTCR_ to value 0x01"]
impl crate::Resettable for OPTCR__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
