/// Keplerian elements that define an orbit
pub struct OrbitalElements<T> {
    /// Semi-major axis, *a*
    pub semimajor_axis: T,
    /// Eccentricity, *e*
    pub eccentricity: T,
    /// Inclination, *i*
    pub inclination: T,
    /// Argument of Periapsis, *ω*
    pub arg_of_periapsis: T,
    /// Time of Periapsis Passage, *T*
    pub time_of_periapsis_passage: T,
    /// Longitude of Ascending Node, *Ω*
    pub long_of_ascending_node: T,
}
impl<T> OrbitalElements<T> {
    pub fn new(
        semimajor_axis: T, eccentricity: T, inclination: T, arg_of_periapsis: T,
        time_of_periapsis_passage: T, long_of_ascending_node: T,
    ) -> Self {
        Self{
            semimajor_axis, eccentricity, inclination, arg_of_periapsis,
            time_of_periapsis_passage, long_of_ascending_node,
        }
    }
}