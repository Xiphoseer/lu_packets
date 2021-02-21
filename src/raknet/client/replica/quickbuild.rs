use std::io::Result as Res;

use endio::Serialize;
use endio_bit::BEBitWriter;
use lu_packets_derive::{BitVariantTests, ReplicaSerde};

use crate::common::{LVec, ObjId};
use crate::world::Vector3;
use crate::world::gm::client::RebuildChallengeState;
use super::{ComponentConstruction, ComponentSerialization};

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ActivityUserInfo {
	pub user_object_id: ObjId,
	// todo[min_const_generics]
	// pub activity_values: [f32; 10],
	pub activity_value_0: f32,
	pub activity_value_1: f32,
	pub activity_value_2: f32,
	pub activity_value_3: f32,
	pub activity_value_4: f32,
	pub activity_value_5: f32,
	pub activity_value_6: f32,
	pub activity_value_7: f32,
	pub activity_value_8: f32,
	pub activity_value_9: f32,
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildConstructionInfo {
	pub current_state: RebuildChallengeState,
	pub show_reset_effect: bool,
	pub has_activator: bool,
	pub duration_timer: f32,
	pub total_incomplete_time: f32,
	pub unknown: Option<u32>,
	pub activator_position: Vector3,
	pub reposition_player: bool,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildConstruction {
	pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
	pub quickbuild_construction_info: Option<QuickbuildConstructionInfo>,
}

impl ComponentConstruction for QuickbuildConstruction {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildSerializationInfo {
	pub current_state: RebuildChallengeState,
	pub show_reset_effect: bool,
	pub has_activator: bool,
	pub duration_timer: f32,
	pub total_incomplete_time: f32,
}

#[derive(BitVariantTests, Debug, PartialEq, ReplicaSerde)]
pub struct QuickbuildSerialization {
	pub activity_user_infos: Option<LVec<u32, ActivityUserInfo>>,
	pub quickbuild_serialization_info: Option<QuickbuildSerializationInfo>,
}

impl ComponentSerialization for QuickbuildSerialization {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()> {
		self.serialize(writer)
	}
}
