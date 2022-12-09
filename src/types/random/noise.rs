use std::fmt::{self, Debug, Display};

use crate::prelude::*;

#[derive(Clone)]
pub struct Noise {
    seed: u32,
    perlin: PerlinNoise,
}

impl Noise {
    pub fn new(seed: u32) -> Self {
        let perlin = PerlinNoise::new(seed);
        Self { seed, perlin }
    }

    /// Fractional Brown Motion
    pub fn get<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(
        &mut self,
        x: X,
        y: Y,
        z: Z,
        params: Option<NoiseParams>,
    ) -> f64 {
        let mut x = x.into();
        let mut y = y.into();
        let mut z = z.into();
        let params = params.unwrap_or_default();

        let mut amplitude = params.amplitude;
        let mut frequency = params.frequency;

        let mut value = 0.0;

        for _ in 0..params.octaves {
            // Sample `Noise` with reguards to amplitude and frequency
            value += amplitude * self.perlin.get([x * frequency, y * frequency, z * frequency]);

            // Shift each axis to stop repeated features in future octaves
            x -= params.x_shift;
            y -= params.y_shift;
            z -= params.z_shift;

            // Adjust amplitude and frequency for the next octave
            amplitude *= params.persistence;
            frequency *= params.lacunarity;
        }

        value
    }

    /// Fractional Brown Motion
    pub fn get_many<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(
        &mut self,
        start_x: X,
        start_y: Y,
        start_z: Z,
        step_x: X,
        step_y: Y,
        step_z: Z,
        count: usize,
        params: Option<NoiseParams>,
    ) -> Vec<f64> {
        let mut x = start_x.into();
        let mut y = start_y.into();
        let mut z = start_z.into();
        let step_x = step_x.into();
        let step_y = step_y.into();
        let step_z = step_z.into();

        let params = params.unwrap_or_default();

        let mut ret = Vec::with_capacity(count);

        for _ in 0..count {
            let mut current_x = x;
            let mut current_y = y;
            let mut current_z = z;

            let mut amplitude = params.amplitude;
            let mut frequency = params.frequency;

            let mut value = 0.0;

            for _ in 0..params.octaves {
                // Sample `Noise` with reguards to amplitude and frequency
                value += amplitude *
                    self.perlin.get([
                        current_x * frequency,
                        current_y * frequency,
                        current_z * frequency,
                    ]);

                // Shift each axis to stop repeated features in future octaves
                current_x -= params.x_shift;
                current_y -= params.y_shift;
                current_z -= params.z_shift;

                // Adjust amplitude and frequency for the next octave
                amplitude *= params.persistence;
                frequency *= params.lacunarity;
            }

            ret.push(value);
            x += step_x;
            y += step_y;
            z += step_z;
        }

        ret
    }

    /// Fractional Brown Motion
    pub fn get_many_f32<X: Into<f32>, Y: Into<f32>, Z: Into<f32>>(
        &mut self,
        start_x: X,
        start_y: Y,
        start_z: Z,
        step_x: X,
        step_y: Y,
        step_z: Z,
        count: usize,
        params: Option<NoiseParams>,
    ) -> Vec<f32> {
        let mut x = start_x.into() as f64;
        let mut y = start_y.into() as f64;
        let mut z = start_z.into() as f64;
        let step_x = step_x.into() as f64;
        let step_y = step_y.into() as f64;
        let step_z = step_z.into() as f64;

        let params = params.unwrap_or_default();

        let mut ret = Vec::with_capacity(count);

        for _ in 0..count {
            let mut current_x = x;
            let mut current_y = y;
            let mut current_z = z;

            let mut amplitude = params.amplitude;
            let mut frequency = params.frequency;

            let mut value = 0.0;

            for _ in 0..params.octaves {
                // Sample `Noise` with reguards to amplitude and frequency
                value += amplitude *
                    self.perlin.get([
                        current_x * frequency,
                        current_y * frequency,
                        current_z * frequency,
                    ]);

                // Shift each axis to stop repeated features in future octaves
                current_x -= params.x_shift;
                current_y -= params.y_shift;
                current_z -= params.z_shift;

                // Adjust amplitude and frequency for the next octave
                amplitude *= params.persistence;
                frequency *= params.lacunarity;
            }

            ret.push(value as f32);
            x += step_x;
            y += step_y;
            z += step_z;
        }

        ret
    }

    /// Fractional Brown Motion
    pub fn get_many_f32_normalized<
        X: Into<f32>,
        Y: Into<f32>,
        Z: Into<f32>,
        Low: Into<f32>,
        High: Into<f32>,
    >(
        &mut self,
        start_x: X,
        start_y: Y,
        start_z: Z,
        step_x: X,
        step_y: Y,
        step_z: Z,
        count: usize,
        low: Low,
        high: High,
        params: Option<NoiseParams>,
    ) -> Vec<f32> {
        let mut x = start_x.into() as f64;
        let mut y = start_y.into() as f64;
        let mut z = start_z.into() as f64;
        let step_x = step_x.into() as f64;
        let step_y = step_y.into() as f64;
        let step_z = step_z.into() as f64;
        let low = low.into();
        let high = high.into();

        let params = params.unwrap_or_default();

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut ret = Vec::with_capacity(count);

        for _ in 0..count {
            let mut current_x = x;
            let mut current_y = y;
            let mut current_z = z;

            let mut amplitude = params.amplitude;
            let mut frequency = params.frequency;

            let mut value = 0.0;

            for _ in 0..params.octaves {
                // Sample `Noise` with reguards to amplitude and frequency
                value += amplitude *
                    self.perlin.get([
                        current_x * frequency,
                        current_y * frequency,
                        current_z * frequency,
                    ]);

                // Shift each axis to stop repeated features in future octaves
                current_x -= params.x_shift;
                current_y -= params.y_shift;
                current_z -= params.z_shift;

                // Adjust amplitude and frequency for the next octave
                amplitude *= params.persistence;
                frequency *= params.lacunarity;
            }
            if value > max {
                max = value;
            };
            if value < min {
                min = value;
            };
            ret.push(value as f32);
            x += step_x;
            y += step_y;
            z += step_z;
        }

        let mut new_ret = Vec::with_capacity(count);
        for value in ret {
            new_ret.push(map_range_f32(value, (min as f32, max as f32), (low, high)));
        }

        new_ret
    }
}

#[cfg(feature = "serialize")]
impl Serialize for Noise {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let mut state = serializer.serialize_struct("Noise", 1)?;
        state.serialize_field("seed", &self.seed)?;
        state.end()
    }
}

#[cfg(feature = "serialize")]
impl<'de> Deserialize<'de> for Noise {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Seed,
        }
        struct NoiseVisitor;
        impl<'de> Visitor<'de> for NoiseVisitor {
            type Value = Noise;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Noise")
            }

            fn visit_seq<A>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error>
            where A: SeqAccess<'de> {
                let seed = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Noise::new(seed))
            }

            fn visit_map<A>(self, mut map: A) -> core::result::Result<Self::Value, A::Error>
            where A: MapAccess<'de> {
                let mut seed = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Seed => {
                            if seed.is_some() {
                                return Err(de::Error::duplicate_field("seed"));
                            }
                            seed = Some(map.next_value()?);
                        },
                    }
                }
                let seed = seed.ok_or_else(|| de::Error::missing_field("seed"))?;
                Ok(Noise::new(seed))
            }
        }
        const FIELDS: &[&str] = &["seed"];
        deserializer.deserialize_struct("Noise", FIELDS, NoiseVisitor)
    }
}

impl Debug for Noise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Noise {{ seed:{} }}", self.seed))
    }
}

impl Display for Noise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Noise {{ seed:{} }}", self.seed))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoiseParams {
    x_shift: f64,
    y_shift: f64,
    z_shift: f64,

    amplitude: f64,
    frequency: f64,
    octaves: u32,
    persistence: f64,
    lacunarity: f64,
}

impl Display for NoiseParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NoiseParams: {{\n\tx_shift: {:.8}\n\ty_shift: {:.8}\n\tz_shift: {:.8}\n\n\tamplitude: \
             {:.8}\n\tfrequency: {:.8}\n\toctaves: {}\n\tpersistence: {:.8}\n\tlacunarity: {:.8}\n}}",
            self.x_shift,
            self.y_shift,
            self.z_shift,
            self.amplitude,
            self.frequency,
            self.octaves,
            self.persistence,
            self.lacunarity
        )
    }
}

impl Default for NoiseParams {
    fn default() -> Self {
        Self {
            x_shift: 0.0,
            y_shift: 0.0,
            z_shift: 0.0,

            amplitude: 1.0,
            frequency: 1.0,
            octaves: 1,
            persistence: 0.5,
            lacunarity: 2.0,
        }
    }
}

impl NoiseParams {
    /// An adjustment on the `x` axis. (`West` to `East`) of the noise map
    /// Default: 0.0
    pub fn set_x_shift(&mut self, x_shift: f64) -> &mut Self {
        self.x_shift = x_shift;
        self
    }

    /// An adjustment on the `y` axis. (`South` to `North`) of the noise map
    /// Default: 0.0
    pub fn set_y_shift(&mut self, y_shift: f64) -> &mut Self {
        self.y_shift = y_shift;
        self
    }

    /// An adjustment on the `z` axis. (`Down` to `Up`) of the noise map
    /// Default: 0.0
    pub fn set_z_shift(&mut self, z_shift: f64) -> &mut Self {
        self.z_shift = z_shift;
        self
    }

    /// Multiplier for the value from the `Noise` function
    /// Default: 1.0
    pub fn set_amplitude(&mut self, amplitude: f64) -> &mut Self {
        self.amplitude = amplitude.max(f64::EPSILON);
        self
    }

    /// Multiplier for "Positional Scaling"
    /// Default: 1.0
    pub fn set_frequency(&mut self, frequency: f64) -> &mut Self {
        self.frequency = frequency;
        self
    }

    /// Number of times to call the `Noise` function
    /// Default: 1
    pub fn set_octaves(&mut self, octaves: u32) -> &mut Self {
        self.octaves = octaves.min(64);
        self
    }

    /// How fast the later octaves "die out"
    /// Value: 0..=1
    /// Default: 0.5
    pub fn set_persistence(&mut self, persistence: f64) -> &mut Self {
        self.persistence = persistence.clamp(0.0, 1.0);
        self
    }

    /// How much finer each subsequent octave should use
    /// Value: > 1
    /// Default: 2.0
    pub fn set_lacunarity(&mut self, lacunarity: f64) -> &mut Self {
        self.lacunarity = lacunarity.max(1.0);
        self
    }
}
