//! Complete the two TODOs in this file.
use embassy_stm32::{Peripherals, can::Can, usart::BufferedUart};
use embassy_stm32::gpio::{Level, Output, Speed, Flex};
use embassy_stm32::usart::{Config as UartConfig};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use crate::swd::Swd;

// 为 TODO 2 绑定 UART7 的硬件中断
bind_interrupts!(struct Irqs {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

pub struct Ports {
    pub can: Can<'static>,
    pub uart: Option<BufferedUart<'static>>,
}

pub fn configure(p: Peripherals) -> (crate::board::Usb, Ports, Option<Swd>) {
    let usb = crate::board::usb(p.USB_OTG_HS, p.PA12, p.PA11);

    // TODO 1: Pick two exposed 3.3 V GPIOs and pass them to Swd::new.
    // Use Output for SWCLK (initially low), and Flex for bidirectional SWDIO.
    // 方案：选用开发板外露舵机(PWM)接口中的 PA00 (SWCLK) 和 PA02 (SWDIO)。
    let swclk = Output::new(p.PA00, Level::Low, Speed::Low);
    let swdio = Flex::new(p.PA02);
    let swd = Some(Swd::new(swclk, swdio));

    // TODO 2: Configure a buffered UART at 115200 baud, 8N1.
    // Identify the peripheral, RX/TX pins and interrupt from the schematic.
    // Bind the interrupt and provide static RX and TX buffers.
    // 方案：选用开发板上独立的 UART7 串口接口（TX=PE08, RX=PE07）。
    
    // 声明静态收发缓冲区
    static mut TX_BUF: [u8; 128] = [0u8; 128];
    static mut RX_BUF: [u8; 128] = [0u8; 128];

    // 配置波特率为 115200，默认即为 8N1
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 115200;

    // 实例化具有中断和静态缓冲区的 BufferedUart
    let uart_driver = unsafe {
        BufferedUart::new(
            p.UART7,
            p.PE07, // RX 引脚
            p.PE08, // TX 引脚
            &mut TX_BUF,
            &mut RX_BUF,
            Irqs,
            uart_config,
        ).unwrap()
    };
    let uart = Some(uart_driver);

    let can = crate::board::can1(p.FDCAN1, p.PD0, p.PD1);

    (usb, Ports { can, uart }, swd)
}
