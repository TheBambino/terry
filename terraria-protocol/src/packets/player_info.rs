use crate::packets::PacketBody;
use crate::structures::RGB;
use crate::SliceCursor;

const MAX_HAIR_VARIANT: u8 = 162;

/// Player information.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct PlayerInfo {
    pub player_id: u8,
    pub skin_variant: u8,
    /// Will be 0 if it's set to something greater than 162.
    pub hair_variant: u8,
    pub name: String,
    pub hair_dye: u8,
    pub hide_visuals: [u8; 2],
    pub hide_misc: u8,
    pub hair_color: RGB,
    pub skin_color: RGB,
    pub eye_color: RGB,
    pub shirt_color: RGB,
    pub undershirt_color: RGB,
    pub pants_color: RGB,
    pub shoes_color: RGB,
    pub difficulty: u8,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        PlayerInfo {
            player_id: 0,
            skin_variant: 0,
            hair_variant: 0,
            name: "terry".to_string(),
            hair_dye: 0,
            hide_visuals: [0; 2],
            hide_misc: 0,
            hair_color: RGB {
                r: 215,
                g: 90,
                b: 55,
            },
            skin_color: RGB {
                r: 255,
                g: 125,
                b: 90,
            },
            eye_color: RGB {
                r: 105,
                g: 90,
                b: 75,
            },
            shirt_color: RGB {
                r: 175,
                g: 165,
                b: 140,
            },
            undershirt_color: RGB {
                r: 160,
                g: 180,
                b: 215,
            },
            pants_color: RGB {
                r: 255,
                g: 230,
                b: 175,
            },
            shoes_color: RGB {
                r: 160,
                g: 105,
                b: 60,
            },
            difficulty: 0,
        }
    }
}

impl PacketBody for PlayerInfo {
    const TAG: u8 = 4;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.skin_variant);
        if self.hair_variant > MAX_HAIR_VARIANT {
            cursor.write(&0u8);
        } else {
            cursor.write(&self.hair_variant);
        }
        cursor.write(&self.name);
        cursor.write(&self.hair_dye);
        cursor.write(&self.hide_visuals[0]);
        cursor.write(&self.hide_visuals[1]);
        cursor.write(&self.hide_misc);
        cursor.write(&self.hair_color);
        cursor.write(&self.skin_color);
        cursor.write(&self.eye_color);
        cursor.write(&self.shirt_color);
        cursor.write(&self.undershirt_color);
        cursor.write(&self.pants_color);
        cursor.write(&self.shoes_color);
        cursor.write(&self.difficulty);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            skin_variant: cursor.read(),
            hair_variant: cursor.read(),
            name: cursor.read(),
            hair_dye: cursor.read(),
            hide_visuals: [cursor.read(), cursor.read()],
            hide_misc: cursor.read(),
            hair_color: cursor.read(),
            skin_color: cursor.read(),
            eye_color: cursor.read(),
            shirt_color: cursor.read(),
            undershirt_color: cursor.read(),
            pants_color: cursor.read(),
            shoes_color: cursor.read(),
            difficulty: cursor.read(),
        }
    }
}
