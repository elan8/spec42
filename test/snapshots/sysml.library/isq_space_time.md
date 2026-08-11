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
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime"))) (kind "package") (name "ISQSpaceTime") (declared-name "ISQSpaceTime"))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (kind "attribute def") (name "AccelerationUnit") (declared-name "AccelerationUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (kind "attribute def") (name "AccelerationValue") (declared-name "AccelerationValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (kind "attribute def") (name "AngularAccelerationUnit") (declared-name "AngularAccelerationUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (kind "attribute def") (name "AngularAccelerationValue") (declared-name "AngularAccelerationValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularAccelerationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (kind "attribute def") (name "AngularFrequencyUnit") (declared-name "AngularFrequencyUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (kind "attribute def") (name "AngularFrequencyValue") (declared-name "AngularFrequencyValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularFrequencyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (kind "attribute def") (name "AngularMeasureUnit") (declared-name "AngularMeasureUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (kind "attribute def") (name "AngularMeasureValue") (declared-name "AngularMeasureValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (kind "attribute def") (name "AngularRepetencyUnit") (declared-name "AngularRepetencyUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (kind "attribute def") (name "AngularRepetencyValue") (declared-name "AngularRepetencyValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularRepetencyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (kind "attribute def") (name "AngularVelocityUnit") (declared-name "AngularVelocityUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (kind "attribute def") (name "AngularVelocityValue") (declared-name "AngularVelocityValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularVelocityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberUnit"))) (kind "alias") (name "AngularWavenumberUnit") (declared-name "AngularWavenumberUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AngularWavenumberValue"))) (kind "alias") (name "AngularWavenumberValue") (declared-name "AngularWavenumberValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (kind "attribute def") (name "AreaUnit") (declared-name "AreaUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (kind "attribute def") (name "AreaValue") (declared-name "AreaValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AreaUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (kind "attribute def") (name "AttenuationUnit") (declared-name "AttenuationUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (kind "attribute def") (name "AttenuationValue") (declared-name "AttenuationValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AttenuationUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAcceleration3dCoordinateFrame") (declared-name "CartesianAcceleration3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (kind "attribute def") (name "CartesianAcceleration3dVector") (declared-name "CartesianAcceleration3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAcceleration3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularAcceleration3dCoordinateFrame") (declared-name "CartesianAngularAcceleration3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularAccelerationUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (kind "attribute def") (name "CartesianAngularAcceleration3dVector") (declared-name "CartesianAngularAcceleration3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularAcceleration3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularVelocity3dCoordinateFrame") (declared-name "CartesianAngularVelocity3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularVelocityUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (kind "attribute def") (name "CartesianAngularVelocity3dVector") (declared-name "CartesianAngularVelocity3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularVelocity3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (kind "attribute def") (name "CartesianDisplacement3dVector") (declared-name "CartesianDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (kind "attribute") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (kind "attribute") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (kind "attribute def") (name "CartesianPosition3dVector") (declared-name "CartesianPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (kind "attribute") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (kind "attribute") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (kind "attribute def") (name "CartesianSpatial3dCoordinateFrame") (declared-name "CartesianSpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (kind "attribute") (name "xUnit") (declared-name "xUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (kind "attribute") (name "yUnit") (declared-name "yUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (kind "attribute") (name "zUnit") (declared-name "zUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (kind "attribute def") (name "CartesianVelocity3dCoordinateFrame") (declared-name "CartesianVelocity3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (kind "attribute def") (name "CartesianVelocity3dVector") (declared-name "CartesianVelocity3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianVelocity3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (kind "attribute def") (name "CartesianWave3dVector") (declared-name "CartesianWave3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianWaveVector3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (kind "attribute def") (name "CartesianWaveVector3dCoordinateFrame") (declared-name "CartesianWaveVector3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "RepetencyUnit")) (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (kind "attribute def") (name "CurvatureUnit") (declared-name "CurvatureUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (kind "attribute def") (name "CurvatureValue") (declared-name "CurvatureValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CurvatureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (kind "attribute def") (name "CylindricalDisplacement3dVector") (declared-name "CylindricalDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CylindricalSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (kind "attribute def") (name "CylindricalPosition3dVector") (declared-name "CylindricalPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (kind "attribute") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CylindricalSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (kind "attribute def") (name "CylindricalSpatial3dCoordinateFrame") (declared-name "CylindricalSpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (kind "attribute") (name "azimuthUnit") (declared-name "azimuthUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind "attribute") (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (kind "attribute") (name "zUnit") (declared-name "zUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (kind "attribute def") (name "DampingCoefficientUnit") (declared-name "DampingCoefficientUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (kind "attribute def") (name "DampingCoefficientValue") (declared-name "DampingCoefficientValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DampingCoefficientUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (kind "attribute def") (name "Displacement3dVector") (declared-name "Displacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Spatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionUnit"))) (kind "alias") (name "ExtinctionUnit") (declared-name "ExtinctionUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::ExtinctionValue"))) (kind "alias") (name "ExtinctionValue") (declared-name "ExtinctionValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (kind "attribute def") (name "FrequencyUnit") (declared-name "FrequencyUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (kind "attribute def") (name "FrequencyValue") (declared-name "FrequencyValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "FrequencyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (kind "attribute def") (name "LogarithmicDecrementValue") (declared-name "LogarithmicDecrementValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (kind "attribute def") (name "PhaseCoefficientUnit") (declared-name "PhaseCoefficientUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (kind "attribute def") (name "PhaseCoefficientValue") (declared-name "PhaseCoefficientValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhaseCoefficientUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedUnit"))) (kind "alias") (name "PhaseSpeedUnit") (declared-name "PhaseSpeedUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseSpeedValue"))) (kind "alias") (name "PhaseSpeedValue") (declared-name "PhaseSpeedValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (kind "attribute def") (name "PhaseVelocityUnit") (declared-name "PhaseVelocityUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (kind "attribute def") (name "PhaseVelocityValue") (declared-name "PhaseVelocityValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PhaseVelocityUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleUnit"))) (kind "alias") (name "PlaneAngleUnit") (declared-name "PlaneAngleUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlaneAngleValue"))) (kind "alias") (name "PlaneAngleValue") (declared-name "PlaneAngleValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (kind "attribute def") (name "PlanetaryPosition3dVector") (declared-name "PlanetaryPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (kind "attribute") (name "altitude") (declared-name "altitude") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (kind "attribute") (name "latitude") (declared-name "latitude") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (kind "attribute") (name "longitude") (declared-name "longitude") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "PlanetarySpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (kind "attribute def") (name "PlanetarySpatial3dCoordinateFrame") (declared-name "PlanetarySpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (kind "attribute") (name "altitudeUnit") (declared-name "altitudeUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (kind "attribute") (name "latitudeUnit") (declared-name "latitudeUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (kind "attribute") (name "longitudeUnit") (declared-name "longitudeUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (kind "attribute def") (name "Position3dVector") (declared-name "Position3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "Spatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (kind "attribute def") (name "PropagationCoefficientUnit") (declared-name "PropagationCoefficientUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (kind "attribute def") (name "PropagationCoefficientValue") (declared-name "PropagationCoefficientValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PropagationCoefficientUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (kind "attribute def") (name "RepetencyUnit") (declared-name "RepetencyUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (kind "attribute def") (name "RepetencyValue") (declared-name "RepetencyValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RepetencyUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (kind "attribute def") (name "SolidAngularMeasureUnit") (declared-name "SolidAngularMeasureUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (kind "attribute def") (name "SolidAngularMeasureValue") (declared-name "SolidAngularMeasureValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SolidAngularMeasureUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (kind "attribute def") (name "Spatial3dCoordinateFrame") (declared-name "Spatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (kind "attribute def") (name "SpeedUnit") (declared-name "SpeedUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (kind "attribute def") (name "SpeedValue") (declared-name "SpeedValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (kind "attribute def") (name "SphericalDisplacement3dVector") (declared-name "SphericalDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (kind "attribute") (name "inclination") (declared-name "inclination") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "SphericalSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (kind "attribute def") (name "SphericalPosition3dVector") (declared-name "SphericalPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (kind "attribute") (name "azimuth") (declared-name "azimuth") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (kind "attribute") (name "inclination") (declared-name "inclination") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "SphericalSpatial3dCoordinateFrame")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (kind "attribute") (name "radialDistance") (declared-name "radialDistance") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (kind "attribute def") (name "SphericalSpatial3dCoordinateFrame") (declared-name "SphericalSpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Spatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (kind "attribute") (name "azimuthUnit") (declared-name "azimuthUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (kind "attribute") (name "inclinationUnit") (declared-name "inclinationUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind "attribute") (name "radialDistanceUnit") (declared-name "radialDistanceUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeUnit"))) (kind "alias") (name "TimeUnit") (declared-name "TimeUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::TimeValue"))) (kind "alias") (name "TimeValue") (declared-name "TimeValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (kind "attribute def") (name "VolumeUnit") (declared-name "VolumeUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (kind "attribute def") (name "VolumeValue") (declared-name "VolumeValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberUnit"))) (kind "alias") (name "WavenumberUnit") (declared-name "WavenumberUnit") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::WavenumberValue"))) (kind "alias") (name "WavenumberValue") (declared-name "WavenumberValue") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (kind "attribute def") (name "acceleration") (declared-name "acceleration") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::altitude"))) (kind "alias") (name "altitude") (declared-name "altitude") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (kind "attribute def") (name "angularAcceleration") (declared-name "angularAcceleration") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularDisplacement"))) (kind "alias") (name "angularDisplacement") (declared-name "angularDisplacement") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (kind "attribute def") (name "angularFrequency") (declared-name "angularFrequency") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularFrequencyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (kind "attribute def") (name "angularMeasure") (declared-name "angularMeasure") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (kind "attribute def") (name "angularRepetency") (declared-name "angularRepetency") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularRepetencyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (kind "attribute def") (name "angularVelocity") (declared-name "angularVelocity") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::angularWavenumber"))) (kind "alias") (name "angularWavenumber") (declared-name "angularWavenumber") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::arcLength"))) (kind "alias") (name "arcLength") (declared-name "arcLength") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (kind "attribute def") (name "area") (declared-name "area") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (kind "attribute def") (name "attenuation") (declared-name "attenuation") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AttenuationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::breadth"))) (kind "alias") (name "breadth") (declared-name "breadth") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (kind "attribute def") (name "cartesianAcceleration3dVector") (declared-name "cartesianAcceleration3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAcceleration3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (kind "attribute def") (name "cartesianAngularAcceleration3dVector") (declared-name "cartesianAngularAcceleration3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularAcceleration3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (kind "attribute def") (name "cartesianAngularVelocity3dVector") (declared-name "cartesianAngularVelocity3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularVelocity3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (kind "attribute def") (name "cartesianDisplacement3dVector") (declared-name "cartesianDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianDisplacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (kind "attribute def") (name "cartesianPosition3dVector") (declared-name "cartesianPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianPosition3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (kind "attribute def") (name "cartesianVelocity3dVector") (declared-name "cartesianVelocity3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianVelocity3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (kind "attribute def") (name "cartesianWave3dVector") (declared-name "cartesianWave3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianWave3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (kind "attribute def") (name "curvature") (declared-name "curvature") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CurvatureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (kind "attribute def") (name "cylindricalDisplacement3dVector") (declared-name "cylindricalDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CylindricalDisplacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (kind "attribute def") (name "cylindricalPosition3dVector") (declared-name "cylindricalPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CylindricalPosition3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (kind "attribute def") (name "dampingCoefficient") (declared-name "dampingCoefficient") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DampingCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::depth"))) (kind "alias") (name "depth") (declared-name "depth") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (kind "attribute def") (name "diameter") (declared-name "diameter") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::diameter::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (kind "attribute def") (name "displacement3dVector") (declared-name "displacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Displacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (kind "attribute def") (name "distance") (declared-name "distance") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::distance::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::distance"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::extinction"))) (kind "alias") (name "extinction") (declared-name "extinction") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (kind "attribute def") (name "frequency") (declared-name "frequency") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupSpeed"))) (kind "alias") (name "groupSpeed") (declared-name "groupSpeed") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (kind "attribute def") (name "groupVelocity") (declared-name "groupVelocity") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (kind "attribute def") (name "height") (declared-name "height") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::height::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::height"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (kind "attribute def") (name "logarithmicDecrement") (declared-name "logarithmicDecrement") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicDecrementValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (kind "attribute def") (name "pathLength") (declared-name "pathLength") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::pathLength::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::period"))) (kind "alias") (name "period") (declared-name "period") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (kind "attribute def") (name "periodDuration") (declared-name "periodDuration") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (kind "attribute def") (name "phaseAngle") (declared-name "phaseAngle") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (kind "attribute def") (name "phaseCoefficient") (declared-name "phaseCoefficient") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseSpeed"))) (kind "alias") (name "phaseSpeed") (declared-name "phaseSpeed") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (kind "attribute def") (name "phaseVelocity") (declared-name "phaseVelocity") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseVelocityValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::planeAngle"))) (kind "alias") (name "planeAngle") (declared-name "planeAngle") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (kind "attribute def") (name "planetaryPosition3dVector") (declared-name "planetaryPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PlanetaryPosition3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (kind "attribute def") (name "position3dVector") (declared-name "position3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "Position3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (kind "attribute def") (name "propagationCoefficient") (declared-name "propagationCoefficient") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "PropagationCoefficientValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (kind "attribute def") (name "radialDistance") (declared-name "radialDistance") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (kind "attribute def") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radius::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::radius"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (kind "attribute def") (name "radiusOfCurvature") (declared-name "radiusOfCurvature") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (kind "attribute def") (name "repetency") (declared-name "repetency") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "RepetencyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (kind "attribute def") (name "rotation") (declared-name "rotation") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (kind "attribute def") (name "rotationalDisplacement") (declared-name "rotationalDisplacement") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (kind "attribute def") (name "rotationalFrequency") (declared-name "rotationalFrequency") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (kind "attribute def") (name "solidAngularMeasure") (declared-name "solidAngularMeasure") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SolidAngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (kind "attribute def") (name "speed") (declared-name "speed") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (kind "attribute def") (name "sphericalDisplacement3dVector") (declared-name "sphericalDisplacement3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SphericalDisplacement3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (kind "attribute def") (name "sphericalPosition3dVector") (declared-name "sphericalPosition3dVector") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "SphericalPosition3dVector")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (kind "attribute def") (name "thickness") (declared-name "thickness") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::thickness::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::time"))) (kind "alias") (name "time") (declared-name "time") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (kind "attribute def") (name "timeConstant") (declared-name "timeConstant") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (kind "attribute def") (name "universalCartesianSpatial3dCoordinateFrame") (declared-name "universalCartesianSpatial3dCoordinateFrame") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianSpatial3dCoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (kind "attribute") (name "transformation") (declared-name "transformation") (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transformation")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (kind "attribute def") (name "volume") (declared-name "volume") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (kind "attribute def") (name "wavelength") (declared-name "wavelength") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavelength::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::wavenumber"))) (kind "alias") (name "wavenumber") (declared-name "wavenumber") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (kind "attribute def") (name "width") (declared-name "width") (parent (node (document "d0") (qualified-name "ISQSpaceTime"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "ISQSpaceTime::width::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ISQSpaceTime::width"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularFrequencyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularRepetencyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AttenuationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAcceleration3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularAcceleration3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularVelocity3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::y"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::z"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::xUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::yUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::zUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianWaveVector3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::zUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DampingCoefficientUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseVelocityUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::altitude"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::latitude"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::longitude"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanetarySpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::altitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::latitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::longitudeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PropagationCoefficientUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::inclination"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::azimuth"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::inclination"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "Spatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::azimuthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::inclinationUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::radialDistanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularAcceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularFrequencyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularMeasure"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularRepetency"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularRepetencyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::angularVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::area"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AreaValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::attenuation"))) (kind featureTyping) (ordinal 0)) (authored-target "AttenuationValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAcceleration3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularAcceleration3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularAcceleration3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianAngularVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularVelocity3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianDisplacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianPosition3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianVelocity3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianVelocity3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cartesianWave3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianWave3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::curvature"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalDisplacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::cylindricalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CylindricalPosition3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::dampingCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "DampingCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::displacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Displacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::distance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::frequency"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::groupVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::height"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::logarithmicDecrement"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicDecrementValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::LogarithmicDecrementValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::pathLength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::periodDuration"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::phaseVelocity"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseVelocityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::planetaryPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanetaryPosition3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::position3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "Position3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::Position3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::propagationCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "PropagationCoefficientValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radialDistance"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::radiusOfCurvature"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::repetency"))) (kind featureTyping) (ordinal 0)) (authored-target "RepetencyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotation"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalDisplacement"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::rotationalFrequency"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::solidAngularMeasure"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::speed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SpeedValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalDisplacement3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalDisplacement3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::sphericalPosition3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "SphericalPosition3dVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::thickness"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::timeConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianSpatial3dCoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::volume"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQSpaceTime::VolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::wavelength"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQSpaceTime::width"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 428 22) (end 428 25)) (probe (position 428 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 428 22) (end 428 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::num") (range (start 428 8) (end 428 32)))
        )
      )
    )
    (query (range (start 453 22) (end 453 25)) (probe (position 453 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 453 22) (end 453 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AreaValue::num") (range (start 453 8) (end 453 32)))
        )
      )
    )
    (query (range (start 478 22) (end 478 25)) (probe (position 478 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 478 22) (end 478 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::num") (range (start 478 8) (end 478 32)))
        )
      )
    )
    (query (range (start 503 22) (end 503 25)) (probe (position 503 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 503 22) (end 503 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::num") (range (start 503 8) (end 503 32)))
        )
      )
    )
    (query (range (start 564 22) (end 564 25)) (probe (position 564 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 564 22) (end 564 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::num") (range (start 564 8) (end 564 32)))
        )
      )
    )
    (query (range (start 620 22) (end 620 25)) (probe (position 620 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 620 22) (end 620 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::num") (range (start 620 8) (end 620 32)))
        )
      )
    )
    (query (range (start 646 22) (end 646 25)) (probe (position 646 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 646 22) (end 646 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::num") (range (start 646 8) (end 646 32)))
        )
      )
    )
    (query (range (start 697 22) (end 697 25)) (probe (position 697 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 697 22) (end 697 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::num") (range (start 697 8) (end 697 32)))
        )
      )
    )
    (query (range (start 747 22) (end 747 25)) (probe (position 747 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 747 22) (end 747 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::num") (range (start 747 8) (end 747 32)))
        )
      )
    )
    (query (range (start 847 22) (end 847 25)) (probe (position 847 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 847 22) (end 847 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::num") (range (start 847 8) (end 847 32)))
        )
      )
    )
    (query (range (start 888 22) (end 888 25)) (probe (position 888 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 888 22) (end 888 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::num") (range (start 888 8) (end 888 32)))
        )
      )
    )
    (query (range (start 929 22) (end 929 25)) (probe (position 929 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 929 22) (end 929 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::num") (range (start 929 8) (end 929 32)))
        )
      )
    )
    (query (range (start 984 22) (end 984 25)) (probe (position 984 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 984 22) (end 984 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::num") (range (start 984 8) (end 984 32)))
        )
      )
    )
    (query (range (start 1013 22) (end 1013 25)) (probe (position 1013 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1013 22) (end 1013 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::num") (range (start 1013 8) (end 1013 32)))
        )
      )
    )
    (query (range (start 1061 22) (end 1061 25)) (probe (position 1061 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1061 22) (end 1061 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::num") (range (start 1061 8) (end 1061 32)))
        )
      )
    )
    (query (range (start 1103 22) (end 1103 25)) (probe (position 1103 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1103 22) (end 1103 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::num") (range (start 1103 8) (end 1103 32)))
        )
      )
    )
    (query (range (start 1132 22) (end 1132 25)) (probe (position 1132 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1132 22) (end 1132 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::num") (range (start 1132 8) (end 1132 32)))
        )
      )
    )
    (query (range (start 1157 22) (end 1157 25)) (probe (position 1157 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 1157 22) (end 1157 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::num") (range (start 1157 8) (end 1157 32)))
        )
      )
    )
    (query (range (start 317 22) (end 317 26)) (probe (position 317 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 317 22) (end 317 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::mRef") (range (start 317 8) (end 317 56)))
        )
      )
    )
    (query (range (start 326 22) (end 326 26)) (probe (position 326 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 326 22) (end 326 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianPosition3dVector::mRef") (range (start 326 8) (end 326 66)))
        )
      )
    )
    (query (range (start 334 22) (end 334 26)) (probe (position 334 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 334 22) (end 334 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CylindricalPosition3dVector::mRef") (range (start 334 8) (end 334 68)))
        )
      )
    )
    (query (range (start 342 22) (end 342 26)) (probe (position 342 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 342 22) (end 342 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SphericalPosition3dVector::mRef") (range (start 342 8) (end 342 66)))
        )
      )
    )
    (query (range (start 350 22) (end 350 26)) (probe (position 350 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 350 22) (end 350 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PlanetaryPosition3dVector::mRef") (range (start 350 8) (end 350 66)))
        )
      )
    )
    (query (range (start 369 22) (end 369 26)) (probe (position 369 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 369 22) (end 369 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::mRef") (range (start 369 8) (end 369 56)))
        )
      )
    )
    (query (range (start 378 22) (end 378 26)) (probe (position 378 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 378 22) (end 378 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianDisplacement3dVector::mRef") (range (start 378 8) (end 378 66)))
        )
      )
    )
    (query (range (start 386 22) (end 386 26)) (probe (position 386 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 386 22) (end 386 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CylindricalDisplacement3dVector::mRef") (range (start 386 8) (end 386 68)))
        )
      )
    )
    (query (range (start 394 22) (end 394 26)) (probe (position 394 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 394 22) (end 394 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SphericalDisplacement3dVector::mRef") (range (start 394 8) (end 394 66)))
        )
      )
    )
    (query (range (start 429 22) (end 429 26)) (probe (position 429 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 429 22) (end 429 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CurvatureValue::mRef") (range (start 429 8) (end 429 45)))
        )
      )
    )
    (query (range (start 454 22) (end 454 26)) (probe (position 454 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 454 22) (end 454 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AreaValue::mRef") (range (start 454 8) (end 454 40)))
        )
      )
    )
    (query (range (start 479 22) (end 479 26)) (probe (position 479 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 479 22) (end 479 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::VolumeValue::mRef") (range (start 479 8) (end 479 42)))
        )
      )
    )
    (query (range (start 504 22) (end 504 26)) (probe (position 504 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 504 22) (end 504 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularMeasureValue::mRef") (range (start 504 8) (end 504 50)))
        )
      )
    )
    (query (range (start 565 22) (end 565 26)) (probe (position 565 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 565 22) (end 565 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SolidAngularMeasureValue::mRef") (range (start 565 8) (end 565 55)))
        )
      )
    )
    (query (range (start 595 22) (end 595 26)) (probe (position 595 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 595 22) (end 595 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::mRef") (range (start 595 8) (end 595 66)))
        )
      )
    )
    (query (range (start 621 22) (end 621 26)) (probe (position 621 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 621 22) (end 621 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SpeedValue::mRef") (range (start 621 8) (end 621 41)))
        )
      )
    )
    (query (range (start 647 22) (end 647 26)) (probe (position 647 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 647 22) (end 647 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AccelerationValue::mRef") (range (start 647 8) (end 647 48)))
        )
      )
    )
    (query (range (start 672 22) (end 672 26)) (probe (position 672 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 672 22) (end 672 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::mRef") (range (start 672 8) (end 672 70)))
        )
      )
    )
    (query (range (start 698 22) (end 698 26)) (probe (position 698 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 698 22) (end 698 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityValue::mRef") (range (start 698 8) (end 698 51)))
        )
      )
    )
    (query (range (start 722 22) (end 722 26)) (probe (position 722 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 722 22) (end 722 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::mRef") (range (start 722 8) (end 722 73)))
        )
      )
    )
    (query (range (start 748 22) (end 748 26)) (probe (position 748 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 748 22) (end 748 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationValue::mRef") (range (start 748 8) (end 748 55)))
        )
      )
    )
    (query (range (start 772 22) (end 772 26)) (probe (position 772 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 772 22) (end 772 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::mRef") (range (start 772 8) (end 772 77)))
        )
      )
    )
    (query (range (start 848 22) (end 848 26)) (probe (position 848 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 848 22) (end 848 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::FrequencyValue::mRef") (range (start 848 8) (end 848 45)))
        )
      )
    )
    (query (range (start 889 22) (end 889 26)) (probe (position 889 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 889 22) (end 889 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyValue::mRef") (range (start 889 8) (end 889 52)))
        )
      )
    )
    (query (range (start 930 22) (end 930 26)) (probe (position 930 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 930 22) (end 930 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::RepetencyValue::mRef") (range (start 930 8) (end 930 45)))
        )
      )
    )
    (query (range (start 959 22) (end 959 26)) (probe (position 959 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 959 22) (end 959 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::mRef") (range (start 959 8) (end 959 68)))
        )
      )
    )
    (query (range (start 985 22) (end 985 26)) (probe (position 985 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 985 22) (end 985 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyValue::mRef") (range (start 985 8) (end 985 52)))
        )
      )
    )
    (query (range (start 1014 22) (end 1014 26)) (probe (position 1014 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1014 22) (end 1014 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityValue::mRef") (range (start 1014 8) (end 1014 49)))
        )
      )
    )
    (query (range (start 1062 22) (end 1062 26)) (probe (position 1062 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1062 22) (end 1062 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientValue::mRef") (range (start 1062 8) (end 1062 54)))
        )
      )
    )
    (query (range (start 1104 22) (end 1104 26)) (probe (position 1104 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1104 22) (end 1104 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AttenuationValue::mRef") (range (start 1104 8) (end 1104 47)))
        )
      )
    )
    (query (range (start 1133 22) (end 1133 26)) (probe (position 1133 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1133 22) (end 1133 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientValue::mRef") (range (start 1133 8) (end 1133 52)))
        )
      )
    )
    (query (range (start 1158 22) (end 1158 26)) (probe (position 1158 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 1158 22) (end 1158 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientValue::mRef") (range (start 1158 8) (end 1158 58)))
        )
      )
    )
    (query (range (start 183 22) (end 183 27)) (probe (position 183 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 183 22) (end 183 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::mRefs") (range (start 183 8) (end 183 44)))
        )
      )
    )
    (query (range (start 193 22) (end 193 27)) (probe (position 193 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 193 22) (end 193 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::mRefs") (range (start 193 8) (end 193 222)))
        )
      )
    )
    (query (range (start 231 22) (end 231 27)) (probe (position 231 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 231 22) (end 231 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::mRefs") (range (start 231 8) (end 231 71)))
        )
      )
    )
    (query (range (start 259 22) (end 259 27)) (probe (position 259 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 259 22) (end 259 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::mRefs") (range (start 259 8) (end 259 81)))
        )
      )
    )
    (query (range (start 298 22) (end 298 27)) (probe (position 298 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 298 22) (end 298 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::mRefs") (range (start 298 8) (end 298 74)))
        )
      )
    )
    (query (range (start 603 22) (end 603 27)) (probe (position 603 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 603 22) (end 603 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::mRefs") (range (start 603 8) (end 603 42)))
        )
      )
    )
    (query (range (start 680 22) (end 680 27)) (probe (position 680 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 680 22) (end 680 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::mRefs") (range (start 680 8) (end 680 49)))
        )
      )
    )
    (query (range (start 730 22) (end 730 27)) (probe (position 730 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 730 22) (end 730 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::mRefs") (range (start 730 8) (end 730 52)))
        )
      )
    )
    (query (range (start 780 22) (end 780 27)) (probe (position 780 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 780 22) (end 780 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::mRefs") (range (start 780 8) (end 780 56)))
        )
      )
    )
    (query (range (start 967 22) (end 967 27)) (probe (position 967 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 967 22) (end 967 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::mRefs") (range (start 967 8) (end 967 46)))
        )
      )
    )
    (query (range (start 17 19) (end 17 26)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQBase::*")
        (range (start 17 19) (end 17 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 165 22) (end 165 29)) (probe (position 165 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 165 22) (end 165 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::Spatial3dCoordinateFrame::isBound") (range (start 165 8) (end 165 37)))
        )
      )
    )
    (query (range (start 316 22) (end 316 29)) (probe (position 316 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 316 22) (end 316 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::Position3dVector::isBound") (range (start 316 8) (end 316 37)))
        )
      )
    )
    (query (range (start 368 22) (end 368 29)) (probe (position 368 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 368 22) (end 368 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::Displacement3dVector::isBound") (range (start 368 8) (end 368 38)))
        )
      )
    )
    (query (range (start 594 22) (end 594 29)) (probe (position 594 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 594 22) (end 594 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dVector::isBound") (range (start 594 8) (end 594 38)))
        )
      )
    )
    (query (range (start 601 22) (end 601 29)) (probe (position 601 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 601 22) (end 601 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isBound") (range (start 601 8) (end 601 38)))
        )
      )
    )
    (query (range (start 671 22) (end 671 29)) (probe (position 671 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 671 22) (end 671 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dVector::isBound") (range (start 671 8) (end 671 38)))
        )
      )
    )
    (query (range (start 678 22) (end 678 29)) (probe (position 678 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 678 22) (end 678 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isBound") (range (start 678 8) (end 678 38)))
        )
      )
    )
    (query (range (start 721 22) (end 721 29)) (probe (position 721 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 721 22) (end 721 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dVector::isBound") (range (start 721 8) (end 721 38)))
        )
      )
    )
    (query (range (start 728 22) (end 728 29)) (probe (position 728 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 728 22) (end 728 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isBound") (range (start 728 8) (end 728 38)))
        )
      )
    )
    (query (range (start 771 22) (end 771 29)) (probe (position 771 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 771 22) (end 771 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dVector::isBound") (range (start 771 8) (end 771 38)))
        )
      )
    )
    (query (range (start 778 22) (end 778 29)) (probe (position 778 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 778 22) (end 778 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isBound") (range (start 778 8) (end 778 38)))
        )
      )
    )
    (query (range (start 958 22) (end 958 29)) (probe (position 958 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 958 22) (end 958 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianWave3dVector::isBound") (range (start 958 8) (end 958 38)))
        )
      )
    )
    (query (range (start 965 22) (end 965 29)) (probe (position 965 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 965 22) (end 965 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isBound") (range (start 965 8) (end 965 38)))
        )
      )
    )
    (query (range (start 15 19) (end 15 29)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 15 19) (end 15 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 184 22) (end 184 34)) (probe (position 184 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 184 22) (end 184 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianSpatial3dCoordinateFrame::isOrthogonal") (range (start 184 8) (end 184 42)))
        )
      )
    )
    (query (range (start 232 22) (end 232 34)) (probe (position 232 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 232 22) (end 232 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CylindricalSpatial3dCoordinateFrame::isOrthogonal") (range (start 232 8) (end 232 42)))
        )
      )
    )
    (query (range (start 260 22) (end 260 34)) (probe (position 260 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 260 22) (end 260 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SphericalSpatial3dCoordinateFrame::isOrthogonal") (range (start 260 8) (end 260 42)))
        )
      )
    )
    (query (range (start 299 22) (end 299 34)) (probe (position 299 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 299 22) (end 299 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PlanetarySpatial3dCoordinateFrame::isOrthogonal") (range (start 299 8) (end 299 42)))
        )
      )
    )
    (query (range (start 602 22) (end 602 34)) (probe (position 602 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 602 22) (end 602 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianVelocity3dCoordinateFrame::isOrthogonal") (range (start 602 8) (end 602 42)))
        )
      )
    )
    (query (range (start 679 22) (end 679 34)) (probe (position 679 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 679 22) (end 679 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAcceleration3dCoordinateFrame::isOrthogonal") (range (start 679 8) (end 679 42)))
        )
      )
    )
    (query (range (start 729 22) (end 729 34)) (probe (position 729 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 729 22) (end 729 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularVelocity3dCoordinateFrame::isOrthogonal") (range (start 729 8) (end 729 42)))
        )
      )
    )
    (query (range (start 779 22) (end 779 34)) (probe (position 779 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 779 22) (end 779 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianAngularAcceleration3dCoordinateFrame::isOrthogonal") (range (start 779 8) (end 779 42)))
        )
      )
    )
    (query (range (start 966 22) (end 966 34)) (probe (position 966 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 966 22) (end 966 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CartesianWaveVector3dCoordinateFrame::isOrthogonal") (range (start 966 8) (end 966 42)))
        )
      )
    )
    (query (range (start 199 22) (end 199 36)) (probe (position 199 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation"))
        (kind redefinition) (ordinal 0) (authored-target "transformation")
        (range (start 199 22) (end 199 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::universalCartesianSpatial3dCoordinateFrame::transformation") (range (start 199 8) (end 199 219)))
        )
      )
    )
    (query (range (start 436 22) (end 436 39)) (probe (position 436 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 436 22) (end 436 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::CurvatureUnit::quantityDimension") (range (start 436 8) (end 436 80)))
        )
      )
    )
    (query (range (start 461 22) (end 461 39)) (probe (position 461 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 461 22) (end 461 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AreaUnit::quantityDimension") (range (start 461 8) (end 461 80)))
        )
      )
    )
    (query (range (start 486 22) (end 486 39)) (probe (position 486 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 486 22) (end 486 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::VolumeUnit::quantityDimension") (range (start 486 8) (end 486 80)))
        )
      )
    )
    (query (range (start 629 22) (end 629 39)) (probe (position 629 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 629 22) (end 629 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::SpeedUnit::quantityDimension") (range (start 629 8) (end 629 94)))
        )
      )
    )
    (query (range (start 655 22) (end 655 39)) (probe (position 655 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 655 22) (end 655 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AccelerationUnit::quantityDimension") (range (start 655 8) (end 655 94)))
        )
      )
    )
    (query (range (start 705 22) (end 705 39)) (probe (position 705 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 705 22) (end 705 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularVelocityUnit::quantityDimension") (range (start 705 8) (end 705 82)))
        )
      )
    )
    (query (range (start 755 22) (end 755 39)) (probe (position 755 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 755 22) (end 755 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularAccelerationUnit::quantityDimension") (range (start 755 8) (end 755 82)))
        )
      )
    )
    (query (range (start 855 22) (end 855 39)) (probe (position 855 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 855 22) (end 855 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::FrequencyUnit::quantityDimension") (range (start 855 8) (end 855 82)))
        )
      )
    )
    (query (range (start 896 22) (end 896 39)) (probe (position 896 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 896 22) (end 896 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularFrequencyUnit::quantityDimension") (range (start 896 8) (end 896 82)))
        )
      )
    )
    (query (range (start 937 22) (end 937 39)) (probe (position 937 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 937 22) (end 937 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::RepetencyUnit::quantityDimension") (range (start 937 8) (end 937 80)))
        )
      )
    )
    (query (range (start 992 22) (end 992 39)) (probe (position 992 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 992 22) (end 992 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AngularRepetencyUnit::quantityDimension") (range (start 992 8) (end 992 80)))
        )
      )
    )
    (query (range (start 1022 22) (end 1022 39)) (probe (position 1022 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1022 22) (end 1022 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseVelocityUnit::quantityDimension") (range (start 1022 8) (end 1022 94)))
        )
      )
    )
    (query (range (start 1069 22) (end 1069 39)) (probe (position 1069 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1069 22) (end 1069 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::DampingCoefficientUnit::quantityDimension") (range (start 1069 8) (end 1069 82)))
        )
      )
    )
    (query (range (start 1111 22) (end 1111 39)) (probe (position 1111 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1111 22) (end 1111 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::AttenuationUnit::quantityDimension") (range (start 1111 8) (end 1111 80)))
        )
      )
    )
    (query (range (start 1140 22) (end 1140 39)) (probe (position 1140 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1140 22) (end 1140 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PhaseCoefficientUnit::quantityDimension") (range (start 1140 8) (end 1140 80)))
        )
      )
    )
    (query (range (start 1165 22) (end 1165 39)) (probe (position 1165 22))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 1165 22) (end 1165 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ISQSpaceTime::PropagationCoefficientUnit::quantityDimension") (range (start 1165 8) (end 1165 80)))
        )
      )
    )
    (query (range (start 14 19) (end 14 37)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 14 19) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 19) (end 16 40)) (probe (position 16 19))
      (reference
        (source (document "d0") (qualified-name "ISQSpaceTime::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 16 19) (end 16 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
