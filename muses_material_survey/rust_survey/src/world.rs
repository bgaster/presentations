//! Handle outside world interactions
//! 
//! Copyright: Benedict R. Gaster
//! 
//! 

use std::fs::File;

extern crate csv;

use crate::msg;

pub type PressContacts = Vec<(u128, f32, f32, f32)>;

pub struct PressData {
    /// radius of circle at start
    pub circle_radius: f32,
    /// radius of ring
    pub ring_radius: f32,
    /// contacts record timestamp, pressure, x, and y
    pub contacts: PressContacts,
    /// did the user complete the press test for given params
    pub success: bool,
}

pub struct World {
    pub id: u32,
    pub csv: csv::Writer<File>,
}

impl World {
    pub fn new(id: u32, file: File) -> Self {
        let csv = csv::Writer::from_writer(file);
        World {
            id: id,
            csv: csv,
        }
    }

    /// create an ID label to be written at front of each new entry in CSV
    fn create_id(&self) -> String {
        format!("id={}", self.id)
    }

    /// write likert data to CSV
    pub fn writeLikert(&mut self, likert: msg::Likert) {
        self.csv.write_record(&[&self.create_id(), &likert.name, &likert.value.to_string()]).unwrap();
        self.csv.flush().unwrap();
    }

    /// write respose data out to CSV
    pub fn writeResponse(&mut self, name: String, materials: Vec<String>) {
        let mut out = vec![self.create_id(), name];
        out.extend(materials);
        self.csv.write_record(out);
        self.csv.flush().unwrap();
    }

    /// write gesture data out to CSV
    pub fn writeGesture(
        &mut self,
        name: String,
        material: u32,
        circle_ring_radius: Vec<(f32, f32)>,
        contacts: Vec<PressContacts>) {
        
        for i in 0..circle_ring_radius.len() {
            // id=id, press, material, circleRadius, ringRadius, (timestamp, pressure, x, y), ...
            let mut out = vec![
                self.create_id(), 
                name.clone(), 
                material.to_string(),
                circle_ring_radius[i].0.to_string(),
                circle_ring_radius[i].1.to_string()];
            
            for contact in &contacts[i] {
                out.push(format!(
                    "({}{}{}{})", 
                    contact.0.to_string(), 
                    contact.1.to_string(), 
                    contact.2.to_string(),
                    contact.3.to_string()));
            }

            self.csv.write_record(out);
            self.csv.flush().unwrap();
        }
    }

    /// In some cases, when in a loop, for example, we don't want to flush until end of slide.
    pub fn flushCSV(&mut self) {
        self.csv.flush().unwrap();
    }
}