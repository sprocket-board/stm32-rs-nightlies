#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DU` reader - DU"]
pub type DU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU` writer - DU"]
pub type DU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT` reader - DT"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT` writer - DT"]
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MU` reader - MU"]
pub type MU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MU` writer - MU"]
pub type MU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MT` reader - MT"]
pub type MT_R = crate::BitReader<bool>;
#[doc = "Field `MT` writer - MT"]
pub type MT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
#[doc = "Field `WDU` reader - WDU"]
pub type WDU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDU` writer - WDU"]
pub type WDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 3, O>;
#[doc = "Field `YU` reader - YU"]
pub type YU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YU` writer - YU"]
pub type YU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
#[doc = "Field `YT` reader - YT"]
pub type YT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YT` writer - YT"]
pub type YT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - YU"]
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - YT"]
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DU"]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<0> {
        DU_W::new(self)
    }
    #[doc = "Bits 4:5 - DT"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<4> {
        DT_W::new(self)
    }
    #[doc = "Bits 8:11 - MU"]
    #[inline(always)]
    pub fn mu(&mut self) -> MU_W<8> {
        MU_W::new(self)
    }
    #[doc = "Bit 12 - MT"]
    #[inline(always)]
    pub fn mt(&mut self) -> MT_W<12> {
        MT_W::new(self)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W<13> {
        WDU_W::new(self)
    }
    #[doc = "Bits 16:19 - YU"]
    #[inline(always)]
    pub fn yu(&mut self) -> YU_W<16> {
        YU_W::new(self)
    }
    #[doc = "Bits 20:23 - YT"]
    #[inline(always)]
    pub fn yt(&mut self) -> YT_W<20> {
        YT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0x2101"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2101
    }
}
