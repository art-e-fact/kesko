use std::{
    collections::HashMap,
    ops::RangeInclusive
};

use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui}, EguiContext};

use nora_object_interaction::event::SelectEvent;
use nora_physics::{
    multibody::MultibodyRoot,
    joint::{
        Joint,
        JointAxis,
        JointLimits,
        JointType, 
        JointMotorEvent,
        MotorAction
    },
};

use crate::interaction::multibody_selection::MultibodySelectionEvent;


// Joint data to store relevant information and values to display and control the joints
#[derive(Debug)]
struct JointData {
    name: String,
    joint_type: JointType,
    val_axis_1: f32,
    val_axis_2: f32,
    val_axis_3: f32,
    current_val_axis_1: f32,
    current_val_axis_2: f32,
    current_val_axis_3: f32,
    limits: Option<JointLimits<f32>>
}


/// UI Component that handles showing multibody information and also controlling their joints
#[derive(Component, Default)]
pub(crate) struct MultibodyUIComponent {
    ui_open: bool,
    multibody_root: Option<Entity>,
    multibody_name: Option<String>,
    multibody_joints: Option<HashMap<Entity, JointData>>,
}

impl MultibodyUIComponent {

    pub(crate) fn show_system(
        mut multibody_select_event_reader: EventReader<MultibodySelectionEvent>,
        mut select_event_writer: EventWriter<SelectEvent>,
        mut joint_motor_event_writer: EventWriter<JointMotorEvent>,
        mut egui_context: ResMut<EguiContext>,
        mut comp: Query<&mut Self>,
        body_query: Query<&MultibodyRoot>,
        joint_query: Query<&Joint>
    ) {

        // get a reference to 'self'
        let mut comp = comp.get_single_mut().expect("We should have a component");

        // update if we have a multibody selected or not
        comp.parse_events(&body_query, &mut multibody_select_event_reader);

        // We have a body selected -> update joint data
        if let Some(root_entity) = comp.multibody_root {
            if let Ok(root) = body_query.get(root_entity) {

                if comp.multibody_joints.is_none() {
                    // Build map with relevant joint data to be displayed
                    let mut joints = HashMap::<Entity, JointData>::new();
                    for (name, entity) in root.joint_name_2_entity.iter() {
                        if let Ok(joint) = joint_query.get(*entity) {

                            // check which type of joint we are dealing with and add it to the component
                            let joint_type = joint.get_type();
                            match joint_type {
                                JointType::Revolute => {
                                    joints.insert(*entity, JointData {
                                        name: name.clone(),
                                        joint_type, 
                                        val_axis_1: 0.0,
                                        val_axis_2: 0.0,
                                        val_axis_3: 0.0,
                                        current_val_axis_1: 0.0,
                                        current_val_axis_2: 0.0,
                                        current_val_axis_3: 0.0,
                                        limits: joint.get_limits(),
                                    });
                                },
                                JointType::Spherical => {
                                    joints.insert(*entity, JointData {
                                        name: name.clone(),
                                        joint_type, 
                                        val_axis_1: 0.0,
                                        val_axis_2: 0.0,
                                        val_axis_3: 0.0,
                                        current_val_axis_1: 0.0,
                                        current_val_axis_2: 0.0,
                                        current_val_axis_3: 0.0,
                                        // TODO: Add here
                                        limits: None
                                    });
                                },
                                JointType::Prismatic => {
                                    joints.insert(*entity, JointData {
                                        name: name.clone(),
                                        joint_type, 
                                        val_axis_1: 0.0,
                                        val_axis_2: 0.0,
                                        val_axis_3: 0.0,
                                        current_val_axis_1: 0.0,
                                        current_val_axis_2: 0.0,
                                        current_val_axis_3: 0.0,
                                        limits: joint.get_limits()
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                    comp.multibody_joints = Some(joints);

                } else if let Some(joints) = &mut comp.multibody_joints {
                    // just update the component data with current joint values
                    for (joint_entity, joint_data) in joints.iter_mut() {
                        if let Ok(joint) = joint_query.get(*joint_entity) {
                            joint_data.current_val_axis_1 = joint.get_rot_x();
                            joint_data.current_val_axis_2 = joint.get_rot_y();
                            joint_data.current_val_axis_3 = joint.get_rot_z();
                        }
                    }
                }
            }
        }
        
        // run ui logic
        comp.show(egui_context.ctx_mut(), &mut select_event_writer, &mut joint_motor_event_writer);

    }

    /// read possible events that a multibody has been selected/deselected and update component state
    fn parse_events(&mut self, body_query: &Query<&MultibodyRoot>, multibody_select_event_reader: &mut EventReader<MultibodySelectionEvent>) {
        for event in multibody_select_event_reader.iter() {
            match event {
                MultibodySelectionEvent::Selected(root_entity) => {
                    self.ui_open = true;
                    self.multibody_root = Some(*root_entity);
                    if let Ok(root) = body_query.get(*root_entity) {
                        self.multibody_name = Some(root.name.clone());
                    }
                    self.multibody_joints = None;
                },
                MultibodySelectionEvent::Deselected(root_entity) => {
                    // make sure we are only closing when the correct entity is deselected
                    if Some(*root_entity) == self.multibody_root {
                        self.ui_open = false;
                        self.multibody_root = None;
                        self.multibody_name = None;
                        self.multibody_joints = None;
                    }
                }
            }
        }
    }

    /// UI logic
    fn show(
        &mut self, 
        ctx: &egui::Context, 
        select_event_writer: &mut EventWriter<SelectEvent>, 
        joint_motor_event_writer: &mut EventWriter<JointMotorEvent>
    ) {
        let Self {
            ui_open,
            multibody_root,
            multibody_name,
            multibody_joints,
        } = self;

        egui::Window::new("Multibody")
            .open(ui_open)
            .resizable(true)
            .hscroll(false)
            .vscroll(true)
            .title_bar(true)
            .show(ctx, |ui| {

                ui.vertical(|ui| {

                    // display name
                    if let Some(name) = &multibody_name {
                        ui.heading("Name");
                        ui.label(format!("{}", name));
                        ui.separator();
                    }

                    // display joints and controls
                    if let Some(joints) = multibody_joints {
                        ui.vertical(|ui| {
                            ui.heading("Joints");
                            egui::Grid::new("joint_grid")
                            .num_columns(7)
                            .spacing([20.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {

                                // headers
                                ui.label("Name");
                                ui.label("Type");
                                ui.label("X");
                                ui.label("Y");
                                ui.label("Z");
                                ui.label("Set value");
                                ui.end_row();

                                // joint rows
                                for (joint_entity, joint_data) in joints.iter_mut() {

                                    ui.label(joint_data.name.clone());
                                    ui.label(format!("{:?}", joint_data.joint_type));
                                    ui.label(format!("{:.1}", Self::smooth_joint_val(joint_data.current_val_axis_1.to_degrees())));
                                    ui.label(format!("{:.1}", Self::smooth_joint_val(joint_data.current_val_axis_2.to_degrees())));
                                    ui.label(format!("{:.1}", Self::smooth_joint_val(joint_data.current_val_axis_3.to_degrees())));

                                    match joint_data.joint_type {
                                        JointType::Revolute => {
                                            Self::revolute_slider(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        },
                                        JointType::Spherical => {
                                            // TODO: Spherical joints seems to be broken in rapier at the moment, implement when fixed
                                            // Self::spherical_sliders(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        },
                                        JointType::Prismatic => { 
                                            Self::prismatic_slider(ui, joint_entity, joint_data, joint_motor_event_writer);
                                        }
                                        _ => {}
                                    }
                                    ui.end_row();
                                }
                            });
                        });
                        ui.separator();
                    }

                    // close button
                    if ui.button("Close And Deselect").clicked() {
                        if let Some(entity) = multibody_root {
                            select_event_writer.send(SelectEvent::Deselect(*entity))
                        }
                    }
                });
            });
    }
    
    /// Add slider for a revolute joint
    fn revolute_slider(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").step_by(0.1)).changed() {
            // send motor action if the value has changed
            joint_motor_event_writer.send(JointMotorEvent {
                entity: *joint_entity,
                action: MotorAction::PositionRevolute { 
                    position: joint_data.val_axis_1.to_radians(), 
                    damping: 0.1, 
                    stiffness: 1.0 
                }
            });
        }
    }

    fn prismatic_slider(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("m").step_by(0.01)).changed() {
            // send motor action if the value has changed
            joint_motor_event_writer.send(JointMotorEvent {
                entity: *joint_entity,
                action: MotorAction::PositionPrismatic { position: joint_data.val_axis_1, damping: 0.1, stiffness: 1.0 }
            });
        }
    }

    /// Add sliders for a spherical joints to control x, y and z axis and send motor actions
    fn spherical_sliders(ui: &mut Ui, joint_entity: &Entity, joint_data: &mut JointData, joint_motor_event_writer: &mut EventWriter<JointMotorEvent>) {
        ui.horizontal(|ui| {
            // X
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_1, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("X")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.val_axis_1.to_radians(),
                        axis: JointAxis::AngX,
                        damping: 0.1, 
                        stiffness: 2.0 
                    }
                });
            }
            // Y
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_2, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("Y")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.val_axis_2.to_radians(),
                        axis: JointAxis::AngY,
                        damping: 0.1, 
                        stiffness: 2.0 
                    }
                });
            }
            // Z
            if ui.add(egui::Slider::new(&mut joint_data.val_axis_3, Self::get_slider_range(joint_data.limits, &joint_data.joint_type)).suffix("°").text("Z")).changed() {
                joint_motor_event_writer.send(JointMotorEvent {
                    entity: *joint_entity,
                    action: MotorAction::PositionSpherical { 
                        position: joint_data.val_axis_3.to_radians(),
                        axis: JointAxis::AngZ,
                        damping: 0.1, 
                        stiffness: 2.0 
                    }
                });
            }
        });
    }

    /// Create a slider range from limits
    fn get_slider_range(limits: Option<JointLimits<f32>>, joint_type: &JointType) -> RangeInclusive<f32> {

        match joint_type {
            &JointType::Revolute => {
                match limits {
                    Some(limits) => RangeInclusive::<f32>::new(limits.min.to_degrees(), limits.max.to_degrees()),
                    None => RangeInclusive::<f32>::new(-180.0, 180.0)
                }
            },
            &JointType::Prismatic => {
                match limits {
                    Some(limits) => RangeInclusive::<f32>::new(limits.min, limits.max),
                    None => RangeInclusive::<f32>::new(-1.0, 1.0)
                }
            }
            _ => {
                RangeInclusive::<f32>::new(-180.0, 180.0)
            }
        }
    }

    /// to smoothen the displayed joint values since they can often be -0.0/0.0 which
    /// result in a very erratic UI behavior
    fn smooth_joint_val(val: f32) -> f32 {
        if val.abs() < 1e-2 && val.is_sign_negative(){
            return 0.0
        }
        val
    }
}
