
Submissions must include:

   - Abstract
   - Session type
   - Session length
   - Expected prior knowledge / intended audience
   - Speaker bio
   - Links to code / slides / material for the talk (optional)
   - Links to previous talks by the speaker

Please submit through https://penta.fosdem.org/submission/FOSDEM20

*Important dates:*

   - Call for papers available: 14 October 2019
   - Call for participation closes: 25 November 2019
   - Devroom schedule available: 16 December 2019
   - Devroom day: Sunday 2 February 2020 (09:00 to 17:00)

---

## Title

Rusty Instruments: Building Digtial Musical Instruments with Rust and Friends

 ## Abstract

This talk will introduce the Muses project, which applies programming language
theory and practice, physical computing, networking, and musicial theory to
design and implementation of Digitial Musical Instruments. Rust is a key
ingredient in the Muses project, providing a robust and performant foundation
for cross platform, desktop, and embedded system development. 

The talk will give a brief introdution to Muses project as a whole and then
focus on the use of Rust in developing a selection of very different components
in the system and its benefits for these wildy varing use cases.

Demos of the Digitial Musical Instruments with Rust at their heart will shown
through out the talk.

## Description

Controller and gesture interaction with audio and/or visual media is today
ubiquitous, requiring the development of intuitive software solutions for
interaction design. Designing and building these interfaces often require
extensive domain expertise in audio and visual media creation, e.g. the
musician, but additionally in engineering and software development. In this talk
we focus on custom controller-based interactive systems for sound and musical
performance, with a focus on an intuitive and simple design process that is
accessible to artists. 

A large part of the software developed for these systems is low-level system
code, where direct access to hardware and understandable performance are hard
requirements for these systems. Historically, these systems are written in C/C++
and in the case of embedded systems C is still the language of choice. With the
emergence of the system programming language Rust an alternative for developing
these systems is now with us with its support for high-level features such as
traits, type inference, pattern matching, and of course it's affine based type
system for pointers. 

This talk will introduce the Muses project, which applies programming language
theory and practice, physical computing, networking, and musicial theory to
design and implementation of Digitial Musical Instruments. Rust is a key
ingredient in the Muses project, providing a robust and performant foundation
for cross platform, desktop, and embedded system development.

A high-level overview of the schedule is:

   - Introduction to the Muses project
   - 100 foot view of the Muses architecture
   - Experience using Rust for audio and interface development
   - Demostration

A demonstration will include the following physical components:

   - Custom interface
      - Hardware encoders + arcade buttons
      - Sensel touch interface with custom interface
      - STM32 based embedded hardware platform, all running Rust

   - Raspberry PI for Sound
      - Pure Data for sound synthesis
      - Rust based driver for communicating with custom hardware
      - Rust based Open Sound Control (OSC) server for custom control messages

The framework also includes an approach to automatically generating interfaces
from DSL for SVG interfaces, written in Haskell with an tessellation pipeline
written in Rust. However, while this will be mentioned in passing it is not 
the intention of this talk to cover this aspect of the system in detail. (For 
more information on this see the provide link for the project website and the 
associated papers, also linked from the site.)

 ## Session type

Talk, but will also include demo(s).

 ## Session length: 
 
 40 minutes, but happy to do 20 minute talk too.

 ## Expected prior knowledge / intended audience

Knowledge of programming will be expected and prior use of C/C++, Rust, or other
systems programming language would be useful.

Audio topics will be introduced through out the talk and it is not epxected that
audience members have a musical background. 

 ## Speaker bio

Dr Benedict R. Gaster is an Associate Professorat University of West of England,
he is the director of the Computer Science Research Centre, which within he also
leads the Physical Computing group. He research focuses on the design embedded
platforms for musical expression and more generally the IoT, he is the
co-founder of Bristol LoRaWAN a low power wide area network for Bristol city, is
the technical lead for city wide project on city pollution monitoring for
communities, having developed UWE Sense a hardware platform for cheap sensing,
which launches in December 2017. Along with his PhD students and in
collaboration with UWE's music tech department, is developing a new audio
platform based on ARM micro-controllers using the Rust programming language to
build faster and more robust sound!

Previously Benedict work at Qualcomm and AMD where he was a co-designer on the
programming language OpenCL, including the lead developer on AMD's OpenCL
compiler. He has a PhD in computer science for his work on type systems for
extensible records and variants. He has published extensively, has given
numerous presentations, including ones at FOSDEM on Rust and LoRaWAN.

 ## Links to code / slides / material for the talk (optional)
 
This work is part of the Muses Project, which is looking at the the design and
implementation of digitial musical instruments. It involves the development of
methodologies along side hardware and software outputs, and musical outputs. 

As performace is of major concern for real-time audio Rust is the main
devlelopment language for the project, however it is not the only one, and in
particular Haskell is also used when performance is not the main focus and also
web development is mostly done via Javascript. 

The following links lead to different aspects of the project:

   - Project's main website: [https://muses-dmi.github.io/](https://muses-dmi.github.io/).
   - Papers releted to the project: [https://muses-dmi.github.io/papers/](https://muses-dmi.github.io/papers/).
   - Sub projects: [https://muses-dmi.github.io/projects/](https://muses-dmi.github.io/projects/).
   - Finally the code is on the project's Github project page: [https://github.com/muses-dmi](https://github.com/muses-dmi).

All code is licensed under under either of

    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
    MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Mozilla Public License 2.0

at your option.

## Links to previous talks by the speaker

Below are are some examples of recent talks:

   - [Fun with Interfaces (SVG Interfaces for Musical Expression).](https://bgaster.github.io/farm19/) FARM'19: 7th International Workshop on Functional Art, Music, Modeling and Design (FARM). 
   - [Rustarm AKA A project looking at Rust for Embedded Systems.](https://archive.fosdem.org/2018/schedule/event/rustyarm_rust_on_embedded_platforms/) FOSDEM’18.
   - [Outside the Block Syndicate: Translating Faust's Algebra of Blocks to the Arrows Framework](https://archive.fosdem.org/2017/schedule/event/lorawan/). International Faust Conference (IFC-18), 2018
   - [LoRaWAN for exploring the Internet of Things. Talk Hard: A technical, political, and cultural look at LoRaWAN for IoT.](https://archive.fosdem.org/2017/schedule/event/lorawan/) FOSDEM’17.
         