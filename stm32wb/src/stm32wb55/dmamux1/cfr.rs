#[doc = "Register `CFR` writer"]
pub struct W(crate::W<CFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR_SPEC>;
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
impl From<crate::W<CFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSOF0` writer - Synchronization Clear Overrun Flag 0"]
pub type CSOF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF1` writer - Synchronization Clear Overrun Flag 1"]
pub type CSOF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF2` writer - Synchronization Clear Overrun Flag 2"]
pub type CSOF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF3` writer - Synchronization Clear Overrun Flag 3"]
pub type CSOF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF4` writer - Synchronization Clear Overrun Flag 4"]
pub type CSOF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF5` writer - Synchronization Clear Overrun Flag 5"]
pub type CSOF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF6` writer - Synchronization Clear Overrun Flag 6"]
pub type CSOF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF7` writer - Synchronization Clear Overrun Flag 7"]
pub type CSOF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF8` writer - Synchronization Clear Overrun Flag 8"]
pub type CSOF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF9` writer - Synchronization Clear Overrun Flag 9"]
pub type CSOF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF10` writer - Synchronization Clear Overrun Flag 10"]
pub type CSOF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF11` writer - Synchronization Clear Overrun Flag 11"]
pub type CSOF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF12` writer - Synchronization Clear Overrun Flag 12"]
pub type CSOF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
#[doc = "Field `CSOF13` writer - Synchronization Clear Overrun Flag 13"]
pub type CSOF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Synchronization Clear Overrun Flag 0"]
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF0_W<0> {
        CSOF0_W::new(self)
    }
    #[doc = "Bit 1 - Synchronization Clear Overrun Flag 1"]
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF1_W<1> {
        CSOF1_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization Clear Overrun Flag 2"]
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF2_W<2> {
        CSOF2_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Clear Overrun Flag 3"]
    #[inline(always)]
    pub fn csof3(&mut self) -> CSOF3_W<3> {
        CSOF3_W::new(self)
    }
    #[doc = "Bit 4 - Synchronization Clear Overrun Flag 4"]
    #[inline(always)]
    pub fn csof4(&mut self) -> CSOF4_W<4> {
        CSOF4_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization Clear Overrun Flag 5"]
    #[inline(always)]
    pub fn csof5(&mut self) -> CSOF5_W<5> {
        CSOF5_W::new(self)
    }
    #[doc = "Bit 6 - Synchronization Clear Overrun Flag 6"]
    #[inline(always)]
    pub fn csof6(&mut self) -> CSOF6_W<6> {
        CSOF6_W::new(self)
    }
    #[doc = "Bit 7 - Synchronization Clear Overrun Flag 7"]
    #[inline(always)]
    pub fn csof7(&mut self) -> CSOF7_W<7> {
        CSOF7_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization Clear Overrun Flag 8"]
    #[inline(always)]
    pub fn csof8(&mut self) -> CSOF8_W<8> {
        CSOF8_W::new(self)
    }
    #[doc = "Bit 9 - Synchronization Clear Overrun Flag 9"]
    #[inline(always)]
    pub fn csof9(&mut self) -> CSOF9_W<9> {
        CSOF9_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Clear Overrun Flag 10"]
    #[inline(always)]
    pub fn csof10(&mut self) -> CSOF10_W<10> {
        CSOF10_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization Clear Overrun Flag 11"]
    #[inline(always)]
    pub fn csof11(&mut self) -> CSOF11_W<11> {
        CSOF11_W::new(self)
    }
    #[doc = "Bit 12 - Synchronization Clear Overrun Flag 12"]
    #[inline(always)]
    pub fn csof12(&mut self) -> CSOF12_W<12> {
        CSOF12_W::new(self)
    }
    #[doc = "Bit 13 - Synchronization Clear Overrun Flag 13"]
    #[inline(always)]
    pub fn csof13(&mut self) -> CSOF13_W<13> {
        CSOF13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Clear Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr](index.html) module"]
pub struct CFR_SPEC;
impl crate::RegisterSpec for CFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cfr::W](W) writer structure"]
impl crate::Writable for CFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
