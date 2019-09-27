//! Slide    
//! 
//! Copyright: Benedict R. Gaster
//! 
//! 

extern crate rand;
use rand::Rng;

use std::time::{Duration, Instant};
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::world::*;

use crate::msg;

use crate::ws_server;
use crate::world;

use std::io::prelude::*;

pub trait Slide {
    fn run(&self,
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>);
}

//-----------------------------------------------------------------------------
// SLIDES
//-----------------------------------------------------------------------------

/// Front page of survey presentation
pub struct FrontMatter {
}

impl FrontMatter {
    pub fn new() -> Self {
        FrontMatter {

        }
    }
}

impl Slide for FrontMatter {
    fn run(&self, 
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>) {

        // jump to frontmatter page
        outbound_msg.send(msg::gotoFrontMatter());

        // wait for user to press begin
        loop {
            match inbound_msg.recv() {
                Ok(data) => {
                    if msg::is_begin(data) {
                        return;
                    }
                },
                _ => {},
            }
        }
    }
}

/// Front page of survey presentation
pub struct Consent {
}

impl Consent {
    pub fn new() -> Self {
        Consent {
        }
    }
}

impl Slide for Consent {
    fn run(&self, 
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>) {

        // jump to consent page
        outbound_msg.send(msg::consentID(world.id));
        outbound_msg.send(msg::gotoConsent());

        // wait for user to press begin
        loop {
            match inbound_msg.recv() {
                Ok(data) => {
                    if msg::is_consent(data) {
                        return;
                    }
                },
                _ => {},
            }
        }
    }
}


pub struct Likert {
    material: u32,
    gesture: String,
}

impl Likert {
    pub fn new(material: u32, gesture: String) -> Self {
        Likert {
            material: material,
            gesture: gesture,
        }
    }
}

impl Slide for Likert {
    fn run(&self, 
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>) {

        // set material and gesture
        outbound_msg.send(msg::materialIndex(self.material, msg::likert_num()));
        outbound_msg.send(msg::gestureType(self.gesture.clone()));

        // jump to likert page
        outbound_msg.send(msg::gotoLikert());

        // now wait for Likert response (3 likert messages expected)
        let mut msgs_received = 3;
        loop {
            match inbound_msg.recv() {
                Ok(data) => {
                    match msg::likert(data) {
                        Ok(l) => {
                            world.writeLikert(l);
                            msgs_received = msgs_received - 1;
                        },
                        _ => {}
                    }
                },
                _ => {},
            }

            if msgs_received == 0 {
                return;
            }
        }
    }
}

/// Press page of survey presentation
pub struct Press {
    material: u32,
    duration: u64,
    tolerance: f32,
    // max_circle_adius: u32,
    // max_ring_adius: u32,
}

impl Press {
    const OUTPUT_START: f32 = 20.0;
    const OUTPUT_RING_MIN: f32 = 30.0;
    const OUTPUT_END: f32   = 100.0;
    const INPUT_START: f32  = 20.0;
    const INPUT_END: f32    = 1500.0;

    pub fn new(material: u32, duration: u64, tolerance: f32) -> Self {
        Press {
            material: material,
            duration: duration,
            tolerance: tolerance,
        }
    }

    fn round (f: f32) -> f32 {
        f.floor() + 0.5
    }

    fn range(output_start: f32, output_end: f32, input_start: f32, input_end: f32, input: f32) -> f32 {
        let slope = 1.0 * (output_end - output_start) / (input_end - input_start);
        output_start + Press::round(slope * (input - input_start))
    } 
}


impl Slide for Press { 
    fn run(&self, 
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>) {

        // jump to frontmatter page
        outbound_msg.send(msg::materialIndex(self.material, msg::press_num()));
        outbound_msg.send(msg::gotoPress());

        let mut rng = rand::thread_rng();

        let circle_radius     = Press::OUTPUT_START;
        let mut ring_radius   = rng.gen_range(Press::OUTPUT_RING_MIN, Press::OUTPUT_END);

        outbound_msg.send(msg::press(circle_radius, ring_radius));

        // track if touch is causing circle radius ~ ring radius, within a given tolerance
        let mut tolerance_timer = Instant::now();
        let mut within_tolerance = false;

        let overall_timer  = Instant::now();
        
        let mut data: Vec<world::PressContacts> = vec![vec![]]; 
        let mut circle_ring_radius: Vec<(f32, f32)> = vec![(circle_radius, ring_radius)];
        let mut num_presses = 0;

        let foo = overall_timer.elapsed().as_millis();

        // pressure input, until time is done
        while overall_timer.elapsed().as_secs() < self.duration {
            
            match inbound_osc.try_recv() {
                Ok((input_type, pressure, x, y, material)) => {
                    if material == self.material {
                        // map pressure into range and then send radius to frontend
                        let circle_radius = Press::range(
                            Press::OUTPUT_START, 
                            Press::OUTPUT_END, 
                            Press::INPUT_START, 
                            Press::INPUT_END, 
                            pressure);
                            
                        data[num_presses].push((overall_timer.elapsed().as_millis(), pressure, x, y));

                        // is circle radius ~ ring radius
                        if (ring_radius - circle_radius).abs() <= self.tolerance {
                            if within_tolerance {
                                if tolerance_timer.elapsed().as_secs() >= 1 {
                                    ring_radius = rng.gen_range(Press::OUTPUT_RING_MIN, Press::OUTPUT_END);
                                    within_tolerance = false;

                                    // update press storage
                                    num_presses = num_presses + 1;
                                    data.push(vec![]);
                                    circle_ring_radius.push((circle_radius, ring_radius));
                                }
                            }
                            else {
                                within_tolerance = true;
                                tolerance_timer = Instant::now();
                            }
                        }
                        else {
                            // need to make sure we reset seen tolerance if we fall out
                            within_tolerance = false;
                        }

                        outbound_msg.send(msg::press(circle_radius, ring_radius));
                    }
                },
                _ => {},
            }
        }
        world.writeGesture("press".to_string(), self.material, circle_ring_radius, data);
    }
}

//-----------------------------------------------------------------------------
// Case when users reports properties about one or more materials
//-----------------------------------------------------------------------------

pub struct Response {
    // name of property to requested for material(s)
    name: String,
    // number of materials to be responded on
    num_materials: u32,
    // number of slide on client slide (required as there can be different response pages)
    slide_num: u32,
}

impl Response {
    pub fn new(name: String, num_materials: u32, slide_num: u32) -> Self {
        Response {
            name: name,
            num_materials: num_materials,
            slide_num: slide_num,
        }
    }
}

impl Slide for Response {
    fn run(&self, 
        world: &mut world::World,
        inbound_osc: &Receiver<msg::SenselMessage>,
        outbound_msg: &ws_server::WSServer, 
        inbound_msg:  &Receiver<msg::Message>) {

        // empty osc channel, in case of any after touches
        loop {
            match inbound_osc.try_recv() {
                Ok(_) => {
                },
                _ => { 
                    break;
                }
            }
        }

        // goto slide
        outbound_msg.send(msg::gotoSlide(self.slide_num));

        let mut num_materials = self.num_materials;
        let mut start_happened = false;

        // responses for page reported as single CSV entry
        let mut materials: Vec<String> = Vec::with_capacity(num_materials as usize);

        while num_materials > 0 {
            match inbound_osc.recv() {
                Ok((input_type, pressure, x, y, material)) => {
                    // track initial touch
                    if input_type == msg::InputType::Start {
                        start_happened = true;
                    }
                    else if input_type == msg::InputType::End && start_happened { 
                        // on release record material as response
                        materials.push(material.to_string());
                        num_materials = num_materials - 1;
                    }
                },
                _ => {},
            }
        }
        // write out response(s) to CSV
        world.writeResponse(self.name.clone(), materials);
    }
}
