use bevy::prelude::*;
use rapier3d::prelude::*;

use crate::conversions::IntoRapier;

use super::{AsAnyJoint, JointTrait};


pub struct RevoluteJoint {
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub axis: Vec3,
    pub limits: Option<Vec2>,
    pub damping: f32,
    pub stiffness: f32,
    pub max_motor_force: Real
}

impl AsAnyJoint for RevoluteJoint {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Default for RevoluteJoint {
    fn default() -> Self {
        Self { 
            parent_anchor: Transform::default(), 
            child_anchor: Transform::default(), 
            axis: Vec3::X, 
            limits: None,
            damping: 0.0,
            stiffness: 0.0,
            max_motor_force: Real::MAX
        }
    }
}

impl JointTrait for RevoluteJoint {
    fn parent_anchor(&self) -> Transform {
        self.parent_anchor
    }
    fn child_anchor(&self) -> Transform {
        self.child_anchor
    }
}

impl From<RevoluteJoint> for GenericJoint {
    fn from(joint: RevoluteJoint) -> GenericJoint {
        
        let mut builder = RevoluteJointBuilder::new(joint.axis.into_rapier())
            .local_anchor1(joint.parent_anchor.translation.into_rapier())
            .local_anchor2(joint.child_anchor.translation.into_rapier());

        if joint.stiffness > 0.0 || joint.damping > 0.0 {
            builder = builder.motor(0.0, 0.0, joint.stiffness, joint.damping).motor_max_force(joint.max_motor_force);
        }

        if let Some(limits) = joint.limits {
            builder = builder.limits(limits.into());
        }

        let mut generic: GenericJoint = builder.into();
        generic.local_frame1.rotation = joint.parent_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic.local_frame1.rotation = joint.child_anchor.rotation.into_rapier() * generic.local_frame1.rotation;
        generic
    }
}

impl From<GenericJoint> for RevoluteJoint {
    fn from(_joint: GenericJoint) -> Self {
        todo!("Implement this when we need to convert back to the specific joint");
    }
}


#[cfg(test)]
mod tests {

    use bevy::math::Vec2;
    use bevy::prelude::{Transform, Vec3};
    use rapier3d::dynamics::JointAxis;
    use rapier3d::prelude::GenericJoint;
    use crate::{default, IntoRapier};
    use super::RevoluteJoint;

    #[test]
    fn only_translation() {

        let expected_parent_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let expected_child_transform = Transform::from_translation(Vec3::new(4.0, 5.0, 6.0));

        let fixed_joint = RevoluteJoint {
            parent_anchor: expected_parent_transform,
            child_anchor: expected_child_transform,
            axis: Vec3::X,
            ..default()
        };

        let generic: GenericJoint = fixed_joint.into();

        assert!(generic.as_revolute().is_some());
        assert_eq!(generic.local_anchor1(), expected_parent_transform.translation.into_rapier());
        assert_eq!(generic.local_anchor2(), expected_child_transform.translation.into_rapier());
    }

    #[test]
    fn with_limits() {

        let limit_min = -1.0;
        let limit_max = 1.0;

        let fixed_joint = RevoluteJoint {
            axis: Vec3::X,
            limits: Some(Vec2::new(-1.0, 1.0)),
            ..default()
        };

        let generic: GenericJoint = fixed_joint.into();

        println!("{:?}", generic.limits);

        let limits = generic.limits(JointAxis::AngX).expect("No limits for AngX");
        assert_eq!(limits.min, limit_min);
        assert_eq!(limits.max, limit_max);
    }

    #[test]
    fn default_values() {

        let joint = RevoluteJoint::default();

        let generic: GenericJoint = joint.into();

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}