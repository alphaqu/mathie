use crate::prefix;

prefix!(Yottameter 1000000000000000000000000.0 Ym);
prefix!(Zettameter 1000000000000000000000.0 Zm);
prefix!(Exameter 1000000000000000000.0 Em);
prefix!(Petameter 1000000000000000.0 Pm);
prefix!(Terameter 1000000000000.0 Tm);
prefix!(Gigameter 1000000000.0 Gm);
prefix!(Megameter 1000000.0 Mm);
prefix!(Kilometer 1000.0 km);
prefix!(Hectometer 100.0 hm);
prefix!(Decameter 10.0 dam);
prefix!(Decimeter 0.1 dm);
prefix!(Centimeter 0.01 cm);
prefix!(Millimeter 0.001 mm);
prefix!(Micrometer 0.000001 μm);
prefix!(Nanometer 0.000000001 nm);
prefix!(Picometer 0.000000000001 pm);
prefix!(Femtometer 0.000000000000001 fm);
prefix!(Attometer 0.000000000000000001 am);
prefix!(Zeptometer 0.000000000000000000001 zm);
prefix!(Yoctometer 0.000000000000000000000001 ym);


#[cfg(test)]
mod tests {
    use crate::types::vec2::Vec2D;
    use super::*;

    #[test]
    fn m_to_cm() {
        let v0 = Vec2D::new_any(1, 1);
        let v1 = v0.convert::<cm>();
        assert_eq!(v1, Vec2D::new(100, 100));
    }

    #[test]
    fn cm_to_m_float() {
        let v0 = Vec2D::<u32, cm>::new(250, 250);
        let v1 = v0.try_cast::<f32>().unwrap().convert::<()>();
        assert_eq!(v1, Vec2D::new(2.5, 2.5));
    }

    #[test]
    fn cm_to_m() {
        let v0 = Vec2D::<u32, cm>::new(250, 250);
        let v1 = v0.convert::<()>();
        assert_eq!(v1, Vec2D::new(2, 2));
    }

}
