use crate::gui::ControlMessage;
use crate::World;
use nalgebra::{vector, SMatrix};
use rapier3d::crossbeam;
use rapier3d::prelude::{
    ActiveEvents, BroadPhaseMultiSap, CCDSolver,
    ChannelEventCollector, ColliderBuilder, ColliderSet, ImpulseJointSet,
    IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase,
    PhysicsPipeline, QueryPipeline, RigidBodyBuilder, RigidBodySet,
};
use std::time::Duration;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::{mpsc, watch};
use tokio::time::{interval, Instant};
use crate::world::celestials::Celestial;
use crate::world::spaceship::Spaceship;

struct Rapier {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
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
        let (contact_force_send, _) = crossbeam::channel::unbounded();

        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
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

impl Rapier {
    pub fn add_celestial(&mut self, c: &mut Celestial) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(c.pos().into())
            .linvel(c.vel().into())
            .additional_mass(c.mass() as f32)
            .build();
        let collider = ColliderBuilder::ball(c.rad() as f32)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(
            collider,
            handle,
            &mut self.rigid_body_set,
        );
        c.set_rapier_handle(handle);
    }

    pub fn add_spaceship(&mut self, s: &mut Spaceship) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(s.pos().into())
            .linvel(s.vel().into())
            .additional_mass(s.mass() as f32)
            .build();
        let collider = ColliderBuilder::cuboid(0.5, 0.5, 0.5)
            .restitution(0.7)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(
            collider,
            handle,
            &mut self.rigid_body_set,
        );
        s.set_rapier_handle(handle);
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
        mut world: World,
        simulation_fps: u32,
        time_speed: f64,
        control: mpsc::Receiver<ControlMessage>,
    ) -> (Self, watch::Receiver<World>) {
        let (world_publisher, world_watch) = watch::channel(world.clone());
        let delta_t = time_speed / simulation_fps as f64;
        let mut rapier = Rapier::default();
        rapier.integration_parameters.dt = delta_t as f32;

        for c in world.celestials.get_mut().values_mut() {
            rapier.add_celestial(c);
        }

        for s in &mut world.spaceships.values_mut() {
            rapier.add_spaceship(s);
        }

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

            for (_, c) in self.world.celestials.get() {
                let handle = c.rapier_handle.unwrap();
                let body = self.rapier.rigid_body_set.get_mut(handle).unwrap();
                let a = self
                    .world
                    .celestials
                    .get_global_acceleration(c.pos());
                body.reset_forces(true);
                body.add_force(a.into(), true);
            }

            for s in &mut self.world.spaceships.values_mut() {
                let handle = s.rapier_handle.unwrap();
                let body = self.rapier.rigid_body_set.get_mut(handle).unwrap();
                let a = self
                    .world
                    .celestials
                    .get_global_acceleration(s.pos());
                body.reset_forces(true);
                body.add_force(a.into(), true);
            }

            self.rapier.physics_pipeline.step(
                &self.rapier.gravity,
                &self.rapier.integration_parameters,
                &mut self.rapier.island_manager,
                &mut self.rapier.broad_phase,
                &mut self.rapier.narrow_phase,
                &mut self.rapier.rigid_body_set,
                &mut self.rapier.collider_set,
                &mut self.rapier.impulse_joint_set,
                &mut self.rapier.multibody_joint_set,
                &mut self.rapier.ccd_solver,
                Some(&mut self.rapier.query_pipeline),
                &self.rapier.physics_hooks,
                &self.rapier.event_handler,
            );

            for c in self.world.celestials.get_mut().values_mut() {
                let handle = c.rapier_handle.unwrap();
                let body = self.rapier.rigid_body_set.get(handle).unwrap();
                let pos = body.position().translation.into();
                c.set_pos(pos);
            }

            for s in &mut self.world.spaceships.values_mut() {
                let handle = s.rapier_handle.unwrap();
                let body = self.rapier.rigid_body_set.get(handle).unwrap();
                let pos = body.position().translation.into();
                s.set_pos(pos);
            }

            self.world_publisher
                .send(self.world.clone())
                .map_err(|e| format!("World publisher died: {}", e))?;
        }
    }
}
