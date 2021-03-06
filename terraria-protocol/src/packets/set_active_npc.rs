use crate::packets::PacketBody;
use crate::SliceCursor;

/// Set the active NPC.
///
/// Direction: Server <-> Client (Sync).
#[derive(Debug)]
pub struct SetActiveNpc {
    pub player_id: u8,
    pub npc_talk_target: i16,
}

impl PacketBody for SetActiveNpc {
    const TAG: u8 = 40;

    fn write_body(&self, cursor: &mut SliceCursor) {
        cursor.write(&self.player_id);
        cursor.write(&self.npc_talk_target);
    }

    fn from_body(cursor: &mut SliceCursor) -> Self {
        Self {
            player_id: cursor.read(),
            npc_talk_target: cursor.read(),
        }
    }
}
