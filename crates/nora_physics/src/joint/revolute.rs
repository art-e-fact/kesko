use bevy::prelude::*;
use rapier3d::prelude::{GenericJoint, JointAxis, RevoluteJointBuilder};
use crate::conversions::IntoRapier;


#[derive(Default)]
pub struct RevoluteJoint {
    pub parent_anchor: Transform,
    pub child_anchor: Transform,
    pub axis: Vec3,
    pub limits: Option<Vec2>
}

impl From<RevoluteJoint> for GenericJoint {
    fn from(joint: RevoluteJoint) -> GenericJoint {
        let mut builder = RevoluteJointBuilder::new(joint.axis.into_rapier());

        if let Some(limits) = joint.limits {
            builder = builder.limits(limits.into());
        }

        let mut generic: GenericJoint = builder.into();
        *generic
            .set_local_frame1(joint.parent_anchor.into_rapier())
            .set_local_frame2(joint.child_anchor.into_rapier())
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
            ..default()
        };

        let generic: GenericJoint = fixed_joint.into();

        assert!(generic.as_revolute().is_some());
        assert_eq!(generic.local_frame1, expected_parent_transform.into_rapier());
        assert_eq!(generic.local_frame2, expected_child_transform.into_rapier());
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

        assert_eq!(generic.local_frame1, Transform::default().into_rapier());
        assert_eq!(generic.local_frame2, Transform::default().into_rapier());

        assert!(generic.limits(JointAxis::AngX).is_none());
        assert!(generic.limits(JointAxis::AngY).is_none());
        assert!(generic.limits(JointAxis::AngZ).is_none());
        assert!(generic.limits(JointAxis::X).is_none());
        assert!(generic.limits(JointAxis::Y).is_none());
        assert!(generic.limits(JointAxis::Z).is_none());
    }
}
