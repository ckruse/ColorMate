/* Application.vala
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

namespace ColorMate {
  public class Application : Gtk.Application {
    public Application () {
      Object (
        application_id: "com.github.ckruse.ColorMate",
        flags: ApplicationFlags.FLAGS_NONE
        );
    }

    protected override void activate () {
      var win = this.active_window;
      if (win == null) {
        win = new ColorMate.Window (this) { title = "ColorMate" };
      }

      var granite_settings = Granite.Settings.get_default ();
      var gtk_settings = Gtk.Settings.get_default ();

      gtk_settings.gtk_application_prefer_dark_theme = (
        granite_settings.prefers_color_scheme == Granite.Settings.ColorScheme.DARK
      );

      granite_settings.notify["prefers-color-scheme"].connect (() => {
        gtk_settings.gtk_application_prefer_dark_theme = (
          granite_settings.prefers_color_scheme == Granite.Settings.ColorScheme.DARK
        );
      });

      win.present ();
    }
  }
}
