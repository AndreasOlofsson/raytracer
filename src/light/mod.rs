mod hemi;

pub use hemi::Hemi;

pub enum Light
{
    // Point(Point),
    Hemi(Hemi),
    // Sun(Sun),
}