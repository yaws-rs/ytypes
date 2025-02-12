use capacity::{Bounded, Capacity, CapacityIndicator, Setting};

#[derive(Clone, Debug)]
pub struct CapacityHolder;

#[derive(Clone, Debug)]
pub enum CapacityKind {
    CoreQueue,
    RegisteredFd,
    PendingCompletions,
    Buffers,
    Futexes,
}

impl Bounded<CapacityKind> for CapacityHolder {
    fn maximum(&self, v: &CapacityKind) -> usize {
        match v {
            _ => 64,
        }
    }
    fn minimum(&self, v: &CapacityKind) -> usize {
        match v {
            _ => 1,
        }
    }
}

impl Setting<CapacityKind> for CapacityHolder {
    fn setting(&self, v: &CapacityKind) -> usize {
        match v {
            CapacityKind::CoreQueue => 1,
            CapacityKind::RegisteredFd => 2,
            CapacityKind::PendingCompletions => 3,
            CapacityKind::Buffers => 4,
            CapacityKind::Futexes => 5,
        }
    }
}

fn main() {
    let cap = Capacity::<CapacityHolder, CapacityKind>::with_planned(CapacityHolder {});

    assert_eq!(cap.of_unbounded(&CapacityKind::CoreQueue), 1);
    assert_eq!(
        cap.of_bounded(&CapacityKind::CoreQueue),
        CapacityIndicator::WithinBounds(1)
    );
    assert_eq!(cap.maximum(&CapacityKind::CoreQueue), 64);
}
