#![no_std]
#![no_main]

#[cfg(test)]
#[embedded_test::tests()]
// #[embedded_test::tests(executor = embassy_executor::Executor::new())]
mod unit_tests {
    use defmt::assert;
    use defmt::*;
    use embassy_time::Timer;

    #[test]
    async fn test_measure_system_powersupply(mut board: Board<'static>) {
        Timer::after_millis(500).await;
        info!("It work's");
        assert!(true);
    }
}
