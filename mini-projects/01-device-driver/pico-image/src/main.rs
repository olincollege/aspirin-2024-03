//! Pico Game Controller with USB Interface

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use embedded_hal::digital::{InputPin, OutputPin, PinState};
use rp_pico::hal;
use rp_pico::hal::Sio;
use rp_pico::Pins;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

// Misc
use core::fmt::Write;
use heapless::String;

// Enum for Device State Machine
#[repr(i32)]
#[derive(Clone, Copy)]
enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

// Heartbeat LED Delay
const LED_TOGGLE_DELAY: u64 = 500_000;
const SERIAL_TX_PERIOD: u64 = 100_000;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().expect("Peripherals should be accessible on entry");

    // Initialize LED Pins
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LEDs
    let mut heartbeat_led = pins.led.into_push_pull_output();
    let mut red_led = pins.gpio18.into_push_pull_output();
    let mut yellow_led = pins.gpio17.into_push_pull_output();
    let mut green_led = pins.gpio16.into_push_pull_output();

    // Buttons
    let mut left_button = pins.gpio13.into_pull_down_input();
    let mut top_button = pins.gpio15.into_pull_down_input();
    let mut bottom_button = pins.gpio14.into_pull_down_input();
    let mut right_button = pins.gpio12.into_pull_down_input();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .expect("Clocks should never fail to initialize");

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Rustbox Studio")
            .product("Rusty Ports")
            .serial_number("RustboxController0")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    // Initialize state variables
    let mut device_state = DeviceState::PendingInit;
    let mut led_state = PinState::High;
    let mut last_led_transition_time = timer.get_counter().ticks();
    heartbeat_led
        .set_high()
        .expect("GPIOs should never fail to change stated");
    let mut rsg_led_states = (PinState::Low, PinState::Low, PinState::Low);
    let mut in_debug_mode = false;
    let mut last_button_state_transmission_time: u64 = 0;

    loop {
        let current_time = timer.get_counter().ticks();

        // Toggle heartbeat LED
        if current_time - last_led_transition_time > LED_TOGGLE_DELAY {
            led_state = match led_state {
                PinState::High => PinState::Low,
                PinState::Low => PinState::High,
            };
            heartbeat_led
                .set_state(led_state)
                .expect("GPIOs should never fail to change stated");
            last_led_transition_time = timer.get_counter().ticks();
        }

        // Debug print the current state, along with the RSG LED States
        if in_debug_mode {
            let mut debug_text: String<60> = String::new();
            writeln!(
                debug_text,
                "st: {}, r: {}, s: {}, g: {}",
                device_state as u32,
                (rsg_led_states.0 == PinState::High) as u8,
                (rsg_led_states.1 == PinState::High) as u8,
                (rsg_led_states.2 == PinState::High) as u8
            )
            .expect("String buffer has been conservatively allocated to fit full payload");

            // Only possible error is when USB Buffer is full, which just means
            // that this specific message will be dropped.
            let _ = serial.write(debug_text.as_bytes());
        }

        // Check for new data
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            match serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    let serial_cmd = match core::str::from_utf8(&buf[..count]) {
                        Ok(s) => s.trim(), // If valid UTF-8, convert to String
                        Err(_) => {
                            "" // Returning an empty string for invalid UTF-8
                        }
                    };

                    if serial_cmd.contains("enable debug") {
                        in_debug_mode = true;
                    }
                    if serial_cmd.contains("disable debug") {
                        in_debug_mode = false;
                    }

                    // State Machine Updates only based on Serial Input
                    match device_state {
                        DeviceState::PendingInit => {
                            if serial_cmd.contains("init controller") {
                                device_state = DeviceState::PendingStart;
                            }
                        }
                        DeviceState::PendingStart => {
                            if serial_cmd.contains("set ready led") {
                                rsg_led_states = (PinState::High, PinState::Low, PinState::Low);
                            } else if serial_cmd.contains("set set led") {
                                rsg_led_states = (PinState::Low, PinState::High, PinState::Low);
                            } else if serial_cmd.contains("set go led") {
                                rsg_led_states = (PinState::Low, PinState::Low, PinState::High);
                            } else if serial_cmd.contains("set all leds") {
                                rsg_led_states = (PinState::High, PinState::High, PinState::High);
                            } else if serial_cmd.contains("clear all leds") {
                                rsg_led_states = (PinState::Low, PinState::Low, PinState::Low);
                            }

                            if serial_cmd.contains("start controller") {
                                device_state = DeviceState::Running;
                            }
                        }
                        DeviceState::Running => {
                            if serial_cmd.contains("stop controller") {
                                device_state = DeviceState::Complete;
                            }
                        }
                        DeviceState::Complete => {
                            if serial_cmd.contains("reset") {
                                device_state = DeviceState::PendingInit;
                            }
                            if serial_cmd.contains("restart") {
                                device_state = DeviceState::PendingStart;
                            }
                            if serial_cmd.contains("start controller") {
                                device_state = DeviceState::Running;
                            }
                        }
                    }
                }
            }
        }
        
        // Actions based on the current state
        match device_state {
            DeviceState::PendingInit => {}
            DeviceState::PendingStart => {
                red_led
                    .set_state(rsg_led_states.0)
                    .expect("GPIOs should never fail to change stated");
                yellow_led
                    .set_state(rsg_led_states.1)
                    .expect("GPIOs should never fail to change stated");
                green_led
                    .set_state(rsg_led_states.2)
                    .expect("GPIOs should never fail to change stated");
            }
            DeviceState::Running => {
                if current_time - last_button_state_transmission_time > SERIAL_TX_PERIOD {
                    last_button_state_transmission_time = current_time;
                    let mut button_text: String<20> = String::new();
                    let button_data = (left_button
                        .is_high()
                        .expect("GPIOs should never fail to read state")
                        as u8)
                        + ((top_button
                            .is_high()
                            .expect("GPIOs should never fail to read state")
                            as u8)
                            << 1)
                        + ((bottom_button
                            .is_high()
                            .expect("GPIOs should never fail to read state")
                            as u8)
                            << 2)
                        + ((right_button
                            .is_high()
                            .expect("GPIOs should never fail to read state")
                            as u8)
                            << 3);
                    writeln!(button_text, "{button_data}")
                        .expect("GPIOs should never fail to read state");

                    // Only possible error is when USB Buffer is full, which just means
                    // that this specific message will be dropped.
                    let _ = serial.write(button_text.as_bytes());
                    let _ = serial.flush();
                }
            }
            DeviceState::Complete => {
                red_led
                    .set_low()
                    .expect("GPIOs should never fail to change stated");
                yellow_led
                    .set_low()
                    .expect("GPIOs should never fail to change stated");
                green_led
                    .set_low()
                    .expect("GPIOs should never fail to change stated");
                rsg_led_states = (PinState::Low, PinState::Low, PinState::Low);
            }
        }
    }
}

// End of file
