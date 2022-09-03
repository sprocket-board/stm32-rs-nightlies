#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1NOER` reader - Tamper 1 no erase"]
pub type TAMP1NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1NOER` writer - Tamper 1 no erase"]
pub type TAMP1NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2NOER` reader - Tamper 2 no erase"]
pub type TAMP2NOER_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2NOER` writer - Tamper 2 no erase"]
pub type TAMP2NOER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
pub type TAMP1MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
pub type TAMP2MSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
pub type TAMP2TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<0> {
        TAMP1NOER_W::new(self)
    }
    #[doc = "Bit 1 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<1> {
        TAMP2NOER_W::new(self)
    }
    #[doc = "Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<16> {
        TAMP1MSK_W::new(self)
    }
    #[doc = "Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set."]
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<17> {
        TAMP2MSK_W::new(self)
    }
    #[doc = "Bit 24 - Active level for tamper 1 input (active mode disabled) If TAMPFLT = 00 Tamper 1 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 1 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<24> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 25 - Active level for tamper 2 input (active mode disabled) If TAMPFLT = 00 Tamper 2 input rising edge and high level triggers a tamper detection event. If TAMPFLT = 00 Tamper 2 input falling edge and low level triggers a tamper detection event."]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<25> {
        TAMP2TRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
