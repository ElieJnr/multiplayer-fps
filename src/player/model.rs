#[derive(Resource)]
pub struct PreloadedPlayerAnimations {
    pub model: Handle<Scene>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub player_entity: Entity,
    pub animation_player_entity: Option<Entity>,
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct Player;

// pub enum Animation {
//     Idle,
//     Run,
//     BackwardRun,
//     Shoot,
//     Reload
// }

// pub struct AnimationStr{
    
// }

// impl Animation{
//     fn create_animation()->Self{

//     }
// }