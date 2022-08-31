/* GDK - The GIMP Drawing Kit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

/*
 * Rust conversion by Robert R. Russell 2022.
 */

#[derive(Clone, Copy)]
pub enum GdkCursorType {
    BlankCursor = -2,
    CursorIsPixmap = -1,
    XCursor = 0,
    Arrow = 2,
    BasedArrowDown = 4,
    BasedArrowUp = 6,
    Boat = 8,
    Bogosity = 10,
    BottomLeftCorner = 12,
    BottomRightCorner = 14,
    BottomSide = 16,
    BottomTee = 18,
    BoxSpiral = 20,
    CenterPtr = 22,
    Circle = 24,
    Clock = 26,
    CoffeeMug = 28,
    Cross = 30,
    CrossReverse = 32,
    Crosshair = 34,
    DiamondCross = 36,
    Dot = 38,
    DotBox = 40,
    DoubleArrow = 42,
    DraftLarge = 44,
    DraftSmall = 46,
    DrapedBox = 48,
    Exchange = 50,
    Fleur = 52,
    Gobbler = 54,
    Gumby = 56,
    Hand1 = 58,
    Hand2 = 60,
    Heart = 62,
    Icon = 64,
    IronCross = 66,
    LeftPtr = 68,
    LeftSide = 70,
    LeftTee = 72,
    LeftButton = 74,
    LLAngle = 76,
    LRAngle = 78,
    Man = 80,
    MiddleButton = 82,
    Mouse = 84,
    Pencil = 86,
    Pirate = 88,
    Plus = 90,
    QuestionArrow = 92,
    RightPtr = 94,
    RightSide = 96,
    RightTee = 98,
    RightButton = 100,
    RtlLogo = 102,
    Sailboat = 104,
    SbDownArrow = 106,
    SbHDoubleArrow = 108,
    SbLeftArrow = 110,
    SbRightArrow = 112,
    SbUpArrow = 114,
    SbVDoubleArrow = 116,
    Shuttle = 118,
    Sizing = 120,
    Spider = 122,
    Spraycan = 124,
    Star = 126,
    Target = 128,
    TCross = 130,
    TopLeftArrow = 132,
    TopLeftCorner = 134,
    TopRightCorner = 136,
    TopSide = 138,
    TopTee = 140,
    Trek = 142,
    ULAngle = 144,
    Umbrella = 146,
    URAngle = 148,
    Watch = 150,
    Xterm = 152,
    LastCursor = 154,
}

pub struct GdkCursor {
    cursor_type: GdkCursorType,
    ref_count: usize,
}

impl GdkCursor {
    /**
     * Replaces GdkCursor* gdk_cursor_ref(GdkCursor *cursor);
     */
    pub fn cursor_ref(self: &mut Self) {
        self.ref_count += 1;
    }

    /**
     * Replaces void gdk_cursor_unref(GdkCursor *cursor);
     */
    pub fn cursor_unref(self: &mut Self) {
        if self.ref_count > 0 {
            self.ref_count -= 1;
        }
    }

    pub fn get_cursor_type(self: &Self) -> GdkCursorType {
        return self.cursor_type;
    }
}

/*
GdkCursor* gdk_cursor_new_for_display (GdkDisplay *display, GdkCursorType cursor_type );

GdkCursor* gdk_cursor_new(GdkCursorType cursor_type);

GdkCursor* gdk_cursor_new_from_pixmap(GdkPixmap *source, GdkPixmap *mask,
    const GdkColor *fg, const GdkColor *bg, gint x, gint y);

GdkCursor* gdk_cursor_new_from_pixbuf(GdkDisplay *display, GdkPixbuf *pixbuf,
    gint x, gint y);

GdkDisplay* gdk_cursor_get_display(GdkCursor *cursor);

GdkCursor* gdk_cursor_new_from_name(GdkDisplay *display, const gchar *name);

GdkPixbuf* gdk_cursor_get_image(GdkCursor *cursor);

 */
