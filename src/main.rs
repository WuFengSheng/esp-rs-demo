#![no_std]
#![no_main]
#![allow(dead_code)]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::{Peripherals, I2C0},
    Blocking,
    prelude::*,
    system::SystemControl,
    gpio::{Io, Level, Output},
    i2c::I2C
};

const I2C_ADDR: u8 = 0x27; // LCD1602的I2C地址

// LCD 指令
const LCD_CMD_CLEAR: u8 = 0x01;
const LCD_CMD_HOME: u8 = 0x02;
const LCD_CMD_ENTRY_MODE: u8 = 0x04;
const LCD_CMD_DISPLAY_CONTROL: u8 = 0x08;
const LCD_CMD_FUNCTION_SET: u8 = 0x20;
const LCD_CMD_SET_DDRAM_ADDR: u8 = 0x80;

// LCD 控制位
const LCD_BACKLIGHT: u8 = 0x08;
const ENABLE: u8 = 0x04;
const RW_WRITE: u8 = 0x00;
const RS_DATA: u8 = 0x01;
const RS_COMMAND: u8 = 0x00;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    // Set GPIO0 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio0, Level::High);

    // 初始化 I2C
    let mut i2c: I2C<I2C0, Blocking> = I2C::new(peripherals.I2C0, io.pins.gpio6, io.pins.gpio5, 400.kHz(), &clocks);
    log::info!("The type of i2c is: {}", core::any::type_name_of_val(&i2c));

    // 初始化 LCD
    lcd_init(&mut i2c);
    lcd_write_string(&mut i2c, "Hello, World!");

    loop {
        log::info!("Hello world! \n");
        led.toggle();
        delay.delay(500.millis());
    }
}

fn lcd_init(i2c: &mut I2C<I2C0, Blocking>) {
    // 设置 4-bit 模式
    lcd_command(i2c, 0x33); // 初始化指令
    lcd_command(i2c, 0x32); // 设置4-bit模式
    lcd_command(i2c, LCD_CMD_FUNCTION_SET | 0x08); // 2行显示
    lcd_command(i2c, LCD_CMD_DISPLAY_CONTROL | 0x0F); // 打开显示，有光标，会闪烁
    lcd_command(i2c, LCD_CMD_CLEAR); // 清屏
    lcd_command(i2c, LCD_CMD_ENTRY_MODE | 0x02); // 设置光标移动方向
}

fn lcd_command(i2c: &mut I2C<I2C0, Blocking>, command: u8) {
    lcd_write(i2c, command, RS_COMMAND);
}

fn lcd_data(i2c: &mut I2C<I2C0, Blocking>, data: u8) {
    lcd_write(i2c, data, RS_DATA);
}

fn lcd_write(i2c: &mut I2C<I2C0, Blocking>, data: u8, mode: u8) {
    let high_nibble = data & 0xF0;
    let low_nibble = (data << 4) & 0xF0;
    lcd_send_nibble(i2c, high_nibble | mode);
    lcd_send_nibble(i2c, low_nibble | mode);
}

fn lcd_send_nibble(i2c: &mut I2C<I2C0, Blocking>, nibble: u8) {
    // 发送高4位
    let data = nibble | LCD_BACKLIGHT;
    let _ = i2c.write(I2C_ADDR, &[data | ENABLE]);
    let _ = i2c.write(I2C_ADDR, &[data & !ENABLE]);
}

fn lcd_write_string(i2c: &mut I2C<I2C0, Blocking>, s: &str) {
    for c in s.chars() {
        lcd_data(i2c, c as u8);
    }
}
