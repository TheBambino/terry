use crate::packets::PacketBody;
use crate::SliceCursor;

/// Display doll item sync.
///
/// Direction: Client <-> Server.
#[derive(Debug)]
pub struct DollSync {
    pub player_id: u8,
    pub tileentity_id: i32,
    pub item_index: u8,
    pub item_id: u16,
    pub stack: u16,
    pub prefix: u8,
}

impl PacketBody for DollSync {
    const TAG: u8 = 121;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.tileentity_id);
        cursor.write(&self.item_index);
        cursor.write(&self.item_id);
        cursor.write(&self.stack);
        cursor.write(&self.prefix);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            tileentity_id: cursor.read(),
            item_index: cursor.read(),
            item_id: cursor.read(),
            stack: cursor.read(),
            prefix: cursor.read(),
        }
    }
}
