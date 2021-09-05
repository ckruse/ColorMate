/* window.vala
 *
 * Copyright 2021 Christian Kruse
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
 */

struct Rgb {
  int r;
  int g;
  int b;
}

struct Hsl {
  int h;
  double s;
  double l;
}

namespace ColorMate {
  [GtkTemplate (ui = "/com/github/ckruse/ColorMate/ui/window.ui")]
  public class Window : Gtk.ApplicationWindow {
    [GtkChild]
    unowned Gtk.Label no_rgb_lbl;
    [GtkChild]
    unowned Gtk.Label rgb_lbl;
    [GtkChild]
    unowned Gtk.Label hsl_lbl;

    [GtkChild]
    unowned Gtk.Button copy_no_rgb_btn;
    [GtkChild]
    unowned Gtk.Button copy_rgb_btn;
    [GtkChild]
    unowned Gtk.Button copy_hsl_btn;

    [GtkChild]
    unowned Gtk.ColorButton color_chooser;

    [GtkChild]
    unowned Gtk.Entry color_entry;

    private GLib.Settings settings;

    public Window (Gtk.Application app) {
      Object (application: app, border_width: 0);
    }

    construct {
      var clipboard = Gtk.Clipboard.get(Gdk.SELECTION_CLIPBOARD);
      color_entry.changed.connect (this.on_color_entry_changed);
      copy_rgb_btn.clicked.connect(() => {
        clipboard.set_text (rgb_lbl.get_text(), -1);
      });
      copy_no_rgb_btn.clicked.connect(() => {
        clipboard.set_text (no_rgb_lbl.get_text(), -1);
      });
      copy_hsl_btn.clicked.connect(() => {
        clipboard.set_text (hsl_lbl.get_text(), -1);
      });

      settings = new GLib.Settings ("com.github.ckruse.ColorMate");

      move (settings.get_int ("window-x"), settings.get_int ("window-y"));
      resize (settings.get_int ("window-width"), settings.get_int ("window-height"));

      delete_event.connect (e => {
        return before_destroy ();
      });
    }

    public void on_color_entry_changed () {
      GLib.Regex rexp_rgb_no = /^\s*(?:#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}))$/;
      GLib.Regex rexp_rgb = /^(?:rgb\()?(\d+)\s*,\s*(\d+)\s*,\s*(\d+)(?:\))?$/;
      GLib.Regex rexp_hsl = /^(?:hsl\()?\s*(\d+)\s*,\s*(\d+(?:\.\d+)?)%\s*,(\d+(?:\.\d+)?)%\s*(?:\))?$/;
      var text = color_entry.get_text ();
      GLib.MatchInfo match;

      Rgb rgb;
      Hsl hsl;

      if (rexp_rgb_no.match (text, 0, out match)) {
        var matches = match.fetch_all ();

        parse_rgb_no (matches, out rgb);
        rgb2hsl (rgb, out hsl);
        change_ui (rgb, hsl);
      } else if (rexp_rgb.match (text, 0, out match)) {
        var matches = match.fetch_all ();

        parse_rgb (matches, out rgb);
        rgb2hsl (rgb, out hsl);
        change_ui (rgb, hsl);
      } else if(rexp_hsl.match (text, 0, out match)) {
        var matches = match.fetch_all ();
        parse_hsl (matches, out hsl);
        hsl2rgb(hsl, out rgb);
        change_ui (rgb, hsl);
      }
    }

    private void change_ui (Rgb rgb, Hsl hsl) {
      no_rgb_lbl.set_text ("#%02X%02X%02X".printf (rgb.r, rgb.g, rgb.b));
      rgb_lbl.set_text ("rgb(%d, %d, %d)".printf (rgb.r, rgb.g, rgb.b));
      hsl_lbl.set_text ("hsl(%d, %.1f%%, %.1f%%)".printf (hsl.h, hsl.s, hsl.l));

      var color = Gdk.Color() {
        red = (uint16)(rgb.r * 256),
        green = (uint16)(rgb.g * 256),
        blue = (uint16)(rgb.b * 256),
        pixel = 0,
      };
      color_chooser.set_color(color);
    }

    private void parse_rgb_no (string[] matches, out Rgb rgb) {
      GLib.Regex rx;
      if (matches[1].length == 3) {
        rx = /([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])/;
      } else {
        rx = /([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})/;
      }

      GLib.MatchInfo color_match;
      rx.match (matches[1], 0, out color_match);
      var values = color_match.fetch_all ();

      int r, g, b;
      if (matches[1].length == 3) {
        int.try_parse(values[1] + values[1], out r, null, 16);
        int.try_parse(values[2] + values[2], out g, null, 16);
        int.try_parse(values[3] + values[3], out b, null, 16);
      } else {
        int.try_parse(values[1], out r, null, 16);
        int.try_parse(values[2], out g, null, 16);
        int.try_parse(values[3], out b, null, 16);
      }

      rgb = Rgb() { r = r, g = g, b = b };
    }

    private void parse_rgb (string[] matches, out Rgb rgb) {
      int r, g, b;
      int.try_parse(matches[1], out r);
      int.try_parse(matches[2], out g);
      int.try_parse(matches[3], out b);

      rgb = Rgb() { r = r, g = g, b = b };
    }

    private void parse_hsl (string[] matches, out Hsl hsl) {
      int h;
      double s, l;
      int.try_parse(matches[1], out h);
      double.try_parse(matches[2], out s);
      double.try_parse(matches[3], out l);

      hsl = Hsl() { h = h, s = s, l = l };
    }

    private void rgb2hsl (Rgb rgb, out Hsl hsl) {
      double r = rgb.r / 255.0;
      double g = rgb.g / 255.0;
      double b = rgb.b / 255.0;

      // Find greatest and smallest channel values
      var cmin = double.min (double.min (r, g), b);
      var cmax = double.max (double.max(r, g), b);
      var delta = cmax - cmin;
      double h = 0, s = 0, l = 0;

      // Calculate hue
      if (delta == 0) {
        h = 0;
      }
      else if (cmax == r) {
        h = ((g - b) / delta) % 6;
      }
      else if (cmax == g) {
        h = (b - r) / delta + 2;
      }
      else {
        h = (r - g) / delta + 4;
      }

      h = Math.round (h * 60);

      if (h < 0) {
        h += 360;
      }

      // Calculate lightness
      l = (cmax + cmin) / 2;

      // Calculate saturation
      s = delta == 0 ? 0 : delta / (1 - (2 * l - 1).abs());

      // Multiply l and s by 100
      s = (s * 100.0).abs();
      l = (l * 100.0).abs();

      hsl = Hsl() { h = (int)h, l = l, s = s };
    }

    private void hsl2rgb (Hsl hsl, out Rgb rgb) {
      double r = 0, g = 0, b = 0;
      double h = hsl.h / 60f, s = hsl.s / 100f, l = hsl.l / 100f;

      double c = (1 - (2 * l - 1).abs()) * s;
      double x = c * (1 - (h % 2 - 1).abs());

      if (h < 0) {
        r = 0;
        g = 0;
        b = 0;
      } else if (h >= 0 && h < 1) {
        r = c;
        b = x;
        b = 0;
      } else if (h >= 1 && h < 2) {
        r = x;
        g = c;
        b = 0;
      } else if (h >= 2 && h < 3) {
        r = 0;
        g = c;
        b = x;
      } else if (h >= 3 && h < 4) {
        r = 0;
        g = x;
        b = c;
      } else if (h >= 4 && h < 5) {
        r = x;
        g = 0;
        b = c;
      } else if (h >= 5 && h < 6) {
        r = c;
        g = 0;
        b = x;
      }

      double m = l - (c / 2);

      rgb = Rgb() { r = to255(r + m), g = to255(g + m), b = to255(b + m) };
    }

    private int to255(double v) {
      return (int)double.min (255f, 256f * v);
    }

    private bool before_destroy () {
        int x, y, width, height;

        get_position (out x, out y);
        get_size (out width, out height);

        settings.set_int ("window-x", x);
        settings.set_int ("window-y", y);
        settings.set_int ("window-width", width);
        settings.set_int ("window-height", height);

        return false;
    }
  }
}
