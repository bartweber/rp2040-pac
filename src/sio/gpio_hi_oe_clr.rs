#[doc = "Register `GPIO_HI_OE_CLR` writer"]
pub type W = crate::W<GPIO_HI_OE_CLR_SPEC>;
#[doc = "Field `GPIO_HI_OE_CLR` writer - Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &amp;= ~wdata`"]
pub type GPIO_HI_OE_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl W {
    #[doc = "Bits 0:5 - Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &amp;= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hi_oe_clr(&mut self) -> GPIO_HI_OE_CLR_W<GPIO_HI_OE_CLR_SPEC> {
        GPIO_HI_OE_CLR_W::new(self, 0)
    }
}
#[doc = "QSPI output enable clear  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_oe_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_OE_CLR_SPEC;
impl crate::RegisterSpec for GPIO_HI_OE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_hi_oe_clr::W`](W) writer structure"]
impl crate::Writable for GPIO_HI_OE_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OE_CLR to value 0"]
impl crate::Resettable for GPIO_HI_OE_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
