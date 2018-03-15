use samp_sdk::amx::AMX;
use samp_sdk::types::Cell;
use samp_sdk::consts::*;

pub struct Plugin;

impl Plugin {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {

    }

    pub fn amx_load(&self, amx: &AMX) -> Cell {
        AMX_ERR_NONE
    }

    pub fn amx_unload(&self, amx: &AMX) -> Cell {
        AMX_ERR_NONE
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Plugin
    }
}