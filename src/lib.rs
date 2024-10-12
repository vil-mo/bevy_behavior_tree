use bevy::{ecs::query::ReadOnlyQueryData, prelude::Component};


#[derive(Component)]
pub struct BehaviorTree<T: Node> {
    pub root: T,
}

pub trait Node {
    type Data: ReadOnlyQueryData;

    fn tick(&mut self, data: &mut Self::Data);
}