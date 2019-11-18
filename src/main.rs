#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use core::fmt::Write;
use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    serial::{self, Config, Serial},
    timer::{ Timer, Event, CountDownTimer },
    gpio::{gpiob::PB12, Output, PushPull },
};

#[rtfm::app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {

    struct Resources {
        led: PB12<Output<PushPull>>,
        timer: CountDownTimer<pac::TIM1>,
        rx2: serial::Rx<pac::USART2>,
        tx2: serial::Tx<pac::USART2>,
        tx3: serial::Tx<pac::USART3>
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let _device = cx.device;

        let mut flash = _device.FLASH.constrain();
        let mut rcc = _device.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        // let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(72.mhz()).pclk1(36.mhz()).freeze(&mut flash.acr);

        let mut gpiob = _device.GPIOB.split(&mut rcc.apb2);

        let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

        led.set_low().unwrap();


        let mut afio = _device.AFIO.constrain(&mut rcc.apb2);
        let mut gpioa = _device.GPIOA.split(&mut rcc.apb2);

        //USART2_TX PA2
        //USART2_RX PA3
        let uart2_tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let uart2_rx = gpioa.pa3;

        let mut serial2 = Serial::usart2(
            _device.USART2,
            (uart2_tx, uart2_rx),
            &mut afio.mapr,
            Config::default().baudrate(115200.bps()),
            clocks,
            &mut rcc.apb1,
        );

        serial2.listen(serial::Event::Rxne);

        let (mut tx2, rx2) = serial2.split();

        writeln!(tx2, "let's start {} Example!", "RTFM").unwrap();

        //USART3_TX PB10
        //USART3_RX PB11
        let uart3_tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
        let uart3_rx = gpiob.pb11;

        let serial3 = Serial::usart3(
            _device.USART3,
            (uart3_tx, uart3_rx),
            &mut afio.mapr,
            Config::default().baudrate(115200.bps()),
            clocks,
            &mut rcc.apb1,
        );

        let (tx3, _) = serial3.split();

        // Configure the syst timer to trigger an update every second and enables interrupt
        let mut timer = Timer::tim1(_device.TIM1, &clocks, &mut rcc.apb2)
            .start_count_down(3.hz());
        timer.listen(Event::Update);


        // Return the initialised resources.
        init::LateResources {
            led: led,
            timer: timer,
            rx2: rx2,
            tx2: tx2,
            tx3: tx3
        }
    }

    #[task(binds = USART2, resources = [rx2, tx2], priority = 2)]
    fn usart2(cx: usart2::Context) {

        let usart2::Resources {
            rx2,
            tx2
        } = cx.resources;

        match rx2.read() {
            Ok(b) => {
                tx2.write(b).unwrap();
            }
            Err(_e) => {
                writeln!(tx2, "Serial Error: {:?}", _e).unwrap();
            }
        }

    }

    #[task(binds = TIM1_UP, resources = [led, timer, tx3])]
    fn tim1_up(cx: tim1_up::Context) {
        static mut STATE: bool = false;
        static mut COUNT: u32 = 0;

        // Clear the interrupt flag.
        cx.resources.timer.clear_update_interrupt_flag();

        if *STATE {
            cx.resources.led.set_low().unwrap();
            *STATE = false;
        } else {
            cx.resources.led.set_high().unwrap();
            *STATE = true;
        }

        writeln!(cx.resources.tx3, "{}", COUNT).unwrap();
        *COUNT += 1;
    }
};
