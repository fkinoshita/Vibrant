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
    #[template(resource = "/io/github/fkinoshita/Vibrant/window.ui")]
    pub struct VibrantWindow {
        // Template widgets
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,

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

        #[template_child]
        pub colors_row: TemplateChild<adw::ActionRow>,
        pub colors_button: gtk::Button,

        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
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

        let icon = gtk::Image::builder()
            .icon_name("go-next-symbolic")
            .build();

        imp.colors_row.add_suffix(&icon);
        imp.colors_row.set_activatable_widget(Some(&imp.colors_button));
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

        imp.colors_row.connect_activated(
            clone!(@strong self as this => move |_row| {
                this.imp().leaflet.navigate(adw::NavigationDirection::Forward);
            })
        );

        imp.back_button.connect_clicked(
            clone!(@strong self as this => move |_button| {
                this.imp().leaflet.navigate(adw::NavigationDirection::Back);
            })
        );
    }

    fn set_gradient_css(&self, css: &str) {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(css);

        self.imp().gradient_box.style_context().add_provider(&provider, 1000);
    }
}
