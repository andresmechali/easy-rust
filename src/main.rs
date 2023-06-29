#![allow(dead_code)]

mod closures;
mod custom_trait;
mod display;
mod from_trait;
mod iterators;
mod multithread;
mod strings;

fn main() {
    multithread::run();
}
