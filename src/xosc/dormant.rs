#[doc = "Register `DORMANT` reader"]
pub type R = crate::R<DORMANT_SPEC>;
#[doc = "Register `DORMANT` writer"]
pub type W = crate::W<DORMANT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DORMANT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Crystal Oscillator pause control  
 This is used to save power by pausing the XOSC  
 On power-up this field is initialised to WAKE  
 An invalid write will also select WAKE  
 WARNING: stop the PLLs before selecting dormant mode  
 WARNING: setup the irq before selecting dormant mode  

You can [`read`](crate::generic::Reg::read) this register and get [`dormant::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dormant::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DORMANT_SPEC;
impl crate::RegisterSpec for DORMANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dormant::R`](R) reader structure"]
impl crate::Readable for DORMANT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dormant::W`](W) writer structure"]
impl crate::Writable for DORMANT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DORMANT to value 0"]
impl crate::Resettable for DORMANT_SPEC {
    const RESET_VALUE: u32 = 0;
}
