/* window.rs
 *
 * Copyright 2023 Felipe Kinoshita
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use glib::clone;

use gtk::prelude::*;
use gtk::{gio, glib};

use adw::subclass::prelude::*;
use adw::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/fkinoshita/Gradients/window.ui")]
    pub struct GradientsWindow {
        // Template widgets
        #[template_child]
        pub gradient_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub gradient_overlay: TemplateChild<gtk::Overlay>,

        #[template_child]
        pub type_combo: TemplateChild<adw::ComboRow>,

        #[template_child]
        pub angle_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub angle_adjustment: TemplateChild<gtk::Adjustment>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GradientsWindow {
        const NAME: &'static str = "GradientsWindow";
        type Type = super::GradientsWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GradientsWindow {}
    impl WidgetImpl for GradientsWindow {}
    impl WindowImpl for GradientsWindow {}
    impl ApplicationWindowImpl for GradientsWindow {}
    impl AdwApplicationWindowImpl for GradientsWindow {}
}

glib::wrapper! {
    pub struct GradientsWindow(ObjectSubclass<imp::GradientsWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl GradientsWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let win: GradientsWindow = glib::Object::builder()
            .property("application", application)
            .build();

        win.init();
        win.setup_signals();

        win
    }

    fn init(&self) {
        // let imp = self.imp();
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        imp.type_combo.connect_notify_local(Some("selected"),
            clone!(@strong self as this => move |combo, _| {
                this.imp().angle_row.set_visible(false);

                if combo.selected() == 0 {
                    this.set_gradient_css(format!(".gradient-box {{background: linear-gradient({}deg, blue, pink);}}", 45).trim());
                    this.imp().angle_row.set_visible(true);
                    return;
                }

                if combo.selected() == 1 {
                    this.set_gradient_css(format!(".gradient-box {{background: radial-gradient(blue, pink);}}").trim());
                    return;
                }

                if combo.selected() == 2 {
                    this.set_gradient_css(format!(".gradient-box {{background: conic-gradient(blue, pink);}}").trim());
                    return;
                }
            })
        );

        imp.angle_adjustment.connect_value_changed(
            clone!(@strong self as this => move |adjust| {
                let degrees = adjust.value().floor();

                this.set_gradient_css(format!(".gradient-box {{background: linear-gradient({}deg, blue, pink);}}", degrees).trim());
            })
        );
    }

    fn set_gradient_css(&self, css: &str) {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(css);

        self.imp().gradient_box.style_context().add_provider(&provider, 1000);
    }
}
