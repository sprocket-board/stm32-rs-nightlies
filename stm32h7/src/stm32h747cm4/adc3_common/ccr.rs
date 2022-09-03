#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DUAL_R = crate::FieldReader<u8, DUAL_A>;
#[doc = "Dual ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DUAL_A {
    #[doc = "0: Independent mode"]
    Independent = 0,
    #[doc = "1: Dual, combined regular simultaneous + injected simultaneous mode"]
    DualRj = 1,
    #[doc = "2: Dual, combined regular simultaneous + alternate trigger mode"]
    DualRa = 2,
    #[doc = "3: Dual, combined interleaved mode + injected simultaneous mode"]
    DualIj = 3,
    #[doc = "5: Dual, injected simultaneous mode only"]
    DualJ = 5,
    #[doc = "6: Dual, regular simultaneous mode only"]
    DualR = 6,
    #[doc = "7: Dual, interleaved mode only"]
    DualI = 7,
    #[doc = "9: Dual, alternate trigger mode only"]
    DualA = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
impl DUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUAL_A> {
        match self.bits {
            0 => Some(DUAL_A::Independent),
            1 => Some(DUAL_A::DualRj),
            2 => Some(DUAL_A::DualRa),
            3 => Some(DUAL_A::DualIj),
            5 => Some(DUAL_A::DualJ),
            6 => Some(DUAL_A::DualR),
            7 => Some(DUAL_A::DualI),
            9 => Some(DUAL_A::DualA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Independent`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL_A::Independent
    }
    #[doc = "Checks if the value of the field is `DualRj`"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL_A::DualRj
    }
    #[doc = "Checks if the value of the field is `DualRa`"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL_A::DualRa
    }
    #[doc = "Checks if the value of the field is `DualIj`"]
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL_A::DualIj
    }
    #[doc = "Checks if the value of the field is `DualJ`"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL_A::DualJ
    }
    #[doc = "Checks if the value of the field is `DualR`"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL_A::DualR
    }
    #[doc = "Checks if the value of the field is `DualI`"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL_A::DualI
    }
    #[doc = "Checks if the value of the field is `DualA`"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL_A::DualA
    }
}
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, DUAL_A, 5, O>;
impl<'a, const O: u8> DUAL_W<'a, O> {
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::Independent)
    }
    #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DualRj)
    }
    #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DualRa)
    }
    #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DualIj)
    }
    #[doc = "Dual, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DualJ)
    }
    #[doc = "Dual, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DualR)
    }
    #[doc = "Dual, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DualI)
    }
    #[doc = "Dual, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DualA)
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DAMDF` reader - Dual ADC Mode Data Format"]
pub type DAMDF_R = crate::FieldReader<u8, DAMDF_A>;
#[doc = "Dual ADC Mode Data Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAMDF_A {
    #[doc = "0: Without data packing, CDR/CDR2 not used"]
    NoPack = 0,
    #[doc = "2: CDR formatted for 32-bit down to 10-bit resolution"]
    Format32to10 = 2,
    #[doc = "3: CDR formatted for 8-bit resolution"]
    Format8 = 3,
}
impl From<DAMDF_A> for u8 {
    #[inline(always)]
    fn from(variant: DAMDF_A) -> Self {
        variant as _
    }
}
impl DAMDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAMDF_A> {
        match self.bits {
            0 => Some(DAMDF_A::NoPack),
            2 => Some(DAMDF_A::Format32to10),
            3 => Some(DAMDF_A::Format8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoPack`"]
    #[inline(always)]
    pub fn is_no_pack(&self) -> bool {
        *self == DAMDF_A::NoPack
    }
    #[doc = "Checks if the value of the field is `Format32to10`"]
    #[inline(always)]
    pub fn is_format32to10(&self) -> bool {
        *self == DAMDF_A::Format32to10
    }
    #[doc = "Checks if the value of the field is `Format8`"]
    #[inline(always)]
    pub fn is_format8(&self) -> bool {
        *self == DAMDF_A::Format8
    }
}
#[doc = "Field `DAMDF` writer - Dual ADC Mode Data Format"]
pub type DAMDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, DAMDF_A, 2, O>;
impl<'a, const O: u8> DAMDF_W<'a, O> {
    #[doc = "Without data packing, CDR/CDR2 not used"]
    #[inline(always)]
    pub fn no_pack(self) -> &'a mut W {
        self.variant(DAMDF_A::NoPack)
    }
    #[doc = "CDR formatted for 32-bit down to 10-bit resolution"]
    #[inline(always)]
    pub fn format32to10(self) -> &'a mut W {
        self.variant(DAMDF_A::Format32to10)
    }
    #[doc = "CDR formatted for 8-bit resolution"]
    #[inline(always)]
    pub fn format8(self) -> &'a mut W {
        self.variant(DAMDF_A::Format8)
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<u8, CKMODE_A>;
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    Asynchronous = 0,
    #[doc = "1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    SyncDiv1 = 1,
    #[doc = "2: Use AHB clock rcc_hclk3 divided by 2"]
    SyncDiv2 = 2,
    #[doc = "3: Use AHB clock rcc_hclk3 divided by 4"]
    SyncDiv4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::Asynchronous,
            1 => CKMODE_A::SyncDiv1,
            2 => CKMODE_A::SyncDiv2,
            3 => CKMODE_A::SyncDiv4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Asynchronous`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE_A::Asynchronous
    }
    #[doc = "Checks if the value of the field is `SyncDiv1`"]
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE_A::SyncDiv1
    }
    #[doc = "Checks if the value of the field is `SyncDiv2`"]
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE_A::SyncDiv2
    }
    #[doc = "Checks if the value of the field is `SyncDiv4`"]
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE_A::SyncDiv4
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, CKMODE_A, 2, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::Asynchronous)
    }
    #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv1)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv2)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv4)
    }
}
#[doc = "Field `PRESC` reader - ADC prescaler"]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "ADC prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: adc_ker_ck_input not divided"]
    Div1 = 0,
    #[doc = "1: adc_ker_ck_input divided by 2"]
    Div2 = 1,
    #[doc = "2: adc_ker_ck_input divided by 4"]
    Div4 = 2,
    #[doc = "3: adc_ker_ck_input divided by 6"]
    Div6 = 3,
    #[doc = "4: adc_ker_ck_input divided by 8"]
    Div8 = 4,
    #[doc = "5: adc_ker_ck_input divided by 10"]
    Div10 = 5,
    #[doc = "6: adc_ker_ck_input divided by 12"]
    Div12 = 6,
    #[doc = "7: adc_ker_ck_input divided by 16"]
    Div16 = 7,
    #[doc = "8: adc_ker_ck_input divided by 32"]
    Div32 = 8,
    #[doc = "9: adc_ker_ck_input divided by 64"]
    Div64 = 9,
    #[doc = "10: adc_ker_ck_input divided by 128"]
    Div128 = 10,
    #[doc = "11: adc_ker_ck_input divided by 256"]
    Div256 = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::Div1),
            1 => Some(PRESC_A::Div2),
            2 => Some(PRESC_A::Div4),
            3 => Some(PRESC_A::Div6),
            4 => Some(PRESC_A::Div8),
            5 => Some(PRESC_A::Div10),
            6 => Some(PRESC_A::Div12),
            7 => Some(PRESC_A::Div16),
            8 => Some(PRESC_A::Div32),
            9 => Some(PRESC_A::Div64),
            10 => Some(PRESC_A::Div128),
            11 => Some(PRESC_A::Div256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::Div256
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, PRESC_A, 4, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "adc_ker_ck_input not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::Div1)
    }
    #[doc = "adc_ker_ck_input divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::Div2)
    }
    #[doc = "adc_ker_ck_input divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::Div4)
    }
    #[doc = "adc_ker_ck_input divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESC_A::Div6)
    }
    #[doc = "adc_ker_ck_input divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::Div8)
    }
    #[doc = "adc_ker_ck_input divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESC_A::Div10)
    }
    #[doc = "adc_ker_ck_input divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESC_A::Div12)
    }
    #[doc = "adc_ker_ck_input divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::Div16)
    }
    #[doc = "adc_ker_ck_input divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::Div32)
    }
    #[doc = "adc_ker_ck_input divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::Div64)
    }
    #[doc = "adc_ker_ck_input divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::Div128)
    }
    #[doc = "adc_ker_ck_input divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::Div256)
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
#[doc = "VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: V_REFINT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_REFINT channel enabled"]
    Enabled = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::Disabled,
            true => VREFEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::Enabled
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VREFEN_A, O>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Disabled)
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Enabled)
    }
}
#[doc = "Field `VSENSEEN` reader - Temperature sensor enable"]
pub type VSENSEEN_R = crate::BitReader<VSENSEEN_A>;
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSENSEEN_A {
    #[doc = "0: Temperature sensor channel disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor channel enabled"]
    Enabled = 1,
}
impl From<VSENSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: VSENSEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VSENSEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSENSEEN_A {
        match self.bits {
            false => VSENSEEN_A::Disabled,
            true => VSENSEEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSEEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSEEN_A::Enabled
    }
}
#[doc = "Field `VSENSEEN` writer - Temperature sensor enable"]
pub type VSENSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VSENSEEN_A, O>;
impl<'a, const O: u8> VSENSEEN_W<'a, O> {
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VSENSEEN_A::Disabled)
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VSENSEEN_A::Enabled)
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATEN_A {
    #[doc = "0: V_BAT channel disabled"]
    Disabled = 0,
    #[doc = "1: V_BAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::Disabled,
            true => VBATEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN_A::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATEN_A, O>;
impl<'a, const O: u8> VBATEN_W<'a, O> {
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Disabled)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Dual ADC Mode Data Format"]
    #[inline(always)]
    pub fn damdf(&self) -> DAMDF_R {
        DAMDF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn vsenseen(&self) -> VSENSEEN_R {
        VSENSEEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    #[doc = "Bits 14:15 - Dual ADC Mode Data Format"]
    #[inline(always)]
    pub fn damdf(&mut self) -> DAMDF_W<14> {
        DAMDF_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W<16> {
        CKMODE_W::new(self)
    }
    #[doc = "Bits 18:21 - ADC prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn vsenseen(&mut self) -> VSENSEEN_W<23> {
        VSENSEEN_W::new(self)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
