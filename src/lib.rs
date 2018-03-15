#[macro_use]
extern crate samp_sdk;

mod plugin;
use plugin::Plugin;

new_plugin!(Plugin);