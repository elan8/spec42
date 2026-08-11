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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQSpaceTime"))) (name "ISQSpaceTime") (declared-name "ISQSpaceTime")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQSpaceTime::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (name "AccelerationUnit") (declared-name "AccelerationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (name "AccelerationValue") (declared-name "AccelerationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (name "AngularAccelerationUnit") (declared-name "AngularAccelerationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (name "AngularAccelerationValue") (declared-name "AngularAccelerationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (name "AngularFrequencyUnit") (declared-name "AngularFrequencyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (name "AngularFrequencyValue") (declared-name "AngularFrequencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (name "AngularMeasureUnit") (declared-name "AngularMeasureUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (name "AngularRepetencyUnit") (declared-name "AngularRepetencyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (name "AngularRepetencyValue") (declared-name "AngularRepetencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (name "AngularVelocityUnit") (declared-name "AngularVelocityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (name "AngularVelocityValue") (declared-name "AngularVelocityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberUnit"))) (name "AngularWavenumberUnit") (declared-name "AngularWavenumberUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberValue"))) (name "AngularWavenumberValue") (declared-name "AngularWavenumberValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (name "AreaUnit") (declared-name "AreaUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (name "AreaValue") (declared-name "AreaValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (name "AttenuationUnit") (declared-name "AttenuationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (name "AttenuationValue") (declared-name "AttenuationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (name "CartesianAcceleration3dCoordinateFrame") (declared-name "CartesianAcceleration3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (name "CartesianAcceleration3dVector") (declared-name "CartesianAcceleration3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (name "CartesianAngularAcceleration3dCoordinateFrame") (declared-name "CartesianAngularAcceleration3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (name "CartesianAngularAcceleration3dVector") (declared-name "CartesianAngularAcceleration3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (name "CartesianAngularVelocity3dCoordinateFrame") (declared-name "CartesianAngularVelocity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (name "CartesianAngularVelocity3dVector") (declared-name "CartesianAngularVelocity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (name "CartesianDisplacement3dVector") (declared-name "CartesianDisplacement3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (name "y") (declared-name "y") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (name "z") (declared-name "z") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (name "CartesianPosition3dVector") (declared-name "CartesianPosition3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (name "y") (declared-name "y") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (name "z") (declared-name "z") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (name "xUnit") (declared-name "xUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (name "yUnit") (declared-name "yUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (name "zUnit") (declared-name "zUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (name "CartesianVelocity3dCoordinateFrame") (declared-name "CartesianVelocity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (name "CartesianVelocity3dVector") (declared-name "CartesianVelocity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (name "CartesianWave3dVector") (declared-name "CartesianWave3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (name "CartesianWaveVector3dCoordinateFrame") (declared-name "CartesianWaveVector3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (name "CurvatureUnit") (declared-name "CurvatureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (name "CurvatureValue") (declared-name "CurvatureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (name "CylindricalDisplacement3dVector") (declared-name "CylindricalDisplacement3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (name "azimuth") (declared-name "azimuth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (name "radialDistance") (declared-name "radialDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (name "CylindricalPosition3dVector") (declared-name "CylindricalPosition3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (name "azimuth") (declared-name "azimuth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (name "height") (declared-name "height") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (name "radialDistance") (declared-name "radialDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (name "CylindricalSpatial3dCoordinateFrame") (declared-name "CylindricalSpatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (name "azimuthUnit") (declared-name "azimuthUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (name "zUnit") (declared-name "zUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (name "DampingCoefficientUnit") (declared-name "DampingCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (name "DampingCoefficientValue") (declared-name "DampingCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (name "Displacement3dVector") (declared-name "Displacement3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionUnit"))) (name "ExtinctionUnit") (declared-name "ExtinctionUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionValue"))) (name "ExtinctionValue") (declared-name "ExtinctionValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (name "FrequencyUnit") (declared-name "FrequencyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (name "FrequencyValue") (declared-name "FrequencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (name "LogarithmicDecrementValue") (declared-name "LogarithmicDecrementValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (name "PhaseCoefficientUnit") (declared-name "PhaseCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (name "PhaseCoefficientValue") (declared-name "PhaseCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedUnit"))) (name "PhaseSpeedUnit") (declared-name "PhaseSpeedUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedValue"))) (name "PhaseSpeedValue") (declared-name "PhaseSpeedValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (name "PhaseVelocityUnit") (declared-name "PhaseVelocityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (name "PhaseVelocityValue") (declared-name "PhaseVelocityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleUnit"))) (name "PlaneAngleUnit") (declared-name "PlaneAngleUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleValue"))) (name "PlaneAngleValue") (declared-name "PlaneAngleValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (name "PlanetaryPosition3dVector") (declared-name "PlanetaryPosition3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (name "altitude") (declared-name "altitude") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (name "latitude") (declared-name "latitude") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (name "longitude") (declared-name "longitude") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (name "PlanetarySpatial3dCoordinateFrame") (declared-name "PlanetarySpatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (name "altitudeUnit") (declared-name "altitudeUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (name "latitudeUnit") (declared-name "latitudeUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (name "longitudeUnit") (declared-name "longitudeUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (name "Position3dVector") (declared-name "Position3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (name "PropagationCoefficientUnit") (declared-name "PropagationCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (name "PropagationCoefficientValue") (declared-name "PropagationCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (name "RepetencyUnit") (declared-name "RepetencyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (name "RepetencyValue") (declared-name "RepetencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (name "SolidAngularMeasureUnit") (declared-name "SolidAngularMeasureUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (name "SolidAngularMeasureValue") (declared-name "SolidAngularMeasureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (name "Spatial3dCoordinateFrame") (declared-name "Spatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (name "SpeedUnit") (declared-name "SpeedUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (name "SpeedValue") (declared-name "SpeedValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (name "SphericalDisplacement3dVector") (declared-name "SphericalDisplacement3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (name "azimuth") (declared-name "azimuth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (name "inclination") (declared-name "inclination") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (name "radialDistance") (declared-name "radialDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (name "SphericalPosition3dVector") (declared-name "SphericalPosition3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (name "azimuth") (declared-name "azimuth") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (name "inclination") (declared-name "inclination") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (name "radialDistance") (declared-name "radialDistance") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (name "SphericalSpatial3dCoordinateFrame") (declared-name "SphericalSpatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (name "azimuthUnit") (declared-name "azimuthUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (name "inclinationUnit") (declared-name "inclinationUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeUnit"))) (name "TimeUnit") (declared-name "TimeUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeValue"))) (name "TimeValue") (declared-name "TimeValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (name "VolumeUnit") (declared-name "VolumeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (name "VolumeValue") (declared-name "VolumeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberUnit"))) (name "WavenumberUnit") (declared-name "WavenumberUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberValue"))) (name "WavenumberValue") (declared-name "WavenumberValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (name "acceleration") (declared-name "acceleration") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::altitude"))) (name "altitude") (declared-name "altitude"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (name "angularAcceleration") (declared-name "angularAcceleration") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularDisplacement"))) (name "angularDisplacement") (declared-name "angularDisplacement"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (name "angularFrequency") (declared-name "angularFrequency") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (name "angularMeasure") (declared-name "angularMeasure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (name "angularRepetency") (declared-name "angularRepetency") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (name "angularVelocity") (declared-name "angularVelocity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::angularWavenumber"))) (name "angularWavenumber") (declared-name "angularWavenumber"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::arcLength"))) (name "arcLength") (declared-name "arcLength"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (name "area") (declared-name "area") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (name "attenuation") (declared-name "attenuation") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::breadth"))) (name "breadth") (declared-name "breadth"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (name "cartesianAcceleration3dVector") (declared-name "cartesianAcceleration3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (name "cartesianAngularAcceleration3dVector") (declared-name "cartesianAngularAcceleration3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (name "cartesianAngularVelocity3dVector") (declared-name "cartesianAngularVelocity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (name "cartesianDisplacement3dVector") (declared-name "cartesianDisplacement3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (name "cartesianPosition3dVector") (declared-name "cartesianPosition3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (name "cartesianVelocity3dVector") (declared-name "cartesianVelocity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (name "cartesianWave3dVector") (declared-name "cartesianWave3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (name "curvature") (declared-name "curvature") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (name "cylindricalDisplacement3dVector") (declared-name "cylindricalDisplacement3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (name "cylindricalPosition3dVector") (declared-name "cylindricalPosition3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (name "dampingCoefficient") (declared-name "dampingCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::depth"))) (name "depth") (declared-name "depth"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::diameter")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (name "displacement3dVector") (declared-name "displacement3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (name "distance") (declared-name "distance") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::distance::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::distance")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::extinction"))) (name "extinction") (declared-name "extinction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (name "frequency") (declared-name "frequency") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::groupSpeed"))) (name "groupSpeed") (declared-name "groupSpeed"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (name "groupVelocity") (declared-name "groupVelocity") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (name "height") (declared-name "height") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::height::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::height")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (name "logarithmicDecrement") (declared-name "logarithmicDecrement") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (name "pathLength") (declared-name "pathLength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::pathLength")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::period"))) (name "period") (declared-name "period"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (name "periodDuration") (declared-name "periodDuration") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (name "phaseAngle") (declared-name "phaseAngle") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (name "phaseCoefficient") (declared-name "phaseCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseSpeed"))) (name "phaseSpeed") (declared-name "phaseSpeed"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (name "phaseVelocity") (declared-name "phaseVelocity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::planeAngle"))) (name "planeAngle") (declared-name "planeAngle"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (name "planetaryPosition3dVector") (declared-name "planetaryPosition3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (name "position3dVector") (declared-name "position3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (name "propagationCoefficient") (declared-name "propagationCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (name "radialDistance") (declared-name "radialDistance") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::radius::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::radius")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (name "radiusOfCurvature") (declared-name "radiusOfCurvature") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (name "repetency") (declared-name "repetency") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (name "rotation") (declared-name "rotation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::rotation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (name "rotationalDisplacement") (declared-name "rotationalDisplacement") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (name "rotationalFrequency") (declared-name "rotationalFrequency") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (name "solidAngularMeasure") (declared-name "solidAngularMeasure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (name "speed") (declared-name "speed") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (name "sphericalDisplacement3dVector") (declared-name "sphericalDisplacement3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (name "sphericalPosition3dVector") (declared-name "sphericalPosition3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (name "thickness") (declared-name "thickness") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::thickness")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::time"))) (name "time") (declared-name "time"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (name "timeConstant") (declared-name "timeConstant") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (name "transformation") (declared-name "transformation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (name "volume") (declared-name "volume") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (name "wavelength") (declared-name "wavelength") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::wavelength")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQSpaceTime::wavenumber"))) (name "wavenumber") (declared-name "wavenumber"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (name "width") (declared-name "width") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQSpaceTime::width::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQSpaceTime::width")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::diameter::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::distance::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::height::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::pathLength::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::radius::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::rotation::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::thickness::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::wavelength::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::width::_documentation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (to (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/isq_space_time.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 19) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 4) (end 23 593))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 4) (end 41 746))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 4) (end 61 447))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 4) (end 77 475))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 4) (end 93 450))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 4) (end 109 724))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 4) (end 127 658))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 4) (end 143 782))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 4) (end 160 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 165 8) (end 165 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 8) (end 180 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 181 8) (end 181 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 182 8) (end 182 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 183 8) (end 183 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 8) (end 183 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 184 8) (end 184 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 199 8) (end 199 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 8) (end 228 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 230 8) (end 230 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 231 8) (end 231 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 232 8) (end 232 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 256 8) (end 256 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 259 8) (end 259 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 260 8) (end 260 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 8) (end 297 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 298 8) (end 298 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 299 8) (end 299 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 303 4) (end 303 722))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 316 8) (end 316 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 317 8) (end 317 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 323 8) (end 323 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 324 8) (end 324 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 325 8) (end 325 61))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 326 8) (end 326 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 331 8) (end 331 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 8) (end 333 70))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 334 8) (end 334 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 339 8) (end 339 78))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 342 8) (end 342 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 349 8) (end 349 72))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 350 8) (end 350 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 355 4) (end 355 756))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 368 8) (end 368 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 369 8) (end 369 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 375 8) (end 375 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 376 8) (end 376 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 377 8) (end 377 61))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 378 8) (end 378 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 383 8) (end 383 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 385 8) (end 385 70))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 386 8) (end 386 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 391 8) (end 391 78))
      )
      (diagnostic
        (severity error)
        (code "redefinition_type_incompatible")
        (source "semantic")
        (range (start 394 8) (end 394 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 399 4) (end 399 622))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 415 4) (end 415 597))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 428 8) (end 428 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 428 8) (end 428 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 429 8) (end 429 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 434 4) (end 434 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 435 8) (end 435 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 436 8) (end 436 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 4) (end 440 762))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 453 8) (end 453 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 453 8) (end 453 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 454 8) (end 454 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 459 4) (end 459 233))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 460 8) (end 460 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 461 8) (end 461 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 465 4) (end 465 708))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 478 8) (end 478 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 478 8) (end 478 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 479 8) (end 479 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 484 4) (end 484 235))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 485 8) (end 485 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 486 8) (end 486 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 490 4) (end 490 914))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 503 8) (end 503 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 503 8) (end 503 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 504 8) (end 504 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 509 4) (end 509 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 551 4) (end 551 972))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 564 8) (end 564 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 564 8) (end 564 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 565 8) (end 565 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 570 4) (end 570 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 581 4) (end 581 912))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 594 8) (end 594 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 595 8) (end 595 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 600 4) (end 600 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 601 8) (end 601 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 602 8) (end 602 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 603 8) (end 603 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 607 4) (end 607 508))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 620 8) (end 620 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 620 8) (end 620 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 621 8) (end 621 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 626 4) (end 626 354))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 627 8) (end 627 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 628 8) (end 628 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 629 8) (end 629 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 633 4) (end 633 796))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 646 8) (end 646 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 646 8) (end 646 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 647 8) (end 647 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 652 4) (end 652 361))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 653 8) (end 653 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 654 8) (end 654 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 655 8) (end 655 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 658 4) (end 658 842))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 671 8) (end 671 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 672 8) (end 672 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 677 4) (end 677 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 678 8) (end 678 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 679 8) (end 679 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 680 8) (end 680 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 684 4) (end 684 1001))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 697 8) (end 697 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 697 8) (end 697 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 698 8) (end 698 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 703 4) (end 703 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 704 8) (end 704 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 705 8) (end 705 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 708 4) (end 708 1047))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 721 8) (end 721 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 722 8) (end 722 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 727 4) (end 727 225))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 728 8) (end 728 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 729 8) (end 729 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 730 8) (end 730 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 734 4) (end 734 766))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 747 8) (end 747 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 747 8) (end 747 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 748 8) (end 748 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 753 4) (end 753 253))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 754 8) (end 754 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 755 8) (end 755 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 758 4) (end 758 812))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 771 8) (end 771 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 772 8) (end 772 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 777 4) (end 777 233))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 778 8) (end 778 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 779 8) (end 779 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 780 8) (end 780 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 784 4) (end 784 550))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 802 4) (end 802 757))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 818 4) (end 818 629))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 834 4) (end 834 590))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 847 8) (end 847 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 847 8) (end 847 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 848 8) (end 848 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 853 4) (end 853 243))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 854 8) (end 854 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 855 8) (end 855 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 875 4) (end 875 632))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 888 8) (end 888 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 888 8) (end 888 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 889 8) (end 889 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 894 4) (end 894 250))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 895 8) (end 895 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 896 8) (end 896 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 900 4) (end 900 457))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 916 4) (end 916 597))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 929 8) (end 929 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 929 8) (end 929 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 930 8) (end 930 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 935 4) (end 935 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 936 8) (end 936 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 937 8) (end 937 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 945 4) (end 945 639))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 958 8) (end 958 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 959 8) (end 959 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 964 4) (end 964 214))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 965 8) (end 965 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 966 8) (end 966 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 967 8) (end 967 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 971 4) (end 971 654))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 984 8) (end 984 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 984 8) (end 984 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 985 8) (end 985 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 990 4) (end 990 246))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 991 8) (end 991 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 992 8) (end 992 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1000 4) (end 1000 948))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1013 8) (end 1013 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1013 8) (end 1013 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1014 8) (end 1014 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1019 4) (end 1019 362))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1020 8) (end 1020 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1021 8) (end 1021 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1022 8) (end 1022 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1048 4) (end 1048 573))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1061 8) (end 1061 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1061 8) (end 1061 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1062 8) (end 1062 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1067 4) (end 1067 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1068 8) (end 1068 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1069 8) (end 1069 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1073 4) (end 1073 515))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1090 4) (end 1090 773))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1103 8) (end 1103 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1103 8) (end 1103 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1104 8) (end 1104 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1109 4) (end 1109 241))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1110 8) (end 1110 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1111 8) (end 1111 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1119 4) (end 1119 756))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1132 8) (end 1132 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1132 8) (end 1132 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1133 8) (end 1133 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1138 4) (end 1138 246))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1139 8) (end 1139 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1140 8) (end 1140 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1144 4) (end 1144 782))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1157 8) (end 1157 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1157 8) (end 1157 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1158 8) (end 1158 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1163 4) (end 1163 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1164 8) (end 1164 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1165 8) (end 1165 80))
      )
    )
  )
)
~~~
