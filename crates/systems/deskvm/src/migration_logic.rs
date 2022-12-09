use std::collections::{BTreeSet, HashMap};

use dprocess::{
    dprocess::DProcessId,
    migration_logic::{MigrateSuggestion, MigrationLogic},
    processor::ProcessorName,
    processor_attachment::ProcessorAttachment,
    status::DProcessStatus,
    status_update::StatusUpdate,
    vm_ref::VmRef,
};

#[derive(Debug, Default)]
/// This is a migration logic that is supported officially.
/// This should have the same capability as Erlang VM's one in the future.
pub struct OfficialMigrationLogic {
    new_dprocesses: Vec<DProcessId>,
    status_updates: Vec<StatusUpdate>,
    dprocess_status: HashMap<DProcessId, DProcessStatus>,
    attachments: HashMap<ProcessorName, BTreeSet<DProcessId>>,
}

impl MigrationLogic for OfficialMigrationLogic {
    fn suggest_migration<'a>(&mut self, vm: VmRef) -> Vec<MigrateSuggestion> {
        let mut suggestions = vec![];

        // Acquire the read lock
        let processors = vm.read_processors();
        if processors.is_empty() {
            return suggestions;
        }

        // New d-process will be attached.
        let mut to_be_attached: BTreeSet<_> = self.new_dprocesses.drain(..).collect();

        // Handle exited d-processes.
        for status_update in self.status_updates.drain(..) {
            use DProcessStatus::*;
            match status_update.status {
                Running => {
                    to_be_attached.insert(status_update.dprocess_id);
                }
                Halted { .. } | Crashed(_) | Returned(_) | HaltedByLink(_) => {
                    suggestions.push(MigrateSuggestion {
                        process_id: status_update.dprocess_id,
                        to: ProcessorAttachment::Detached,
                    });
                }
                WaitingForMessage(_) | Deferred { .. } => {
                    // Keep the process attached.
                }
            }
        }

        // Count the number of running d-processes.
        let running_dprocess_count = self
            .dprocess_status
            .iter()
            .filter(|(_, status)| **status == DProcessStatus::Running)
            .count();

        // Caluculate a number of d-processes that should be attached to each processor.
        let mut assignments = vec![running_dprocess_count / processors.len(); processors.len()];
        for assignment in assignments
            .iter_mut()
            .take(running_dprocess_count % processors.len())
        {
            *assignment += 1;
        }
        debug_assert_eq!(assignments.iter().sum::<usize>(), running_dprocess_count);

        // Collect overflowed d-processes.
        let mut running_counts = HashMap::new();
        for ((name, attachments), assignment) in self.attachments.iter_mut().zip(assignments.iter())
        {
            let attached_running: Vec<_> = attachments
                .iter()
                .filter(|id| self.dprocess_status.get(id) == Some(&DProcessStatus::Running))
                .cloned()
                .collect();
            running_counts.insert(name.clone(), attached_running.len().min(*assignment));
            if attached_running.len() > *assignment {
                let overflowed = attached_running.len() - *assignment;
                for id in attached_running.into_iter().take(overflowed) {
                    attachments.remove(&id);
                    to_be_attached.insert(id);
                }
            }
        }

        // Assign d-processes to processors.
        for ((name, attachments), assignment) in self.attachments.iter_mut().zip(assignments.iter())
        {
            let running_count = running_counts.get(name).unwrap();
            for _ in 0..*assignment - running_count {
                // TODO: use pop_first() instead when it is stabilized.
                let id = to_be_attached.iter().next().unwrap().clone();
                to_be_attached.remove(&id);

                attachments.insert(id.clone());
                suggestions.push(MigrateSuggestion {
                    process_id: id,
                    to: ProcessorAttachment::Attached(name.clone()),
                });
            }
        }
        debug_assert!(to_be_attached.is_empty());

        suggestions
    }

    fn notify_new_dprocess(&mut self, dprocess_id: &DProcessId) {
        // This is required because notify_status is not called for new processes.
        self.dprocess_status
            .insert(dprocess_id.clone(), DProcessStatus::Running);
        self.new_dprocesses.push(dprocess_id.clone());
    }

    fn notify_deleted_dprocess(&mut self, dprocess_id: &DProcessId) {
        self.dprocess_status.remove(dprocess_id);
    }

    fn notify_status(&mut self, status_update: &StatusUpdate) {
        self.status_updates.push(status_update.clone());
    }

    fn notify_new_processor(&mut self, processor_name: &ProcessorName) {
        self.attachments
            .insert(processor_name.clone(), Default::default());
    }

    fn notify_deleted_processor(&mut self, processor_name: &ProcessorName) {
        self.attachments.remove(processor_name);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dprocess::{
        dprocess_manifest::DProcessManifest, processor::ProcessorManifest, value::Value,
    };
    use mir::{
        block::BasicBlock,
        mir::{ControlFlowGraph, ControlFlowGraphId, Mir},
        scope::{Scope, ScopeId},
        stmt::{Const, Stmt, StmtBind, Terminator},
        var::{Var, VarId, Vars},
    };
    use miri::try_create_miri_builder;
    use ty::Type;

    use crate::{desk_vm::DeskVm, scheduler::OfficialScheduler};

    use super::*;

    #[test]
    fn suggest_migration_for_empty_processors() {
        let logic = OfficialMigrationLogic {
            new_dprocesses: vec![],
            status_updates: vec![],
            dprocess_status: HashMap::new(),
            attachments: HashMap::new(),
        };
        let vm = DeskVm::new(logic);
        vm.spawn(&dprocess_manifest());
        // This should not panic.
        vm.run_migration_logic();
    }

    #[test]
    fn attach_new_dprocesses_to_processors() {
        let logic = OfficialMigrationLogic {
            new_dprocesses: vec![],
            status_updates: vec![],
            dprocess_status: HashMap::new(),
            attachments: HashMap::new(),
        };
        let vm = DeskVm::new(logic);
        let processor_1 = ProcessorName("processor 1".into());
        vm.add_processor(processor_manifest(processor_1.clone()));
        let processor_2 = ProcessorName("processor 2".into());
        vm.add_processor(processor_manifest(processor_2.clone()));
        let dprocess_1 = vm.spawn(&dprocess_manifest());
        let dprocess_2 = vm.spawn(&dprocess_manifest());
        vm.run_migration_logic();
        let mut attatchments = vec![
            vm.read_dprocesses()
                .get(&dprocess_1)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_2)
                .unwrap()
                .read_processor_attachment()
                .clone(),
        ];
        attatchments.sort_unstable();
        assert_eq!(
            attatchments,
            vec![
                ProcessorAttachment::Attached(processor_1),
                ProcessorAttachment::Attached(processor_2),
            ]
        );
    }

    #[test]
    fn detach_exited_dprocesses() {
        let logic = OfficialMigrationLogic {
            new_dprocesses: vec![],
            status_updates: vec![],
            dprocess_status: HashMap::new(),
            attachments: HashMap::new(),
        };
        let vm = DeskVm::new(logic);
        let processor = ProcessorName("processor 1".into());
        vm.add_processor(processor_manifest(processor.clone()));
        let dprocess_id = vm.spawn(&dprocess_manifest());
        vm.run_migration_logic();
        // Attached
        let dprocess = vm.read_dprocesses().get(&dprocess_id).unwrap().clone();
        assert_eq!(
            *dprocess.read_processor_attachment(),
            ProcessorAttachment::Attached(processor)
        );

        vm.vm_ref()
            .notify_status(&dprocess, &DProcessStatus::Returned(Arc::new(Value::Unit)));
        vm.run_migration_logic();
        assert_eq!(
            *dprocess.read_processor_attachment(),
            ProcessorAttachment::Detached
        );
    }

    #[test]
    fn distribute_dprocesses() {
        let logic = OfficialMigrationLogic {
            new_dprocesses: vec![],
            status_updates: vec![],
            dprocess_status: HashMap::new(),
            attachments: HashMap::new(),
        };
        let vm = DeskVm::new(logic);
        let processor_1 = ProcessorName("processor 1".into());
        vm.add_processor(processor_manifest(processor_1.clone()));
        let processor_2 = ProcessorName("processor 2".into());
        vm.add_processor(processor_manifest(processor_2.clone()));
        let dprocess_1 = vm.spawn(&dprocess_manifest());
        let dprocess_2 = vm.spawn(&dprocess_manifest());
        let dprocess_3 = vm.spawn(&dprocess_manifest());
        let dprocess_4 = vm.spawn(&dprocess_manifest());
        vm.run_migration_logic();
        let mut attatchments = vec![
            vm.read_dprocesses()
                .get(&dprocess_1)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_2)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_3)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_4)
                .unwrap()
                .read_processor_attachment()
                .clone(),
        ];
        attatchments.sort_unstable();
        assert_eq!(
            attatchments,
            vec![
                ProcessorAttachment::Attached(processor_1.clone()),
                ProcessorAttachment::Attached(processor_1.clone()),
                ProcessorAttachment::Attached(processor_2.clone()),
                ProcessorAttachment::Attached(processor_2.clone()),
            ]
        );

        // Create new processor
        let processor_3 = ProcessorName("processor 3".into());
        vm.add_processor(processor_manifest(processor_3.clone()));

        // Delete d-process 1
        vm.delete_dprocess(&dprocess_1);

        vm.run_migration_logic();
        let mut attatchments = vec![
            vm.read_dprocesses()
                .get(&dprocess_2)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_3)
                .unwrap()
                .read_processor_attachment()
                .clone(),
            vm.read_dprocesses()
                .get(&dprocess_4)
                .unwrap()
                .read_processor_attachment()
                .clone(),
        ];
        attatchments.sort_unstable();
        assert_eq!(
            attatchments,
            vec![
                ProcessorAttachment::Attached(processor_1),
                ProcessorAttachment::Attached(processor_2),
                ProcessorAttachment::Attached(processor_3),
            ]
        );
    }

    fn dprocess_manifest() -> DProcessManifest {
        DProcessManifest::new(
            try_create_miri_builder(
                Mir {
                    entrypoint: ControlFlowGraphId(0),
                    cfgs: vec![ControlFlowGraph {
                        parameters: vec![],
                        captured: vec![],
                        output: Type::Real,
                        vars: Vars(vec![Var {
                            ty: Type::Real,
                            scope: ScopeId(0),
                        }]),
                        scopes: vec![Scope { super_scope: None }],
                        blocks: vec![BasicBlock {
                            stmts: vec![StmtBind {
                                var: VarId(0),
                                stmt: Stmt::Const(Const::Int(1)),
                            }],
                            terminator: Terminator::Return(VarId(0)),
                        }],
                        links: vec![],
                    }],
                },
                &Default::default(),
            )
            .unwrap(),
            Default::default(),
            Default::default(),
        )
    }

    fn processor_manifest(name: ProcessorName) -> ProcessorManifest {
        ProcessorManifest::new(name, OfficialScheduler::default(), Default::default())
    }
}
