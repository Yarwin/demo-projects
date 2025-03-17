/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

struct HotReload;

#[gdextension]
unsafe impl ExtensionLibrary for HotReload {
    fn on_level_init(_level: InitLevel) {
        println!("[Rust]      Init level {:?}", _level);
    }

    fn on_level_deinit(_level: InitLevel) {
        println!("[Rust]      Deinit level {:?}", _level);
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// A RustDoc comment appearing under the editor help docs.
#[derive(GodotClass)]
#[class(base=Node)]
struct Reloadable {
    /// A planet!
    #[export]
    favorite_planet: Planet,
    //
    // HOT-RELOAD: uncomment this to add a new exported field (also update init() below).
    // #[export]
    // some_string: GString,
}

#[godot_api]
impl INode for Reloadable {
    fn init(_base: Base<Self::Base>) -> Self {
        // HOT-RELOAD: change values to initialize with different defaults.
        Self {
            favorite_planet: Planet::Earth,
            //some_string: "Hello, world!".into(),
        }
    }
}

#[godot_api]
impl Reloadable {
    /// A function to return a number.
    #[func]
    fn get_number(&self) -> i64 {
        // HOT-RELOAD: change returned value for dynamic code change.
        100
    }

    /// Constructor from a string.
    #[func]
    fn from_string(s: GString) -> Gd<Self> {
        Gd::from_object(Reloadable {
            favorite_planet: Planet::from_godot(s),
        })
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// A planet enum.
#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Planet {
    Earth,
    Mars,
    Venus,
    //
    // HOT-RELOAD: uncomment this to extend enum.
    //Jupiter,
}
