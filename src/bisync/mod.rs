// Bidirectional synchronization
//
// Enables two-way sync with conflict detection and resolution.

pub mod classifier;
pub mod engine;
pub mod lock;
pub mod resolver;
pub mod state;

pub use classifier::{Change, ChangeType, classify_changes};
pub use engine::{BisyncEngine, BisyncOptions};
#[allow(unused_imports)]
pub(crate) use engine::{BisyncResult, BisyncStats, ConflictInfo};
pub use lock::SyncLock;
pub use resolver::{ConflictResolution, ResolvedChanges, SyncAction, conflict_filename, resolve_changes};
pub use state::{BisyncStateDb, Side, SyncState};
