#[doc = "Register `ATCR1` reader"]
pub struct R(crate::R<ATCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR1` writer"]
pub struct W(crate::W<ATCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR1_SPEC>;
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
impl From<crate::W<ATCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1AM` reader - TAMP1AM"]
pub type TAMP1AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1AM` writer - TAMP1AM"]
pub type TAMP1AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP2AM` reader - TAMP2AM"]
pub type TAMP2AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2AM` writer - TAMP2AM"]
pub type TAMP2AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP3AM` reader - TAMP3AM"]
pub type TAMP3AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3AM` writer - TAMP3AM"]
pub type TAMP3AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP4AM` reader - TAMP4AM"]
pub type TAMP4AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP4AM` writer - TAMP4AM"]
pub type TAMP4AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP5AM` reader - TAMP5AM"]
pub type TAMP5AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP5AM` writer - TAMP5AM"]
pub type TAMP5AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP6AM` reader - TAMP6AM"]
pub type TAMP6AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP6AM` writer - TAMP6AM"]
pub type TAMP6AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP7AM` reader - TAMP7AM"]
pub type TAMP7AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP7AM` writer - TAMP7AM"]
pub type TAMP7AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `TAMP8AM` reader - TAMP8AM"]
pub type TAMP8AM_R = crate::BitReader<bool>;
#[doc = "Field `TAMP8AM` writer - TAMP8AM"]
pub type TAMP8AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `ATOSEL1` reader - ATOSEL1"]
pub type ATOSEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL1` writer - ATOSEL1"]
pub type ATOSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL2` reader - ATOSEL2"]
pub type ATOSEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL2` writer - ATOSEL2"]
pub type ATOSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL3` reader - ATOSEL3"]
pub type ATOSEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL3` writer - ATOSEL3"]
pub type ATOSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSEL4` reader - ATOSEL4"]
pub type ATOSEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATOSEL4` writer - ATOSEL4"]
pub type ATOSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATCKSEL` reader - ATCKSEL"]
pub type ATCKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATCKSEL` writer - ATCKSEL"]
pub type ATCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATPER` reader - ATPER"]
pub type ATPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATPER` writer - ATPER"]
pub type ATPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ATOSHARE` reader - ATOSHARE"]
pub type ATOSHARE_R = crate::BitReader<bool>;
#[doc = "Field `ATOSHARE` writer - ATOSHARE"]
pub type ATOSHARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
#[doc = "Field `FLTEN` reader - FLTEN"]
pub type FLTEN_R = crate::BitReader<bool>;
#[doc = "Field `FLTEN` writer - FLTEN"]
pub type FLTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&self) -> TAMP1AM_R {
        TAMP1AM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&self) -> TAMP2AM_R {
        TAMP2AM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&self) -> TAMP3AM_R {
        TAMP3AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TAMP4AM"]
    #[inline(always)]
    pub fn tamp4am(&self) -> TAMP4AM_R {
        TAMP4AM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TAMP5AM"]
    #[inline(always)]
    pub fn tamp5am(&self) -> TAMP5AM_R {
        TAMP5AM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TAMP6AM"]
    #[inline(always)]
    pub fn tamp6am(&self) -> TAMP6AM_R {
        TAMP6AM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TAMP7AM"]
    #[inline(always)]
    pub fn tamp7am(&self) -> TAMP7AM_R {
        TAMP7AM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TAMP8AM"]
    #[inline(always)]
    pub fn tamp8am(&self) -> TAMP8AM_R {
        TAMP8AM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&self) -> ATOSEL1_R {
        ATOSEL1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&self) -> ATOSEL2_R {
        ATOSEL2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&self) -> ATOSEL3_R {
        ATOSEL3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&self) -> ATOSEL4_R {
        ATOSEL4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&self) -> ATCKSEL_R {
        ATCKSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - ATPER"]
    #[inline(always)]
    pub fn atper(&self) -> ATPER_R {
        ATPER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&self) -> ATOSHARE_R {
        ATOSHARE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&self) -> FLTEN_R {
        FLTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1AM"]
    #[inline(always)]
    pub fn tamp1am(&mut self) -> TAMP1AM_W<0> {
        TAMP1AM_W::new(self)
    }
    #[doc = "Bit 1 - TAMP2AM"]
    #[inline(always)]
    pub fn tamp2am(&mut self) -> TAMP2AM_W<1> {
        TAMP2AM_W::new(self)
    }
    #[doc = "Bit 2 - TAMP3AM"]
    #[inline(always)]
    pub fn tamp3am(&mut self) -> TAMP3AM_W<2> {
        TAMP3AM_W::new(self)
    }
    #[doc = "Bit 3 - TAMP4AM"]
    #[inline(always)]
    pub fn tamp4am(&mut self) -> TAMP4AM_W<3> {
        TAMP4AM_W::new(self)
    }
    #[doc = "Bit 4 - TAMP5AM"]
    #[inline(always)]
    pub fn tamp5am(&mut self) -> TAMP5AM_W<4> {
        TAMP5AM_W::new(self)
    }
    #[doc = "Bit 5 - TAMP6AM"]
    #[inline(always)]
    pub fn tamp6am(&mut self) -> TAMP6AM_W<5> {
        TAMP6AM_W::new(self)
    }
    #[doc = "Bit 6 - TAMP7AM"]
    #[inline(always)]
    pub fn tamp7am(&mut self) -> TAMP7AM_W<6> {
        TAMP7AM_W::new(self)
    }
    #[doc = "Bit 7 - TAMP8AM"]
    #[inline(always)]
    pub fn tamp8am(&mut self) -> TAMP8AM_W<7> {
        TAMP8AM_W::new(self)
    }
    #[doc = "Bits 8:9 - ATOSEL1"]
    #[inline(always)]
    pub fn atosel1(&mut self) -> ATOSEL1_W<8> {
        ATOSEL1_W::new(self)
    }
    #[doc = "Bits 10:11 - ATOSEL2"]
    #[inline(always)]
    pub fn atosel2(&mut self) -> ATOSEL2_W<10> {
        ATOSEL2_W::new(self)
    }
    #[doc = "Bits 12:13 - ATOSEL3"]
    #[inline(always)]
    pub fn atosel3(&mut self) -> ATOSEL3_W<12> {
        ATOSEL3_W::new(self)
    }
    #[doc = "Bits 14:15 - ATOSEL4"]
    #[inline(always)]
    pub fn atosel4(&mut self) -> ATOSEL4_W<14> {
        ATOSEL4_W::new(self)
    }
    #[doc = "Bits 16:17 - ATCKSEL"]
    #[inline(always)]
    pub fn atcksel(&mut self) -> ATCKSEL_W<16> {
        ATCKSEL_W::new(self)
    }
    #[doc = "Bits 24:25 - ATPER"]
    #[inline(always)]
    pub fn atper(&mut self) -> ATPER_W<24> {
        ATPER_W::new(self)
    }
    #[doc = "Bit 30 - ATOSHARE"]
    #[inline(always)]
    pub fn atoshare(&mut self) -> ATOSHARE_W<30> {
        ATOSHARE_W::new(self)
    }
    #[doc = "Bit 31 - FLTEN"]
    #[inline(always)]
    pub fn flten(&mut self) -> FLTEN_W<31> {
        FLTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP active tamper control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr1](index.html) module"]
pub struct ATCR1_SPEC;
impl crate::RegisterSpec for ATCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr1::R](R) reader structure"]
impl crate::Readable for ATCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr1::W](W) writer structure"]
impl crate::Writable for ATCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCR1 to value 0x0007_0000"]
impl crate::Resettable for ATCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_0000
    }
}
