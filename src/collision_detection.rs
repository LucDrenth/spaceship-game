use bevy::{prelude::*, utils::HashMap};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

#[derive(Component)]
pub struct Collider {
    radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut collisions = HashMap::<Entity, Vec<Entity>>::new();

    for (entity, transform, collider) in query.iter() {
        for (target_entity, target_transform, target_collider) in query.iter() {
            if entity == target_entity {
                // prevent entity from colliding with itself
                continue;
            }

            let distance = transform
                .translation()
                .distance(target_transform.translation());

            if distance <= collider.radius + target_collider.radius {
                collisions
                    .entry(entity)
                    .or_insert_with(Vec::new)
                    .push(target_entity);
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        match collisions.get(&entity) {
            Some(list) => collider.colliding_entities = list.clone(),
            None => collider.colliding_entities = vec![],
        }
    }
}
