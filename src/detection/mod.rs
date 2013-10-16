pub use detection::collision::bodies_bodies::BodiesBodies;
pub use BodiesBodiesDispatcher = detection::collision::bodies_bodies::Dispatcher;
pub use detection::joint::joint_manager::JointManager;
pub use detection::island_activation_manager::IslandActivationManager;

// modules
pub mod constraint;
pub mod detector;

pub mod collision {
    pub mod bodies_bodies;
}

pub mod joint {
    pub mod joint_manager;
    pub mod anchor;
    pub mod ball_in_socket;
    pub mod fixed;
}

pub mod island_activation_manager;