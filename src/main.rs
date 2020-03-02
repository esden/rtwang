/*
 * Copyright (c) 2020, Piotr Esden-Tempski <piotr@esden.net>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

extern crate piston_window;
extern crate sdl2_window;

use piston_window::*;
use sdl2_window::Sdl2Window;

mod led_string;
use led_string::*;

mod screensaver;
//mod utils;

const LED_SIZE: u32 = 12;
const LED_MARGIN: u32 = 1;
const LED_COLOR: [u8; 3] = [0; 3];
const LED_STRING_LENGTH: u32 = 144;
const LED_STRING_STATUS: u32 = 13;

fn main() {

    // Create a window for our simulated LEDs
	let window_dimensions = [((LED_SIZE + LED_MARGIN) * LED_STRING_LENGTH) + LED_MARGIN, (LED_SIZE + (LED_MARGIN * 2)) + LED_STRING_STATUS + LED_MARGIN];
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Rusty Spring aka rTWANG!", Size::from(window_dimensions))
        .exit_on_esc(true)
        .resizable(false)
        .graphics_api(OpenGL::V3_2)
        .fullscreen(false)
        .build()
        .unwrap();
    // Try to get as close as possible to 60fps
    window.set_ups(60);

    println!("dim {:?}", window_dimensions);

    // Game init
    let mut led_string = LEDString::new(LED_COLOR, LED_STRING_LENGTH);

    // Game loop
    let mut red: u8 = 100;
    let mut frames = 0;
    let mut passed = 0.0;
    let mut ftime = 0.0;
    let mut time: u32; // time in msec
    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([0.33; 4], graphics);
                for i in 0..led_string.len() {
                    let led = &led_string[i];
                    // convert to f32 and apply inverse gamma to match LEDs
                    let r = (led.r as f32 / 255.0).powf(1.0/2.2);
                    let g = (led.g as f32 / 255.0).powf(1.0/2.2);
                    let b = (led.b as f32 / 255.0).powf(1.0/2.2);
                    rectangle([r, g, b, 1.0],
                              [1.0 + ((LED_SIZE + LED_MARGIN) * (i as u32)) as f64, LED_MARGIN as f64, LED_SIZE as f64, LED_SIZE as f64],
	                          context.transform,
	                          graphics);
	           }
            });
            frames += 1;
        }

        if let Some(u) = event.update_args() {
            red = red.wrapping_add(1);

            passed += u.dt;
            ftime += u.dt;
            time = (ftime * 1_000.0).round() as u32;

            if passed > 1.0 {
                let fps = (frames as f64) / passed;
                println!("FPS: {:.2} TIM: {}", fps, time);
                frames = 0;
                passed = 0.0;
            }

            screensaver::tick(&mut led_string, time);
        }
    }
}
