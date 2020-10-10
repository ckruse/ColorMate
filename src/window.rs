use colorsys::{Hsl, Rgb};
use fancy_regex::{Captures, Regex};
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    pub color_entry: gtk::Entry,
    pub rgb_hex_entry: gtk::Entry,
    pub rgb_entry: gtk::Entry,
    pub hsl_entry: gtk::Entry,
    pub color_btn: gtk::ColorButton,
    pub copy_hex_rgb: gtk::Button,
    pub copy_rgb: gtk::Button,
    pub copy_hsl: gtk::Button,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_resource("/de/wwwtech/ColorMate/window.ui");
        let widget: gtk::ApplicationWindow = builder.get_object("ColormateWindow").unwrap();

        let color_entry: gtk::Entry = builder.get_object("color_entry").unwrap();

        let rgb_hex_entry: gtk::Entry = builder.get_object("rgb_hex_entry").unwrap();
        let rgb_entry: gtk::Entry = builder.get_object("rgb_entry").unwrap();
        let hsl_entry: gtk::Entry = builder.get_object("hsl_entry").unwrap();

        let color_btn: gtk::ColorButton = builder.get_object("color_btn").unwrap();

        let copy_hex_rgb: gtk::Button = builder.get_object("copy_hex_rgb").unwrap();
        let copy_rgb: gtk::Button = builder.get_object("copy_rgb").unwrap();
        let copy_hsl: gtk::Button = builder.get_object("copy_hsl").unwrap();

        let buf = Pixbuf::from_resource("/de/wwwtech/ColorMate/de.wwwtech.ColorMate.svg").unwrap();
        widget.set_icon(Some(&buf));

        color_btn.set_color(&gdk::Color {
            red: 65535,
            green: 65535,
            blue: 65535,
            pixel: 0,
        });

        Self {
            widget,
            color_entry,
            rgb_hex_entry,
            rgb_entry,
            hsl_entry,
            color_btn,
            copy_hex_rgb,
            copy_rgb,
            copy_hsl,
        }
    }

    pub fn init(&self) {
        let color_btn_clone = self.color_btn.clone();
        let rgb_hex_entry_clone = self.rgb_hex_entry.clone();
        let rgb_entry_clone = self.rgb_entry.clone();
        let hsl_entry_clone = self.hsl_entry.clone();
        self.color_entry.connect_changed(move |entry| {
            Self::parse_color(
                entry,
                &color_btn_clone,
                &rgb_hex_entry_clone,
                &rgb_entry_clone,
                &hsl_entry_clone,
            )
        });

        let rgb_hex_entry_clone = self.rgb_hex_entry.clone();
        let rgb_entry_clone = self.rgb_entry.clone();
        let hsl_entry_clone = self.hsl_entry.clone();
        self.color_btn.connect_color_set(move |btn| {
            let color = btn.get_color();
            Self::from_gdk_color(
                &color,
                btn,
                &rgb_hex_entry_clone,
                &rgb_entry_clone,
                &hsl_entry_clone,
            )
        });

        let rgb_hex_entry_clone = self.rgb_hex_entry.clone();
        self.copy_hex_rgb.connect_clicked(move |_| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(rgb_hex_entry_clone.get_text().as_str());
        });

        let rgb_entry_clone = self.rgb_entry.clone();
        self.copy_rgb.connect_clicked(move |_| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(rgb_entry_clone.get_text().as_str());
        });

        let hsl_entry_clone = self.hsl_entry.clone();
        self.copy_hsl.connect_clicked(move |_| {
            let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
            clipboard.set_text(hsl_entry_clone.get_text().as_str());
        });
    }

    pub fn parse_color(
        entry: &gtk::Entry,
        color_btn: &gtk::ColorButton,
        rgb_hex_entry: &gtk::Entry,
        rgb_entry: &gtk::Entry,
        hsl_entry: &gtk::Entry,
    ) {
        lazy_static! {
            static ref RX_RGB_HEX: Regex = Regex::new("^#?(?:(?:([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2}))|(?:([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])))$").unwrap();
            static ref RX_RGB: Regex = Regex::new("^(?:rgb\\()?(\\d+)\\s*,\\s*(\\d+)\\s*,\\s*(\\d+)(?:\\))?$").unwrap();
            static ref RX_HSL: Regex = Regex::new("^(?:hsl\\()?\\s*(\\d+)\\s*,\\s*(\\d+(?:\\.\\d+)?)%\\s*,(\\d+(?:\\.\\d+)?)%\\s*(?:\\))?$").unwrap();
        }

        let text = entry.get_text().to_string();
        let text_str = text.as_str();

        if let Some(captures) = RX_RGB_HEX.captures(text_str).unwrap() {
            let (r, g, b) = Self::from_hex_rgb_captures(&captures);
            Self::update_from_rgb(r, g, b, color_btn, rgb_hex_entry, rgb_entry, hsl_entry);
            return;
        }

        if let Some(captures) = RX_RGB.captures(text_str).unwrap() {
            let (r, g, b) = Self::from_rgb_captures(&captures);
            Self::update_from_rgb(r, g, b, color_btn, rgb_hex_entry, rgb_entry, hsl_entry);
            return;
        }

        if let Some(captures) = RX_HSL.captures(text_str).unwrap() {
            let (h, s, l) = Self::from_hsl_captures(&captures);
            Self::update_from_hsl(h, s, l, color_btn, rgb_hex_entry, rgb_entry, hsl_entry);
            return;
        }

        rgb_hex_entry.set_text("");
        rgb_entry.set_text("");
        hsl_entry.set_text("");
        color_btn.set_color(&gdk::Color {
            red: 65535,
            green: 65535,
            blue: 65535,
            pixel: 0,
        })
    }

    pub fn from_hex_rgb_captures(captures: &Captures) -> (u16, u16, u16) {
        let r = match captures.get(1) {
            Some(s) => s.as_str().to_string(),
            None => captures.get(4).unwrap().as_str().repeat(2),
        };

        let g = match captures.get(2) {
            Some(s) => s.as_str().to_string(),
            None => captures.get(5).unwrap().as_str().repeat(2),
        };

        let b = match captures.get(3) {
            Some(s) => s.as_str().to_string(),
            None => captures.get(6).unwrap().as_str().repeat(2),
        };

        return (
            u16::from_str_radix(r.as_str(), 16).unwrap(),
            u16::from_str_radix(g.as_str(), 16).unwrap(),
            u16::from_str_radix(b.as_str(), 16).unwrap(),
        );
    }

    pub fn from_rgb_captures(captures: &Captures) -> (u16, u16, u16) {
        let r = captures.get(1).unwrap().as_str();
        let g = captures.get(2).unwrap().as_str();
        let b = captures.get(3).unwrap().as_str();

        return (
            u16::from_str_radix(r, 10).unwrap(),
            u16::from_str_radix(g, 10).unwrap(),
            u16::from_str_radix(b, 10).unwrap(),
        );
    }

    pub fn from_hsl_captures(captures: &Captures) -> (f64, f64, f64) {
        let h = captures.get(1).unwrap().as_str();
        let s = captures.get(2).unwrap().as_str();
        let l = captures.get(3).unwrap().as_str();

        return (h.parse().unwrap(), s.parse().unwrap(), l.parse().unwrap());
    }

    pub fn from_gdk_color(
        color: &gdk::Color,
        color_btn: &gtk::ColorButton,
        rgb_hex_entry: &gtk::Entry,
        rgb_entry: &gtk::Entry,
        hsl_entry: &gtk::Entry,
    ) {
        let (r, g, b) = (color.red / 255, color.green / 255, color.blue / 255);
        let rgb_color = Rgb::new(r as f64, g as f64, b as f64, None);
        let hsl_color = Hsl::from(rgb_color);

        Self::set_text(
            r,
            g,
            b,
            hsl_color.get_hue(),
            hsl_color.get_saturation(),
            hsl_color.get_lightness(),
            color_btn,
            rgb_hex_entry,
            rgb_entry,
            hsl_entry,
        );
    }

    pub fn update_from_rgb(
        r: u16,
        g: u16,
        b: u16,
        color_btn: &gtk::ColorButton,
        rgb_hex_entry: &gtk::Entry,
        rgb_entry: &gtk::Entry,
        hsl_entry: &gtk::Entry,
    ) {
        let rgb_color = Rgb::new(r as f64, g as f64, b as f64, None);
        let hsl_color = Hsl::from(rgb_color);

        Self::set_text(
            r,
            g,
            b,
            hsl_color.get_hue(),
            hsl_color.get_saturation(),
            hsl_color.get_lightness(),
            color_btn,
            rgb_hex_entry,
            rgb_entry,
            hsl_entry,
        );
    }

    pub fn update_from_hsl(
        h: f64,
        s: f64,
        l: f64,
        color_btn: &gtk::ColorButton,
        rgb_hex_entry: &gtk::Entry,
        rgb_entry: &gtk::Entry,
        hsl_entry: &gtk::Entry,
    ) {
        let hsl_color = Hsl::new(h, s, l, None);
        let rgb = Rgb::from(hsl_color);

        Self::set_text(
            (rgb.get_red()) as u16,
            (rgb.get_green()) as u16,
            (rgb.get_blue()) as u16,
            h,
            s,
            l,
            color_btn,
            rgb_hex_entry,
            rgb_entry,
            hsl_entry,
        );
    }

    pub fn set_text(
        r: u16,
        g: u16,
        b: u16,
        h: f64,
        s: f64,
        l: f64,
        color_btn: &gtk::ColorButton,
        rgb_hex_entry: &gtk::Entry,
        rgb_entry: &gtk::Entry,
        hsl_entry: &gtk::Entry,
    ) {
        lazy_static! {
            static ref REPLACE_RE: Regex =
                Regex::new(r"^#([\da-fA-F])\1([\da-fA-F])\2([\da-fA-F])\3$").unwrap();
        }

        let rgb_hex_str = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let rgb_str = format!("rgb({}, {}, {})", r, g, b);
        let hsl_str = format!("hsl({:.0}, {:.1}%, {:.1}%)", h, s, l).replace(".0", "");

        let clean_rgb_hex_str = match REPLACE_RE.captures(rgb_hex_str.as_str()).unwrap() {
            Some(captures) => format!(
                "#{}{}{}",
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
                captures.get(3).unwrap().as_str()
            ),
            _ => rgb_hex_str,
        };

        let color = gdk::Color {
            red: r * 256,
            green: g * 256,
            blue: b * 256,
            pixel: 0,
        };
        color_btn.set_color(&color);

        rgb_hex_entry.set_text(clean_rgb_hex_str.as_str());
        rgb_entry.set_text(rgb_str.as_str());
        hsl_entry.set_text(hsl_str.as_str());
    }
}
