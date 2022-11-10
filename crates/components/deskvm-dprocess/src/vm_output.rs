use crate::{dprocess::DProcessId, exit_status::ExitStatus};
use types::Effect;

#[derive(Debug, PartialEq)]
pub enum VmOutput {
    EffectPerformed {
        dprocess_id: DProcessId,
        effect: Effect,
    },
    ProcessExited {
        dprocess_id: DProcessId,
        exit_status: ExitStatus,
    },
}

#[derive(Debug, PartialEq, Default)]
pub struct VmOutputs(pub Vec<VmOutput>);

impl VmOutputs {
    pub fn merge(outputs: impl IntoIterator<Item = VmOutputs>) -> Self {
        outputs
            .into_iter()
            .fold(VmOutputs(vec![]), |mut acc, outputs| {
                acc.0.extend(outputs.0);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use types::Type;

    use super::*;

    #[test]
    fn merges_outputs() {
        let dprocess_a = DProcessId::new();
        let dprocess_b = DProcessId::new();
        let dprocess_c = DProcessId::new();
        let dprocess_d = DProcessId::new();
        assert_eq!(
            VmOutputs::merge(vec![
                VmOutputs(vec![
                    VmOutput::EffectPerformed {
                        dprocess_id: dprocess_a.clone(),
                        effect: Effect {
                            input: Type::Number,
                            output: Type::String,
                        },
                    },
                    VmOutput::ProcessExited {
                        dprocess_id: dprocess_b.clone(),
                        exit_status: ExitStatus::Finished,
                    },
                ]),
                VmOutputs(vec![
                    VmOutput::EffectPerformed {
                        dprocess_id: dprocess_c.clone(),
                        effect: Effect {
                            input: Type::String,
                            output: Type::Number,
                        },
                    },
                    VmOutput::ProcessExited {
                        dprocess_id: dprocess_d.clone(),
                        exit_status: ExitStatus::Finished,
                    },
                ]),
            ]),
            VmOutputs(vec![
                VmOutput::EffectPerformed {
                    dprocess_id: dprocess_a,
                    effect: Effect {
                        input: Type::Number,
                        output: Type::String,
                    },
                },
                VmOutput::ProcessExited {
                    dprocess_id: dprocess_b,
                    exit_status: ExitStatus::Finished,
                },
                VmOutput::EffectPerformed {
                    dprocess_id: dprocess_c,
                    effect: Effect {
                        input: Type::String,
                        output: Type::Number,
                    },
                },
                VmOutput::ProcessExited {
                    dprocess_id: dprocess_d,
                    exit_status: ExitStatus::Finished,
                },
            ])
        );
    }
}
