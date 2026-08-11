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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_space_time.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 8) (end 183 44))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 256 8) (end 256 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 8) (end 297 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 303 4) (end 303 722))
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 339 8) (end 339 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 349 8) (end 349 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 355 4) (end 355 756))
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 391 8) (end 391 78))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 4) (end 440 762))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 465 4) (end 465 708))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 490 4) (end 490 914))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 600 4) (end 600 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 607 4) (end 607 508))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 633 4) (end 633 796))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 658 4) (end 658 842))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 677 4) (end 677 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 684 4) (end 684 1001))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 708 4) (end 708 1047))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 727 4) (end 727 225))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 734 4) (end 734 766))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 758 4) (end 758 812))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 777 4) (end 777 233))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 875 4) (end 875 632))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 945 4) (end 945 639))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 964 4) (end 964 214))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 971 4) (end 971 654))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1000 4) (end 1000 948))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1048 4) (end 1048 573))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1119 4) (end 1119 756))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1144 4) (end 1144 782))
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
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bbb530df111ffa6b58066b31dd0213800713e0a1b29f7284b561c6d9456f3ddb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime"))) (kind "package") (name "ISQSpaceTime") (declared-name "ISQSpaceTime") (range (start (line 0) (character 0)) (end (line 0) (character 54699))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (kind "attribute def") (name "AccelerationUnit") (declared-name "AccelerationUnit") (range (start (line 652) (character 4)) (end (line 652) (character 361))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 654) (character 8)) (end (line 654) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 653) (character 8)) (end (line 653) (character 102))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 655) (character 8)) (end (line 655) (character 94))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 655) (character 22)) (end (line 655) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (kind "attribute def") (name "AccelerationValue") (declared-name "AccelerationValue") (range (start (line 633) (character 4)) (end (line 633) (character 796))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 633) (character 4)) (end (line 633) (character 796))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 647) (character 8)) (end (line 647) (character 48))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 647) (character 22)) (end (line 647) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 646) (character 8)) (end (line 646) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 646) (character 22)) (end (line 646) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (kind "attribute def") (name "AngularAccelerationUnit") (declared-name "AngularAccelerationUnit") (range (start (line 753) (character 4)) (end (line 753) (character 253))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 754) (character 8)) (end (line 754) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 755) (character 8)) (end (line 755) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 755) (character 22)) (end (line 755) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (kind "attribute def") (name "AngularAccelerationValue") (declared-name "AngularAccelerationValue") (range (start (line 734) (character 4)) (end (line 734) (character 766))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 734) (character 4)) (end (line 734) (character 766))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 748) (character 8)) (end (line 748) (character 55))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularAccelerationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 748) (character 22)) (end (line 748) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 747) (character 8)) (end (line 747) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 747) (character 22)) (end (line 747) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (kind "attribute def") (name "AngularFrequencyUnit") (declared-name "AngularFrequencyUnit") (range (start (line 894) (character 4)) (end (line 894) (character 250))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 895) (character 8)) (end (line 895) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 896) (character 8)) (end (line 896) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 896) (character 22)) (end (line 896) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (kind "attribute def") (name "AngularFrequencyValue") (declared-name "AngularFrequencyValue") (range (start (line 875) (character 4)) (end (line 875) (character 632))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 875) (character 4)) (end (line 875) (character 632))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 889) (character 8)) (end (line 889) (character 52))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularFrequencyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 889) (character 22)) (end (line 889) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 888) (character 8)) (end (line 888) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 888) (character 22)) (end (line 888) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (kind "attribute def") (name "AngularMeasureUnit") (declared-name "AngularMeasureUnit") (range (start (line 509) (character 4)) (end (line 509) (character 64))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (kind "attribute def") (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (range (start (line 490) (character 4)) (end (line 490) (character 914))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 490) (character 4)) (end (line 490) (character 914))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 504) (character 8)) (end (line 504) (character 50))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 504) (character 22)) (end (line 504) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 503) (character 8)) (end (line 503) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 503) (character 22)) (end (line 503) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (kind "attribute def") (name "AngularRepetencyUnit") (declared-name "AngularRepetencyUnit") (range (start (line 990) (character 4)) (end (line 990) (character 246))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 991) (character 8)) (end (line 991) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 992) (character 8)) (end (line 992) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 992) (character 22)) (end (line 992) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (kind "attribute def") (name "AngularRepetencyValue") (declared-name "AngularRepetencyValue") (range (start (line 971) (character 4)) (end (line 971) (character 654))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 971) (character 4)) (end (line 971) (character 654))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 985) (character 8)) (end (line 985) (character 52))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularRepetencyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 985) (character 22)) (end (line 985) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 984) (character 8)) (end (line 984) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 984) (character 22)) (end (line 984) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (kind "attribute def") (name "AngularVelocityUnit") (declared-name "AngularVelocityUnit") (range (start (line 703) (character 4)) (end (line 703) (character 249))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 704) (character 8)) (end (line 704) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 705) (character 8)) (end (line 705) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 705) (character 22)) (end (line 705) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (kind "attribute def") (name "AngularVelocityValue") (declared-name "AngularVelocityValue") (range (start (line 684) (character 4)) (end (line 684) (character 1001))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 684) (character 4)) (end (line 684) (character 1001))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 698) (character 8)) (end (line 698) (character 51))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularVelocityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 698) (character 22)) (end (line 698) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 697) (character 8)) (end (line 697) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 697) (character 22)) (end (line 697) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberUnit"))) (kind "alias") (name "AngularWavenumberUnit") (declared-name "AngularWavenumberUnit") (range (start (line 995) (character 4)) (end (line 995) (character 57))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberValue"))) (kind "alias") (name "AngularWavenumberValue") (declared-name "AngularWavenumberValue") (range (start (line 996) (character 4)) (end (line 996) (character 59))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (kind "attribute def") (name "AreaUnit") (declared-name "AreaUnit") (range (start (line 459) (character 4)) (end (line 459) (character 233))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 460) (character 8)) (end (line 460) (character 102))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 461) (character 8)) (end (line 461) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 461) (character 22)) (end (line 461) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (kind "attribute def") (name "AreaValue") (declared-name "AreaValue") (range (start (line 440) (character 4)) (end (line 440) (character 762))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::_documentation"))) (kind "documentation") (name "") (range (start (line 440) (character 4)) (end (line 440) (character 762))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 454) (character 8)) (end (line 454) (character 40))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AreaUnit") (range none)) (redefinition (reference "mRef") (range (start (line 454) (character 22)) (end (line 454) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 453) (character 8)) (end (line 453) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 453) (character 22)) (end (line 453) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (kind "attribute def") (name "AttenuationUnit") (declared-name "AttenuationUnit") (range (start (line 1109) (character 4)) (end (line 1109) (character 241))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1110) (character 8)) (end (line 1110) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1111) (character 8)) (end (line 1111) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1111) (character 22)) (end (line 1111) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (kind "attribute def") (name "AttenuationValue") (declared-name "AttenuationValue") (range (start (line 1090) (character 4)) (end (line 1090) (character 773))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1090) (character 4)) (end (line 1090) (character 773))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1104) (character 8)) (end (line 1104) (character 47))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AttenuationUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1104) (character 22)) (end (line 1104) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1103) (character 8)) (end (line 1103) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1103) (character 22)) (end (line 1103) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAcceleration3dCoordinateFrame") (declared-name "CartesianAcceleration3dCoordinateFrame") (range (start (line 677) (character 4)) (end (line 677) (character 219))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 678) (character 8)) (end (line 678) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 678) (character 22)) (end (line 678) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 679) (character 8)) (end (line 679) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 679) (character 22)) (end (line 679) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 680) (character 8)) (end (line 680) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 680) (character 22)) (end (line 680) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (kind "attribute def") (name "CartesianAcceleration3dVector") (declared-name "CartesianAcceleration3dVector") (range (start (line 658) (character 4)) (end (line 658) (character 842))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 658) (character 4)) (end (line 658) (character 842))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 671) (character 8)) (end (line 671) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 671) (character 22)) (end (line 671) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 672) (character 8)) (end (line 672) (character 70))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAcceleration3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 672) (character 22)) (end (line 672) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularAcceleration3dCoordinateFrame") (declared-name "CartesianAngularAcceleration3dCoordinateFrame") (range (start (line 777) (character 4)) (end (line 777) (character 233))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 778) (character 8)) (end (line 778) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 778) (character 22)) (end (line 778) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 779) (character 8)) (end (line 779) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 779) (character 22)) (end (line 779) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 780) (character 8)) (end (line 780) (character 56))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularAccelerationUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 780) (character 22)) (end (line 780) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (kind "attribute def") (name "CartesianAngularAcceleration3dVector") (declared-name "CartesianAngularAcceleration3dVector") (range (start (line 758) (character 4)) (end (line 758) (character 812))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 758) (character 4)) (end (line 758) (character 812))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 771) (character 8)) (end (line 771) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 771) (character 22)) (end (line 771) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 772) (character 8)) (end (line 772) (character 77))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularAcceleration3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 772) (character 22)) (end (line 772) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularVelocity3dCoordinateFrame") (declared-name "CartesianAngularVelocity3dCoordinateFrame") (range (start (line 727) (character 4)) (end (line 727) (character 225))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 728) (character 8)) (end (line 728) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 728) (character 22)) (end (line 728) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 729) (character 8)) (end (line 729) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 729) (character 22)) (end (line 729) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 730) (character 8)) (end (line 730) (character 52))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularVelocityUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 730) (character 22)) (end (line 730) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (kind "attribute def") (name "CartesianAngularVelocity3dVector") (declared-name "CartesianAngularVelocity3dVector") (range (start (line 708) (character 4)) (end (line 708) (character 1047))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 708) (character 4)) (end (line 708) (character 1047))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 721) (character 8)) (end (line 721) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 721) (character 22)) (end (line 721) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 722) (character 8)) (end (line 722) (character 73))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularVelocity3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 722) (character 22)) (end (line 722) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (kind "attribute def") (name "CartesianDisplacement3dVector") (declared-name "CartesianDisplacement3dVector") (range (start (line 374) (character 4)) (end (line 374) (character 332))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 378) (character 8)) (end (line 378) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 378) (character 22)) (end (line 378) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 375) (character 8)) (end (line 375) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (kind "attribute") (name "y") (declared-name "y") (range (start (line 376) (character 8)) (end (line 376) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (kind "attribute") (name "z") (declared-name "z") (range (start (line 377) (character 8)) (end (line 377) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (kind "attribute def") (name "CartesianPosition3dVector") (declared-name "CartesianPosition3dVector") (range (start (line 322) (character 4)) (end (line 322) (character 324))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 326) (character 8)) (end (line 326) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 326) (character 22)) (end (line 326) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 323) (character 8)) (end (line 323) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (kind "attribute") (name "y") (declared-name "y") (range (start (line 324) (character 8)) (end (line 324) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (kind "attribute") (name "z") (declared-name "z") (range (start (line 325) (character 8)) (end (line 325) (character 61))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (kind "attribute def") (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame") (range (start (line 168) (character 4)) (end (line 168) (character 901))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 168) (character 4)) (end (line 168) (character 901))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 184) (character 8)) (end (line 184) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 184) (character 22)) (end (line 184) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 183) (character 8)) (end (line 183) (character 44))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 183) (character 22)) (end (line 183) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (kind "attribute") (name "xUnit") (declared-name "xUnit") (range (start (line 180) (character 8)) (end (line 180) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (kind "attribute") (name "yUnit") (declared-name "yUnit") (range (start (line 181) (character 8)) (end (line 181) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (kind "attribute") (name "zUnit") (declared-name "zUnit") (range (start (line 182) (character 8)) (end (line 182) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (kind "attribute def") (name "CartesianVelocity3dCoordinateFrame") (declared-name "CartesianVelocity3dCoordinateFrame") (range (start (line 600) (character 4)) (end (line 600) (character 208))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 601) (character 8)) (end (line 601) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 601) (character 22)) (end (line 601) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 602) (character 8)) (end (line 602) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 602) (character 22)) (end (line 602) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 603) (character 8)) (end (line 603) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 603) (character 22)) (end (line 603) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (kind "attribute def") (name "CartesianVelocity3dVector") (declared-name "CartesianVelocity3dVector") (range (start (line 581) (character 4)) (end (line 581) (character 912))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 581) (character 4)) (end (line 581) (character 912))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 594) (character 8)) (end (line 594) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 594) (character 22)) (end (line 594) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 595) (character 8)) (end (line 595) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianVelocity3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 595) (character 22)) (end (line 595) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (kind "attribute def") (name "CartesianWave3dVector") (declared-name "CartesianWave3dVector") (range (start (line 945) (character 4)) (end (line 945) (character 639))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 945) (character 4)) (end (line 945) (character 639))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 958) (character 8)) (end (line 958) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 958) (character 22)) (end (line 958) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 959) (character 8)) (end (line 959) (character 68))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianWaveVector3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 959) (character 22)) (end (line 959) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (kind "attribute def") (name "CartesianWaveVector3dCoordinateFrame") (declared-name "CartesianWaveVector3dCoordinateFrame") (range (start (line 964) (character 4)) (end (line 964) (character 214))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 965) (character 8)) (end (line 965) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 965) (character 22)) (end (line 965) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 966) (character 8)) (end (line 966) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 966) (character 22)) (end (line 966) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 967) (character 8)) (end (line 967) (character 46))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "RepetencyUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 967) (character 22)) (end (line 967) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (kind "attribute def") (name "CurvatureUnit") (declared-name "CurvatureUnit") (range (start (line 434) (character 4)) (end (line 434) (character 239))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 435) (character 8)) (end (line 435) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 436) (character 8)) (end (line 436) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 436) (character 22)) (end (line 436) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (kind "attribute def") (name "CurvatureValue") (declared-name "CurvatureValue") (range (start (line 415) (character 4)) (end (line 415) (character 597))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 415) (character 4)) (end (line 415) (character 597))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 429) (character 8)) (end (line 429) (character 45))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CurvatureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 429) (character 22)) (end (line 429) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 428) (character 8)) (end (line 428) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 428) (character 22)) (end (line 428) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (kind "attribute def") (name "CylindricalDisplacement3dVector") (declared-name "CylindricalDisplacement3dVector") (range (start (line 382) (character 4)) (end (line 382) (character 385))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (range (start (line 384) (character 8)) (end (line 384) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 385) (character 8)) (end (line 385) (character 70))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 386) (character 8)) (end (line 386) (character 68))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CylindricalSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 386) (character 22)) (end (line 386) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (range (start (line 383) (character 8)) (end (line 383) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (kind "attribute def") (name "CylindricalPosition3dVector") (declared-name "CylindricalPosition3dVector") (range (start (line 330) (character 4)) (end (line 330) (character 377))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (range (start (line 332) (character 8)) (end (line 332) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (kind "attribute") (name "height") (declared-name "height") (range (start (line 333) (character 8)) (end (line 333) (character 70))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 334) (character 8)) (end (line 334) (character 68))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CylindricalSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 334) (character 22)) (end (line 334) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (range (start (line 331) (character 8)) (end (line 331) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (kind "attribute def") (name "CylindricalSpatial3dCoordinateFrame") (declared-name "CylindricalSpatial3dCoordinateFrame") (range (start (line 207) (character 4)) (end (line 207) (character 1824))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 207) (character 4)) (end (line 207) (character 1824))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (kind "attribute") (name "azimuthUnit") (declared-name "azimuthUnit") (range (start (line 229) (character 8)) (end (line 229) (character 51))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 232) (character 8)) (end (line 232) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 232) (character 22)) (end (line 232) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 231) (character 8)) (end (line 231) (character 71))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs") (range (start (line 231) (character 22)) (end (line 231) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind "attribute") (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (range (start (line 228) (character 8)) (end (line 228) (character 50))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (kind "attribute") (name "zUnit") (declared-name "zUnit") (range (start (line 230) (character 8)) (end (line 230) (character 37))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (kind "attribute def") (name "DampingCoefficientUnit") (declared-name "DampingCoefficientUnit") (range (start (line 1067) (character 4)) (end (line 1067) (character 252))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1068) (character 8)) (end (line 1068) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1069) (character 8)) (end (line 1069) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1069) (character 22)) (end (line 1069) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (kind "attribute def") (name "DampingCoefficientValue") (declared-name "DampingCoefficientValue") (range (start (line 1048) (character 4)) (end (line 1048) (character 573))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1048) (character 4)) (end (line 1048) (character 573))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1062) (character 8)) (end (line 1062) (character 54))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DampingCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1062) (character 22)) (end (line 1062) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1061) (character 8)) (end (line 1061) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1061) (character 22)) (end (line 1061) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (kind "attribute def") (name "Displacement3dVector") (declared-name "Displacement3dVector") (range (start (line 355) (character 4)) (end (line 355) (character 756))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 355) (character 4)) (end (line 355) (character 756))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 368) (character 8)) (end (line 368) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 368) (character 22)) (end (line 368) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 369) (character 8)) (end (line 369) (character 56))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 369) (character 22)) (end (line 369) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionUnit"))) (kind "alias") (name "ExtinctionUnit") (declared-name "ExtinctionUnit") (range (start (line 1114) (character 4)) (end (line 1114) (character 45))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionValue"))) (kind "alias") (name "ExtinctionValue") (declared-name "ExtinctionValue") (range (start (line 1115) (character 4)) (end (line 1115) (character 47))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (kind "attribute def") (name "FrequencyUnit") (declared-name "FrequencyUnit") (range (start (line 853) (character 4)) (end (line 853) (character 243))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 854) (character 8)) (end (line 854) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 855) (character 8)) (end (line 855) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 855) (character 22)) (end (line 855) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (kind "attribute def") (name "FrequencyValue") (declared-name "FrequencyValue") (range (start (line 834) (character 4)) (end (line 834) (character 590))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 834) (character 4)) (end (line 834) (character 590))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 848) (character 8)) (end (line 848) (character 45))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "FrequencyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 848) (character 22)) (end (line 848) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 847) (character 8)) (end (line 847) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 847) (character 22)) (end (line 847) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (kind "attribute def") (name "LogarithmicDecrementValue") (declared-name "LogarithmicDecrementValue") (range (start (line 1073) (character 4)) (end (line 1073) (character 515))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1073) (character 4)) (end (line 1073) (character 515))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (kind "attribute def") (name "PhaseCoefficientUnit") (declared-name "PhaseCoefficientUnit") (range (start (line 1138) (character 4)) (end (line 1138) (character 246))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1139) (character 8)) (end (line 1139) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1140) (character 8)) (end (line 1140) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1140) (character 22)) (end (line 1140) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (kind "attribute def") (name "PhaseCoefficientValue") (declared-name "PhaseCoefficientValue") (range (start (line 1119) (character 4)) (end (line 1119) (character 756))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1119) (character 4)) (end (line 1119) (character 756))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1133) (character 8)) (end (line 1133) (character 52))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhaseCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1133) (character 22)) (end (line 1133) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1132) (character 8)) (end (line 1132) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1132) (character 22)) (end (line 1132) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedUnit"))) (kind "alias") (name "PhaseSpeedUnit") (declared-name "PhaseSpeedUnit") (range (start (line 1025) (character 4)) (end (line 1025) (character 47))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedValue"))) (kind "alias") (name "PhaseSpeedValue") (declared-name "PhaseSpeedValue") (range (start (line 1026) (character 4)) (end (line 1026) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (kind "attribute def") (name "PhaseVelocityUnit") (declared-name "PhaseVelocityUnit") (range (start (line 1019) (character 4)) (end (line 1019) (character 362))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1021) (character 8)) (end (line 1021) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1020) (character 8)) (end (line 1020) (character 102))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1022) (character 8)) (end (line 1022) (character 94))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1022) (character 22)) (end (line 1022) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (kind "attribute def") (name "PhaseVelocityValue") (declared-name "PhaseVelocityValue") (range (start (line 1000) (character 4)) (end (line 1000) (character 948))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1000) (character 4)) (end (line 1000) (character 948))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1014) (character 8)) (end (line 1014) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhaseVelocityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1014) (character 22)) (end (line 1014) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1013) (character 8)) (end (line 1013) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1013) (character 22)) (end (line 1013) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleUnit"))) (kind "alias") (name "PlaneAngleUnit") (declared-name "PlaneAngleUnit") (range (start (line 512) (character 4)) (end (line 512) (character 48))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleValue"))) (kind "alias") (name "PlaneAngleValue") (declared-name "PlaneAngleValue") (range (start (line 513) (character 4)) (end (line 513) (character 50))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (kind "attribute def") (name "PlanetaryPosition3dVector") (declared-name "PlanetaryPosition3dVector") (range (start (line 346) (character 4)) (end (line 346) (character 377))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (kind "attribute") (name "altitude") (declared-name "altitude") (range (start (line 349) (character 8)) (end (line 349) (character 72))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (kind "attribute") (name "latitude") (declared-name "latitude") (range (start (line 347) (character 8)) (end (line 347) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (kind "attribute") (name "longitude") (declared-name "longitude") (range (start (line 348) (character 8)) (end (line 348) (character 83))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 350) (character 8)) (end (line 350) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "PlanetarySpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 350) (character 22)) (end (line 350) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (kind "attribute def") (name "PlanetarySpatial3dCoordinateFrame") (declared-name "PlanetarySpatial3dCoordinateFrame") (range (start (line 263) (character 5)) (end (line 263) (character 3253))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 263) (character 5)) (end (line 263) (character 3253))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (kind "attribute") (name "altitudeUnit") (declared-name "altitudeUnit") (range (start (line 297) (character 8)) (end (line 297) (character 44))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 299) (character 8)) (end (line 299) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 299) (character 22)) (end (line 299) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (kind "attribute") (name "latitudeUnit") (declared-name "latitudeUnit") (range (start (line 295) (character 8)) (end (line 295) (character 52))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (kind "attribute") (name "longitudeUnit") (declared-name "longitudeUnit") (range (start (line 296) (character 8)) (end (line 296) (character 53))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 298) (character 8)) (end (line 298) (character 74))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs") (range (start (line 298) (character 22)) (end (line 298) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (kind "attribute def") (name "Position3dVector") (declared-name "Position3dVector") (range (start (line 303) (character 4)) (end (line 303) (character 722))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 303) (character 4)) (end (line 303) (character 722))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 316) (character 8)) (end (line 316) (character 37))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 316) (character 22)) (end (line 316) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 317) (character 8)) (end (line 317) (character 56))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 317) (character 22)) (end (line 317) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (kind "attribute def") (name "PropagationCoefficientUnit") (declared-name "PropagationCoefficientUnit") (range (start (line 1163) (character 4)) (end (line 1163) (character 252))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1164) (character 8)) (end (line 1164) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1165) (character 8)) (end (line 1165) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1165) (character 22)) (end (line 1165) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (kind "attribute def") (name "PropagationCoefficientValue") (declared-name "PropagationCoefficientValue") (range (start (line 1144) (character 4)) (end (line 1144) (character 782))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1144) (character 4)) (end (line 1144) (character 782))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1158) (character 8)) (end (line 1158) (character 58))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropagationCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1158) (character 22)) (end (line 1158) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1157) (character 8)) (end (line 1157) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1157) (character 22)) (end (line 1157) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (kind "attribute def") (name "RepetencyUnit") (declared-name "RepetencyUnit") (range (start (line 935) (character 4)) (end (line 935) (character 239))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 936) (character 8)) (end (line 936) (character 103))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 937) (character 8)) (end (line 937) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 937) (character 22)) (end (line 937) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (kind "attribute def") (name "RepetencyValue") (declared-name "RepetencyValue") (range (start (line 916) (character 4)) (end (line 916) (character 597))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 916) (character 4)) (end (line 916) (character 597))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 930) (character 8)) (end (line 930) (character 45))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RepetencyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 930) (character 22)) (end (line 930) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 929) (character 8)) (end (line 929) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 929) (character 22)) (end (line 929) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (kind "attribute def") (name "SolidAngularMeasureUnit") (declared-name "SolidAngularMeasureUnit") (range (start (line 570) (character 4)) (end (line 570) (character 69))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (kind "attribute def") (name "SolidAngularMeasureValue") (declared-name "SolidAngularMeasureValue") (range (start (line 551) (character 4)) (end (line 551) (character 972))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 551) (character 4)) (end (line 551) (character 972))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 565) (character 8)) (end (line 565) (character 55))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SolidAngularMeasureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 565) (character 22)) (end (line 565) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 564) (character 8)) (end (line 564) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 564) (character 22)) (end (line 564) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (kind "attribute def") (name "Spatial3dCoordinateFrame") (declared-name "Spatial3dCoordinateFrame") (range (start (line 160) (character 4)) (end (line 160) (character 198))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 160) (character 4)) (end (line 160) (character 198))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 165) (character 8)) (end (line 165) (character 37))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 165) (character 22)) (end (line 165) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (kind "attribute def") (name "SpeedUnit") (declared-name "SpeedUnit") (range (start (line 626) (character 4)) (end (line 626) (character 354))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 628) (character 8)) (end (line 628) (character 105))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 627) (character 8)) (end (line 627) (character 102))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 629) (character 8)) (end (line 629) (character 94))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 629) (character 22)) (end (line 629) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (kind "attribute def") (name "SpeedValue") (declared-name "SpeedValue") (range (start (line 607) (character 4)) (end (line 607) (character 508))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::_documentation"))) (kind "documentation") (name "") (range (start (line 607) (character 4)) (end (line 607) (character 508))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 621) (character 8)) (end (line 621) (character 41))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedUnit") (range none)) (redefinition (reference "mRef") (range (start (line 621) (character 22)) (end (line 621) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 620) (character 8)) (end (line 620) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 620) (character 22)) (end (line 620) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (kind "attribute def") (name "SphericalDisplacement3dVector") (declared-name "SphericalDisplacement3dVector") (range (start (line 390) (character 4)) (end (line 390) (character 393))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (range (start (line 393) (character 8)) (end (line 393) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (kind "attribute") (name "inclination") (declared-name "inclination") (range (start (line 392) (character 8)) (end (line 392) (character 85))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 394) (character 8)) (end (line 394) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "SphericalSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 394) (character 22)) (end (line 394) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (range (start (line 391) (character 8)) (end (line 391) (character 78))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (kind "attribute def") (name "SphericalPosition3dVector") (declared-name "SphericalPosition3dVector") (range (start (line 338) (character 4)) (end (line 338) (character 385))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (range (start (line 341) (character 8)) (end (line 341) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (kind "attribute") (name "inclination") (declared-name "inclination") (range (start (line 340) (character 8)) (end (line 340) (character 85))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 342) (character 8)) (end (line 342) (character 66))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "SphericalSpatial3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 342) (character 22)) (end (line 342) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (range (start (line 339) (character 8)) (end (line 339) (character 78))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (kind "attribute def") (name "SphericalSpatial3dCoordinateFrame") (declared-name "SphericalSpatial3dCoordinateFrame") (range (start (line 235) (character 4)) (end (line 235) (character 1950))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 235) (character 4)) (end (line 235) (character 1950))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (kind "attribute") (name "azimuthUnit") (declared-name "azimuthUnit") (range (start (line 258) (character 8)) (end (line 258) (character 51))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (kind "attribute") (name "inclinationUnit") (declared-name "inclinationUnit") (range (start (line 257) (character 8)) (end (line 257) (character 55))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 260) (character 8)) (end (line 260) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 260) (character 22)) (end (line 260) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 259) (character 8)) (end (line 259) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs") (range (start (line 259) (character 22)) (end (line 259) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind "attribute") (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (range (start (line 256) (character 8)) (end (line 256) (character 50))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeUnit"))) (kind "alias") (name "TimeUnit") (declared-name "TimeUnit") (range (start (line 576) (character 4)) (end (line 576) (character 36))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeValue"))) (kind "alias") (name "TimeValue") (declared-name "TimeValue") (range (start (line 577) (character 4)) (end (line 577) (character 38))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (kind "attribute def") (name "VolumeUnit") (declared-name "VolumeUnit") (range (start (line 484) (character 4)) (end (line 484) (character 235))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 485) (character 8)) (end (line 485) (character 102))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 486) (character 8)) (end (line 486) (character 80))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 486) (character 22)) (end (line 486) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (kind "attribute def") (name "VolumeValue") (declared-name "VolumeValue") (range (start (line 465) (character 4)) (end (line 465) (character 708))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 465) (character 4)) (end (line 465) (character 708))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 479) (character 8)) (end (line 479) (character 42))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 479) (character 22)) (end (line 479) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 478) (character 8)) (end (line 478) (character 32))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 478) (character 22)) (end (line 478) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberUnit"))) (kind "alias") (name "WavenumberUnit") (declared-name "WavenumberUnit") (range (start (line 940) (character 4)) (end (line 940) (character 43))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberValue"))) (kind "alias") (name "WavenumberValue") (declared-name "WavenumberValue") (range (start (line 941) (character 4)) (end (line 941) (character 45))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 54699))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (kind "attribute def") (name "acceleration") (declared-name "acceleration") (range (start (line 650) (character 4)) (end (line 650) (character 79))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::altitude"))) (kind "alias") (name "altitude") (declared-name "altitude") (range (start (line 58) (character 4)) (end (line 58) (character 30))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (kind "attribute def") (name "angularAcceleration") (declared-name "angularAcceleration") (range (start (line 751) (character 4)) (end (line 751) (character 93))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularDisplacement"))) (kind "alias") (name "angularDisplacement") (declared-name "angularDisplacement") (range (start (line 532) (character 4)) (end (line 532) (character 57))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (kind "attribute def") (name "angularFrequency") (declared-name "angularFrequency") (range (start (line 892) (character 4)) (end (line 892) (character 87))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularFrequencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (kind "attribute def") (name "angularMeasure") (declared-name "angularMeasure") (range (start (line 507) (character 4)) (end (line 507) (character 83))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (kind "attribute def") (name "angularRepetency") (declared-name "angularRepetency") (range (start (line 988) (character 4)) (end (line 988) (character 87))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularRepetencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (kind "attribute def") (name "angularVelocity") (declared-name "angularVelocity") (range (start (line 701) (character 4)) (end (line 701) (character 85))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularWavenumber"))) (kind "alias") (name "angularWavenumber") (declared-name "angularWavenumber") (range (start (line 997) (character 4)) (end (line 997) (character 49))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::arcLength"))) (kind "alias") (name "arcLength") (declared-name "arcLength") (range (start (line 124) (character 4)) (end (line 124) (character 35))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (kind "attribute def") (name "area") (declared-name "area") (range (start (line 457) (character 4)) (end (line 457) (character 63))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (kind "attribute def") (name "attenuation") (declared-name "attenuation") (range (start (line 1107) (character 4)) (end (line 1107) (character 77))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AttenuationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::breadth"))) (kind "alias") (name "breadth") (declared-name "breadth") (range (start (line 38) (character 4)) (end (line 38) (character 28))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (kind "attribute def") (name "cartesianAcceleration3dVector") (declared-name "cartesianAcceleration3dVector") (range (start (line 675) (character 4)) (end (line 675) (character 95))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAcceleration3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (kind "attribute def") (name "cartesianAngularAcceleration3dVector") (declared-name "cartesianAngularAcceleration3dVector") (range (start (line 775) (character 4)) (end (line 775) (character 109))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularAcceleration3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (kind "attribute def") (name "cartesianAngularVelocity3dVector") (declared-name "cartesianAngularVelocity3dVector") (range (start (line 725) (character 4)) (end (line 725) (character 101))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularVelocity3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (kind "attribute def") (name "cartesianDisplacement3dVector") (declared-name "cartesianDisplacement3dVector") (range (start (line 380) (character 4)) (end (line 380) (character 100))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianDisplacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (kind "attribute def") (name "cartesianPosition3dVector") (declared-name "cartesianPosition3dVector") (range (start (line 328) (character 4)) (end (line 328) (character 88))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (kind "attribute def") (name "cartesianVelocity3dVector") (declared-name "cartesianVelocity3dVector") (range (start (line 598) (character 4)) (end (line 598) (character 87))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianVelocity3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (kind "attribute def") (name "cartesianWave3dVector") (declared-name "cartesianWave3dVector") (range (start (line 962) (character 4)) (end (line 962) (character 79))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianWave3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (kind "attribute def") (name "curvature") (declared-name "curvature") (range (start (line 432) (character 4)) (end (line 432) (character 73))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CurvatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (kind "attribute def") (name "cylindricalDisplacement3dVector") (declared-name "cylindricalDisplacement3dVector") (range (start (line 388) (character 4)) (end (line 388) (character 104))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CylindricalDisplacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (kind "attribute def") (name "cylindricalPosition3dVector") (declared-name "cylindricalPosition3dVector") (range (start (line 336) (character 4)) (end (line 336) (character 92))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CylindricalPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (kind "attribute def") (name "dampingCoefficient") (declared-name "dampingCoefficient") (range (start (line 1065) (character 4)) (end (line 1065) (character 91))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DampingCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::depth"))) (kind "alias") (name "depth") (declared-name "depth") (range (start (line 56) (character 4)) (end (line 56) (character 27))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (kind "attribute def") (name "diameter") (declared-name "diameter") (range (start (line 77) (character 4)) (end (line 77) (character 475))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter::_documentation"))) (kind "documentation") (name "") (range (start (line 77) (character 4)) (end (line 77) (character 475))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (kind "attribute def") (name "displacement3dVector") (declared-name "displacement3dVector") (range (start (line 372) (character 4)) (end (line 372) (character 77))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (kind "attribute def") (name "distance") (declared-name "distance") (range (start (line 127) (character 4)) (end (line 127) (character 658))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::distance::_documentation"))) (kind "documentation") (name "") (range (start (line 127) (character 4)) (end (line 127) (character 658))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::distance"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::extinction"))) (kind "alias") (name "extinction") (declared-name "extinction") (range (start (line 1116) (character 4)) (end (line 1116) (character 37))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (kind "attribute def") (name "frequency") (declared-name "frequency") (range (start (line 851) (character 4)) (end (line 851) (character 73))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupSpeed"))) (kind "alias") (name "groupSpeed") (declared-name "groupSpeed") (range (start (line 1045) (character 4)) (end (line 1045) (character 39))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (kind "attribute def") (name "groupVelocity") (declared-name "groupVelocity") (range (start (line 1030) (character 4)) (end (line 1030) (character 654))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity::_documentation"))) (kind "documentation") (name "") (range (start (line 1030) (character 4)) (end (line 1030) (character 654))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (kind "attribute def") (name "height") (declared-name "height") (range (start (line 41) (character 4)) (end (line 41) (character 746))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::height::_documentation"))) (kind "documentation") (name "") (range (start (line 41) (character 4)) (end (line 41) (character 746))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::height"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (kind "attribute def") (name "logarithmicDecrement") (declared-name "logarithmicDecrement") (range (start (line 1087) (character 4)) (end (line 1087) (character 82))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicDecrementValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (kind "attribute def") (name "pathLength") (declared-name "pathLength") (range (start (line 109) (character 4)) (end (line 109) (character 724))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength::_documentation"))) (kind "documentation") (name "") (range (start (line 109) (character 4)) (end (line 109) (character 724))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::period"))) (kind "alias") (name "period") (declared-name "period") (range (start (line 799) (character 4)) (end (line 799) (character 36))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (kind "attribute def") (name "periodDuration") (declared-name "periodDuration") (range (start (line 784) (character 4)) (end (line 784) (character 550))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration::_documentation"))) (kind "documentation") (name "") (range (start (line 784) (character 4)) (end (line 784) (character 550))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (kind "attribute def") (name "phaseAngle") (declared-name "phaseAngle") (range (start (line 535) (character 4)) (end (line 535) (character 692))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle::_documentation"))) (kind "documentation") (name "") (range (start (line 535) (character 4)) (end (line 535) (character 692))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (kind "attribute def") (name "phaseCoefficient") (declared-name "phaseCoefficient") (range (start (line 1136) (character 4)) (end (line 1136) (character 87))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseSpeed"))) (kind "alias") (name "phaseSpeed") (declared-name "phaseSpeed") (range (start (line 1027) (character 4)) (end (line 1027) (character 39))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (kind "attribute def") (name "phaseVelocity") (declared-name "phaseVelocity") (range (start (line 1017) (character 4)) (end (line 1017) (character 81))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseVelocityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::planeAngle"))) (kind "alias") (name "planeAngle") (declared-name "planeAngle") (range (start (line 514) (character 4)) (end (line 514) (character 40))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (kind "attribute def") (name "planetaryPosition3dVector") (declared-name "planetaryPosition3dVector") (range (start (line 352) (character 4)) (end (line 352) (character 88))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PlanetaryPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (kind "attribute def") (name "position3dVector") (declared-name "position3dVector") (range (start (line 320) (character 4)) (end (line 320) (character 69))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (kind "attribute def") (name "propagationCoefficient") (declared-name "propagationCoefficient") (range (start (line 1161) (character 4)) (end (line 1161) (character 99))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PropagationCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (kind "attribute def") (name "radialDistance") (declared-name "radialDistance") (range (start (line 143) (character 4)) (end (line 143) (character 782))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance::_documentation"))) (kind "documentation") (name "") (range (start (line 143) (character 4)) (end (line 143) (character 782))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (kind "attribute def") (name "radius") (declared-name "radius") (range (start (line 93) (character 4)) (end (line 93) (character 450))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radius::_documentation"))) (kind "documentation") (name "") (range (start (line 93) (character 4)) (end (line 93) (character 450))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::radius"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (kind "attribute def") (name "radiusOfCurvature") (declared-name "radiusOfCurvature") (range (start (line 399) (character 4)) (end (line 399) (character 622))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature::_documentation"))) (kind "documentation") (name "") (range (start (line 399) (character 4)) (end (line 399) (character 622))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (kind "attribute def") (name "repetency") (declared-name "repetency") (range (start (line 933) (character 4)) (end (line 933) (character 73))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "RepetencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (kind "attribute def") (name "rotation") (declared-name "rotation") (range (start (line 818) (character 4)) (end (line 818) (character 629))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation::_documentation"))) (kind "documentation") (name "") (range (start (line 818) (character 4)) (end (line 818) (character 629))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (kind "attribute def") (name "rotationalDisplacement") (declared-name "rotationalDisplacement") (range (start (line 517) (character 4)) (end (line 517) (character 1013))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement::_documentation"))) (kind "documentation") (name "") (range (start (line 517) (character 4)) (end (line 517) (character 1013))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (kind "attribute def") (name "rotationalFrequency") (declared-name "rotationalFrequency") (range (start (line 859) (character 4)) (end (line 859) (character 621))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency::_documentation"))) (kind "documentation") (name "") (range (start (line 859) (character 4)) (end (line 859) (character 621))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (kind "attribute def") (name "solidAngularMeasure") (declared-name "solidAngularMeasure") (range (start (line 568) (character 4)) (end (line 568) (character 93))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SolidAngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (kind "attribute def") (name "speed") (declared-name "speed") (range (start (line 624) (character 4)) (end (line 624) (character 65))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (kind "attribute def") (name "sphericalDisplacement3dVector") (declared-name "sphericalDisplacement3dVector") (range (start (line 396) (character 4)) (end (line 396) (character 100))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SphericalDisplacement3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (kind "attribute def") (name "sphericalPosition3dVector") (declared-name "sphericalPosition3dVector") (range (start (line 344) (character 4)) (end (line 344) (character 88))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SphericalPosition3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (kind "attribute def") (name "thickness") (declared-name "thickness") (range (start (line 61) (character 4)) (end (line 61) (character 447))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness::_documentation"))) (kind "documentation") (name "") (range (start (line 61) (character 4)) (end (line 61) (character 447))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::time"))) (kind "alias") (name "time") (declared-name "time") (range (start (line 578) (character 4)) (end (line 578) (character 28))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (kind "attribute def") (name "timeConstant") (declared-name "timeConstant") (range (start (line 802) (character 4)) (end (line 802) (character 757))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant::_documentation"))) (kind "documentation") (name "") (range (start (line 802) (character 4)) (end (line 802) (character 757))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (kind "attribute def") (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame") (range (start (line 187) (character 4)) (end (line 187) (character 737))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (range (start (line 187) (character 4)) (end (line 187) (character 737))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 193) (character 8)) (end (line 193) (character 222))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs") (range (start (line 193) (character 22)) (end (line 193) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs::_documentation"))) (kind "documentation") (name "") (range (start (line 193) (character 8)) (end (line 193) (character 222))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (kind "attribute") (name "transformation") (declared-name "transformation") (range (start (line 199) (character 8)) (end (line 199) (character 219))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transformation") (range (start (line 199) (character 22)) (end (line 199) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation::_documentation"))) (kind "documentation") (name "") (range (start (line 199) (character 8)) (end (line 199) (character 219))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (kind "attribute def") (name "volume") (declared-name "volume") (range (start (line 482) (character 4)) (end (line 482) (character 67))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (kind "attribute def") (name "wavelength") (declared-name "wavelength") (range (start (line 900) (character 4)) (end (line 900) (character 457))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength::_documentation"))) (kind "documentation") (name "") (range (start (line 900) (character 4)) (end (line 900) (character 457))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavenumber"))) (kind "alias") (name "wavenumber") (declared-name "wavenumber") (range (start (line 942) (character 4)) (end (line 942) (character 35))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (kind "attribute def") (name "width") (declared-name "width") (range (start (line 23) (character 4)) (end (line 23) (character 593))) (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::width::_documentation"))) (kind "documentation") (name "") (range (start (line 23) (character 4)) (end (line 23) (character 593))) (parent (node (document "d0") (qualified-name "ISQSpaceTime::width"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 655) (character 22)) (end (line 655) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 647) (character 22)) (end (line 647) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 646) (character 22)) (end (line 646) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 755) (character 22)) (end (line 755) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 748) (character 22)) (end (line 748) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 747) (character 22)) (end (line 747) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 896) (character 22)) (end (line 896) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularFrequencyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 889) (character 22)) (end (line 889) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 888) (character 22)) (end (line 888) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 504) (character 22)) (end (line 504) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 503) (character 22)) (end (line 503) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 992) (character 22)) (end (line 992) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularRepetencyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 985) (character 22)) (end (line 985) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 984) (character 22)) (end (line 984) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 705) (character 22)) (end (line 705) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 698) (character 22)) (end (line 698) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 697) (character 22)) (end (line 697) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 461) (character 22)) (end (line 461) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 454) (character 22)) (end (line 454) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 453) (character 22)) (end (line 453) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1111) (character 22)) (end (line 1111) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AttenuationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1104) (character 22)) (end (line 1104) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1103) (character 22)) (end (line 1103) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 678) (character 22)) (end (line 678) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 679) (character 22)) (end (line 679) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 680) (character 22)) (end (line 680) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 671) (character 22)) (end (line 671) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAcceleration3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 672) (character 22)) (end (line 672) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 778) (character 22)) (end (line 778) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 779) (character 22)) (end (line 779) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 780) (character 22)) (end (line 780) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 771) (character 22)) (end (line 771) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularAcceleration3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 772) (character 22)) (end (line 772) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 728) (character 22)) (end (line 728) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 729) (character 22)) (end (line 729) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 730) (character 22)) (end (line 730) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 721) (character 22)) (end (line 721) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularVelocity3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 722) (character 22)) (end (line 722) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 378) (character 22)) (end (line 378) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 326) (character 22)) (end (line 326) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 184) (character 22)) (end (line 184) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 183) (character 22)) (end (line 183) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 601) (character 22)) (end (line 601) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 602) (character 22)) (end (line 602) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 603) (character 22)) (end (line 603) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 594) (character 22)) (end (line 594) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 595) (character 22)) (end (line 595) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 958) (character 22)) (end (line 958) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianWaveVector3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 959) (character 22)) (end (line 959) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 965) (character 22)) (end (line 965) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 966) (character 22)) (end (line 966) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 967) (character 22)) (end (line 967) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 436) (character 22)) (end (line 436) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 429) (character 22)) (end (line 429) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 428) (character 22)) (end (line 428) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 386) (character 22)) (end (line 386) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 334) (character 22)) (end (line 334) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 232) (character 22)) (end (line 232) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 231) (character 22)) (end (line 231) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1069) (character 22)) (end (line 1069) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DampingCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1062) (character 22)) (end (line 1062) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1061) (character 22)) (end (line 1061) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 368) (character 22)) (end (line 368) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 369) (character 22)) (end (line 369) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 855) (character 22)) (end (line 855) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 848) (character 22)) (end (line 848) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 847) (character 22)) (end (line 847) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1140) (character 22)) (end (line 1140) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1133) (character 22)) (end (line 1133) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1132) (character 22)) (end (line 1132) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1022) (character 22)) (end (line 1022) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseVelocityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1014) (character 22)) (end (line 1014) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1013) (character 22)) (end (line 1013) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanetarySpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 350) (character 22)) (end (line 350) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 299) (character 22)) (end (line 299) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 298) (character 22)) (end (line 298) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 316) (character 22)) (end (line 316) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 317) (character 22)) (end (line 317) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1165) (character 22)) (end (line 1165) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PropagationCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1158) (character 22)) (end (line 1158) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1157) (character 22)) (end (line 1157) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 937) (character 22)) (end (line 937) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 930) (character 22)) (end (line 930) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 929) (character 22)) (end (line 929) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 565) (character 22)) (end (line 565) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 564) (character 22)) (end (line 564) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 165) (character 22)) (end (line 165) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 629) (character 22)) (end (line 629) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 621) (character 22)) (end (line 621) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 620) (character 22)) (end (line 620) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 394) (character 22)) (end (line 394) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 342) (character 22)) (end (line 342) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 260) (character 22)) (end (line 260) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 259) (character 22)) (end (line 259) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 486) (character 22)) (end (line 486) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 479) (character 22)) (end (line 479) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 478) (character 22)) (end (line 478) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularFrequencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularRepetencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (kind featureTyping) (ordinal 0)) (authored-target "AttenuationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAcceleration3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularAcceleration3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularVelocity3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianDisplacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianPosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianWave3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalDisplacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalPosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "DampingCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicDecrementValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseVelocityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanetaryPosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "PropagationCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalDisplacement3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalPosition3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 193) (character 22)) (end (line 193) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (range (start (line 199) (character 22)) (end (line 199) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
