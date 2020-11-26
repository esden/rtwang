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

use crate::led_string::*;
use crate::utils::*;

pub struct Player {
    position: i32,
    direction: i32,
    attack_width: i32,
    attacikng: bool,
    attacking_millis: u32,
    attack_duration: u32,
    pub speed: i32,
}

impl Player {
    pub fn new(direction: i32) -> Player {
        Player {
            position: 0,
            direction,
            attack_width: 8,
            attacikng: false,
            attacking_millis: 0,
            attack_duration: 500,
            speed: 0,
        }
    }

    pub fn draw(&self, led_string: &mut LEDString, time: u32) {
        if !self.attacikng {
            led_string[self.position as usize].set_rgb([0, 255, 0]);
        } else {
            self.draw_attack(led_string, time);
        }
    }

    fn draw_attack(&self, led_string: &mut LEDString, time: u32) {
        let mut n = range_map(time - self.attacking_millis, 0, self.attack_duration, 100, 5) as u8;
        for i in (self.position - (self.attack_width / 2) + 1)..(self.position + (self.attack_width / 2)) {
            led_string[i as usize].set_rgb([0, 0, n]);
        }
        if n > 90 {
            n = 255;
            led_string[self.position as usize].set_rgb([255, 255, 255]);
        } else {
            n = 0;
            led_string[self.position as usize].set_rgb([0, 255, 0]);
        }
        led_string[(self.position - (self.attack_width / 2)) as usize].set_rgb([n, n, 255]);
        led_string[(self.position + (self.attack_width / 2)) as usize].set_rgb([n, n, 255]);
    }

    pub fn tick(&mut self, led_string: &LEDString, time: u32) {
        if self.attacikng {
            if self.attacking_millis + self.attack_duration < time {
                self.attacikng = false;
            }
            return;
        }
        let amount = self.speed * self.direction;
        let len = led_string.len() as i32;
        self.position += amount;
        if self.position < 0 {
            self.position = 0
        } else if self.position >= len {
            self.position = len - 1
        }
    }

    pub fn attack(&mut self, time: u32) {
        self.attacking_millis = time;
        self.attacikng = true;
    }
}