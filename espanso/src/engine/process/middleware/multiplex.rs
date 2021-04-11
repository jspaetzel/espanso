/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use log::{debug, error};

use super::super::Middleware;
use crate::engine::{event::{Event, text::{TextInjectRequest, TextInjectMode}, matches::MatchSelectedEvent}, process::{MatchFilter, MatchSelector, Multiplexer}};

pub struct MultiplexMiddleware<'a> {
  multiplexer: &'a dyn Multiplexer,
}

impl<'a> MultiplexMiddleware<'a> {
  pub fn new(multiplexer: &'a dyn Multiplexer) -> Self {
    Self { multiplexer }
  }
}

impl<'a> Middleware for MultiplexMiddleware<'a> {
  fn next(&self, event: Event, _: &mut dyn FnMut(Event)) -> Event {
    if let Event::MatchSelected(m_event) = event {
      return match self.multiplexer.convert(m_event.chosen.id, m_event.chosen.trigger, m_event.chosen.args) {
        Some(event) => event,
        None => {
          error!("match multiplexing failed");
          Event::NOOP
        },
      }
    }

    event
  }
}

// TODO: test
