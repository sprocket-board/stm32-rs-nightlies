#[doc = "Register `CCMR2_Input` reader"]
pub struct R(crate::R<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR2_Input` writer"]
pub struct W(crate::W<CCMR2_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_INPUT_SPEC>;
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
impl From<crate::W<CCMR2_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection"]
pub type CC3S_R = crate::FieldReader<u8, CC3S_A>;
#[doc = "Capture/Compare 3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC3S_A {
    #[doc = "0: CCx channel is configured as output"]
    Output = 0,
    #[doc = "1: CCx channel is configured as input, ICx is mapped on TI1"]
    Ti1 = 1,
    #[doc = "2: CCx channel is configured as input, ICx is mapped on TI2"]
    Ti2 = 2,
    #[doc = "3: CCx channel is configured as input, ICx is mapped on TRC"]
    Trc = 3,
}
impl From<CC3S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC3S_A) -> Self {
        variant as _
    }
}
impl CC3S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC3S_A {
        match self.bits {
            0 => CC3S_A::Output,
            1 => CC3S_A::Ti1,
            2 => CC3S_A::Ti2,
            3 => CC3S_A::Trc,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CC3S_A::Output
    }
    #[doc = "Checks if the value of the field is `Ti1`"]
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC3S_A::Ti1
    }
    #[doc = "Checks if the value of the field is `Ti2`"]
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC3S_A::Ti2
    }
    #[doc = "Checks if the value of the field is `Trc`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S_A::Trc
    }
}
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection"]
pub type CC3S_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, CC3S_A, 2, O>;
impl<'a, const O: u8> CC3S_W<'a, O> {
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(CC3S_A::Output)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI1"]
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC3S_A::Ti1)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TI2"]
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC3S_A::Ti2)
    }
    #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3S_A::Trc)
    }
}
#[doc = "Field `IC3PSC` reader - Input capture 3 prescaler"]
pub type IC3PSC_R = crate::FieldReader<u8, IC3PSC_A>;
#[doc = "Input capture 3 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC3PSC_A {
    #[doc = "0: CCx channel is configured as output"]
    Output = 0,
    #[doc = "1: Capture is done once every 2 events"]
    Capture2 = 1,
    #[doc = "2: Capture is done once every 4 events"]
    Capture4 = 2,
    #[doc = "3: Capture is done once every 8 events"]
    Capture8 = 3,
}
impl From<IC3PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC3PSC_A) -> Self {
        variant as _
    }
}
impl IC3PSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC3PSC_A {
        match self.bits {
            0 => IC3PSC_A::Output,
            1 => IC3PSC_A::Capture2,
            2 => IC3PSC_A::Capture4,
            3 => IC3PSC_A::Capture8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Output`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == IC3PSC_A::Output
    }
    #[doc = "Checks if the value of the field is `Capture2`"]
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == IC3PSC_A::Capture2
    }
    #[doc = "Checks if the value of the field is `Capture4`"]
    #[inline(always)]
    pub fn is_capture4(&self) -> bool {
        *self == IC3PSC_A::Capture4
    }
    #[doc = "Checks if the value of the field is `Capture8`"]
    #[inline(always)]
    pub fn is_capture8(&self) -> bool {
        *self == IC3PSC_A::Capture8
    }
}
#[doc = "Field `IC3PSC` writer - Input capture 3 prescaler"]
pub type IC3PSC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, IC3PSC_A, 2, O>;
impl<'a, const O: u8> IC3PSC_W<'a, O> {
    #[doc = "CCx channel is configured as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(IC3PSC_A::Output)
    }
    #[doc = "Capture is done once every 2 events"]
    #[inline(always)]
    pub fn capture2(self) -> &'a mut W {
        self.variant(IC3PSC_A::Capture2)
    }
    #[doc = "Capture is done once every 4 events"]
    #[inline(always)]
    pub fn capture4(self) -> &'a mut W {
        self.variant(IC3PSC_A::Capture4)
    }
    #[doc = "Capture is done once every 8 events"]
    #[inline(always)]
    pub fn capture8(self) -> &'a mut W {
        self.variant(IC3PSC_A::Capture8)
    }
}
#[doc = "Field `IC3F` reader - Input capture 3 filter"]
pub type IC3F_R = crate::FieldReader<u8, IC3F_A>;
#[doc = "Input capture 3 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC3F_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NoFilter = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FckIntN2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FckIntN4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FckIntN8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FdtsDiv2N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FdtsDiv2N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FdtsDiv4N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FdtsDiv4N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FdtsDiv8N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FdtsDiv8N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FdtsDiv16N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FdtsDiv16N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FdtsDiv16N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FdtsDiv32N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FdtsDiv32N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FdtsDiv32N8 = 15,
}
impl From<IC3F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC3F_A) -> Self {
        variant as _
    }
}
impl IC3F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC3F_A {
        match self.bits {
            0 => IC3F_A::NoFilter,
            1 => IC3F_A::FckIntN2,
            2 => IC3F_A::FckIntN4,
            3 => IC3F_A::FckIntN8,
            4 => IC3F_A::FdtsDiv2N6,
            5 => IC3F_A::FdtsDiv2N8,
            6 => IC3F_A::FdtsDiv4N6,
            7 => IC3F_A::FdtsDiv4N8,
            8 => IC3F_A::FdtsDiv8N6,
            9 => IC3F_A::FdtsDiv8N8,
            10 => IC3F_A::FdtsDiv16N5,
            11 => IC3F_A::FdtsDiv16N6,
            12 => IC3F_A::FdtsDiv16N8,
            13 => IC3F_A::FdtsDiv32N5,
            14 => IC3F_A::FdtsDiv32N6,
            15 => IC3F_A::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoFilter`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC3F_A::NoFilter
    }
    #[doc = "Checks if the value of the field is `FckIntN2`"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC3F_A::FckIntN2
    }
    #[doc = "Checks if the value of the field is `FckIntN4`"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC3F_A::FckIntN4
    }
    #[doc = "Checks if the value of the field is `FckIntN8`"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC3F_A::FckIntN8
    }
    #[doc = "Checks if the value of the field is `FdtsDiv2N6`"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC3F_A::FdtsDiv2N6
    }
    #[doc = "Checks if the value of the field is `FdtsDiv2N8`"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC3F_A::FdtsDiv2N8
    }
    #[doc = "Checks if the value of the field is `FdtsDiv4N6`"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC3F_A::FdtsDiv4N6
    }
    #[doc = "Checks if the value of the field is `FdtsDiv4N8`"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC3F_A::FdtsDiv4N8
    }
    #[doc = "Checks if the value of the field is `FdtsDiv8N6`"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC3F_A::FdtsDiv8N6
    }
    #[doc = "Checks if the value of the field is `FdtsDiv8N8`"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC3F_A::FdtsDiv8N8
    }
    #[doc = "Checks if the value of the field is `FdtsDiv16N5`"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC3F_A::FdtsDiv16N5
    }
    #[doc = "Checks if the value of the field is `FdtsDiv16N6`"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC3F_A::FdtsDiv16N6
    }
    #[doc = "Checks if the value of the field is `FdtsDiv16N8`"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC3F_A::FdtsDiv16N8
    }
    #[doc = "Checks if the value of the field is `FdtsDiv32N5`"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC3F_A::FdtsDiv32N5
    }
    #[doc = "Checks if the value of the field is `FdtsDiv32N6`"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC3F_A::FdtsDiv32N6
    }
    #[doc = "Checks if the value of the field is `FdtsDiv32N8`"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC3F_A::FdtsDiv32N8
    }
}
#[doc = "Field `IC3F` writer - Input capture 3 filter"]
pub type IC3F_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCMR2_INPUT_SPEC, u8, IC3F_A, 4, O>;
impl<'a, const O: u8> IC3F_W<'a, O> {
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC3F_A::NoFilter)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC3F_A::FckIntN2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC3F_A::FckIntN4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FckIntN8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv2N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv2N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv4N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv4N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv8N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv8N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv16N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv16N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv16N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv32N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv32N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC3F_A::FdtsDiv32N8)
    }
}
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub use CC3S_R as CC4S_R;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub use CC3S_W as CC4S_W;
#[doc = "Field `IC4F` reader - Input capture 4 filter"]
pub use IC3F_R as IC4F_R;
#[doc = "Field `IC4F` writer - Input capture 4 filter"]
pub use IC3F_W as IC4F_W;
#[doc = "Field `IC4PSC` reader - Input capture 4 prescaler"]
pub use IC3PSC_R as IC4PSC_R;
#[doc = "Field `IC4PSC` writer - Input capture 4 prescaler"]
pub use IC3PSC_W as IC4PSC_W;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<2> {
        IC3PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<4> {
        IC3F_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<10> {
        IC4PSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<12> {
        IC4F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_input](index.html) module"]
pub struct CCMR2_INPUT_SPEC;
impl crate::RegisterSpec for CCMR2_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr2_input::R](R) reader structure"]
impl crate::Readable for CCMR2_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr2_input::W](W) writer structure"]
impl crate::Writable for CCMR2_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR2_Input to value 0"]
impl crate::Resettable for CCMR2_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
