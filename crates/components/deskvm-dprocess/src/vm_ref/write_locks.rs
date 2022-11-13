use std::{collections::HashMap, ops::DerefMut, sync::Arc};

use crate::{
    dprocess::{DProcess, DProcessId},
    migration_logic::MigrationLogic,
    status_update::StatusUpdate,
};

use super::VmRef;

// These must be private to prevent deadlocks.
impl<'a> VmRef<'a> {
    pub(crate) fn lock_dprocesses(
        &self,
    ) -> impl DerefMut<Target = HashMap<DProcessId, Arc<DProcess>>> + '_ {
        self.dprocesses.write()
    }

    pub(crate) fn lock_migration_logic(
        &self,
    ) -> impl DerefMut<Target = Box<dyn MigrationLogic>> + '_ {
        self.migration_logic.write()
    }

    pub(crate) fn lock_status_update(&self) -> impl DerefMut<Target = Vec<StatusUpdate>> + '_ {
        self.status_update.write()
    }
}
