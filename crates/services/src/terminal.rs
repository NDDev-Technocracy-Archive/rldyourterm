// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Danil Silantyev, Global CEO NDDev. on.nddev.it.com (OpenNetwork)

pub use rldyourterm_core::{
    ANSI_PALETTE, Attrs, CELL_HEIGHT, CELL_WIDTH, Cell, CellText, Color, Cursor, DEFAULT_BG,
    DEFAULT_FG, Grid, MAX_FEED_BYTES_PER_CALL, MAX_SCROLLBACK_CAP as DEFAULT_SCROLLBACK_CAP,
    MouseFormat, MouseMode, Parser, SearchError, SearchMatch, TerminalState, TerminalTheme,
    UnderlineStyle, color_to_u32,
};
// Key encoding lives in the engine; features reach it through this layer, as
// the VSA dependency graph (scripts/ci/validate_vsa_dependency_graph.sh)
// allows features -> services -> core but not features -> core.
pub use rldyourterm_core::{
    RuntimeKey, RuntimeKeyEvent, RuntimeKeyEventKind, RuntimeKeyModifiers, TerminalModeFlags,
    encode_runtime_key_event,
};
