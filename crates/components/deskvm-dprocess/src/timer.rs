use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerManifest {
    pub name: String,
    pub ty: TimerType,
    pub duration: Duration,
    pub time_kind: TimeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timer {
    ty: TimerType,
    duration: Duration,
    time_kind: TimeKind,
    ellapsed: Duration,
    counter: u64,
    unhandled_events: Vec<TimerEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimerType {
    /// Repeated(0) is one-shot timer.
    Repeated(u64),
    Infinite,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// The kinds of time like ones in the `time` command in Linux.
///
/// Finer things first, bigger things later.
pub enum TimeKind {
    /// The time reduced by the interpreter.
    Interpreter,
    /// The time reduced by the attached processor.
    Processor,
    /// The time reduced by the VM.
    Vm,
    /// the time passed in the real world.
    Real,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimerEvent {
    Ring(u64),
    Finished,
}

impl Timer {
    pub fn new(manifest: TimerManifest) -> Self {
        Timer {
            ty: manifest.ty,
            duration: manifest.duration,
            time_kind: manifest.time_kind,
            ellapsed: Duration::new(0, 0),
            counter: 0,
            unhandled_events: Vec::new(),
        }
    }

    pub fn ty(&self) -> &TimerType {
        &self.ty
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn time_kind(&self) -> &TimeKind {
        &self.time_kind
    }

    pub fn ellapsed(&self) -> &Duration {
        &self.ellapsed
    }

    pub fn tick(&mut self, duration: Duration) {
        self.ellapsed += duration;
        if self.ellapsed >= self.duration {
            match self.ty {
                TimerType::Repeated(time) => {
                    let count = self.ellapsed.as_nanos() / self.duration.as_nanos();
                    let max_rings = (time + 1) - self.counter;
                    let rings = count.min(max_rings as u128);
                    if rings != 0 {
                        for _ in 0..rings {
                            self.unhandled_events.push(TimerEvent::Ring(self.counter));
                            self.counter += 1;
                        }
                        if self.counter > time {
                            self.unhandled_events.push(TimerEvent::Finished);
                        }
                    }
                }
                TimerType::Infinite => {
                    let count = self.ellapsed.as_nanos() / self.duration.as_nanos();
                    for _ in 0..count {
                        self.unhandled_events.push(TimerEvent::Ring(self.counter));
                        self.counter += 1;
                    }
                }
            };
            self.ellapsed = Duration::default();
        }
    }

    pub fn dequeue_events(&mut self) -> Vec<TimerEvent> {
        let mut ret = Vec::new();
        std::mem::swap(&mut self.unhandled_events, &mut ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(0),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        assert_eq!(timer.ellapsed(), &Duration::new(0, 0));
    }

    #[test]
    fn dequeue_events() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(0),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.unhandled_events.push(TimerEvent::Ring(0));
        timer.unhandled_events.push(TimerEvent::Ring(1));
        assert_eq!(
            timer.dequeue_events(),
            vec![TimerEvent::Ring(0), TimerEvent::Ring(1)]
        );
        assert!(timer.unhandled_events.is_empty());
    }

    #[test]
    fn tick_adds_ellapsed() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(0),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_millis(100));
        assert_eq!(timer.ellapsed(), &Duration::from_millis(100));
        timer.tick(Duration::from_millis(50));
        assert_eq!(timer.ellapsed(), &Duration::from_millis(150));
    }

    #[test]
    fn tick_finishes_oneshot() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(0),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_secs(1));
        assert_eq!(
            timer.dequeue_events(),
            vec![TimerEvent::Ring(0), TimerEvent::Finished]
        );
    }

    #[test]
    fn ring_and_finish_occurs_once_for_oneshot() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(0),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_secs(3));
        assert_eq!(
            timer.dequeue_events(),
            vec![TimerEvent::Ring(0), TimerEvent::Finished]
        );
        timer.tick(Duration::from_secs(1));
        assert_eq!(timer.dequeue_events(), vec![]);
    }

    #[test]
    fn infinite_rings() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Infinite,
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_secs(3));
        assert_eq!(
            timer.dequeue_events(),
            vec![
                TimerEvent::Ring(0),
                TimerEvent::Ring(1),
                TimerEvent::Ring(2)
            ]
        );
        timer.tick(Duration::from_secs(1));
        assert_eq!(timer.dequeue_events(), vec![TimerEvent::Ring(3)]);
    }

    #[test]
    fn repeated_rings() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(2),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_secs(2));
        assert_eq!(
            timer.dequeue_events(),
            vec![TimerEvent::Ring(0), TimerEvent::Ring(1)]
        );
        timer.tick(Duration::from_secs(1));
        assert_eq!(
            timer.dequeue_events(),
            vec![TimerEvent::Ring(2), TimerEvent::Finished]
        );
    }

    #[test]
    fn repeated_does_not_exceed_the_time() {
        let mut timer = Timer::new(TimerManifest {
            name: "test".to_string(),
            ty: TimerType::Repeated(2),
            duration: Duration::from_secs(1),
            time_kind: TimeKind::Interpreter,
        });
        timer.tick(Duration::from_secs(5));
        assert_eq!(
            timer.dequeue_events(),
            vec![
                TimerEvent::Ring(0),
                TimerEvent::Ring(1),
                TimerEvent::Ring(2),
                TimerEvent::Finished
            ]
        );
    }
}
