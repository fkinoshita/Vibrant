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

use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::config::PROFILE;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/fkinoshita/Vibrant/window.ui")]
    pub struct VibrantWindow {
        // Template widgets
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,

        #[template_child]
        pub gradient_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub gradient_overlay: TemplateChild<gtk::Overlay>,

        #[template_child]
        pub direction_combo: TemplateChild<adw::ComboRow>,

        #[template_child]
        pub color_one_entry: TemplateChild<adw::EntryRow>,
        #[template_child]
        pub color_two_entry: TemplateChild<adw::EntryRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VibrantWindow {
        const NAME: &'static str = "VibrantWindow";
        type Type = super::VibrantWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VibrantWindow {}
    impl WidgetImpl for VibrantWindow {}
    impl WindowImpl for VibrantWindow {}
    impl ApplicationWindowImpl for VibrantWindow {}
    impl AdwApplicationWindowImpl for VibrantWindow {}
}

glib::wrapper! {
    pub struct VibrantWindow(ObjectSubclass<imp::VibrantWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl VibrantWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let win: VibrantWindow = glib::Object::builder()
            .property("application", application)
            .build();

        win.init();
        win.setup_signals();

        win
    }

    fn init(&self) {
        let imp = self.imp();

        if PROFILE == "Devel" {
            self.add_css_class("devel");
        }

        imp.color_one_entry.set_text("blue");
        imp.color_two_entry.set_text("pink");
        self.set_gradient_css(
            (self.imp().direction_combo.selected() * 90) as u16,
            "blue",
            "pink",
        );
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        imp.direction_combo.connect_notify_local(
            Some("selected"),
            clone!(@strong self as this => move |combo, _| {

                let color1 = this.imp().color_one_entry.text();
                let color2 = this.imp().color_two_entry.text();

                this.set_gradient_css((dbg!(dbg!(combo.selected()) * 90)) as u16, &color1, &color2);
            }),
        );

        imp.color_one_entry.connect_notify_local(
            Some("text"),
            clone!(@strong self as this => move |entry, _| {
                let color1 = entry.text();
                let color2 = this.imp().color_two_entry.text();

                this.set_gradient_css((this.imp().direction_combo.selected() * 90) as u16, &color1, &color2);
            }),
        );

        imp.color_two_entry.connect_notify_local(
            Some("text"),
            clone!(@strong self as this => move |entry, _| {
                let color1 = this.imp().color_one_entry.text();
                let color2 = entry.text();


                this.set_gradient_css((this.imp().direction_combo.selected() * 90) as u16, &color1, &color2);
            }),
        );
    }

    fn set_gradient_css(&self, degrees: u16, color1: &str, color2: &str) {
        let provider = gtk::CssProvider::new();

        let css = format!(
            ".gradient-box {{background: linear-gradient({}deg, {}, {});}}",
            degrees, color1, color2
        );

        provider.load_from_data(css.as_str());

        self.imp()
            .gradient_box
            .style_context()
            .add_provider(&provider, 1000);
    }
}
