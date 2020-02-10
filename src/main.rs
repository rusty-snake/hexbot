/*
 * Copyright © 2019-2020 rusty-snake <print_hello_world+License@protonmail.com>
 *
 * This file is part of rusty-snake's hexbot solution
 *
 * rusty-snake's hexbot solution is free software: you can redistribute
 * it and/or modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * rusty-snake's hexbot solution is distributed in the hope that it
 * will be useful, but WITHOUT ANY WARRANTY; without even the implied
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use gdk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::prelude::*;

use gio::ApplicationFlags;
use gtk::{
    Application, ApplicationWindow, Builder, Button, CheckButton, ColorButton, Grid, Label,
    SpinButton,
};
use hexbot::{Count, Hexbot, Seed, WidthHeight};
use glib::clone;

macro_rules! get_object {
    ($builder:ident, $name:literal) => {
        $builder
            .get_object($name)
            .expect(concat!("Error: get_object: ", $name))
    };
}

const BUILDER_SRC: &str = include_str!("window.glade");
//                                        TODO: ^^ filename

fn build_ui(app: &Application) {
    let builder = Builder::new_from_string(BUILDER_SRC);

    let window: gtk::Window = get_object!(builder, "window");

    // count
    {
        let spin_button: gtk::SpinButton = get_object!(builder, "count_SpinButton");
        let check_button: gtk::CheckButton = get_object!(builder, "count_CheckButton");

        check_button.connect_toggled(move |button| {
            if button.get_active() {
                spin_button.set_sensitive(true);
            } else {
                spin_button.set_sensitive(false);
            }
        });
    }

    // width
    {
        let width_spin_button: gtk::SpinButton = get_object!(builder, "width_SpinButton");
        let heigth_spin_button: gtk::SpinButton = get_object!(builder, "heigth_SpinButton");
        let heigth_check_button: gtk::CheckButton = get_object!(builder, "heigth_CheckButton");
        let width_check_button: gtk::CheckButton = get_object!(builder, "width_CheckButton");

        width_check_button.connect_toggled(clone!(@strong heigth_check_button => move |button| {
            if button.get_active() {
                width_spin_button.set_sensitive(true);
                heigth_check_button.set_active(true);
            } else {
                width_spin_button.set_sensitive(false);
                heigth_check_button.set_active(false);
            }
        }));

        heigth_check_button.connect_toggled(move |button| {
            if button.get_active() {
                heigth_spin_button.set_sensitive(true);
                width_check_button.set_active(true);
            } else {
                heigth_spin_button.set_sensitive(false);
                width_check_button.set_active(false);
            }
        });
    }

    /*
            width: InputWidget {
                check_button: get_object!(builder, "width_CheckButton"),
                input_widget: get_object!(builder, "width_SpinButton"),
            },
            height: InputWidget {
                check_button: get_object!(builder, "heigth_CheckButton"),
                input_widget: get_object!(builder, "heigth_SpinButton"),
            },
            seed: SeedInputWidget {
                check_button: get_object!(builder, "seed_CheckButton"),
                grid: get_object!(builder, "seed_Grid"),
                color_buttons: [
                    InputWidget {
                        check_button: get_object!(builder, "seed_color1_CheckButton"),
                        input_widget: get_object!(builder, "seed_color1_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color2_CheckButton"),
                        input_widget: get_object!(builder, "seed_color2_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color3_CheckButton"),
                        input_widget: get_object!(builder, "seed_color3_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color4_CheckButton"),
                        input_widget: get_object!(builder, "seed_color4_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color5_CheckButton"),
                        input_widget: get_object!(builder, "seed_color5_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color6_CheckButton"),
                        input_widget: get_object!(builder, "seed_color6_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color7_CheckButton"),
                        input_widget: get_object!(builder, "seed_color7_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color8_CheckButton"),
                        input_widget: get_object!(builder, "seed_color8_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color9_CheckButton"),
                        input_widget: get_object!(builder, "seed_color9_ColorButton"),
                    },
                    InputWidget {
                        check_button: get_object!(builder, "seed_color10_CheckButton"),
                        input_widget: get_object!(builder, "seed_color10_ColorButton"),
                    },
                ],
            },
            fetch_button: get_object!(builder, "fetch_Button"),
        },
        output: get_object!(builder, "response_Label"),
    };
    */

    /*


    // seed
    {
        let grid = main_window.input.seed.grid.clone();
        main_window
            .input
            .seed
            .check_button
            .connect_toggled(move |button| {
                if button.get_active() {
                    grid.set_sensitive(true);
                } else {
                    grid.set_sensitive(false);
                }
            });
    }

    /*
    {
        for color_input_widget in &main_window.input.seed.color_buttons {
            let color_button = color_input_widget.input_widget.clone();
            color_input_widget.check_button.connect_toggled(move |button| {
                if button.get_active() {
                    color_button.set_sensitive(true);
                } else {
                    color_button.set_sensitive(false);
                }
            });
        }
    }
    */

    {
        let count_sb = main_window.input.count.input_widget.clone();
        let width_sb = main_window.input.width.input_widget.clone();
        let height_sb = main_window.input.height.input_widget.clone();

        let count_cb = main_window.input.count.check_button.clone();
        let width_cb = main_window.input.width.check_button.clone();
        let height_cb = main_window.input.height.check_button.clone();

        let output_label = main_window.output.clone();

        main_window.input.fetch_button.connect_clicked(move |_| {
            let count;
            let width_height;
            if count_cb.get_active() {
                count = Count::yes(count_sb.get_value_as_int()).unwrap();
            } else {
                count = Count::no();
            }
            if width_cb.get_active() && height_cb.get_active() {
                width_height =
                    WidthHeight::yes(width_sb.get_value_as_int(), height_sb.get_value_as_int())
                        .unwrap();
            } else {
                width_height = WidthHeight::no();
            }
            let hb = Hexbot::fetch(count, width_height, &Seed::no()).expect("Fecthing failed");

            output_label.set_text(&hb.to_string());
        });
    }
    */

    /*
    main_window.window.set_application(Some(app));
    main_window.window.show_all();
    */

    window.set_application(Some(app));
    window.show_all();
}

/*
fn get_seed_colors(color_buttons: [InputWidget<ColorButton>; 10]) -> Vec<i32> {
    let mut vec = Vec::with_capacity(10);
    for color_button in &color_buttons {
        if !color_button.check_button.get_active() {
            continue;
        }
        let gdk::Color {
            red, green, blue, ..
        } = color_button.input_widget.get_color();
    }
    vec.shrink_to_fit();
    vec

    // color_bottons[0].input_widget.get_color().red as i32
}
*/

fn main() {
    let application =
        Application::new(None, Default::default()).expect("Application::new failed");

    application.connect_activate(build_ui);

    //application.run(&args().collect::<Vec<_>>());
    application.run(&[]);
}
