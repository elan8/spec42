# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQSpaceTime
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQSpaceTime {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-3:2019 "Space and Time"
     * see also https://www.iso.org/standard/64974.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* ISO-80000-3 item 3-1.1 length */
    /* See package ISQBase for the declarations of LengthValue and LengthUnit */

    /* ISO-80000-3 item 3-1.2 width, breadth */
    attribute width: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.2 width, breadth
         * symbol(s): `b`, `B`
         * application domain: generic
         * name: Width (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between two parallel straight lines (in two dimensions) or planes (in three dimensions) that enclose a given geometrical shape
         * remarks: This quantity is non-negative.
         */
    }

    alias breadth for width;

    /* ISO-80000-3 item 3-1.3 height, depth, altitude */
    attribute height: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.3 height, depth, altitude
         * symbol(s): `h`, `H`
         * application domain: generic
         * name: Height (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between a point and a reference line or reference surface
         * remarks: This quantity is usually signed. The sign expresses the position of the particular point with respect to the reference line or surface and is chosen by convention. The symbol `H` is often used to denote altitude, i.e. height above sea level.
         */
    }

    alias depth for height;

    alias altitude for height;

    /* ISO-80000-3 item 3-1.4 thickness */
    attribute thickness: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.4 thickness
         * symbol(s): `d`, `δ`
         * application domain: generic
         * name: Thickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.5 diameter */
    attribute diameter: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.5 diameter
         * symbol(s): `d`, `D`
         * application domain: generic
         * name: Diameter (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2) of a circle, cylinder or sphere
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.6 radius */
    attribute radius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.6 radius
         * symbol(s): `r`, `R`
         * application domain: generic
         * name: Radius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: half of a diameter (item 3-1.5)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.7 path length, arc length */
    attribute pathLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.7 path length, arc length
         * symbol(s): `s`
         * application domain: generic
         * name: PathLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length of a rectifiable curve between two of its points
         * remarks: The differential path length at a given point of a curve is: `ds = sqrt(dx^2 + dy^2 + dz^2)` where `x`, `y`, and `z` denote the Cartesian coordinates (ISO 80000-2) of the particular point. There are curves which are not rectifiable, for example fractal curves.
         */
    }

    alias arcLength for pathLength;

    /* ISO-80000-3 item 3-1.8 distance */
    attribute distance: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.8 distance
         * symbol(s): `d`, `r`
         * application domain: generic
         * name: Distance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: shortest path length (item 3-1.7) between two points in a metric space
         * remarks: A metric space might be curved. An example of a curved metric space is the surface of the Earth. In this case, distances are measured along great circles. A metric is not necessarily Euclidean.
         */
    }

    /* ISO-80000-3 item 3-1.9 radial distance */
    attribute radialDistance: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.9 radial distance
         * symbol(s): `r_Q`, `ρ`
         * application domain: generic
         * name: RadialDistance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (item 3-1.8), where one point is located on an axis or within a closed non self-intersecting curve or surface
         * remarks: The subscript Q denotes the point from which the radial distance is measured. Examples of closed non self-intersecting curves are circles or ellipses. Examples of closed non self-intersecting surfaces are surfaces of spheres or egg-shaped objects.
         */
    }

    /* Spatial coordinate frames */
    
    attribute def Spatial3dCoordinateFrame :> '3dCoordinateFrame' {
        doc
        /*
         * Most general spatial 3D coordinate frame
         */
        attribute :>> isBound = true;
    }

    attribute def CartesianSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cartesian spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.1 Cartesian coordinates
         *
         * The components of a vector expressed on a Cartesian spatial coordinate frame are all LengthValues, and denoted with symbols `x`, `y`, `z`.
         *
         * Note 1: The Cartesian basis vectors `vec(e_x)`, `vec(e_y)` and `vec(e_z)` form an orthonormal right-handed coordinate frame.
         * Note 2: The measurement units for the 3 dimensions are typically the same, but may be different.
         */
        attribute xUnit : LengthUnit = mRefs#(1);
        attribute yUnit : LengthUnit = mRefs#(2);
        attribute zUnit : LengthUnit = mRefs#(3);
        attribute :>> mRefs : LengthUnit[3];
        attribute :>> isOrthogonal = true;
    }

    attribute universalCartesianSpatial3dCoordinateFrame : CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
         * A singleton CartesianSpatial3dCoordinateFrame that can be used as a default universal Cartesian 3D coordinate frame.
         */
         
        attribute :>> mRefs default (SI::m, SI::m, SI::m) {
            doc /*
             * By default, the universalCartesianSpatial3dCoordinateFrame uses meters as the units on all three axes.
             */
        }
        
        attribute :>> transformation[0..0] {
            doc /*
             * The universalCartesianSpatial3dCoordinateFrame is the "top-level" coordinate frame, not nested in any other frame.
             */
        }
        
    }

    attribute def CylindricalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cylindrical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.2 cylindrical coordinates
         *
         * The components of a (position) vector to a point P in a cylindrical coordinate frame are:
         * - radialDistance (symbol `ρ`) defined by LengthValue, that is the radial distance from the cylinder axis to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the cylinder axis, in the plane that is orthogonal to the cylinder axis and intersects P
         * - z coordinate (symbol `z`) defined by LengthValue, the coordinate along the clyinder axis.
         *
         * Note 1: The basis vectors `vec(e_ρ)(φ)`, `vec(e_φ)(φ)` and `vec(e_z)` form an orthonormal right-handed coordinate frame, where
         * `vec(e_φ)` is tangent to the circular arc in the `φ` direction.
         * Note 2: In order to enable transformation to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` direction in the cylindrical frame, and the `vec(e_z)` Cartesian basis vector is aligned with
         * the `vec(e_z)` cylindrical basis vector.
         * Note 3: If `z = 0`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Cylindrical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute zUnit : LengthUnit;
        attribute :>> mRefs = (radialDistanceUnit, azimuthUnit, zUnit);
        attribute :>> isOrthogonal = true;
    }

    attribute def SphericalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Spherical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.3 spherical coordinates
         *
         * The components of a (position) vector to a point P specified in a spherical coordinate frame are:
         * - radialDistance (symbol `r`) defined by LengthValue, that is the distance from the origin to P
         * - inclination (symbol `θ`) defined by AngularMeasure, that is the angle between the zenith direction and the line segment from origin to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the origin to the orthogonal projection of P on the reference plane, normal to the zenith direction.
         *
         * Note 1: The basis vectors `vec(e_r)(θ,φ)`, `vec(e_θ)(θ,φ)` and `vec(e_φ)(φ)` form an orthonormal right-handed frame, where
         * `vec(e_θ)` and `vec(e_φ)` are tangent to the respective circular arcs in the `θ` and `φ` directions.
         * Note 2: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `θ=π/4` and `φ=0` direction in the spherical frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `θ=0` zenith direction in the spherical frame.
         * Note 3: If `θ = π/4`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Spherical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute inclinationUnit : AngularMeasureUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute :>> mRefs = (radialDistanceUnit, inclinationUnit, azimuthUnit);
        attribute :>> isOrthogonal = true;
    }

     attribute def PlanetarySpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Planetary spatial 3D coordinate frame
         *
         * A planetary spatial 3D coordinate frame is a generalization for any planet of the geographic coordinate frame and geocentric coordinate
         * for Earth. In such coordinate frames, typically the origin is located at the planet's centre of gravity, and the surface of the planet
         * is approximated by a reference ellipsoid centred on the origin, with its major axes oriented along the south to north pole vector and
         * the equatorial plane.
         *
         * The components of a (position) vector to a point P specified in a planetary coordinate frame are:
         * - latitude (symbol `lat` or `φ`) defined by AngularMeasure, that is the angle between the equatorial plane and the vector from
         *   the origin to P, similar to the inclination in a spherical spatial coordinate frame. Typically, the zero reference latitude is chosen
         *   for positions in the equatorial plane, with positive latitude for positions in the northern hemisphere and negative latitude for positions
         *   in the southern hemisphere.
         * - longitude (symbol `long` or `λ`) defined by AngularMeasure, that is the angle between a reference meridian and the meridian
         *   passing through P, similar to the azimuth of a spherical spatial coordinate frame. The convention is to connotate positive longitude
         *   with eastward direction and negative longitude with westward direction. The reference meridian for `long=0` is chosen to pass
         *   through a particular feature of the planet, e.g., for Earth typically the position of the British Royal Observatory in Greenwich, UK.
         * - altitude (symbol `h`) defined by LengthValue, that is the distance between P and the reference ellipsoid
         *   in the normal direction to the ellipsoid. Positive altitude specifies a position above the reference ellipsoid surface,
         *   while a negative value specifies a position below.
         *
         * Note 1: The reference meridian is also called prime meridian.
         * Note 2: The basis vectors `vec(e_φ)(φ)`, `vec(e_λ)(λ)` and `vec(e_h)(φ,λ)` form an orthonormal right-handed frame, where
         * `vec(e_φ)` and `vec(e_λ)` are tangent to the reference ellipsoid in the respective latitude and longitude directions,
         * and `vec(e_h)` is normal to the reference ellipsoid.
         * Note 3: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` and `λ=0` direction in the planetary frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `λ=π/2` (north pole) direction in the planetary frame.
         * Note 4: See also https://en.wikipedia.org/wiki/Planetary_coordinate_system .
         */
        attribute latitudeUnit : AngularMeasureUnit;
        attribute longitudeUnit : AngularMeasureUnit;
        attribute altitudeUnit : LengthUnit;
        attribute :>> mRefs = (longitudeUnit, latitudeUnit, altitudeUnit);
        attribute :>> isOrthogonal = true;
    }

    /* ISO-80000-3 item 3-1.10 position vector */
    attribute def Position3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.10 position vector
         * symbol(s): `vec(r)`
         * application domain: generic
         * name: PositionVector
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity from the origin of a coordinate system to a point in space
         * remarks: Position vectors are so-called bounded vectors, i.e. their magnitude (ISO 80000-2) and direction depend on the particular coordinate system used.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: Spatial3dCoordinateFrame[1];
    }

    attribute position3dVector: Position3dVector :> vectorQuantities;

    attribute def CartesianPosition3dVector :> Position3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute cartesianPosition3dVector : CartesianPosition3dVector :> position3dVector;

    attribute def CylindricalPosition3dVector :> Position3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute cylindricalPosition3dVector : CylindricalPosition3dVector :> position3dVector;

    attribute def SphericalPosition3dVector :> Position3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute sphericalPosition3dVector : SphericalPosition3dVector :> position3dVector;

    attribute def PlanetaryPosition3dVector :> Position3dVector {
        attribute <lat> latitude : AngularMeasureUnit = num#(1) [mRef.mRefs#(1)];
        attribute <long> longitude : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> altitude : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : PlanetarySpatial3dCoordinateFrame[1];
    }
    attribute planetaryPosition3dVector : PlanetaryPosition3dVector :> position3dVector;

    /* ISO-80000-3 item 3-1.11 displacement */
    attribute def Displacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.11 displacement
         * symbol(s): `vec(Δr)`
         * application domain: generic
         * name: Displacement
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity between any two points in space
         * remarks: Displacement vectors are so-called free vectors, i.e. their magnitude (ISO 80000-2) and direction do not depend on a particular coordinate system. The magnitude of this vector is also called displacement.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: Spatial3dCoordinateFrame[1];
    }

    attribute displacement3dVector: Displacement3dVector :> vectorQuantities;

    attribute def CartesianDisplacement3dVector :> Displacement3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute cartesianDisplacement3dVector : CartesianDisplacement3dVector :> displacement3dVector;

    attribute def CylindricalDisplacement3dVector :> Displacement3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute cylindricalDisplacement3dVector : CylindricalDisplacement3dVector :> displacement3dVector;

    attribute def SphericalDisplacement3dVector :> Displacement3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute sphericalDisplacement3dVector : SphericalDisplacement3dVector :> displacement3dVector;

    /* ISO-80000-3 item 3-1.12 radius of curvature */
    attribute radiusOfCurvature: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.12 radius of curvature
         * symbol(s): `ρ`
         * application domain: generic
         * name: RadiusOfCurvature (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (item 3-1.6) of the osculating circle of a planar curve at a particular point of the curve
         * remarks: The radius of curvature is only defined for curves which are at least twice continuously differentiable.
         */
    }

    /* ISO-80000-3 item 3-2 curvature */
    attribute def CurvatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-2 curvature
         * symbol(s): `κ`
         * application domain: generic
         * name: Curvature
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the radius of curvature (item 3-1.12)
         * remarks: The curvature is given by: `κ = 1/ρ` where `ρ` denotes the radius of curvature (item 3-1.12).
         */
        attribute :>> num: Real;
        attribute :>> mRef: CurvatureUnit[1];
    }

    attribute curvature: CurvatureValue[*] nonunique :> scalarQuantities;

    attribute def CurvatureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-3 area */
    attribute def AreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-3 area
         * symbol(s): `A`, `S`
         * application domain: generic
         * name: Area
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: extent of a two-dimensional geometrical shape
         * remarks: The surface element at a given point of a surface is given by: `dA = g du dv` where `u` and `v` denote the Gaussian surface coordinates and `g` denotes the determinant of the metric tensor (ISO 80000-2) at the particular point. The symbol `dσ` is also used for the surface element.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AreaUnit[1];
    }

    attribute area: AreaValue[*] nonunique :> scalarQuantities;

    attribute def AreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-4 volume */
    attribute def VolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-4 volume
         * symbol(s): `V`, `(S)`
         * application domain: generic
         * name: Volume
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: extent of a three-dimensional geometrical shape
         * remarks: The volume element in Euclidean space is given by: `dV = dx dy dz` where `dx`, `dy`, and `dz` denote the differentials of the Cartesian coordinates (ISO 80000-2). The symbol `dτ` is also used for the volume element.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeUnit[1];
    }

    attribute volume: VolumeValue[*] nonunique :> scalarQuantities;

    attribute def VolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-5 angular measure, plane angle */
    attribute def AngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-5 angular measure, plane angle
         * symbol(s): `α`, `β`, `γ`
         * application domain: generic
         * name: AngularMeasure
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: measure of a geometric figure, called plane angle, formed by two rays, called the sides of the plane angle, emanating from a common point, called the vertex of the plane angle
         * remarks: The angular measure is given by: `α = s/r` where `s` denotes the arc length (item 3-1.7) of the included arc of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. Other symbols are also used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularMeasureUnit[1];
    }

    attribute angularMeasure: AngularMeasureValue[*] nonunique :> scalarQuantities;

    attribute def AngularMeasureUnit :> DimensionOneUnit {
    }

    alias PlaneAngleUnit for AngularMeasureUnit;
    alias PlaneAngleValue for AngularMeasureValue;
    alias planeAngle for angularMeasure;

    /* ISO-80000-3 item 3-6 rotational displacement, angular displacement */
    attribute rotationalDisplacement: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 3-6 rotational displacement, angular displacement
         * symbol(s): `ϑ`, `φ`
         * application domain: generic
         * name: RotationalDisplacement (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: quotient of the traversed circular path length (item 3-1.7) of a point in space during a rotation and its distance (item 3-1.8) from the axis or centre of rotation
         * remarks: The rotational displacement is given by: `φ = s/r` where `s` denotes the traversed path length (item 3-1.7) along the periphery of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. The rotational displacement is signed. The sign denotes the direction of rotation and is chosen by convention. Other symbols are also used.
         */
    }

    alias angularDisplacement for rotationalDisplacement;

    /* ISO-80000-3 item 3-7 phase angle */
    attribute phaseAngle: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 3-7 phase angle
         * symbol(s): `φ`, `ϕ`
         * application domain: generic
         * name: PhaseAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: angular measure (item 3-5) between the positive real axis and the radius of the polar representation of the complex number in the complex plane
         * remarks: The phase angle (often imprecisely referred to as the "phase") is the argument of a complex number. Other symbols are also used.
         */
    }

    /* ISO-80000-3 item 3-8 solid angular measure */
    attribute def SolidAngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-8 solid angular measure
         * symbol(s): `Ω`
         * application domain: generic
         * name: SolidAngularMeasure
         * quantity dimension: 1
         * measurement unit(s): sr, 1
         * tensor order: 0
         * definition: measure of a conical geometric figure, called solid angle, formed by all rays, originating from a common point, called the vertex of the solid angle, and passing through the points of a closed, non-self-intersecting curve in space considered as the border of a surface
         * remarks: The differential solid angular measure expressed in spherical coordinates (ISO 80000-2) is given by: `dΩ = A/r^2 * sin(θ * dθ * dφ)` where `A` is area, `r` is radius, `θ` and `φ` are spherical coordinates.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SolidAngularMeasureUnit[1];
    }

    attribute solidAngularMeasure: SolidAngularMeasureValue[*] nonunique :> scalarQuantities;

    attribute def SolidAngularMeasureUnit :> DimensionOneUnit {
    }

    /* ISO-80000-3 item 3-9 duration, time */
    /* See package ISQBase for the declarations of DurationValue and DurationUnit */

    alias TimeUnit for DurationUnit;
    alias TimeValue for DurationValue;
    alias time for duration;

    /* ISO-80000-3 item 3-10.1 velocity */
    attribute def CartesianVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-10.1 velocity
         * symbol(s): `vec(v)`, `u,v,w`
         * application domain: generic
         * name: Velocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of a position vector (item 3-1.10)
         * remarks: The velocity vector is given by: `vec(v) = (d vec(r)) / (dt)` where `vec(r)` denotes the position vector (item 3-1.10) and `t` the duration (item 3-9). When the general symbol `vec(v)` is not used for the velocity, the symbols `u`, `v`, `w` may be used for the components (ISO 80000-2) of the velocity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianVelocity3dCoordinateFrame[1];
    }

    attribute cartesianVelocity3dVector: CartesianVelocity3dVector :> vectorQuantities;

    attribute def CartesianVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SpeedUnit[3];
    }

    /* ISO-80000-3 item 3-10.2 speed */
    attribute def SpeedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-10.2 speed
         * symbol(s): `v`
         * application domain: generic
         * name: Speed
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the velocity (item 3-10.1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedUnit[1];
    }

    attribute speed: SpeedValue[*] nonunique :> scalarQuantities;

    attribute def SpeedUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-3 item 3-11 acceleration */
    attribute def AccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-11 acceleration (magnitude)
         * symbol(s): `a`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AccelerationUnit[1];
    }

    attribute acceleration: AccelerationValue[*] nonunique :> scalarQuantities;

    attribute def AccelerationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    attribute def CartesianAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-11 acceleration (vector)
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianAcceleration3dVector: CartesianAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AccelerationUnit[3];
    }

    /* ISO-80000-3 item 3-12 angular velocity */
    attribute def AngularVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-12 angular velocity (magnitude)
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularVelocityUnit[1];
    }

    attribute angularVelocity: AngularVelocityValue[*] nonunique :> scalarQuantities;

    attribute def AngularVelocityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    attribute def CartesianAngularVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-12 angular velocity (vector)
         * symbol(s): `vec(ω)`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularVelocity3dCoordinateFrame[1];
    }

    attribute cartesianAngularVelocity3dVector: CartesianAngularVelocity3dVector :> vectorQuantities;

    attribute def CartesianAngularVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularVelocityUnit[3];
    }

    /* ISO-80000-3 item 3-13 angular acceleration */
    attribute def AngularAccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-13 angular acceleration (magnitude)
         * symbol(s): `α`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularAccelerationUnit[1];
    }

    attribute angularAcceleration: AngularAccelerationValue[*] nonunique :> scalarQuantities;

    attribute def AngularAccelerationUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    attribute def CartesianAngularAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-13 angular acceleration (vector)
         * symbol(s): `vec(α)`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianAngularAcceleration3dVector: CartesianAngularAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAngularAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularAccelerationUnit[3];
    }

    /* ISO-80000-3 item 3-14 period duration, period */
    attribute periodDuration: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 3-14 period duration, period
         * symbol(s): `T`
         * application domain: generic
         * name: PeriodDuration (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: A periodic event is an event that occurs regularly with a fixed time interval.
         */
    }

    alias period for periodDuration;

    /* ISO-80000-3 item 3-15 time constant */
    attribute timeConstant: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 3-15 time constant
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: TimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: parameter characterizing the response to a step input of a first-order, linear time-invariant system
         * remarks: If a quantity is a function of the duration (item 3-9) expressed by: `F(t) prop e^(-t/τ)` where `t` denotes the duration (item 3-9), then `τ` denotes the time constant. Here the time constant `τ` applies to an exponentially decaying quantity.
         */
    }

    /* ISO-80000-3 item 3-16 rotation */
    attribute rotation: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 3-16 rotation
         * symbol(s): `N`
         * application domain: generic
         * name: Rotation (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of revolutions
         * remarks: `N` is the number (not necessarily an integer) of revolutions, for example, of a rotating body about a given axis. Its value is given by: `N = φ/(2 π)` where `φ` denotes the measure of rotational displacement (item 3-6).
         */
    }

    /* ISO-80000-3 item 3-17.1 frequency */
    attribute def FrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-17.1 frequency
         * symbol(s): `f`, `ν`
         * application domain: generic
         * name: Frequency
         * quantity dimension: T^-1
         * measurement unit(s): Hz, s^-1
         * tensor order: 0
         * definition: inverse of period duration (item 3-14)
         * remarks: The frequency is given by: `f = 1/T` where `T` denotes the period duration (item 3-14).
         */
        attribute :>> num: Real;
        attribute :>> mRef: FrequencyUnit[1];
    }

    attribute frequency: FrequencyValue[*] nonunique :> scalarQuantities;

    attribute def FrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-17.2 rotational frequency */
    attribute rotationalFrequency: FrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 3-17.2 rotational frequency
         * symbol(s): `n`
         * application domain: generic
         * name: RotationalFrequency (specializes Frequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: The rotational frequency is given by: `n = (dN) / (dt)` where `N` denotes the rotation (item 3-16) and `t` is the duration (item 3-9).
         */
    }

    /* ISO-80000-3 item 3-18 angular frequency */
    attribute def AngularFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-18 angular frequency
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularFrequency
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: rate of change of the phase angle (item 3-7)
         * remarks: The angular frequency is given by: `ω = 2 π f` where `f` denotes the frequency (item 3-17.1).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularFrequencyUnit[1];
    }

    attribute angularFrequency: AngularFrequencyValue[*] nonunique :> scalarQuantities;

    attribute def AngularFrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-19 wavelength */
    attribute wavelength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-19 wavelength
         * symbol(s): `λ`
         * application domain: generic
         * name: Wavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length (item 3-1.1) of the repetition interval of a wave
         * remarks: None.
         */
    }

    /* ISO-80000-3 item 3-20 repetency, wavenumber */
    attribute def RepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-20 repetency, wavenumber
         * symbol(s): `σ`, `ṽ`
         * application domain: generic
         * name: Repetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the wavelength (item 3-19)
         * remarks: The repetency is given by: `σ = 1 / λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RepetencyUnit[1];
    }

    attribute repetency: RepetencyValue[*] nonunique :> scalarQuantities;

    attribute def RepetencyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias WavenumberUnit for RepetencyUnit;
    alias WavenumberValue for RepetencyValue;
    alias wavenumber for repetency;

    /* ISO-80000-3 item 3-21 wave vector */
    attribute def CartesianWave3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-21 wave vector
         * symbol(s): `vec(k)`
         * application domain: generic
         * name: WaveVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector normal to the surfaces of constant phase angle (item 3-7) of a wave, with the magnitude (ISO 80000-2) of repetency (item 3-20)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianWaveVector3dCoordinateFrame[1];
    }

    attribute cartesianWave3dVector: CartesianWave3dVector :> vectorQuantities;

    attribute def CartesianWaveVector3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: RepetencyUnit[3];
    }

    /* ISO-80000-3 item 3-22 angular repetency, angular wavenumber */
    attribute def AngularRepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-22 angular repetency, angular wavenumber
         * symbol(s): `k`
         * application domain: generic
         * name: AngularRepetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the wave vector (item 3-21)
         * remarks: The angular repetency is given by: `κ = (2 π)/λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularRepetencyUnit[1];
    }

    attribute angularRepetency: AngularRepetencyValue[*] nonunique :> scalarQuantities;

    attribute def AngularRepetencyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias AngularWavenumberUnit for AngularRepetencyUnit;
    alias AngularWavenumberValue for AngularRepetencyValue;
    alias angularWavenumber for angularRepetency;

    /* ISO-80000-3 item 3-23.1 phase velocity, phase speed */
    attribute def PhaseVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-23.1 phase velocity, phase speed
         * symbol(s): `c`, `v`, `(ν)`, `c_φ`, `v_φ`, `(ν_φ)`
         * application domain: generic
         * name: PhaseVelocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the phase angle (item 3-7) of a wave propagates in space
         * remarks: The phase velocity is given by: `c = ω/κ` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22). If phase velocities of electromagnetic waves and other phase velocities are both involved, then `c` should be used for the former and `υ` for the latter. Phase velocity can also be written as `c = λ f`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseVelocityUnit[1];
    }

    attribute phaseVelocity: PhaseVelocityValue[*] nonunique :> scalarQuantities;

    attribute def PhaseVelocityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias PhaseSpeedUnit for PhaseVelocityUnit;
    alias PhaseSpeedValue for PhaseVelocityValue;
    alias phaseSpeed for phaseVelocity;

    /* ISO-80000-3 item 3-23.2 group velocity, group speed */
    attribute groupVelocity: SpeedValue :> scalarQuantities {
        doc
        /*
         * source: item 3-23.2 group velocity, group speed
         * symbol(s): `c_g`, `v_g`, `(ν_g)`
         * application domain: generic
         * name: GroupVelocity (specializes Speed)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the envelope of a wave propagates in space
         * remarks: The group velocity is given by: `c_g = (d ω)/ (dk)` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22).
         */
    }

    alias groupSpeed for groupVelocity;

    /* ISO-80000-3 item 3-24 damping coefficient */
    attribute def DampingCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-24 damping coefficient
         * symbol(s): `δ`
         * application domain: generic
         * name: DampingCoefficient
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: inverse of the time constant (item 3-15) of an exponentially varying quantity
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DampingCoefficientUnit[1];
    }

    attribute dampingCoefficient: DampingCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def DampingCoefficientUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-25 logarithmic decrement */
    attribute def LogarithmicDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 3-25 logarithmic decrement
         * symbol(s): `Λ`
         * application domain: generic
         * name: LogarithmicDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of damping coefficient (item 3-24) and period duration (item 3-14)
         * remarks: None.
         */
    }
    attribute logarithmicDecrement: LogarithmicDecrementValue :> scalarQuantities;

    /* ISO-80000-3 item 3-26.1 attenuation, extinction */
    attribute def AttenuationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.1 attenuation, extinction
         * symbol(s): `α`
         * application domain: generic
         * name: Attenuation
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: gradual decrease in magnitude (ISO 80000-2) of any kind of flux through a medium
         * remarks: If a quantity is a function of distance (item 3-1.8) expressed by: `f(x) prop e^(-α x)` where `x` denotes distance (item 3-1.8), then `α` denotes attenuation. The inverse of attenuation is called attenuation length.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AttenuationUnit[1];
    }

    attribute attenuation: AttenuationValue[*] nonunique :> scalarQuantities;

    attribute def AttenuationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias ExtinctionUnit for AttenuationUnit;
    alias ExtinctionValue for AttenuationValue;
    alias extinction for attenuation;

    /* ISO-80000-3 item 3-26.2 phase coefficient */
    attribute def PhaseCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.2 phase coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PhaseCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): rad/m, m^-1
         * tensor order: 0
         * definition: change of phase angle (item 3-7) with the length (item 3-1.1) along the path travelled by a plane wave
         * remarks: If a quantity is a function of distance expressed by: `f(x) prop cos(β(x-x_0))` where `x` denotes distance (item 3-1.8), then `β` denotes the phase coefficient.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseCoefficientUnit[1];
    }

    attribute phaseCoefficient: PhaseCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PhaseCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-26.3 propagation coefficient */
    attribute def PropagationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.3 propagation coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: PropagationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: measure of the change of amplitude and phase angle (item 3-7) of a plane wave propagating in a given direction
         * remarks: The propagation coefficient is given by: `γ = α + iβ` where `α` denotes attenuation (item 3-26.1) and `β` the phase coefficient (item 3-26.2) of a plane wave.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PropagationCoefficientUnit[1];
    }

    attribute propagationCoefficient: PropagationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PropagationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneUnit'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'CountValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dCoordinateFrame'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'isOrthogonal'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
RegularComment,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,KwDefault,OpenParen,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,CloseParen,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,OpenSquare,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
CloseCurly,
RegularComment,
RegularComment,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQSpaceTime'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (comment)
    (comment)
    (attribute_usage 'width' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'breadth' for 'width')
    (comment)
    (attribute_usage 'height' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'depth' for 'height')
    (alias_member 'altitude' for 'height')
    (comment)
    (attribute_usage 'thickness' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'diameter' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'radius' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'pathLength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'arcLength' for 'pathLength')
    (comment)
    (attribute_usage 'distance' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'radialDistance' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'Spatial3dCoordinateFrame' :> ''3dCoordinateFrame''
      (documentation)
      (attribute_usage :>> 'isBound' value))
    (attribute_def 'CartesianSpatial3dCoordinateFrame' :> 'Spatial3dCoordinateFrame'
      (documentation)
      (attribute_usage 'xUnit' : 'LengthUnit' value)
      (attribute_usage 'yUnit' : 'LengthUnit' value)
      (attribute_usage 'zUnit' : 'LengthUnit' value)
      (attribute_usage :>> 'mRefs' : 'LengthUnit' multiplicity)
      (attribute_usage :>> 'isOrthogonal' value))
    (attribute_usage 'universalCartesianSpatial3dCoordinateFrame' : 'CartesianSpatial3dCoordinateFrame' multiplicity
      (documentation)
      (attribute_usage :>> 'mRefs' value
        (documentation))
      (attribute_usage :>> 'transformation' multiplicity
        (documentation)))
    (attribute_def 'CylindricalSpatial3dCoordinateFrame' :> 'Spatial3dCoordinateFrame'
      (documentation)
      (attribute_usage 'radialDistanceUnit' : 'LengthUnit')
      (attribute_usage 'azimuthUnit' : 'AngularMeasureUnit')
      (attribute_usage 'zUnit' : 'LengthUnit')
      (attribute_usage :>> 'mRefs' value)
      (attribute_usage :>> 'isOrthogonal' value))
    (attribute_def 'SphericalSpatial3dCoordinateFrame' :> 'Spatial3dCoordinateFrame'
      (documentation)
      (attribute_usage 'radialDistanceUnit' : 'LengthUnit')
      (attribute_usage 'inclinationUnit' : 'AngularMeasureUnit')
      (attribute_usage 'azimuthUnit' : 'AngularMeasureUnit')
      (attribute_usage :>> 'mRefs' value)
      (attribute_usage :>> 'isOrthogonal' value))
    (attribute_def 'PlanetarySpatial3dCoordinateFrame' :> 'Spatial3dCoordinateFrame'
      (documentation)
      (attribute_usage 'latitudeUnit' : 'AngularMeasureUnit')
      (attribute_usage 'longitudeUnit' : 'AngularMeasureUnit')
      (attribute_usage 'altitudeUnit' : 'LengthUnit')
      (attribute_usage :>> 'mRefs' value)
      (attribute_usage :>> 'isOrthogonal' value))
    (comment)
    (attribute_def 'Position3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'Spatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'position3dVector' : 'Position3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianPosition3dVector' :> 'Position3dVector'
      (attribute_usage 'x' : 'LengthValue' value)
      (attribute_usage 'y' : 'LengthValue' value)
      (attribute_usage 'z' : 'LengthValue' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianPosition3dVector' : 'CartesianPosition3dVector' :> 'position3dVector')
    (attribute_def 'CylindricalPosition3dVector' :> 'Position3dVector'
      (attribute_usage 'radialDistance' : 'LengthValue' value)
      (attribute_usage 'azimuth' : 'AngularMeasureUnit' value)
      (attribute_usage 'height' : 'LengthValue' value)
      (attribute_usage :>> 'mRef' : 'CylindricalSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cylindricalPosition3dVector' : 'CylindricalPosition3dVector' :> 'position3dVector')
    (attribute_def 'SphericalPosition3dVector' :> 'Position3dVector'
      (attribute_usage 'radialDistance' : 'LengthValue' value)
      (attribute_usage 'inclination' : 'AngularMeasureUnit' value)
      (attribute_usage 'azimuth' : 'AngularMeasureUnit' value)
      (attribute_usage :>> 'mRef' : 'SphericalSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'sphericalPosition3dVector' : 'SphericalPosition3dVector' :> 'position3dVector')
    (attribute_def 'PlanetaryPosition3dVector' :> 'Position3dVector'
      (attribute_usage 'latitude' : 'AngularMeasureUnit' value)
      (attribute_usage 'longitude' : 'AngularMeasureUnit' value)
      (attribute_usage 'altitude' : 'LengthValue' value)
      (attribute_usage :>> 'mRef' : 'PlanetarySpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'planetaryPosition3dVector' : 'PlanetaryPosition3dVector' :> 'position3dVector')
    (comment)
    (attribute_def 'Displacement3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'Spatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'displacement3dVector' : 'Displacement3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianDisplacement3dVector' :> 'Displacement3dVector'
      (attribute_usage 'x' : 'LengthValue' value)
      (attribute_usage 'y' : 'LengthValue' value)
      (attribute_usage 'z' : 'LengthValue' value)
      (attribute_usage :>> 'mRef' : 'CartesianSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianDisplacement3dVector' : 'CartesianDisplacement3dVector' :> 'displacement3dVector')
    (attribute_def 'CylindricalDisplacement3dVector' :> 'Displacement3dVector'
      (attribute_usage 'radialDistance' : 'LengthValue' value)
      (attribute_usage 'azimuth' : 'AngularMeasureUnit' value)
      (attribute_usage 'height' : 'LengthValue' value)
      (attribute_usage :>> 'mRef' : 'CylindricalSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'cylindricalDisplacement3dVector' : 'CylindricalDisplacement3dVector' :> 'displacement3dVector')
    (attribute_def 'SphericalDisplacement3dVector' :> 'Displacement3dVector'
      (attribute_usage 'radialDistance' : 'LengthValue' value)
      (attribute_usage 'inclination' : 'AngularMeasureUnit' value)
      (attribute_usage 'azimuth' : 'AngularMeasureUnit' value)
      (attribute_usage :>> 'mRef' : 'SphericalSpatial3dCoordinateFrame' multiplicity))
    (attribute_usage 'sphericalDisplacement3dVector' : 'SphericalDisplacement3dVector' :> 'displacement3dVector')
    (comment)
    (attribute_usage 'radiusOfCurvature' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'CurvatureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CurvatureUnit' multiplicity))
    (attribute_usage 'curvature' : 'CurvatureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CurvatureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AreaValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AreaUnit' multiplicity))
    (attribute_usage 'area' : 'AreaValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AreaUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'VolumeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'VolumeUnit' multiplicity))
    (attribute_usage 'volume' : 'VolumeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'VolumeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AngularMeasureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularMeasureUnit' multiplicity))
    (attribute_usage 'angularMeasure' : 'AngularMeasureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularMeasureUnit' :> 'DimensionOneUnit')
    (alias_member 'PlaneAngleUnit' for 'AngularMeasureUnit')
    (alias_member 'PlaneAngleValue' for 'AngularMeasureValue')
    (alias_member 'planeAngle' for 'angularMeasure')
    (comment)
    (attribute_usage 'rotationalDisplacement' : 'AngularMeasureValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'angularDisplacement' for 'rotationalDisplacement')
    (comment)
    (attribute_usage 'phaseAngle' : 'AngularMeasureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SolidAngularMeasureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SolidAngularMeasureUnit' multiplicity))
    (attribute_usage 'solidAngularMeasure' : 'SolidAngularMeasureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SolidAngularMeasureUnit' :> 'DimensionOneUnit')
    (comment)
    (comment)
    (alias_member 'TimeUnit' for 'DurationUnit')
    (alias_member 'TimeValue' for 'DurationValue')
    (alias_member 'time' for 'duration')
    (comment)
    (attribute_def 'CartesianVelocity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianVelocity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianVelocity3dVector' : 'CartesianVelocity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianVelocity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'SpeedUnit' multiplicity))
    (comment)
    (attribute_def 'SpeedValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpeedUnit' multiplicity))
    (attribute_usage 'speed' : 'SpeedValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpeedUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AccelerationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AccelerationUnit' multiplicity))
    (attribute_usage 'acceleration' : 'AccelerationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AccelerationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAcceleration3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAcceleration3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAcceleration3dVector' : 'CartesianAcceleration3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAcceleration3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AccelerationUnit' multiplicity))
    (comment)
    (attribute_def 'AngularVelocityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularVelocityUnit' multiplicity))
    (attribute_usage 'angularVelocity' : 'AngularVelocityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularVelocityUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAngularVelocity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAngularVelocity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAngularVelocity3dVector' : 'CartesianAngularVelocity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAngularVelocity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AngularVelocityUnit' multiplicity))
    (comment)
    (attribute_def 'AngularAccelerationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularAccelerationUnit' multiplicity))
    (attribute_usage 'angularAcceleration' : 'AngularAccelerationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularAccelerationUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAngularAcceleration3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAngularAcceleration3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAngularAcceleration3dVector' : 'CartesianAngularAcceleration3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAngularAcceleration3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AngularAccelerationUnit' multiplicity))
    (comment)
    (attribute_usage 'periodDuration' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'period' for 'periodDuration')
    (comment)
    (attribute_usage 'timeConstant' : 'DurationValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'rotation' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'FrequencyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'FrequencyUnit' multiplicity))
    (attribute_usage 'frequency' : 'FrequencyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'FrequencyUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'rotationalFrequency' : 'FrequencyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'AngularFrequencyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularFrequencyUnit' multiplicity))
    (attribute_usage 'angularFrequency' : 'AngularFrequencyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularFrequencyUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'wavelength' : 'LengthValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'RepetencyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RepetencyUnit' multiplicity))
    (attribute_usage 'repetency' : 'RepetencyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RepetencyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'WavenumberUnit' for 'RepetencyUnit')
    (alias_member 'WavenumberValue' for 'RepetencyValue')
    (alias_member 'wavenumber' for 'repetency')
    (comment)
    (attribute_def 'CartesianWave3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianWaveVector3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianWave3dVector' : 'CartesianWave3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianWaveVector3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'RepetencyUnit' multiplicity))
    (comment)
    (attribute_def 'AngularRepetencyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularRepetencyUnit' multiplicity))
    (attribute_usage 'angularRepetency' : 'AngularRepetencyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularRepetencyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'AngularWavenumberUnit' for 'AngularRepetencyUnit')
    (alias_member 'AngularWavenumberValue' for 'AngularRepetencyValue')
    (alias_member 'angularWavenumber' for 'angularRepetency')
    (comment)
    (attribute_def 'PhaseVelocityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhaseVelocityUnit' multiplicity))
    (attribute_usage 'phaseVelocity' : 'PhaseVelocityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhaseVelocityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'PhaseSpeedUnit' for 'PhaseVelocityUnit')
    (alias_member 'PhaseSpeedValue' for 'PhaseVelocityValue')
    (alias_member 'phaseSpeed' for 'phaseVelocity')
    (comment)
    (attribute_usage 'groupVelocity' : 'SpeedValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'groupSpeed' for 'groupVelocity')
    (comment)
    (attribute_def 'DampingCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DampingCoefficientUnit' multiplicity))
    (attribute_usage 'dampingCoefficient' : 'DampingCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DampingCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LogarithmicDecrementValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'logarithmicDecrement' : 'LogarithmicDecrementValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AttenuationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AttenuationUnit' multiplicity))
    (attribute_usage 'attenuation' : 'AttenuationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AttenuationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'ExtinctionUnit' for 'AttenuationUnit')
    (alias_member 'ExtinctionValue' for 'AttenuationValue')
    (alias_member 'extinction' for 'attenuation')
    (comment)
    (attribute_def 'PhaseCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhaseCoefficientUnit' multiplicity))
    (attribute_usage 'phaseCoefficient' : 'PhaseCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhaseCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PropagationCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PropagationCoefficientUnit' multiplicity))
    (attribute_usage 'propagationCoefficient' : 'PropagationCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PropagationCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))))
~~~
# FORMAT
~~~sysml
standard library package ISQSpaceTime {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-3:2019 "Space and Time"
     * see also https://www.iso.org/standard/64974.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* ISO-80000-3 item 3-1.1 length */
    /* See package ISQBase for the declarations of LengthValue and LengthUnit */

    /* ISO-80000-3 item 3-1.2 width, breadth */
    attribute width : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.2 width, breadth
         * symbol(s): `b`, `B`
         * application domain: generic
         * name: Width (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between two parallel straight lines (in two dimensions) or planes (in three dimensions) that enclose a given geometrical shape
         * remarks: This quantity is non-negative.
         */
    }

    alias breadth for width;

    /* ISO-80000-3 item 3-1.3 height, depth, altitude */
    attribute height : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.3 height, depth, altitude
         * symbol(s): `h`, `H`
         * application domain: generic
         * name: Height (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between a point and a reference line or reference surface
         * remarks: This quantity is usually signed. The sign expresses the position of the particular point with respect to the reference line or surface and is chosen by convention. The symbol `H` is often used to denote altitude, i.e. height above sea level.
         */
    }

    alias depth for height;

    alias altitude for height;

    /* ISO-80000-3 item 3-1.4 thickness */
    attribute thickness : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.4 thickness
         * symbol(s): `d`, `δ`
         * application domain: generic
         * name: Thickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.5 diameter */
    attribute diameter : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.5 diameter
         * symbol(s): `d`, `D`
         * application domain: generic
         * name: Diameter (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2) of a circle, cylinder or sphere
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.6 radius */
    attribute radius : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.6 radius
         * symbol(s): `r`, `R`
         * application domain: generic
         * name: Radius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: half of a diameter (item 3-1.5)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.7 path length, arc length */
    attribute pathLength : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.7 path length, arc length
         * symbol(s): `s`
         * application domain: generic
         * name: PathLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length of a rectifiable curve between two of its points
         * remarks: The differential path length at a given point of a curve is: `ds = sqrt(dx^2 + dy^2 + dz^2)` where `x`, `y`, and `z` denote the Cartesian coordinates (ISO 80000-2) of the particular point. There are curves which are not rectifiable, for example fractal curves.
         */
    }

    alias arcLength for pathLength;

    /* ISO-80000-3 item 3-1.8 distance */
    attribute distance : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.8 distance
         * symbol(s): `d`, `r`
         * application domain: generic
         * name: Distance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: shortest path length (item 3-1.7) between two points in a metric space
         * remarks: A metric space might be curved. An example of a curved metric space is the surface of the Earth. In this case, distances are measured along great circles. A metric is not necessarily Euclidean.
         */
    }

    /* ISO-80000-3 item 3-1.9 radial distance */
    attribute radialDistance : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.9 radial distance
         * symbol(s): `r_Q`, `ρ`
         * application domain: generic
         * name: RadialDistance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (item 3-1.8), where one point is located on an axis or within a closed non self-intersecting curve or surface
         * remarks: The subscript Q denotes the point from which the radial distance is measured. Examples of closed non self-intersecting curves are circles or ellipses. Examples of closed non self-intersecting surfaces are surfaces of spheres or egg-shaped objects.
         */
    }

    /* Spatial coordinate frames */

    attribute def Spatial3dCoordinateFrame :> '3dCoordinateFrame' {
        doc /*
         * Most general spatial 3D coordinate frame
         */
        attribute :>> isBound = true;
    }

    attribute def CartesianSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc /*
         * Cartesian spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.1 Cartesian coordinates
         *
         * The components of a vector expressed on a Cartesian spatial coordinate frame are all LengthValues, and denoted with symbols `x`, `y`, `z`.
         *
         * Note 1: The Cartesian basis vectors `vec(e_x)`, `vec(e_y)` and `vec(e_z)` form an orthonormal right-handed coordinate frame.
         * Note 2: The measurement units for the 3 dimensions are typically the same, but may be different.
         */
        attribute xUnit : LengthUnit = mRefs#(1);
        attribute yUnit : LengthUnit = mRefs#(2);
        attribute zUnit : LengthUnit = mRefs#(3);
        attribute :>> mRefs : LengthUnit [3];
        attribute :>> isOrthogonal = true;
    }

    attribute universalCartesianSpatial3dCoordinateFrame : CartesianSpatial3dCoordinateFrame [1] {
        doc /*
         * A singleton CartesianSpatial3dCoordinateFrame that can be used as a default universal Cartesian 3D coordinate frame.
         */

        attribute :>> mRefs default = (SI::m, SI::m, SI::m) {
            doc /*
             * By default, the universalCartesianSpatial3dCoordinateFrame uses meters as the units on all three axes.
             */
        }

        attribute :>> transformation [0..0] {
            doc /*
             * The universalCartesianSpatial3dCoordinateFrame is the "top-level" coordinate frame, not nested in any other frame.
             */
        }
    }

    attribute def CylindricalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc /*
         * Cylindrical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.2 cylindrical coordinates
         *
         * The components of a (position) vector to a point P in a cylindrical coordinate frame are:
         * - radialDistance (symbol `ρ`) defined by LengthValue, that is the radial distance from the cylinder axis to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the cylinder axis, in the plane that is orthogonal to the cylinder axis and intersects P
         * - z coordinate (symbol `z`) defined by LengthValue, the coordinate along the clyinder axis.
         *
         * Note 1: The basis vectors `vec(e_ρ)(φ)`, `vec(e_φ)(φ)` and `vec(e_z)` form an orthonormal right-handed coordinate frame, where
         * `vec(e_φ)` is tangent to the circular arc in the `φ` direction.
         * Note 2: In order to enable transformation to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` direction in the cylindrical frame, and the `vec(e_z)` Cartesian basis vector is aligned with
         * the `vec(e_z)` cylindrical basis vector.
         * Note 3: If `z = 0`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Cylindrical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute zUnit : LengthUnit;
        attribute :>> mRefs = (radialDistanceUnit, azimuthUnit, zUnit);
        attribute :>> isOrthogonal = true;
    }

    attribute def SphericalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc /*
         * Spherical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.3 spherical coordinates
         *
         * The components of a (position) vector to a point P specified in a spherical coordinate frame are:
         * - radialDistance (symbol `r`) defined by LengthValue, that is the distance from the origin to P
         * - inclination (symbol `θ`) defined by AngularMeasure, that is the angle between the zenith direction and the line segment from origin to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the origin to the orthogonal projection of P on the reference plane, normal to the zenith direction.
         *
         * Note 1: The basis vectors `vec(e_r)(θ,φ)`, `vec(e_θ)(θ,φ)` and `vec(e_φ)(φ)` form an orthonormal right-handed frame, where
         * `vec(e_θ)` and `vec(e_φ)` are tangent to the respective circular arcs in the `θ` and `φ` directions.
         * Note 2: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `θ=π/4` and `φ=0` direction in the spherical frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `θ=0` zenith direction in the spherical frame.
         * Note 3: If `θ = π/4`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Spherical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute inclinationUnit : AngularMeasureUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute :>> mRefs = (radialDistanceUnit, inclinationUnit, azimuthUnit);
        attribute :>> isOrthogonal = true;
    }

    attribute def PlanetarySpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc /*
         * Planetary spatial 3D coordinate frame
         *
         * A planetary spatial 3D coordinate frame is a generalization for any planet of the geographic coordinate frame and geocentric coordinate
         * for Earth. In such coordinate frames, typically the origin is located at the planet's centre of gravity, and the surface of the planet
         * is approximated by a reference ellipsoid centred on the origin, with its major axes oriented along the south to north pole vector and
         * the equatorial plane.
         *
         * The components of a (position) vector to a point P specified in a planetary coordinate frame are:
         * - latitude (symbol `lat` or `φ`) defined by AngularMeasure, that is the angle between the equatorial plane and the vector from
         *   the origin to P, similar to the inclination in a spherical spatial coordinate frame. Typically, the zero reference latitude is chosen
         *   for positions in the equatorial plane, with positive latitude for positions in the northern hemisphere and negative latitude for positions
         *   in the southern hemisphere.
         * - longitude (symbol `long` or `λ`) defined by AngularMeasure, that is the angle between a reference meridian and the meridian
         *   passing through P, similar to the azimuth of a spherical spatial coordinate frame. The convention is to connotate positive longitude
         *   with eastward direction and negative longitude with westward direction. The reference meridian for `long=0` is chosen to pass
         *   through a particular feature of the planet, e.g., for Earth typically the position of the British Royal Observatory in Greenwich, UK.
         * - altitude (symbol `h`) defined by LengthValue, that is the distance between P and the reference ellipsoid
         *   in the normal direction to the ellipsoid. Positive altitude specifies a position above the reference ellipsoid surface,
         *   while a negative value specifies a position below.
         *
         * Note 1: The reference meridian is also called prime meridian.
         * Note 2: The basis vectors `vec(e_φ)(φ)`, `vec(e_λ)(λ)` and `vec(e_h)(φ,λ)` form an orthonormal right-handed frame, where
         * `vec(e_φ)` and `vec(e_λ)` are tangent to the reference ellipsoid in the respective latitude and longitude directions,
         * and `vec(e_h)` is normal to the reference ellipsoid.
         * Note 3: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` and `λ=0` direction in the planetary frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `λ=π/2` (north pole) direction in the planetary frame.
         * Note 4: See also https://en.wikipedia.org/wiki/Planetary_coordinate_system .
         */
        attribute latitudeUnit : AngularMeasureUnit;
        attribute longitudeUnit : AngularMeasureUnit;
        attribute altitudeUnit : LengthUnit;
        attribute :>> mRefs = (longitudeUnit, latitudeUnit, altitudeUnit);
        attribute :>> isOrthogonal = true;
    }

    /* ISO-80000-3 item 3-1.10 position vector */
    attribute def Position3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-1.10 position vector
         * symbol(s): `vec(r)`
         * application domain: generic
         * name: PositionVector
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity from the origin of a coordinate system to a point in space
         * remarks: Position vectors are so-called bounded vectors, i.e. their magnitude (ISO 80000-2) and direction depend on the particular coordinate system used.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : Spatial3dCoordinateFrame [1];
    }

    attribute position3dVector : Position3dVector :> vectorQuantities;

    attribute def CartesianPosition3dVector :> Position3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }
    attribute cartesianPosition3dVector : CartesianPosition3dVector :> position3dVector;

    attribute def CylindricalPosition3dVector :> Position3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame [1];
    }
    attribute cylindricalPosition3dVector : CylindricalPosition3dVector :> position3dVector;

    attribute def SphericalPosition3dVector :> Position3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame [1];
    }
    attribute sphericalPosition3dVector : SphericalPosition3dVector :> position3dVector;

    attribute def PlanetaryPosition3dVector :> Position3dVector {
        attribute <lat> latitude : AngularMeasureUnit = num#(1) [mRef.mRefs#(1)];
        attribute <long> longitude : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> altitude : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : PlanetarySpatial3dCoordinateFrame [1];
    }
    attribute planetaryPosition3dVector : PlanetaryPosition3dVector :> position3dVector;

    /* ISO-80000-3 item 3-1.11 displacement */
    attribute def Displacement3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-1.11 displacement
         * symbol(s): `vec(Δr)`
         * application domain: generic
         * name: Displacement
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity between any two points in space
         * remarks: Displacement vectors are so-called free vectors, i.e. their magnitude (ISO 80000-2) and direction do not depend on a particular coordinate system. The magnitude of this vector is also called displacement.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : Spatial3dCoordinateFrame [1];
    }

    attribute displacement3dVector : Displacement3dVector :> vectorQuantities;

    attribute def CartesianDisplacement3dVector :> Displacement3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame [1];
    }
    attribute cartesianDisplacement3dVector : CartesianDisplacement3dVector :> displacement3dVector;

    attribute def CylindricalDisplacement3dVector :> Displacement3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame [1];
    }
    attribute cylindricalDisplacement3dVector : CylindricalDisplacement3dVector :> displacement3dVector;

    attribute def SphericalDisplacement3dVector :> Displacement3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame [1];
    }
    attribute sphericalDisplacement3dVector : SphericalDisplacement3dVector :> displacement3dVector;

    /* ISO-80000-3 item 3-1.12 radius of curvature */
    attribute radiusOfCurvature : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-1.12 radius of curvature
         * symbol(s): `ρ`
         * application domain: generic
         * name: RadiusOfCurvature (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (item 3-1.6) of the osculating circle of a planar curve at a particular point of the curve
         * remarks: The radius of curvature is only defined for curves which are at least twice continuously differentiable.
         */
    }

    /* ISO-80000-3 item 3-2 curvature */
    attribute def CurvatureValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-2 curvature
         * symbol(s): `κ`
         * application domain: generic
         * name: Curvature
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the radius of curvature (item 3-1.12)
         * remarks: The curvature is given by: `κ = 1/ρ` where `ρ` denotes the radius of curvature (item 3-1.12).
         */
        attribute :>> num : Real;
        attribute :>> mRef : CurvatureUnit [1];
    }

    attribute curvature : CurvatureValue :> scalarQuantities [*] nonunique;

    attribute def CurvatureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-3 item 3-3 area */
    attribute def AreaValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-3 area
         * symbol(s): `A`, `S`
         * application domain: generic
         * name: Area
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: extent of a two-dimensional geometrical shape
         * remarks: The surface element at a given point of a surface is given by: `dA = g du dv` where `u` and `v` denote the Gaussian surface coordinates and `g` denotes the determinant of the metric tensor (ISO 80000-2) at the particular point. The symbol `dσ` is also used for the surface element.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AreaUnit [1];
    }

    attribute area : AreaValue :> scalarQuantities [*] nonunique;

    attribute def AreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-3 item 3-4 volume */
    attribute def VolumeValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-4 volume
         * symbol(s): `V`, `(S)`
         * application domain: generic
         * name: Volume
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: extent of a three-dimensional geometrical shape
         * remarks: The volume element in Euclidean space is given by: `dV = dx dy dz` where `dx`, `dy`, and `dz` denote the differentials of the Cartesian coordinates (ISO 80000-2). The symbol `dτ` is also used for the volume element.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeUnit [1];
    }

    attribute volume : VolumeValue :> scalarQuantities [*] nonunique;

    attribute def VolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-3 item 3-5 angular measure, plane angle */
    attribute def AngularMeasureValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-5 angular measure, plane angle
         * symbol(s): `α`, `β`, `γ`
         * application domain: generic
         * name: AngularMeasure
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: measure of a geometric figure, called plane angle, formed by two rays, called the sides of the plane angle, emanating from a common point, called the vertex of the plane angle
         * remarks: The angular measure is given by: `α = s/r` where `s` denotes the arc length (item 3-1.7) of the included arc of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. Other symbols are also used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularMeasureUnit [1];
    }

    attribute angularMeasure : AngularMeasureValue :> scalarQuantities [*] nonunique;

    attribute def AngularMeasureUnit :> DimensionOneUnit { }

    alias PlaneAngleUnit for AngularMeasureUnit;
    alias PlaneAngleValue for AngularMeasureValue;
    alias planeAngle for angularMeasure;

    /* ISO-80000-3 item 3-6 rotational displacement, angular displacement */
    attribute rotationalDisplacement : AngularMeasureValue :> scalarQuantities {
        doc /*
         * source: item 3-6 rotational displacement, angular displacement
         * symbol(s): `ϑ`, `φ`
         * application domain: generic
         * name: RotationalDisplacement (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: quotient of the traversed circular path length (item 3-1.7) of a point in space during a rotation and its distance (item 3-1.8) from the axis or centre of rotation
         * remarks: The rotational displacement is given by: `φ = s/r` where `s` denotes the traversed path length (item 3-1.7) along the periphery of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. The rotational displacement is signed. The sign denotes the direction of rotation and is chosen by convention. Other symbols are also used.
         */
    }

    alias angularDisplacement for rotationalDisplacement;

    /* ISO-80000-3 item 3-7 phase angle */
    attribute phaseAngle : AngularMeasureValue :> scalarQuantities {
        doc /*
         * source: item 3-7 phase angle
         * symbol(s): `φ`, `ϕ`
         * application domain: generic
         * name: PhaseAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: angular measure (item 3-5) between the positive real axis and the radius of the polar representation of the complex number in the complex plane
         * remarks: The phase angle (often imprecisely referred to as the "phase") is the argument of a complex number. Other symbols are also used.
         */
    }

    /* ISO-80000-3 item 3-8 solid angular measure */
    attribute def SolidAngularMeasureValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-8 solid angular measure
         * symbol(s): `Ω`
         * application domain: generic
         * name: SolidAngularMeasure
         * quantity dimension: 1
         * measurement unit(s): sr, 1
         * tensor order: 0
         * definition: measure of a conical geometric figure, called solid angle, formed by all rays, originating from a common point, called the vertex of the solid angle, and passing through the points of a closed, non-self-intersecting curve in space considered as the border of a surface
         * remarks: The differential solid angular measure expressed in spherical coordinates (ISO 80000-2) is given by: `dΩ = A/r^2 * sin(θ * dθ * dφ)` where `A` is area, `r` is radius, `θ` and `φ` are spherical coordinates.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SolidAngularMeasureUnit [1];
    }

    attribute solidAngularMeasure : SolidAngularMeasureValue :> scalarQuantities [*] nonunique;

    attribute def SolidAngularMeasureUnit :> DimensionOneUnit { }

    /* ISO-80000-3 item 3-9 duration, time */
    /* See package ISQBase for the declarations of DurationValue and DurationUnit */

    alias TimeUnit for DurationUnit;
    alias TimeValue for DurationValue;
    alias time for duration;

    /* ISO-80000-3 item 3-10.1 velocity */
    attribute def CartesianVelocity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-10.1 velocity
         * symbol(s): `vec(v)`, `u,v,w`
         * application domain: generic
         * name: Velocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of a position vector (item 3-1.10)
         * remarks: The velocity vector is given by: `vec(v) = (d vec(r)) / (dt)` where `vec(r)` denotes the position vector (item 3-1.10) and `t` the duration (item 3-9). When the general symbol `vec(v)` is not used for the velocity, the symbols `u`, `v`, `w` may be used for the components (ISO 80000-2) of the velocity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianVelocity3dCoordinateFrame [1];
    }

    attribute cartesianVelocity3dVector : CartesianVelocity3dVector :> vectorQuantities;

    attribute def CartesianVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : SpeedUnit [3];
    }

    /* ISO-80000-3 item 3-10.2 speed */
    attribute def SpeedValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-10.2 speed
         * symbol(s): `v`
         * application domain: generic
         * name: Speed
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the velocity (item 3-10.1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpeedUnit [1];
    }

    attribute speed : SpeedValue :> scalarQuantities [*] nonunique;

    attribute def SpeedUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-3 item 3-11 acceleration */
    attribute def AccelerationValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-11 acceleration (magnitude)
         * symbol(s): `a`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AccelerationUnit [1];
    }

    attribute acceleration : AccelerationValue :> scalarQuantities [*] nonunique;

    attribute def AccelerationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    attribute def CartesianAcceleration3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-11 acceleration (vector)
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAcceleration3dCoordinateFrame [1];
    }

    attribute cartesianAcceleration3dVector : CartesianAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AccelerationUnit [3];
    }

    /* ISO-80000-3 item 3-12 angular velocity */
    attribute def AngularVelocityValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-12 angular velocity (magnitude)
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularVelocityUnit [1];
    }

    attribute angularVelocity : AngularVelocityValue :> scalarQuantities [*] nonunique;

    attribute def AngularVelocityUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = durationPF;
        }
    }

    attribute def CartesianAngularVelocity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-12 angular velocity (vector)
         * symbol(s): `vec(ω)`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularVelocity3dCoordinateFrame [1];
    }

    attribute cartesianAngularVelocity3dVector : CartesianAngularVelocity3dVector :> vectorQuantities;

    attribute def CartesianAngularVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularVelocityUnit [3];
    }

    /* ISO-80000-3 item 3-13 angular acceleration */
    attribute def AngularAccelerationValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-13 angular acceleration (magnitude)
         * symbol(s): `α`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularAccelerationUnit [1];
    }

    attribute angularAcceleration : AngularAccelerationValue :> scalarQuantities [*] nonunique;

    attribute def AngularAccelerationUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = durationPF;
        }
    }

    attribute def CartesianAngularAcceleration3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-13 angular acceleration (vector)
         * symbol(s): `vec(α)`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularAcceleration3dCoordinateFrame [1];
    }

    attribute cartesianAngularAcceleration3dVector : CartesianAngularAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAngularAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularAccelerationUnit [3];
    }

    /* ISO-80000-3 item 3-14 period duration, period */
    attribute periodDuration : DurationValue :> scalarQuantities {
        doc /*
         * source: item 3-14 period duration, period
         * symbol(s): `T`
         * application domain: generic
         * name: PeriodDuration (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: A periodic event is an event that occurs regularly with a fixed time interval.
         */
    }

    alias period for periodDuration;

    /* ISO-80000-3 item 3-15 time constant */
    attribute timeConstant : DurationValue :> scalarQuantities {
        doc /*
         * source: item 3-15 time constant
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: TimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: parameter characterizing the response to a step input of a first-order, linear time-invariant system
         * remarks: If a quantity is a function of the duration (item 3-9) expressed by: `F(t) prop e^(-t/τ)` where `t` denotes the duration (item 3-9), then `τ` denotes the time constant. Here the time constant `τ` applies to an exponentially decaying quantity.
         */
    }

    /* ISO-80000-3 item 3-16 rotation */
    attribute rotation : CountValue :> scalarQuantities {
        doc /*
         * source: item 3-16 rotation
         * symbol(s): `N`
         * application domain: generic
         * name: Rotation (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of revolutions
         * remarks: `N` is the number (not necessarily an integer) of revolutions, for example, of a rotating body about a given axis. Its value is given by: `N = φ/(2 π)` where `φ` denotes the measure of rotational displacement (item 3-6).
         */
    }

    /* ISO-80000-3 item 3-17.1 frequency */
    attribute def FrequencyValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-17.1 frequency
         * symbol(s): `f`, `ν`
         * application domain: generic
         * name: Frequency
         * quantity dimension: T^-1
         * measurement unit(s): Hz, s^-1
         * tensor order: 0
         * definition: inverse of period duration (item 3-14)
         * remarks: The frequency is given by: `f = 1/T` where `T` denotes the period duration (item 3-14).
         */
        attribute :>> num : Real;
        attribute :>> mRef : FrequencyUnit [1];
    }

    attribute frequency : FrequencyValue :> scalarQuantities [*] nonunique;

    attribute def FrequencyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = durationPF;
        }
    }

    /* ISO-80000-3 item 3-17.2 rotational frequency */
    attribute rotationalFrequency : FrequencyValue :> scalarQuantities {
        doc /*
         * source: item 3-17.2 rotational frequency
         * symbol(s): `n`
         * application domain: generic
         * name: RotationalFrequency (specializes Frequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: The rotational frequency is given by: `n = (dN) / (dt)` where `N` denotes the rotation (item 3-16) and `t` is the duration (item 3-9).
         */
    }

    /* ISO-80000-3 item 3-18 angular frequency */
    attribute def AngularFrequencyValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-18 angular frequency
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularFrequency
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: rate of change of the phase angle (item 3-7)
         * remarks: The angular frequency is given by: `ω = 2 π f` where `f` denotes the frequency (item 3-17.1).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularFrequencyUnit [1];
    }

    attribute angularFrequency : AngularFrequencyValue :> scalarQuantities [*] nonunique;

    attribute def AngularFrequencyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = durationPF;
        }
    }

    /* ISO-80000-3 item 3-19 wavelength */
    attribute wavelength : LengthValue :> scalarQuantities {
        doc /*
         * source: item 3-19 wavelength
         * symbol(s): `λ`
         * application domain: generic
         * name: Wavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length (item 3-1.1) of the repetition interval of a wave
         * remarks: None.
         */
    }

    /* ISO-80000-3 item 3-20 repetency, wavenumber */
    attribute def RepetencyValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-20 repetency, wavenumber
         * symbol(s): `σ`, `ṽ`
         * application domain: generic
         * name: Repetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the wavelength (item 3-19)
         * remarks: The repetency is given by: `σ = 1 / λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RepetencyUnit [1];
    }

    attribute repetency : RepetencyValue :> scalarQuantities [*] nonunique;

    attribute def RepetencyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    alias WavenumberUnit for RepetencyUnit;
    alias WavenumberValue for RepetencyValue;
    alias wavenumber for repetency;

    /* ISO-80000-3 item 3-21 wave vector */
    attribute def CartesianWave3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 3-21 wave vector
         * symbol(s): `vec(k)`
         * application domain: generic
         * name: WaveVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector normal to the surfaces of constant phase angle (item 3-7) of a wave, with the magnitude (ISO 80000-2) of repetency (item 3-20)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianWaveVector3dCoordinateFrame [1];
    }

    attribute cartesianWave3dVector : CartesianWave3dVector :> vectorQuantities;

    attribute def CartesianWaveVector3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : RepetencyUnit [3];
    }

    /* ISO-80000-3 item 3-22 angular repetency, angular wavenumber */
    attribute def AngularRepetencyValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-22 angular repetency, angular wavenumber
         * symbol(s): `k`
         * application domain: generic
         * name: AngularRepetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the wave vector (item 3-21)
         * remarks: The angular repetency is given by: `κ = (2 π)/λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularRepetencyUnit [1];
    }

    attribute angularRepetency : AngularRepetencyValue :> scalarQuantities [*] nonunique;

    attribute def AngularRepetencyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    alias AngularWavenumberUnit for AngularRepetencyUnit;
    alias AngularWavenumberValue for AngularRepetencyValue;
    alias angularWavenumber for angularRepetency;

    /* ISO-80000-3 item 3-23.1 phase velocity, phase speed */
    attribute def PhaseVelocityValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-23.1 phase velocity, phase speed
         * symbol(s): `c`, `v`, `(ν)`, `c_φ`, `v_φ`, `(ν_φ)`
         * application domain: generic
         * name: PhaseVelocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the phase angle (item 3-7) of a wave propagates in space
         * remarks: The phase velocity is given by: `c = ω/κ` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22). If phase velocities of electromagnetic waves and other phase velocities are both involved, then `c` should be used for the former and `υ` for the latter. Phase velocity can also be written as `c = λ f`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseVelocityUnit [1];
    }

    attribute phaseVelocity : PhaseVelocityValue :> scalarQuantities [*] nonunique;

    attribute def PhaseVelocityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    alias PhaseSpeedUnit for PhaseVelocityUnit;
    alias PhaseSpeedValue for PhaseVelocityValue;
    alias phaseSpeed for phaseVelocity;

    /* ISO-80000-3 item 3-23.2 group velocity, group speed */
    attribute groupVelocity : SpeedValue :> scalarQuantities {
        doc /*
         * source: item 3-23.2 group velocity, group speed
         * symbol(s): `c_g`, `v_g`, `(ν_g)`
         * application domain: generic
         * name: GroupVelocity (specializes Speed)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the envelope of a wave propagates in space
         * remarks: The group velocity is given by: `c_g = (d ω)/ (dk)` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22).
         */
    }

    alias groupSpeed for groupVelocity;

    /* ISO-80000-3 item 3-24 damping coefficient */
    attribute def DampingCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-24 damping coefficient
         * symbol(s): `δ`
         * application domain: generic
         * name: DampingCoefficient
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: inverse of the time constant (item 3-15) of an exponentially varying quantity
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DampingCoefficientUnit [1];
    }

    attribute dampingCoefficient : DampingCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def DampingCoefficientUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = durationPF;
        }
    }

    /* ISO-80000-3 item 3-25 logarithmic decrement */
    attribute def LogarithmicDecrementValue :> DimensionOneValue {
        doc /*
         * source: item 3-25 logarithmic decrement
         * symbol(s): `Λ`
         * application domain: generic
         * name: LogarithmicDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of damping coefficient (item 3-24) and period duration (item 3-14)
         * remarks: None.
         */
    }
    attribute logarithmicDecrement : LogarithmicDecrementValue :> scalarQuantities;

    /* ISO-80000-3 item 3-26.1 attenuation, extinction */
    attribute def AttenuationValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-26.1 attenuation, extinction
         * symbol(s): `α`
         * application domain: generic
         * name: Attenuation
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: gradual decrease in magnitude (ISO 80000-2) of any kind of flux through a medium
         * remarks: If a quantity is a function of distance (item 3-1.8) expressed by: `f(x) prop e^(-α x)` where `x` denotes distance (item 3-1.8), then `α` denotes attenuation. The inverse of attenuation is called attenuation length.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AttenuationUnit [1];
    }

    attribute attenuation : AttenuationValue :> scalarQuantities [*] nonunique;

    attribute def AttenuationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    alias ExtinctionUnit for AttenuationUnit;
    alias ExtinctionValue for AttenuationValue;
    alias extinction for attenuation;

    /* ISO-80000-3 item 3-26.2 phase coefficient */
    attribute def PhaseCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-26.2 phase coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PhaseCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): rad/m, m^-1
         * tensor order: 0
         * definition: change of phase angle (item 3-7) with the length (item 3-1.1) along the path travelled by a plane wave
         * remarks: If a quantity is a function of distance expressed by: `f(x) prop cos(β(x-x_0))` where `x` denotes distance (item 3-1.8), then `β` denotes the phase coefficient.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseCoefficientUnit [1];
    }

    attribute phaseCoefficient : PhaseCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def PhaseCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-3 item 3-26.3 propagation coefficient */
    attribute def PropagationCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 3-26.3 propagation coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: PropagationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: measure of the change of amplitude and phase angle (item 3-7) of a plane wave propagating in a given direction
         * remarks: The propagation coefficient is given by: `γ = α + iβ` where `α` denotes attenuation (item 3-26.1) and `β` the phase coefficient (item 3-26.2) of a plane wave.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PropagationCoefficientUnit [1];
    }

    attribute propagationCoefficient : PropagationCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def PropagationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ISQSpaceTime'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQBase'[unresolved])
      (attribute_usage 'width' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'breadth' -> 'ISQSpaceTime::width'[attribute_usage])
      (attribute_usage 'height' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'depth' -> 'ISQSpaceTime::height'[attribute_usage])
      (alias_member 'altitude' -> 'ISQSpaceTime::height'[attribute_usage])
      (attribute_usage 'thickness' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'diameter' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'radius' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'pathLength' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'arcLength' -> 'ISQSpaceTime::pathLength'[attribute_usage])
      (attribute_usage 'distance' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'radialDistance' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'Spatial3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=))))
      (attribute_def 'CartesianSpatial3dCoordinateFrame' :> 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite 'xUnit' : 'LengthUnit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'yUnit' : 'LengthUnit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'zUnit' : 'LengthUnit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'LengthUnit'[unresolved]
          (multiplicity_range [3]))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=))))
      (attribute_usage 'universalCartesianSpatial3dCoordinateFrame' : 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame'[attribute_def]
        (multiplicity_range [1])
        (documentation)
        (attribute_usage composite :>> 'mRefs'[unresolved]
          (feature_value (default =))
          (documentation))
        (attribute_usage composite :>> 'transformation'[unresolved]
          (multiplicity_range [0..0])
          (documentation)))
      (attribute_def 'CylindricalSpatial3dCoordinateFrame' :> 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite 'radialDistanceUnit' : 'LengthUnit'[unresolved])
        (attribute_usage composite 'azimuthUnit' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
        (attribute_usage composite 'zUnit' : 'LengthUnit'[unresolved])
        (attribute_usage composite :>> 'mRefs'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=))))
      (attribute_def 'SphericalSpatial3dCoordinateFrame' :> 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite 'radialDistanceUnit' : 'LengthUnit'[unresolved])
        (attribute_usage composite 'inclinationUnit' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
        (attribute_usage composite 'azimuthUnit' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
        (attribute_usage composite :>> 'mRefs'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=))))
      (attribute_def 'PlanetarySpatial3dCoordinateFrame' :> 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite 'latitudeUnit' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
        (attribute_usage composite 'longitudeUnit' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
        (attribute_usage composite 'altitudeUnit' : 'LengthUnit'[unresolved])
        (attribute_usage composite :>> 'mRefs'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=))))
      (attribute_def 'Position3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'position3dVector' : 'ISQSpaceTime::Position3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianPosition3dVector' :> 'ISQSpaceTime::Position3dVector'[attribute_def]
        (attribute_usage composite 'x' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'y' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'z' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianPosition3dVector' : 'ISQSpaceTime::CartesianPosition3dVector'[attribute_def] :> 'ISQSpaceTime::position3dVector'[attribute_usage])
      (attribute_def 'CylindricalPosition3dVector' :> 'ISQSpaceTime::Position3dVector'[attribute_def]
        (attribute_usage composite 'radialDistance' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'azimuth' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'height' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CylindricalSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cylindricalPosition3dVector' : 'ISQSpaceTime::CylindricalPosition3dVector'[attribute_def] :> 'ISQSpaceTime::position3dVector'[attribute_usage])
      (attribute_def 'SphericalPosition3dVector' :> 'ISQSpaceTime::Position3dVector'[attribute_def]
        (attribute_usage composite 'radialDistance' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'inclination' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'azimuth' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::SphericalSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'sphericalPosition3dVector' : 'ISQSpaceTime::SphericalPosition3dVector'[attribute_def] :> 'ISQSpaceTime::position3dVector'[attribute_usage])
      (attribute_def 'PlanetaryPosition3dVector' :> 'ISQSpaceTime::Position3dVector'[attribute_def]
        (attribute_usage composite 'latitude' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'longitude' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'altitude' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::PlanetarySpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'planetaryPosition3dVector' : 'ISQSpaceTime::PlanetaryPosition3dVector'[attribute_def] :> 'ISQSpaceTime::position3dVector'[attribute_usage])
      (attribute_def 'Displacement3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::Spatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'displacement3dVector' : 'ISQSpaceTime::Displacement3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianDisplacement3dVector' :> 'ISQSpaceTime::Displacement3dVector'[attribute_def]
        (attribute_usage composite 'x' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'y' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'z' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianDisplacement3dVector' : 'ISQSpaceTime::CartesianDisplacement3dVector'[attribute_def] :> 'ISQSpaceTime::displacement3dVector'[attribute_usage])
      (attribute_def 'CylindricalDisplacement3dVector' :> 'ISQSpaceTime::Displacement3dVector'[attribute_def]
        (attribute_usage composite 'radialDistance' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'azimuth' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'height' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CylindricalSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cylindricalDisplacement3dVector' : 'ISQSpaceTime::CylindricalDisplacement3dVector'[attribute_def] :> 'ISQSpaceTime::displacement3dVector'[attribute_usage])
      (attribute_def 'SphericalDisplacement3dVector' :> 'ISQSpaceTime::Displacement3dVector'[attribute_def]
        (attribute_usage composite 'radialDistance' : 'LengthValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'inclination' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite 'azimuth' : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::SphericalSpatial3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'sphericalDisplacement3dVector' : 'ISQSpaceTime::SphericalDisplacement3dVector'[attribute_def] :> 'ISQSpaceTime::displacement3dVector'[attribute_usage])
      (attribute_usage 'radiusOfCurvature' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'CurvatureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CurvatureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'curvature' : 'ISQSpaceTime::CurvatureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'CurvatureUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'AreaValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AreaUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'area' : 'ISQSpaceTime::AreaValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AreaUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'VolumeValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::VolumeUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'volume' : 'ISQSpaceTime::VolumeValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'VolumeUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'AngularMeasureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AngularMeasureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularMeasure' : 'ISQSpaceTime::AngularMeasureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularMeasureUnit' :> 'DimensionOneUnit'[unresolved])
      (alias_member 'PlaneAngleUnit' -> 'ISQSpaceTime::AngularMeasureUnit'[attribute_def])
      (alias_member 'PlaneAngleValue' -> 'ISQSpaceTime::AngularMeasureValue'[attribute_def])
      (alias_member 'planeAngle' -> 'ISQSpaceTime::angularMeasure'[attribute_usage])
      (attribute_usage 'rotationalDisplacement' : 'ISQSpaceTime::AngularMeasureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'angularDisplacement' -> 'ISQSpaceTime::rotationalDisplacement'[attribute_usage])
      (attribute_usage 'phaseAngle' : 'ISQSpaceTime::AngularMeasureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'SolidAngularMeasureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::SolidAngularMeasureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'solidAngularMeasure' : 'ISQSpaceTime::SolidAngularMeasureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SolidAngularMeasureUnit' :> 'DimensionOneUnit'[unresolved])
      (alias_member 'TimeUnit' -> 'DurationUnit'[unresolved])
      (alias_member 'TimeValue' -> 'DurationValue'[unresolved])
      (alias_member 'time' -> 'duration'[unresolved])
      (attribute_def 'CartesianVelocity3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianVelocity3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianVelocity3dVector' : 'ISQSpaceTime::CartesianVelocity3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianVelocity3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQSpaceTime::SpeedUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'SpeedValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::SpeedUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'speed' : 'ISQSpaceTime::SpeedValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpeedUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'AccelerationValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AccelerationUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'acceleration' : 'ISQSpaceTime::AccelerationValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AccelerationUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CartesianAcceleration3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianAcceleration3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianAcceleration3dVector' : 'ISQSpaceTime::CartesianAcceleration3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianAcceleration3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQSpaceTime::AccelerationUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'AngularVelocityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AngularVelocityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularVelocity' : 'ISQSpaceTime::AngularVelocityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularVelocityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CartesianAngularVelocity3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianAngularVelocity3dVector' : 'ISQSpaceTime::CartesianAngularVelocity3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianAngularVelocity3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQSpaceTime::AngularVelocityUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'AngularAccelerationValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AngularAccelerationUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularAcceleration' : 'ISQSpaceTime::AngularAccelerationValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularAccelerationUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CartesianAngularAcceleration3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianAngularAcceleration3dVector' : 'ISQSpaceTime::CartesianAngularAcceleration3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianAngularAcceleration3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQSpaceTime::AngularAccelerationUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_usage 'periodDuration' : 'DurationValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'period' -> 'ISQSpaceTime::periodDuration'[attribute_usage])
      (attribute_usage 'timeConstant' : 'DurationValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'rotation' : 'CountValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'FrequencyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::FrequencyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'frequency' : 'ISQSpaceTime::FrequencyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'FrequencyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'rotationalFrequency' : 'ISQSpaceTime::FrequencyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'AngularFrequencyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AngularFrequencyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularFrequency' : 'ISQSpaceTime::AngularFrequencyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularFrequencyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'wavelength' : 'LengthValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'RepetencyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::RepetencyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'repetency' : 'ISQSpaceTime::RepetencyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'RepetencyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (alias_member 'WavenumberUnit' -> 'ISQSpaceTime::RepetencyUnit'[attribute_def])
      (alias_member 'WavenumberValue' -> 'ISQSpaceTime::RepetencyValue'[attribute_def])
      (alias_member 'wavenumber' -> 'ISQSpaceTime::repetency'[attribute_usage])
      (attribute_def 'CartesianWave3dVector' :> '3dVectorQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::CartesianWaveVector3dCoordinateFrame'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cartesianWave3dVector' : 'ISQSpaceTime::CartesianWave3dVector'[attribute_def] :> 'vectorQuantities'[unresolved])
      (attribute_def 'CartesianWaveVector3dCoordinateFrame' :> '3dCoordinateFrame'[unresolved]
        (attribute_usage composite :>> 'isBound'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'isOrthogonal'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'mRefs'[unresolved] : 'ISQSpaceTime::RepetencyUnit'[attribute_def]
          (multiplicity_range [3])))
      (attribute_def 'AngularRepetencyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AngularRepetencyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'angularRepetency' : 'ISQSpaceTime::AngularRepetencyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AngularRepetencyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (alias_member 'AngularWavenumberUnit' -> 'ISQSpaceTime::AngularRepetencyUnit'[attribute_def])
      (alias_member 'AngularWavenumberValue' -> 'ISQSpaceTime::AngularRepetencyValue'[attribute_def])
      (alias_member 'angularWavenumber' -> 'ISQSpaceTime::angularRepetency'[attribute_usage])
      (attribute_def 'PhaseVelocityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::PhaseVelocityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'phaseVelocity' : 'ISQSpaceTime::PhaseVelocityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'PhaseVelocityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (alias_member 'PhaseSpeedUnit' -> 'ISQSpaceTime::PhaseVelocityUnit'[attribute_def])
      (alias_member 'PhaseSpeedValue' -> 'ISQSpaceTime::PhaseVelocityValue'[attribute_def])
      (alias_member 'phaseSpeed' -> 'ISQSpaceTime::phaseVelocity'[attribute_usage])
      (attribute_usage 'groupVelocity' : 'ISQSpaceTime::SpeedValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'groupSpeed' -> 'ISQSpaceTime::groupVelocity'[attribute_usage])
      (attribute_def 'DampingCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::DampingCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'dampingCoefficient' : 'ISQSpaceTime::DampingCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'DampingCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'LogarithmicDecrementValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'logarithmicDecrement' : 'ISQSpaceTime::LogarithmicDecrementValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'AttenuationValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::AttenuationUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'attenuation' : 'ISQSpaceTime::AttenuationValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'AttenuationUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (alias_member 'ExtinctionUnit' -> 'ISQSpaceTime::AttenuationUnit'[attribute_def])
      (alias_member 'ExtinctionValue' -> 'ISQSpaceTime::AttenuationValue'[attribute_def])
      (alias_member 'extinction' -> 'ISQSpaceTime::attenuation'[attribute_usage])
      (attribute_def 'PhaseCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::PhaseCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'phaseCoefficient' : 'ISQSpaceTime::PhaseCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'PhaseCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'PropagationCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQSpaceTime::PropagationCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'propagationCoefficient' : 'ISQSpaceTime::PropagationCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'PropagationCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=))))))))
~~~
