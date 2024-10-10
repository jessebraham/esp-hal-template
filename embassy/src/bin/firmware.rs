#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{config::WatchdogStatus, prelude::*, rtc_cntl::Rtc, timer::timg::TimerGroup};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.watchdog.rwdt = WatchdogStatus::Enabled(2.secs());
        config
    });
    info!("RWDT watchdog enabled!");

    // Initialize the SYSTIMER peripheral, and then Embassy:
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;

    // Periodically feed the RWDT watchdog timer when our tasks are not running:
    let mut rtc = Rtc::new(peripherals.LPWR);
    loop {
        rtc.rwdt.feed();
        Timer::after(Duration::from_secs(1)).await;
    }
}
