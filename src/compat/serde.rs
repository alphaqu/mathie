use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{Number, Rect, Vec2};

impl<T: Number + Serialize> Serialize for Rect<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		(&self.origin, &self.size).serialize(serializer)
	}
}

impl<T: Number + Serialize> Serialize for Vec2<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
		(&self.x(), &self.y()).serialize(serializer)
	}
}

impl<'de, T: Number + Deserialize<'de>> Deserialize<'de> for Rect<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let (origin, size) = Deserialize::deserialize(deserializer)?;
		Ok(Rect {
			origin,
			size
		})
	}
}

impl<'de, T: Number + Deserialize<'de>> Deserialize<'de> for Vec2<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		let (x, y) = Deserialize::deserialize(deserializer)?;
		Ok(Vec2([x, y]))
	}
}
