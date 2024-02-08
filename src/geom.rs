use num_traits::{Float, Num};
use std::fmt::Debug;

pub enum Geom<R> {
    Ellipse(Ellipse<R>),
    Triangle(Triangle<R>),
}

/// 2D point.
pub struct P2D<R> {
    /// x-coordinate of the point.
    pub x: R,
    /// y-coordinate of the point.
    pub y: R,
}
impl<R> P2D<R> {
    /// Create a new 2D point (`P2D`).
    ///
    /// # Arguments
    ///
    /// * `x` -
    /// * `y` -
    ///
    /// # Return
    ///
    /// New 2D point.
    pub fn new(x: R, y: R) -> P2D<R> {
        P2D { x, y }
    }

    /// Return the square of the distance between a pair of points.
    ///
    /// # Arguments
    ///
    /// * `p1` - First point.
    /// * `p2` - Second point.
    ///
    /// # Return
    ///
    /// Square of the distance between the points. This does not require the square root operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use squiggly_shapes::geom::P2D;
    ///
    /// let p1 = P2D::<i32>::new(1, 4);
    /// let p2 = P2D::<i32>::new(5, 7);
    /// let d: i32 = P2D::distance_between_squared(&p1, &p2);
    /// assert_eq!(d, 25);
    /// ```
    pub fn distance_between_squared(p1: &P2D<R>, p2: &P2D<R>) -> R
    where
        R: Num + Copy,
    {
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        dx * dx + dy * dy
    }

    /// Return the distance between a pair of points.
    ///
    /// # Arguments
    ///
    /// * `p1` - First point.
    /// * `p2` - Second point.
    ///
    /// # Return
    ///
    /// The distance between the points.
    ///
    /// # Examples
    ///
    /// ```
    /// use squiggly_shapes::geom::P2D;
    ///
    /// let p1 = P2D::<f32>::new(1.0, 4.0);
    /// let p2 = P2D::<f32>::new(5.0, 7.0);
    /// let d: f32 = P2D::distance_between(&p1, &p2);
    /// assert_eq!(d, 5.0);
    /// ```
    pub fn distance_between(p1: &P2D<R>, p2: &P2D<R>) -> R
    where
        R: Float,
    {
        Self::distance_between_squared(p1, p2).sqrt()
    }
}

/// Axis-aligned bounding box.
pub struct AABB<R> {
    /// Origin of the bounding box.
    origin: P2D<R>,
    /// Width of the bounding box. This is always greater than zero.
    width: R,
    /// Height of the bounding box. THis is always greater than zero.
    height: R,
}
impl<R> AABB<R>
where
    R: PartialOrd + From<u8>,
{
    /// Create a new axis-aligned bounding box (`AABB`).
    ///
    /// # Arguments
    ///
    /// * `origin` - Origin of the bounding box.
    /// * `width`  - Width of the bounding box. Must be greater than zero.
    /// * `height` - Height of the bounding box. Must be greater than zero.
    ///
    /// # Returns
    ///
    /// A new `AABB` if the fields pass validation.
    pub fn new(origin: P2D<R>, width: R, height: R) -> Option<AABB<R>> {
        if width > R::from(0) && height > R::from(0) {
            Some(AABB {
                origin,
                width,
                height,
            })
        } else {
            None
        }
    }
}

/// Ellipse.
pub struct Ellipse<R> {
    /// Center of the ellipse.
    center: P2D<R>,
    /// x-radius of the ellipse. This is always greater than zero.
    x_radius: R,
    /// y-radius of the ellipse. This is always greater than zero.
    y_radius: R,
    /// Angle between the local x-axis of the ellipse and the global x-axis.
    angle: R,
}
impl<R> Ellipse<R>
where
    R: PartialOrd + From<u8>,
{
    /// Create a new `Ellipse`.
    ///
    /// # Arguments
    ///
    /// * `center`   - Center point of the ellipse.
    /// * `x_radius` - Radius along the x-axis (either semi-major or semi-minor). Must be
    ///                greater than zero.
    /// * `y_radius` - Radius along the y-axis (either semi-major or semi-minor). Must be
    ///                greater than zero.
    /// * `angle`    - Angle between the local x-axis of the ellipse and the global x-axis.
    ///
    /// # Returns
    ///
    /// A new `Ellipse` if the fields pass validation.
    pub fn new(center: P2D<R>, x_radius: R, y_radius: R, angle: R) -> Option<Ellipse<R>> {
        if x_radius > R::from(0) && y_radius > R::from(0) {
            Some(Ellipse {
                center,
                x_radius,
                y_radius,
                angle,
            })
        } else {
            None
        }
    }
}

/// Triangle.
pub struct Triangle<R> {
    /// Points in the triangle.
    points: [P2D<R>; 3],
}
impl<R: Float> Triangle<R>
where
    R: Debug,
{
    /// Create a new `Triangle` from three points.
    ///
    /// The area of the triangle must be larger than the supplied minimum area.
    ///
    /// # Arguments
    ///
    /// * `p1`       - First point on the triangle.
    /// * `p2`       - Second point on the triangle.
    /// * `p3`       - Third point on the triangle.
    /// * `min_area` - Minimum area of a valid triangle.
    ///
    /// # Return
    ///
    /// New triangle if the area is greater than or equal to the `min_area`.
    pub fn new(p1: P2D<R>, p2: P2D<R>, p3: P2D<R>, min_area: R) -> Option<Triangle<R>> {
        let points: [P2D<R>; 3] = [p1, p2, p3];
        let candidate = Triangle { points };
        if candidate.area() >= min_area {
            Some(candidate)
        } else {
            None
        }
    }

    /// Find the area of a triangle.
    ///
    /// This uses [Heron's Formula](https://en.wikipedia.org/wiki/Heron%27s_formula).
    ///
    /// # Examples
    ///
    /// ```
    /// use squiggly_shapes::geom::{P2D, Triangle};
    ///
    /// let o = P2D::<f32>::new(0.0, 0.0);
    /// let x = P2D::<f32>::new(3.0, 0.0);
    /// let y = P2D::<f32>::new(0.0, 4.0);
    /// let tri = Triangle::new(o, x, y, 1e-3).unwrap();
    /// let area = tri.area();
    ///
    /// assert_eq!(area, 6.0);
    /// ```
    pub fn area(&self) -> R {
        // Find the three edge lengths of the triangle.
        let a = P2D::distance_between(&self.points[0], &self.points[1]);
        let b = P2D::distance_between(&self.points[1], &self.points[2]);
        let c = P2D::distance_between(&self.points[2], &self.points[0]);

        // Semiperimeter.
        let half = R::from(0.5).expect("Could not construct an R value from 0.5");
        let s = half * (a + b + c);

        // Heron's formula
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
        area
    }
}
