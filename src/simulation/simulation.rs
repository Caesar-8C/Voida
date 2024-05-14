use crate::gui::ControlMessage;
use crate::World;
use nalgebra::{SMatrix, vector};
use rapier3d::crossbeam;
use rapier3d::prelude::{ActiveEvents, BroadPhase, BroadPhaseMultiSap, CCDSolver, ChannelEventCollector, ColliderBuilder, ColliderSet, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline, QueryPipeline, RigidBodyBuilder, RigidBodySet};
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, watch};
use tokio::time::{interval, Instant};

struct Rapier {
    pub gravity: SMatrix<f32, 3, 1>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhaseMultiSap,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub physics_hooks: (),
    pub event_handler: ChannelEventCollector,
}

impl Default for Rapier {
    fn default() -> Self {
        let (collision_send, _) = crossbeam::channel::unbounded();
        let (contact_force_send, _) =
            crossbeam::channel::unbounded();

        Self {
            gravity: vector![0.0, 0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            event_handler: ChannelEventCollector::new(
                collision_send,
                contact_force_send,
            ),
        }
    }
}

pub struct Simulation {
    world: World,
    world_publisher: watch::Sender<World>,
    control: mpsc::Receiver<ControlMessage>,
    time_speed: f64,
    simulation_fps: u32,
    delta_t: f64,
    rapier: Rapier,
}

impl Simulation {
    pub fn new(
        world: World,
        simulation_fps: u32,
        time_speed: f64,
        control: mpsc::Receiver<ControlMessage>,
    ) -> (Self, watch::Receiver<World>) {
        let (world_publisher, world_watch) = watch::channel(world.clone());
        let delta_t = time_speed / simulation_fps as f64;
        let mut rapier = Rapier::default();
        rapier.integration_parameters.dt = delta_t as f32;

        (
            Self {
                world,
                world_publisher,
                control,
                time_speed,
                simulation_fps,
                delta_t,
                rapier,
            },
            world_watch,
        )
    }

    pub async fn spin(&mut self) -> Result<(), String> {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![30.0, 4.0, 0.0])
            .build();
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let body_1_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(
            collider,
            body_1_handle,
            &mut rigid_body_set,
        );

        let mut interval =
            interval(Duration::from_secs_f64(1. / self.simulation_fps as f64));

        let mut start = Instant::now();
        let mut fps_counter = 0;

        loop {
            interval.tick().await;
            fps_counter += 1;
            if start.elapsed().as_secs() >= 1 {
                self.world.true_sim_fps = fps_counter;
                start = Instant::now();
                fps_counter = 0;
            }

            loop {
                match self.control.try_recv() {
                    Ok(ControlMessage::Shutdown)
                    | Err(TryRecvError::Disconnected) => return Ok(()),
                    Ok(ControlMessage::Speedup) => {
                        self.world
                            .spaceships
                            .get_mut("ISS2")
                            .unwrap()
                            .speedup();
                    }
                    Ok(ControlMessage::SetTimeSpeed(speed)) => {
                        self.time_speed = speed;
                        self.delta_t =
                            self.time_speed / self.simulation_fps as f64;
                    }
                    _ => break,
                }
            }

            for spaceship in self.world.spaceships.values_mut() {
                let a = self
                    .world
                    .celestials
                    .get_global_acceleration(spaceship.pos());
                spaceship.apply_gravity(a, self.delta_t);
            }
            self.world.celestials.update(self.delta_t);

            self.world_publisher
                .send(self.world.clone())
                .map_err(|e| format!("World publisher died: {}", e))?;

            self.rapier.physics_pipeline.step(
                &self.rapier.gravity,
                &self.rapier.integration_parameters,
                &mut self.rapier.island_manager,
                &mut self.rapier.broad_phase,
                &mut self.rapier.narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut self.rapier.impulse_joint_set,
                &mut self.rapier.multibody_joint_set,
                &mut self.rapier.ccd_solver,
                Some(&mut self.rapier.query_pipeline),
                &self.rapier.physics_hooks,
                &self.rapier.event_handler,
            );
        }
    }
}
