# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQMechanics
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQMechanics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-4:2019 "Mechanics"
     * see also https://www.iso.org/standard/64975.html
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

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-4 item 4-1 mass */
    /* See package ISQBase for the declarations of MassValue and MassUnit */

    /* ISO-80000-4 item 4-2 mass density, density */
    attribute def MassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-2 mass density, density
         * symbol(s): `ρ`, `ρ_m`
         * application domain: generic
         * name: MassDensity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quantity representing the spatial distribution of mass of a continuous material: `ρ(vec(r)) = (dm)/(dV)` where `m` is mass of the material contained in an infinitesimal domain at point `vec(r)` and `V` is volume of this domain
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassDensityUnit[1];
    }

    attribute massDensity: MassDensityValue[*] nonunique :> scalarQuantities;

    attribute def MassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias DensityUnit for MassDensityUnit;
    alias DensityValue for MassDensityValue;
    alias density for massDensity;

    /* ISO-80000-4 item 4-3 specific volume */
    attribute def SpecificVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-3 specific volume
         * symbol(s): `v`
         * application domain: generic
         * name: SpecificVolume
         * quantity dimension: L^3*M^-1
         * measurement unit(s): kg^-1*m^3
         * tensor order: 0
         * definition: reciprocal of mass density `ρ` (item 4-2): `v = 1/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificVolumeUnit[1];
    }

    attribute specificVolume: SpecificVolumeValue[*] nonunique :> scalarQuantities;

    attribute def SpecificVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-4 item 4-4 relative mass density, relative density */
    attribute def RelativeMassDensityValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-4 relative mass density, relative density
         * symbol(s): `d`
         * application domain: generic
         * name: RelativeMassDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass density of a substance `ρ` and mass density of a reference substance `ρ_0` : `d = ρ/ρ_0`
         * remarks: Conditions and material should be specified for the reference substance.
         */
    }
    attribute relativeMassDensity: RelativeMassDensityValue :> scalarQuantities;

    alias relativeDensity for relativeMassDensity;

    /* ISO-80000-4 item 4-5 surface mass density, surface density */
    attribute def SurfaceMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-5 surface mass density, surface density
         * symbol(s): `ρ_A`
         * application domain: generic
         * name: SurfaceMassDensity
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: quantity representing the areal distribution of mass of a continuous material: `ρ_A(vec(r)) = (dm)/(dA)` where `m` is the mass of the material at position `vec(r)` and `A` is area
         * remarks: The name "grammage" should not be used for this quantity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceMassDensityUnit[1];
    }

    attribute surfaceMassDensity: SurfaceMassDensityValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceMassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias SurfaceDensityUnit for SurfaceMassDensityUnit;
    alias SurfaceDensityValue for SurfaceMassDensityValue;
    alias surfaceDensity for surfaceMassDensity;

    /* ISO-80000-4 item 4-6 linear mass density, linear density */
    attribute def LinearMassDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-6 linear mass density, linear density
         * symbol(s): `ρ_I`
         * application domain: generic
         * name: LinearMassDensity
         * quantity dimension: L^-1*M^1
         * measurement unit(s): kg*m^-1
         * tensor order: 0
         * definition: quantity representing the linear distribution of mass of a continuous material: `ρ_I(vec(r)) = (dm)/(dI)` where `m` is the mass of the material at position `vec(r)` and `l` is length
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearMassDensityUnit[1];
    }

    attribute linearMassDensity: LinearMassDensityValue[*] nonunique :> scalarQuantities;

    attribute def LinearMassDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    alias LinearDensityUnit for LinearMassDensityUnit;
    alias LinearDensityValue for LinearMassDensityValue;
    alias linearDensity for linearMassDensity;

    /* ISO-80000-4 item 4-7 moment of inertia */
    attribute def MomentOfInertiaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentOfInertiaUnit[1];
    }

    attribute momentOfInertia: MomentOfInertiaValue[*] nonunique :> scalarQuantities;

    attribute def MomentOfInertiaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    attribute def Cartesian3dMomentOfInertiaTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-7 moment of inertia (tensor)
         * symbol(s): `vec(vec(J))`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dMomentOfInertiaMeasurementReference[1];
    }

    attribute momentOfInertiaTensor: Cartesian3dMomentOfInertiaTensor :> tensorQuantities;

    attribute def Cartesian3dMomentOfInertiaMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: MomentOfInertiaUnit[9];
    }

    /* ISO-80000-4 item 4-8 momentum */
    attribute def MomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-8 momentum (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 0
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentumUnit[1];
    }

    attribute momentum: MomentumValue[*] nonunique :> scalarQuantities;

    attribute def MomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-8 momentum (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 1
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMomentum3dCoordinateFrame[1];
    }

    attribute cartesianMomentum3dVector: CartesianMomentum3dVector :> vectorQuantities;

    attribute def CartesianMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MomentumUnit[3];
    }

    /* ISO-80000-4 item 4-9.1 force */
    attribute def ForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-9.1 force (magnitude)
         * symbol(s): `F`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ForceUnit[1];
    }

    attribute force: ForceValue[*] nonunique :> scalarQuantities;

    attribute def ForceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.1 force (vector)
         * symbol(s): `vec(F)`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianForce3dVector: CartesianForce3dVector :> vectorQuantities;

    attribute def CartesianForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ForceUnit[3];
    }

    /* ISO-80000-4 item 4-9.2 weight */
    attribute def CartesianWeight3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.2 weight
         * symbol(s): `vec(F_g)`
         * application domain: generic
         * name: Weight (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) acting on a body in the gravitational field of Earth: `vec(F_g) = m vec(g)` where `m` (item 4-1) is the mass of the body and `vec(g)` is the local acceleration of free fall (ISO 80000-3)
         * remarks: In colloquial language, the name "weight" continues to be used where "mass" is meant. This practice should be avoided. Weight is an example of a gravitational force. Weight comprises not only the local gravitational force but also the local centrifugal force due to the rotation of the Earth.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianWeight3dVector: CartesianWeight3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-9.3 static friction force, static friction */
    attribute def CartesianStaticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.3 static friction force, static friction
         * symbol(s): `vec(F_s)`
         * application domain: generic
         * name: StaticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion before a body starts to slide on a surface
         * remarks: For the static friction coefficient, see item 4-23.1.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianStaticFrictionForce3dVector: CartesianStaticFrictionForce3dVector :> vectorQuantities;

    alias cartesianStaticFriction3dVector for cartesianStaticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.4 kinetic friction force, dynamic friction force */
    attribute def CartesianKineticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.4 kinetic friction force, dynamic friction force
         * symbol(s): `vec(F_μ)`
         * application domain: generic
         * name: KineticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body slides on a surface
         * remarks: For the kinetic friction factor, see item 4-23.2.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianKineticFrictionForce3dVector: CartesianKineticFrictionForce3dVector :> vectorQuantities;

    alias cartesianDynamicFrictionForce3dVector for cartesianKineticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.5 rolling resistance, rolling drag, rolling friction force */
    attribute def CartesianRollingResistance3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.5 rolling resistance, rolling drag, rolling friction force
         * symbol(s): `vec(F_"rr")`
         * application domain: generic
         * name: RollingResistance (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body rolls on a surface
         * remarks: For the rolling resistance factor, see item 4-23.3.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianRollingResistance3dVector: CartesianRollingResistance3dVector :> vectorQuantities;

    alias cartesianRollingDrag3dVector for cartesianRollingResistance3dVector;

    alias cartesianRollingFrictionForce3dVector for cartesianRollingResistance3dVector;

    /* ISO-80000-4 item 4-9.6 drag force */
    attribute def CartesianDragForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-9.6 drag force
         * symbol(s): `vec(F_D)`
         * application domain: generic
         * name: DragForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion of a body in a fluid
         * remarks: For the drag coefficient, see item 4-23.4.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianForce3dCoordinateFrame[1];
    }

    attribute cartesianDragForce3dVector: CartesianDragForce3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-10 impulse */
    attribute def ImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-10 impulse (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ImpulseUnit[1];
    }

    attribute impulse: ImpulseValue[*] nonunique :> scalarQuantities;

    attribute def ImpulseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-10 impulse (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianImpulse3dCoordinateFrame[1];
    }

    attribute cartesianImpulse3dVector: CartesianImpulse3dVector :> vectorQuantities;

    attribute def CartesianImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ImpulseUnit[3];
    }

    /* ISO-80000-4 item 4-11 angular momentum */
    attribute def AngularMomentumValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-11 angular momentum (magnitude)
         * symbol(s): `L`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularMomentumUnit[1];
    }

    attribute angularMomentum: AngularMomentumValue[*] nonunique :> scalarQuantities;

    attribute def AngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-11 angular momentum (vector)
         * symbol(s): `vec(L)`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularMomentum3dCoordinateFrame[1];
    }

    attribute cartesianAngularMomentum3dVector: CartesianAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularMomentumUnit[3];
    }

    /* ISO-80000-4 item 4-12.1 moment of force */
    attribute def MomentOfForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.1 moment of force (magnitude)
         * symbol(s): `M`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MomentOfForceUnit[1];
    }

    attribute momentOfForce: MomentOfForceValue[*] nonunique :> scalarQuantities;

    attribute def MomentOfForceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMomentOfForce3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-12.1 moment of force (vector)
         * symbol(s): `vec(M)`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMomentOfForce3dCoordinateFrame[1];
    }

    attribute cartesianMomentOfForce3dVector: CartesianMomentOfForce3dVector :> vectorQuantities;

    attribute def CartesianMomentOfForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MomentOfForceUnit[3];
    }

    /* ISO-80000-4 item 4-12.2 torque */
    attribute def TorqueValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-12.2 torque
         * symbol(s): `T`, `M_Q`
         * application domain: generic
         * name: Torque
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: quantity described by the scalar product: `T = vec(M)*vec(e_Q)` where `vec(M)` is moment of force (item 4-12.1) and `vec(e_Q)` is unit vector of direction with respect to which the torque is considered
         * remarks: For example, torque is the twisting moment of force with respect to the longitudinal axis of a beam or shaft.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TorqueUnit[1];
    }

    attribute torque: TorqueValue[*] nonunique :> scalarQuantities;

    attribute def TorqueUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-13 angular impulse */
    attribute def AngularImpulseValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-13 angular impulse (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularImpulseUnit[1];
    }

    attribute angularImpulse: AngularImpulseValue[*] nonunique :> scalarQuantities;

    attribute def AngularImpulseUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianAngularImpulse3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-13 angular impulse (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularImpulse3dCoordinateFrame[1];
    }

    attribute cartesianAngularImpulse3dVector: CartesianAngularImpulse3dVector :> vectorQuantities;

    attribute def CartesianAngularImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularImpulseUnit[3];
    }

    /* ISO-80000-4 item 4-14.1 pressure */
    attribute def PressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-14.1 pressure
         * symbol(s): `p`
         * application domain: generic
         * name: Pressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of the component of a force normal to a surface and its area: `p = (vec(e_n) * vec(F)) / A` where `vec(e_n)` is unit vector of the surface normal, `vec(F)` is force (item 4-9.1) and `A` is area (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PressureUnit[1];
    }

    attribute pressure: PressureValue[*] nonunique :> scalarQuantities;

    attribute def PressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-14.2 gauge pressure */
    attribute gaugePressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 4-14.2 gauge pressure
         * symbol(s): `p_e`
         * application domain: generic
         * name: GaugePressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure `p` (item 4-14.1) decremented by ambient pressure `p_amb` : `p_e = p - p_amb`
         * remarks: Often, `p_amb` is chosen as a standard pressure. Gauge pressure is positive or negative.
         */
    }

    /* ISO-80000-4 item 4-15 stress */
    attribute def StressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-15 stress (magnitude)
         * symbol(s): `σ`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> num: Real;
        attribute :>> mRef: StressUnit[1];
    }

    attribute stress: StressValue[*] nonunique :> scalarQuantities;

    attribute def StressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def Cartesian3dStressTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-15 stress (tensor)
         * symbol(s): `vec(vec(σ))`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dStressMeasurementReference[1];
    }

    attribute stressTensor: Cartesian3dStressTensor :> tensorQuantities;

    attribute def Cartesian3dStressMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: StressUnit[9];
    }

    /* ISO-80000-4 item 4-16.1 normal stress */
    attribute def NormalStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.1 normal stress
         * symbol(s): `σ_n`, `σ`
         * application domain: generic
         * name: NormalStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `σ_n = (d F_n)/(dA)` where `F_n` is the normal component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter normal to it, and evenly distributed, cause a constant normal stress `σ_n = F A` in the slice (layer).
         */
        attribute :>> num: Real;
        attribute :>> mRef: NormalStressUnit[1];
    }

    attribute normalStress: NormalStressValue[*] nonunique :> scalarQuantities;

    attribute def NormalStressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-16.2 shear stress */
    attribute def ShearStressValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-16.2 shear stress
         * symbol(s): `τ_s`, `τ`
         * application domain: generic
         * name: ShearStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `τ_s = (d F_t)/(dA)` where `F_t` is the tangential component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter parallel to it, and evenly distributed, cause a constant shear stress `τ = F/A` in the slice (layer).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ShearStressUnit[1];
    }

    attribute shearStress: ShearStressValue[*] nonunique :> scalarQuantities;

    attribute def ShearStressUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-17.1 strain */
    attribute def StrainValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (magnitude)
         * symbol(s): `ε`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> num: Real;
        attribute :>> mRef: StrainUnit[1];
    }

    attribute strain: StrainValue[*] nonunique :> scalarQuantities;

    attribute def StrainUnit :> DimensionOneUnit {
    }

    attribute def Cartesian3dStrainTensor :> TensorQuantityValue {
        doc
        /*
         * source: item 4-17.1 strain (tensor)
         * symbol(s): `vec(vec(ε))`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num: Real[9];
        attribute :>> mRef: Cartesian3dStrainMeasurementReference[1];
    }

    attribute strainTensor: Cartesian3dStrainTensor :> tensorQuantities;

    attribute def Cartesian3dStrainMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs: StrainUnit[9];
    }

    /* ISO-80000-4 item 4-17.2 relative linear strain */
    attribute def RelativeLinearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.2 relative linear strain
         * symbol(s): `ε`, `(e)`
         * application domain: generic
         * name: RelativeLinearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in length `Δl` (ISO 80000-3) of an object and its length `l` (ISO 80000-3): `ε = (Δl)/l`
         * remarks: None.
         */
    }
    attribute relativeLinearStrain: RelativeLinearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.3 shear strain */
    attribute def ShearStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.3 shear strain
         * symbol(s): `γ`
         * application domain: generic
         * name: ShearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of parallel displacement `Δx` (ISO 80000-3) of two surfaces of a layer and the thickness `d` (ISO 80000-3) of the layer: `γ = (Δx)/d`
         * remarks: None.
         */
    }
    attribute shearStrain: ShearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.4 relative volume strain */
    attribute def RelativeVolumeStrainValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-17.4 relative volume strain
         * symbol(s): `θ`
         * application domain: generic
         * name: RelativeVolumeStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in volume `ΔV` (ISO 80000-3) of an object and its volume `V_0` (ISO 80000-3): `θ = (ΔV)/V_0`
         * remarks: None.
         */
    }
    attribute relativeVolumeStrain: RelativeVolumeStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-18 Poisson number */
    attribute def PoissonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-18 Poisson number
         * symbol(s): `μ`, `(v)`
         * application domain: generic
         * name: PoissonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in width `Δb` (width is defined in ISO 80000-3) and change in length `Δl` (length is defined in ISO 80000-3) of an object: `μ = (Δb)/(Δl)`
         * remarks: None.
         */
    }
    attribute poissonNumber: PoissonNumberValue :> scalarQuantities;

    /* ISO-80000-4 item 4-19.1 modulus of elasticity, Young modulus */
    attribute def ModulusOfElasticityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.1 modulus of elasticity, Young modulus
         * symbol(s): `E`, `E_m`, `Y`
         * application domain: generic
         * name: ModulusOfElasticity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of normal stress `σ` (item 4-16.1) and relative linear strain `ε` (item 4-17.2): `E = σ/ε`
         * remarks: Conditions should be specified (e.g. adiabatic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfElasticityUnit[1];
    }

    attribute modulusOfElasticity: ModulusOfElasticityValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfElasticityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias YoungModulusUnit for ModulusOfElasticityUnit;
    alias YoungModulusValue for ModulusOfElasticityValue;
    alias youngModulus for modulusOfElasticity;

    /* ISO-80000-4 item 4-19.2 modulus of rigidity, shear modulus */
    attribute def ModulusOfRigidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.2 modulus of rigidity, shear modulus
         * symbol(s): `G`
         * application domain: generic
         * name: ModulusOfRigidity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of shear stress `τ` (item 4-16.2) and shear strain `γ` (item 4-17.3): `G = τ/γ`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfRigidityUnit[1];
    }

    attribute modulusOfRigidity: ModulusOfRigidityValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfRigidityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias ShearModulusUnit for ModulusOfRigidityUnit;
    alias ShearModulusValue for ModulusOfRigidityValue;
    alias shearModulus for modulusOfRigidity;

    /* ISO-80000-4 item 4-19.3 modulus of compression, bulk modulus */
    attribute def ModulusOfCompressionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-19.3 modulus of compression, bulk modulus
         * symbol(s): `K`, `K_m`, `B`
         * application domain: generic
         * name: ModulusOfCompression
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: negative of the quotient of pressure `p` (item 4-14.1) and relative volume strain `θ` (item 4-17.4): `K = -(p/θ)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfCompressionUnit[1];
    }

    attribute modulusOfCompression: ModulusOfCompressionValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfCompressionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias BulkModulusUnit for ModulusOfCompressionUnit;
    alias BulkModulusValue for ModulusOfCompressionValue;
    alias bulkModulus for modulusOfCompression;

    /* ISO-80000-4 item 4-20 compressibility */
    attribute def CompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-20 compressibility
         * symbol(s): `ϰ`
         * application domain: generic
         * name: Compressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume `V` (ISO 80000-3) of an object under pressure `p` (item 4-14.1) expressed by: `ϰ = -(1/V)(dV)/(dp)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process). See also ISO 80000-5.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CompressibilityUnit[1];
    }

    attribute compressibility: CompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def CompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-21.1 second axial moment of area */
    attribute def SecondAxialMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.1 second axial moment of area
         * symbol(s): `I_a`
         * application domain: generic
         * name: SecondAxialMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_a = int int_M r_Q^2 dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis in the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `a`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SecondAxialMomentOfAreaUnit[1];
    }

    attribute secondAxialMomentOfArea: SecondAxialMomentOfAreaValue[*] nonunique :> scalarQuantities;

    attribute def SecondAxialMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-21.2 second polar moment of area */
    attribute def SecondPolarMomentOfAreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-21.2 second polar moment of area
         * symbol(s): `I_p`
         * application domain: generic
         * name: SecondPolarMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_p = int int_M r_Q^2 * dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis perpendicular to the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `p`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SecondPolarMomentOfAreaUnit[1];
    }

    attribute secondPolarMomentOfArea: SecondPolarMomentOfAreaValue[*] nonunique :> scalarQuantities;

    attribute def SecondPolarMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 4; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-22 section modulus */
    attribute def SectionModulusValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-22 section modulus
         * symbol(s): `Z`, `(W)`
         * application domain: generic
         * name: SectionModulus
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `Z = I_a/r_(Q_max)` where `I_a` is the second axial moment of area (item 4-21.1) and `r_(Q,max)` is the maximum radial distance (ISO 80000-3) of any point in the surface considered from the Q-axis with respect to which `I_a` is defined
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SectionModulusUnit[1];
    }

    attribute sectionModulus: SectionModulusValue[*] nonunique :> scalarQuantities;

    attribute def SectionModulusUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-4 item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction */
    attribute def StaticFrictionCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction
         * symbol(s): `μ_s`, `(f_s)`
         * application domain: generic
         * name: StaticFrictionCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the maximum magnitude of the tangential component `F_max` of the static friction force (item 4-9.3) and the magnitude of the normal component `N` of the contact force (item 4-9.1) between two bodies at relative rest with respect to each other: `F_max = μ_s * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both.
         */
    }
    attribute staticFrictionCoefficient: StaticFrictionCoefficientValue :> scalarQuantities;

    alias staticFrictionFactor for staticFrictionCoefficient;

    alias coefficientOfStaticFriction for staticFrictionCoefficient;

    /* ISO-80000-4 item 4-23.2 kinetic friction factor, dynamic friction factor */
    attribute def KineticFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.2 kinetic friction factor, dynamic friction factor
         * symbol(s): `μ`, `(f)`
         * application domain: generic
         * name: KineticFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitudes of the kinetic friction force, `F_μ` (item 4-9.4) and the normal component `N` of the contact force (item 4-9.1): `F_μ = μ * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both. The dynamic friction factor `µ` is independent in first approximation of the contact surface.
         */
    }
    attribute kineticFrictionFactor: KineticFrictionFactorValue :> scalarQuantities;

    alias dynamicFrictionFactor for kineticFrictionFactor;

    /* ISO-80000-4 item 4-23.3 rolling resistance factor */
    attribute def RollingResistanceFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.3 rolling resistance factor
         * symbol(s): `C_"rr"`
         * application domain: generic
         * name: RollingResistanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitude of the tangential component `F` and the magnitude of the normal component `N` of the force applied to a body rolling on a surface at constant speed: `F = C_(rr)*N`
         * remarks: Also known as rolling resistance coefficient, RRC.
         */
    }
    attribute rollingResistanceFactor: RollingResistanceFactorValue :> scalarQuantities;

    /* ISO-80000-4 item 4-23.4 drag coefficient, drag factor */
    attribute def DragCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-23.4 drag coefficient, drag factor
         * symbol(s): `C_D`
         * application domain: generic
         * name: DragCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor proportional to magnitude `F_D` of the drag force (item 4-9.6) of a body moving in a fluid, dependent on the shape and speed `v` (ISO 80000-3) of a body: `F_D = 1/2 * C_D * ρ * v^2 * A` where `ρ` is mass density (item 4-2) of the fluid and `A` is cross-section area (ISO 80000-3) of the body
         * remarks: None.
         */
    }
    attribute dragCoefficient: DragCoefficientValue :> scalarQuantities;

    alias dragFactor for dragCoefficient;

    /* ISO-80000-4 item 4-24 dynamic viscosity, viscosity */
    attribute def DynamicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-24 dynamic viscosity, viscosity
         * symbol(s): `η`
         * application domain: generic
         * name: DynamicViscosity
         * quantity dimension: L^-1*M^1*T^-1
         * measurement unit(s): Pa*s, kg*m^-1*s^-1
         * tensor order: 0
         * definition: for laminar flows, proportionality constant between shear stress `τ_(xz)` (item 4-16.2) in a fluid moving with a velocity `v_x` (ISO 80000-3) and gradient `(d v_x)/dz` perpendicular to the plane of shear: `τ_(xz) = η (d v_x)/(dz)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DynamicViscosityUnit[1];
    }

    attribute dynamicViscosity: DynamicViscosityValue[*] nonunique :> scalarQuantities;

    attribute def DynamicViscosityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias ViscosityUnit for DynamicViscosityUnit;
    alias ViscosityValue for DynamicViscosityValue;
    alias viscosity for dynamicViscosity;

    /* ISO-80000-4 item 4-25 kinematic viscosity */
    attribute def KinematicViscosityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-25 kinematic viscosity
         * symbol(s): `v`
         * application domain: generic
         * name: KinematicViscosity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of dynamic viscosity `η` (item 4-24) and mass density `ρ` (item 4-2) of a fluid: `v = η/ρ`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: KinematicViscosityUnit[1];
    }

    attribute kinematicViscosity: KinematicViscosityValue[*] nonunique :> scalarQuantities;

    attribute def KinematicViscosityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-4 item 4-26 surface tension */
    attribute def SurfaceTensionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-26 surface tension
         * symbol(s): `γ`, `σ`
         * application domain: generic
         * name: SurfaceTension
         * quantity dimension: M^1*T^-2
         * measurement unit(s): N*m^-1, kg*s^-2
         * tensor order: 0
         * definition: magnitude of a force acting against the enlargement of area portion of a surface separating a liquid from its surrounding
         * remarks: The concept of surface energy is closely related to surface tension and has the same dimension.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceTensionUnit[1];
    }

    attribute surfaceTension: SurfaceTensionValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceTensionUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-27.1 power */
    attribute def PowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-27.1 power
         * symbol(s): `P`
         * application domain: generic
         * name: Power
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: quotient of energy (ISO 80000-5) and duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PowerUnit[1];
    }

    attribute power: PowerValue[*] nonunique :> scalarQuantities;

    attribute def PowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-27 mechanical power */
    attribute mechanicalPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 4-27 mechanical power
         * symbol(s): `P`
         * application domain: mechanics
         * name: MechanicalPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, N*m*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: scalar product of force `vec(F)` (item 4-9.1) acting to a body and its velocity `vec(v)` (ISO 80000-3): `P = vec(F) * vec(v)`
         * remarks: None.
         */
    }

    /* ISO-80000-4 item 4-28.1 potential energy */
    attribute potentialEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.1 potential energy
         * symbol(s): `V`, `E_p`
         * application domain: generic
         * name: PotentialEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: for conservative force `vec(F)`, scalar additive quantity obeying condition `vec(F) = -nabla F`, if it exists
         * remarks: For the definition of energy, see ISO 80000-5. A force is conservative when the force field is irrotational, i.e. `rot(F) = 0` , or `vec(F)` is perpendicular to the speed of the body to ensure `vec(F) * d vec(r) = 0` .
         */
    }

    /* ISO-80000-4 item 4-28.2 kinetic energy */
    attribute kineticEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.2 kinetic energy
         * symbol(s): `T`, `E_k`
         * application domain: generic
         * name: KineticEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing a moving body expressed by: `T = 1/2 m v^2` where `m` is mass (item 4-1) of the body and `v` is its speed (ISO 80000-3)
         * remarks: For the definition of energy, see ISO 80000-5.
         */
    }

    /* ISO-80000-4 item 4-28.3 mechanical energy */
    attribute mechanicalEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.3 mechanical energy
         * symbol(s): `E`, `W`
         * application domain: generic
         * name: MechanicalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of kinetic energy `T` (item 4-28.2) and potential energy `V` (item 4-28.1): `E = T+V`
         * remarks: The symbols `E` and `W` are also used for other kinds of energy. This definition is understood in a classical way and it does not include thermal motion.
         */
    }

    /* ISO-80000-4 item 4-28.4 mechanical work, work */
    attribute mechanicalWork: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 4-28.4 mechanical work, work
         * symbol(s): `A`, `W`
         * application domain: generic
         * name: MechanicalWork (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: process quantity describing the total action of a force `vec(F)` (item 4-9.1) along a continuous curve `Γ` in three-dimensional space with infinitesimal displacement (ISO 80000-3) `dvec(r)`, as a line integral of their scalar product: `A = int_Γ vec(F) * d vec(r)`
         * remarks: The definition covers the case `A = -int_Γ p*dV` where `Γ` is a curve in the phase space and implies that work generally depends upon `Γ`, and that type of process must be defined (e.g. isentropic or isothermic).
         */
    }

    alias work for mechanicalWork;

    /* ISO-80000-4 item 4-29 mechanical efficiency */
    attribute def MechanicalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 4-29 mechanical efficiency
         * symbol(s): `η`
         * application domain: mechanics
         * name: MechanicalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of output power `P_"out"` (item 4-27) from a system and input power `P_"in"` (item 4-27) to this system: `η = P_"out"/P_"in"`
         * remarks: The system must be specified. This quantity is often expressed by the unit percent, symbol %.
         */
    }
    attribute mechanicalEfficiency: MechanicalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-4 item 4-30.1 mass flow */
    attribute def MassFlowValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.1 mass flow (magnitude)
         * symbol(s): `j_m`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassFlowUnit[1];
    }

    attribute massFlow: MassFlowValue[*] nonunique :> scalarQuantities;

    attribute def MassFlowUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    attribute def CartesianMassFlow3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 4-30.1 mass flow (vector)
         * symbol(s): `vec(j_m)`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMassFlow3dCoordinateFrame[1];
    }

    attribute cartesianMassFlow3dVector: CartesianMassFlow3dVector :> vectorQuantities;

    attribute def CartesianMassFlow3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MassFlowUnit[3];
    }

    /* ISO-80000-4 item 4-30.2 mass flow rate */
    attribute def MassFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.2 mass flow rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassFlowRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with normal vector `vec(e)_n` of a flowing fluid with mass flow `vec(j)_m` (item 4-30.1) as an integral: `q_m = int int_A vec(j)_m * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassFlowRateUnit[1];
    }

    attribute massFlowRate: MassFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def MassFlowRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-30.3 mass change rate */
    attribute def MassChangeRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-30.3 mass change rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassChangeRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: rate of increment of mass `m` (item 4-1): `q_m = (dm)/(dt)` where `dm` is the infinitesimal mass (item 4-1) increment and `dt` is the infinitesimal duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassChangeRateUnit[1];
    }

    attribute massChangeRate: MassChangeRateValue[*] nonunique :> scalarQuantities;

    attribute def MassChangeRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-4 item 4-31 volume flow rate */
    attribute def VolumeFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-31 volume flow rate
         * symbol(s): `q_v`
         * application domain: generic
         * name: VolumeFlowRate
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with the normal vector `vec(e)_n` of a flowing fluid with velocity `vec(v)` (ISO 80000-3) as an integral: `q_v = int int_A vec(v) * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeFlowRateUnit[1];
    }

    attribute volumeFlowRate: VolumeFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def VolumeFlowRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-4 item 4-32 action quantity */
    attribute def ActionQuantityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-32 action quantity
         * symbol(s): `S`
         * application domain: generic
         * name: ActionQuantity
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: time integral of energy `E` over a time interval `(t_1, t_2)`: `S = int_(t_1)^(t_2) E dt`
         * remarks: The energy may be expressed by a Lagrangian or Hamiltonian function. Note for SysML: the ISQ quantity "action" has been renamed to "action quantity" to avoid the name clash with the SysML action keyword.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ActionQuantityUnit[1];
    }

    attribute actionQuantity: ActionQuantityValue[*] nonunique :> scalarQuantities;

    attribute def ActionQuantityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

}
~~~
# EXPECTED
~~~
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
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
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
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
~~~
# PROBLEMS
~~~
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
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
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
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
semantic.unresolved_name '3dVectorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'vectorQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
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
semantic.unresolved_name 'TensorQuantityValue'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'tensorQuantities'
semantic.unresolved_name 'TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'isBound'
semantic.unresolved_name 'mRefs'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
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
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,
CloseCurly,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQMechanics'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (comment)
    (comment)
    (attribute_def 'MassDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassDensityUnit' multiplicity))
    (attribute_usage 'massDensity' : 'MassDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'DensityUnit' for 'MassDensityUnit')
    (alias_member 'DensityValue' for 'MassDensityValue')
    (alias_member 'density' for 'massDensity')
    (comment)
    (attribute_def 'SpecificVolumeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificVolumeUnit' multiplicity))
    (attribute_usage 'specificVolume' : 'SpecificVolumeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificVolumeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RelativeMassDensityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeMassDensity' : 'RelativeMassDensityValue' :> 'scalarQuantities')
    (alias_member 'relativeDensity' for 'relativeMassDensity')
    (comment)
    (attribute_def 'SurfaceMassDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SurfaceMassDensityUnit' multiplicity))
    (attribute_usage 'surfaceMassDensity' : 'SurfaceMassDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SurfaceMassDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'SurfaceDensityUnit' for 'SurfaceMassDensityUnit')
    (alias_member 'SurfaceDensityValue' for 'SurfaceMassDensityValue')
    (alias_member 'surfaceDensity' for 'surfaceMassDensity')
    (comment)
    (attribute_def 'LinearMassDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearMassDensityUnit' multiplicity))
    (attribute_usage 'linearMassDensity' : 'LinearMassDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearMassDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'LinearDensityUnit' for 'LinearMassDensityUnit')
    (alias_member 'LinearDensityValue' for 'LinearMassDensityValue')
    (alias_member 'linearDensity' for 'linearMassDensity')
    (comment)
    (attribute_def 'MomentOfInertiaValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MomentOfInertiaUnit' multiplicity))
    (attribute_usage 'momentOfInertia' : 'MomentOfInertiaValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MomentOfInertiaUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'Cartesian3dMomentOfInertiaTensor' :> 'TensorQuantityValue'
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'num' : 'Real' multiplicity)
      (attribute_usage :>> 'mRef' : 'Cartesian3dMomentOfInertiaMeasurementReference' multiplicity))
    (attribute_usage 'momentOfInertiaTensor' : 'Cartesian3dMomentOfInertiaTensor' :> 'tensorQuantities')
    (attribute_def 'Cartesian3dMomentOfInertiaMeasurementReference' :> 'TensorMeasurementReference'
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRefs' : 'MomentOfInertiaUnit' multiplicity))
    (comment)
    (attribute_def 'MomentumValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MomentumUnit' multiplicity))
    (attribute_usage 'momentum' : 'MomentumValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MomentumUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMomentum3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMomentum3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMomentum3dVector' : 'CartesianMomentum3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMomentum3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MomentumUnit' multiplicity))
    (comment)
    (attribute_def 'ForceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ForceUnit' multiplicity))
    (attribute_usage 'force' : 'ForceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ForceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianForce3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianForce3dVector' : 'CartesianForce3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianForce3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ForceUnit' multiplicity))
    (comment)
    (attribute_def 'CartesianWeight3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianWeight3dVector' : 'CartesianWeight3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'CartesianStaticFrictionForce3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianStaticFrictionForce3dVector' : 'CartesianStaticFrictionForce3dVector' :> 'vectorQuantities')
    (alias_member 'cartesianStaticFriction3dVector' for 'cartesianStaticFrictionForce3dVector')
    (comment)
    (attribute_def 'CartesianKineticFrictionForce3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianKineticFrictionForce3dVector' : 'CartesianKineticFrictionForce3dVector' :> 'vectorQuantities')
    (alias_member 'cartesianDynamicFrictionForce3dVector' for 'cartesianKineticFrictionForce3dVector')
    (comment)
    (attribute_def 'CartesianRollingResistance3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianRollingResistance3dVector' : 'CartesianRollingResistance3dVector' :> 'vectorQuantities')
    (alias_member 'cartesianRollingDrag3dVector' for 'cartesianRollingResistance3dVector')
    (alias_member 'cartesianRollingFrictionForce3dVector' for 'cartesianRollingResistance3dVector')
    (comment)
    (attribute_def 'CartesianDragForce3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianDragForce3dVector' : 'CartesianDragForce3dVector' :> 'vectorQuantities')
    (comment)
    (attribute_def 'ImpulseValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ImpulseUnit' multiplicity))
    (attribute_usage 'impulse' : 'ImpulseValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ImpulseUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianImpulse3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianImpulse3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianImpulse3dVector' : 'CartesianImpulse3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianImpulse3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ImpulseUnit' multiplicity))
    (comment)
    (attribute_def 'AngularMomentumValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularMomentumUnit' multiplicity))
    (attribute_usage 'angularMomentum' : 'AngularMomentumValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularMomentumUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAngularMomentum3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAngularMomentum3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAngularMomentum3dVector' : 'CartesianAngularMomentum3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAngularMomentum3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AngularMomentumUnit' multiplicity))
    (comment)
    (attribute_def 'MomentOfForceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MomentOfForceUnit' multiplicity))
    (attribute_usage 'momentOfForce' : 'MomentOfForceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MomentOfForceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMomentOfForce3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMomentOfForce3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMomentOfForce3dVector' : 'CartesianMomentOfForce3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMomentOfForce3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MomentOfForceUnit' multiplicity))
    (comment)
    (attribute_def 'TorqueValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TorqueUnit' multiplicity))
    (attribute_usage 'torque' : 'TorqueValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TorqueUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AngularImpulseValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AngularImpulseUnit' multiplicity))
    (attribute_usage 'angularImpulse' : 'AngularImpulseValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AngularImpulseUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianAngularImpulse3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianAngularImpulse3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianAngularImpulse3dVector' : 'CartesianAngularImpulse3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianAngularImpulse3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'AngularImpulseUnit' multiplicity))
    (comment)
    (attribute_def 'PressureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PressureUnit' multiplicity))
    (attribute_usage 'pressure' : 'PressureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PressureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'gaugePressure' : 'PressureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'StressValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'StressUnit' multiplicity))
    (attribute_usage 'stress' : 'StressValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'StressUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'Cartesian3dStressTensor' :> 'TensorQuantityValue'
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'num' : 'Real' multiplicity)
      (attribute_usage :>> 'mRef' : 'Cartesian3dStressMeasurementReference' multiplicity))
    (attribute_usage 'stressTensor' : 'Cartesian3dStressTensor' :> 'tensorQuantities')
    (attribute_def 'Cartesian3dStressMeasurementReference' :> 'TensorMeasurementReference'
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRefs' : 'StressUnit' multiplicity))
    (comment)
    (attribute_def 'NormalStressValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'NormalStressUnit' multiplicity))
    (attribute_usage 'normalStress' : 'NormalStressValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'NormalStressUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ShearStressValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ShearStressUnit' multiplicity))
    (attribute_usage 'shearStress' : 'ShearStressValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ShearStressUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'StrainValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'StrainUnit' multiplicity))
    (attribute_usage 'strain' : 'StrainValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'StrainUnit' :> 'DimensionOneUnit')
    (attribute_def 'Cartesian3dStrainTensor' :> 'TensorQuantityValue'
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'num' : 'Real' multiplicity)
      (attribute_usage :>> 'mRef' : 'Cartesian3dStrainMeasurementReference' multiplicity))
    (attribute_usage 'strainTensor' : 'Cartesian3dStrainTensor' :> 'tensorQuantities')
    (attribute_def 'Cartesian3dStrainMeasurementReference' :> 'TensorMeasurementReference'
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRefs' : 'StrainUnit' multiplicity))
    (comment)
    (attribute_def 'RelativeLinearStrainValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeLinearStrain' : 'RelativeLinearStrainValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ShearStrainValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'shearStrain' : 'ShearStrainValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RelativeVolumeStrainValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeVolumeStrain' : 'RelativeVolumeStrainValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PoissonNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'poissonNumber' : 'PoissonNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ModulusOfElasticityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ModulusOfElasticityUnit' multiplicity))
    (attribute_usage 'modulusOfElasticity' : 'ModulusOfElasticityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ModulusOfElasticityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'YoungModulusUnit' for 'ModulusOfElasticityUnit')
    (alias_member 'YoungModulusValue' for 'ModulusOfElasticityValue')
    (alias_member 'youngModulus' for 'modulusOfElasticity')
    (comment)
    (attribute_def 'ModulusOfRigidityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ModulusOfRigidityUnit' multiplicity))
    (attribute_usage 'modulusOfRigidity' : 'ModulusOfRigidityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ModulusOfRigidityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'ShearModulusUnit' for 'ModulusOfRigidityUnit')
    (alias_member 'ShearModulusValue' for 'ModulusOfRigidityValue')
    (alias_member 'shearModulus' for 'modulusOfRigidity')
    (comment)
    (attribute_def 'ModulusOfCompressionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ModulusOfCompressionUnit' multiplicity))
    (attribute_usage 'modulusOfCompression' : 'ModulusOfCompressionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ModulusOfCompressionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'BulkModulusUnit' for 'ModulusOfCompressionUnit')
    (alias_member 'BulkModulusValue' for 'ModulusOfCompressionValue')
    (alias_member 'bulkModulus' for 'modulusOfCompression')
    (comment)
    (attribute_def 'CompressibilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CompressibilityUnit' multiplicity))
    (attribute_usage 'compressibility' : 'CompressibilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CompressibilityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SecondAxialMomentOfAreaValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SecondAxialMomentOfAreaUnit' multiplicity))
    (attribute_usage 'secondAxialMomentOfArea' : 'SecondAxialMomentOfAreaValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SecondAxialMomentOfAreaUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SecondPolarMomentOfAreaValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SecondPolarMomentOfAreaUnit' multiplicity))
    (attribute_usage 'secondPolarMomentOfArea' : 'SecondPolarMomentOfAreaValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SecondPolarMomentOfAreaUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SectionModulusValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SectionModulusUnit' multiplicity))
    (attribute_usage 'sectionModulus' : 'SectionModulusValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SectionModulusUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'StaticFrictionCoefficientValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'staticFrictionCoefficient' : 'StaticFrictionCoefficientValue' :> 'scalarQuantities')
    (alias_member 'staticFrictionFactor' for 'staticFrictionCoefficient')
    (alias_member 'coefficientOfStaticFriction' for 'staticFrictionCoefficient')
    (comment)
    (attribute_def 'KineticFrictionFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'kineticFrictionFactor' : 'KineticFrictionFactorValue' :> 'scalarQuantities')
    (alias_member 'dynamicFrictionFactor' for 'kineticFrictionFactor')
    (comment)
    (attribute_def 'RollingResistanceFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'rollingResistanceFactor' : 'RollingResistanceFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DragCoefficientValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'dragCoefficient' : 'DragCoefficientValue' :> 'scalarQuantities')
    (alias_member 'dragFactor' for 'dragCoefficient')
    (comment)
    (attribute_def 'DynamicViscosityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DynamicViscosityUnit' multiplicity))
    (attribute_usage 'dynamicViscosity' : 'DynamicViscosityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DynamicViscosityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'ViscosityUnit' for 'DynamicViscosityUnit')
    (alias_member 'ViscosityValue' for 'DynamicViscosityValue')
    (alias_member 'viscosity' for 'dynamicViscosity')
    (comment)
    (attribute_def 'KinematicViscosityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'KinematicViscosityUnit' multiplicity))
    (attribute_usage 'kinematicViscosity' : 'KinematicViscosityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'KinematicViscosityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SurfaceTensionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SurfaceTensionUnit' multiplicity))
    (attribute_usage 'surfaceTension' : 'SurfaceTensionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SurfaceTensionUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PowerValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PowerUnit' multiplicity))
    (attribute_usage 'power' : 'PowerValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PowerUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'mechanicalPower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'potentialEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'kineticEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'mechanicalEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'mechanicalWork' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'work' for 'mechanicalWork')
    (comment)
    (attribute_def 'MechanicalEfficiencyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'mechanicalEfficiency' : 'MechanicalEfficiencyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MassFlowValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassFlowUnit' multiplicity))
    (attribute_usage 'massFlow' : 'MassFlowValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassFlowUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMassFlow3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMassFlow3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMassFlow3dVector' : 'CartesianMassFlow3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMassFlow3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MassFlowUnit' multiplicity))
    (comment)
    (attribute_def 'MassFlowRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassFlowRateUnit' multiplicity))
    (attribute_usage 'massFlowRate' : 'MassFlowRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassFlowRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassChangeRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassChangeRateUnit' multiplicity))
    (attribute_usage 'massChangeRate' : 'MassChangeRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassChangeRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'VolumeFlowRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'VolumeFlowRateUnit' multiplicity))
    (attribute_usage 'volumeFlowRate' : 'VolumeFlowRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'VolumeFlowRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ActionQuantityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ActionQuantityUnit' multiplicity))
    (attribute_usage 'actionQuantity' : 'ActionQuantityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ActionQuantityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))))
~~~
# FORMAT
~~~sysml
standard library package ISQMechanics {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-4:2019 "Mechanics"
     * see also https://www.iso.org/standard/64975.html
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

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-4 item 4-1 mass */
    /* See package ISQBase for the declarations of MassValue and MassUnit */

    /* ISO-80000-4 item 4-2 mass density, density */
    attribute def MassDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-2 mass density, density
         * symbol(s): `ρ`, `ρ_m`
         * application domain: generic
         * name: MassDensity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quantity representing the spatial distribution of mass of a continuous material: `ρ(vec(r)) = (dm)/(dV)` where `m` is mass of the material contained in an infinitesimal domain at point `vec(r)` and `V` is volume of this domain
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassDensityUnit [1];
    }

    attribute massDensity : MassDensityValue :> scalarQuantities [*] nonunique;

    attribute def MassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    alias DensityUnit for MassDensityUnit;
    alias DensityValue for MassDensityValue;
    alias density for massDensity;

    /* ISO-80000-4 item 4-3 specific volume */
    attribute def SpecificVolumeValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-3 specific volume
         * symbol(s): `v`
         * application domain: generic
         * name: SpecificVolume
         * quantity dimension: L^3*M^-1
         * measurement unit(s): kg^-1*m^3
         * tensor order: 0
         * definition: reciprocal of mass density `ρ` (item 4-2): `v = 1/ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificVolumeUnit [1];
    }

    attribute specificVolume : SpecificVolumeValue :> scalarQuantities [*] nonunique;

    attribute def SpecificVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    /* ISO-80000-4 item 4-4 relative mass density, relative density */
    attribute def RelativeMassDensityValue :> DimensionOneValue {
        doc /*
         * source: item 4-4 relative mass density, relative density
         * symbol(s): `d`
         * application domain: generic
         * name: RelativeMassDensity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass density of a substance `ρ` and mass density of a reference substance `ρ_0` : `d = ρ/ρ_0`
         * remarks: Conditions and material should be specified for the reference substance.
         */
    }
    attribute relativeMassDensity : RelativeMassDensityValue :> scalarQuantities;

    alias relativeDensity for relativeMassDensity;

    /* ISO-80000-4 item 4-5 surface mass density, surface density */
    attribute def SurfaceMassDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-5 surface mass density, surface density
         * symbol(s): `ρ_A`
         * application domain: generic
         * name: SurfaceMassDensity
         * quantity dimension: L^-2*M^1
         * measurement unit(s): kg*m^-2
         * tensor order: 0
         * definition: quantity representing the areal distribution of mass of a continuous material: `ρ_A(vec(r)) = (dm)/(dA)` where `m` is the mass of the material at position `vec(r)` and `A` is area
         * remarks: The name "grammage" should not be used for this quantity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceMassDensityUnit [1];
    }

    attribute surfaceMassDensity : SurfaceMassDensityValue :> scalarQuantities [*] nonunique;

    attribute def SurfaceMassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    alias SurfaceDensityUnit for SurfaceMassDensityUnit;
    alias SurfaceDensityValue for SurfaceMassDensityValue;
    alias surfaceDensity for surfaceMassDensity;

    /* ISO-80000-4 item 4-6 linear mass density, linear density */
    attribute def LinearMassDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-6 linear mass density, linear density
         * symbol(s): `ρ_I`
         * application domain: generic
         * name: LinearMassDensity
         * quantity dimension: L^-1*M^1
         * measurement unit(s): kg*m^-1
         * tensor order: 0
         * definition: quantity representing the linear distribution of mass of a continuous material: `ρ_I(vec(r)) = (dm)/(dI)` where `m` is the mass of the material at position `vec(r)` and `l` is length
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearMassDensityUnit [1];
    }

    attribute linearMassDensity : LinearMassDensityValue :> scalarQuantities [*] nonunique;

    attribute def LinearMassDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    alias LinearDensityUnit for LinearMassDensityUnit;
    alias LinearDensityValue for LinearMassDensityValue;
    alias linearDensity for linearMassDensity;

    /* ISO-80000-4 item 4-7 moment of inertia */
    attribute def MomentOfInertiaValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-7 moment of inertia (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentOfInertiaUnit [1];
    }

    attribute momentOfInertia : MomentOfInertiaValue :> scalarQuantities [*] nonunique;

    attribute def MomentOfInertiaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }

    attribute def Cartesian3dMomentOfInertiaTensor :> TensorQuantityValue {
        doc /*
         * source: item 4-7 moment of inertia (tensor)
         * symbol(s): `vec(vec(J))`
         * application domain: generic
         * name: MomentOfInertia
         * quantity dimension: L^2*M^1
         * measurement unit(s): kg*m^2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing rotational inertia of a rigid body relative to a fixed centre of rotation expressed by the tensor product: `vec(L) = vec(vec(J)) vec(ω)` where `vec(L)` is angular momentum (item 4-11) of the body relative to the reference point and `vec(ω)` is its angular velocity (ISO 80000-3)
         * remarks: The calculation of the value requires an integration.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real [9];
        attribute :>> mRef : Cartesian3dMomentOfInertiaMeasurementReference [1];
    }

    attribute momentOfInertiaTensor : Cartesian3dMomentOfInertiaTensor :> tensorQuantities;

    attribute def Cartesian3dMomentOfInertiaMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : MomentOfInertiaUnit [9];
    }

    /* ISO-80000-4 item 4-8 momentum */
    attribute def MomentumValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-8 momentum (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 0
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentumUnit [1];
    }

    attribute momentum : MomentumValue :> scalarQuantities [*] nonunique;

    attribute def MomentumUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianMomentum3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-8 momentum (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: Momentum
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): kg*m*s^-1
         * tensor order: 1
         * definition: product of mass `m` (item 4-1) of a body and velocity `vec(v)` (ISO 80000-3) of its centre of mass: `vec(p) = m  vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMomentum3dCoordinateFrame [1];
    }

    attribute cartesianMomentum3dVector : CartesianMomentum3dVector :> vectorQuantities;

    attribute def CartesianMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MomentumUnit [3];
    }

    /* ISO-80000-4 item 4-9.1 force */
    attribute def ForceValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-9.1 force (magnitude)
         * symbol(s): `F`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ForceUnit [1];
    }

    attribute force : ForceValue :> scalarQuantities [*] nonunique;

    attribute def ForceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianForce3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.1 force (vector)
         * symbol(s): `vec(F)`
         * application domain: generic
         * name: Force
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing interaction between bodies or particles
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianForce3dVector : CartesianForce3dVector :> vectorQuantities;

    attribute def CartesianForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ForceUnit [3];
    }

    /* ISO-80000-4 item 4-9.2 weight */
    attribute def CartesianWeight3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.2 weight
         * symbol(s): `vec(F_g)`
         * application domain: generic
         * name: Weight (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) acting on a body in the gravitational field of Earth: `vec(F_g) = m vec(g)` where `m` (item 4-1) is the mass of the body and `vec(g)` is the local acceleration of free fall (ISO 80000-3)
         * remarks: In colloquial language, the name "weight" continues to be used where "mass" is meant. This practice should be avoided. Weight is an example of a gravitational force. Weight comprises not only the local gravitational force but also the local centrifugal force due to the rotation of the Earth.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianWeight3dVector : CartesianWeight3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-9.3 static friction force, static friction */
    attribute def CartesianStaticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.3 static friction force, static friction
         * symbol(s): `vec(F_s)`
         * application domain: generic
         * name: StaticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion before a body starts to slide on a surface
         * remarks: For the static friction coefficient, see item 4-23.1.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianStaticFrictionForce3dVector : CartesianStaticFrictionForce3dVector :> vectorQuantities;

    alias cartesianStaticFriction3dVector for cartesianStaticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.4 kinetic friction force, dynamic friction force */
    attribute def CartesianKineticFrictionForce3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.4 kinetic friction force, dynamic friction force
         * symbol(s): `vec(F_μ)`
         * application domain: generic
         * name: KineticFrictionForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body slides on a surface
         * remarks: For the kinetic friction factor, see item 4-23.2.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianKineticFrictionForce3dVector : CartesianKineticFrictionForce3dVector :> vectorQuantities;

    alias cartesianDynamicFrictionForce3dVector for cartesianKineticFrictionForce3dVector;

    /* ISO-80000-4 item 4-9.5 rolling resistance, rolling drag, rolling friction force */
    attribute def CartesianRollingResistance3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.5 rolling resistance, rolling drag, rolling friction force
         * symbol(s): `vec(F_"rr")`
         * application domain: generic
         * name: RollingResistance (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion when a body rolls on a surface
         * remarks: For the rolling resistance factor, see item 4-23.3.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianRollingResistance3dVector : CartesianRollingResistance3dVector :> vectorQuantities;

    alias cartesianRollingDrag3dVector for cartesianRollingResistance3dVector;

    alias cartesianRollingFrictionForce3dVector for cartesianRollingResistance3dVector;

    /* ISO-80000-4 item 4-9.6 drag force */
    attribute def CartesianDragForce3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-9.6 drag force
         * symbol(s): `vec(F_D)`
         * application domain: generic
         * name: DragForce (specializes Force)
         * quantity dimension: L^1*M^1*T^-2
         * measurement unit(s): N, kg*m*s^-2
         * tensor order: 1
         * definition: force (item 4-9.1) resisting the motion of a body in a fluid
         * remarks: For the drag coefficient, see item 4-23.4.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianForce3dCoordinateFrame [1];
    }

    attribute cartesianDragForce3dVector : CartesianDragForce3dVector :> vectorQuantities;

    /* ISO-80000-4 item 4-10 impulse */
    attribute def ImpulseValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-10 impulse (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ImpulseUnit [1];
    }

    attribute impulse : ImpulseValue :> scalarQuantities [*] nonunique;

    attribute def ImpulseUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianImpulse3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-10 impulse (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: Impulse
         * quantity dimension: L^1*M^1*T^-1
         * measurement unit(s): N*s, kg*m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of force acting during a time interval: `vec(I) = int_(t_1)^(t_2) vec(F)*dt` where `vec(F)` is force (item 4-9.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(I)(t_1, t_2) = vec(p)(t_1) - vec(p)(t_2) = vec(Δp)` where `vec(p)` is momentum (item 4-8).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianImpulse3dCoordinateFrame [1];
    }

    attribute cartesianImpulse3dVector : CartesianImpulse3dVector :> vectorQuantities;

    attribute def CartesianImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ImpulseUnit [3];
    }

    /* ISO-80000-4 item 4-11 angular momentum */
    attribute def AngularMomentumValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-11 angular momentum (magnitude)
         * symbol(s): `L`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularMomentumUnit [1];
    }

    attribute angularMomentum : AngularMomentumValue :> scalarQuantities [*] nonunique;

    attribute def AngularMomentumUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianAngularMomentum3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-11 angular momentum (vector)
         * symbol(s): `vec(L)`
         * application domain: generic
         * name: AngularMomentum
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(L) = vec(r) xx vec(p)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(p)` is momentum (item 4-8)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularMomentum3dCoordinateFrame [1];
    }

    attribute cartesianAngularMomentum3dVector : CartesianAngularMomentum3dVector :> vectorQuantities;

    attribute def CartesianAngularMomentum3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularMomentumUnit [3];
    }

    /* ISO-80000-4 item 4-12.1 moment of force */
    attribute def MomentOfForceValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-12.1 moment of force (magnitude)
         * symbol(s): `M`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MomentOfForceUnit [1];
    }

    attribute momentOfForce : MomentOfForceValue :> scalarQuantities [*] nonunique;

    attribute def MomentOfForceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianMomentOfForce3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-12.1 moment of force (vector)
         * symbol(s): `vec(M)`
         * application domain: generic
         * name: MomentOfForce
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity described by the vector product: `vec(M) = vec(r) xx vec(F)` where `vec(r)` is position vector (ISO 80000-3) with respect to the axis of rotation and `vec(F)` is force (item 4-9.1)
         * remarks: The bending moment of force is denoted by `vec(M)_b`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMomentOfForce3dCoordinateFrame [1];
    }

    attribute cartesianMomentOfForce3dVector : CartesianMomentOfForce3dVector :> vectorQuantities;

    attribute def CartesianMomentOfForce3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MomentOfForceUnit [3];
    }

    /* ISO-80000-4 item 4-12.2 torque */
    attribute def TorqueValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-12.2 torque
         * symbol(s): `T`, `M_Q`
         * application domain: generic
         * name: Torque
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): N*m, kg*m^2*s^-2
         * tensor order: 0
         * definition: quantity described by the scalar product: `T = vec(M)*vec(e_Q)` where `vec(M)` is moment of force (item 4-12.1) and `vec(e_Q)` is unit vector of direction with respect to which the torque is considered
         * remarks: For example, torque is the twisting moment of force with respect to the longitudinal axis of a beam or shaft.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TorqueUnit [1];
    }

    attribute torque : TorqueValue :> scalarQuantities [*] nonunique;

    attribute def TorqueUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-13 angular impulse */
    attribute def AngularImpulseValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-13 angular impulse (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularImpulseUnit [1];
    }

    attribute angularImpulse : AngularImpulseValue :> scalarQuantities [*] nonunique;

    attribute def AngularImpulseUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianAngularImpulse3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-13 angular impulse (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: AngularImpulse
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): N*m*s, kg*m^2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity describing the effect of moment of force during a time interval: `vec(H)(t_1; t_2) = int_(t_1)^(t_2) vec(M) dt` where `vec(M)` is moment of force (item 4-12.1), `t` is time (ISO 80000-3) and `[t_1, t_2]` is considered time interval
         * remarks: For a time interval `[t_1, t_2]`, `vec(H)(t_1, t_2) = vec(L)(t_1) - vec(L)(t_2) = vec(ΔL)` where `vec(L)` is angular momentum.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularImpulse3dCoordinateFrame [1];
    }

    attribute cartesianAngularImpulse3dVector : CartesianAngularImpulse3dVector :> vectorQuantities;

    attribute def CartesianAngularImpulse3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularImpulseUnit [3];
    }

    /* ISO-80000-4 item 4-14.1 pressure */
    attribute def PressureValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-14.1 pressure
         * symbol(s): `p`
         * application domain: generic
         * name: Pressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of the component of a force normal to a surface and its area: `p = (vec(e_n) * vec(F)) / A` where `vec(e_n)` is unit vector of the surface normal, `vec(F)` is force (item 4-9.1) and `A` is area (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PressureUnit [1];
    }

    attribute pressure : PressureValue :> scalarQuantities [*] nonunique;

    attribute def PressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-14.2 gauge pressure */
    attribute gaugePressure : PressureValue :> scalarQuantities {
        doc /*
         * source: item 4-14.2 gauge pressure
         * symbol(s): `p_e`
         * application domain: generic
         * name: GaugePressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure `p` (item 4-14.1) decremented by ambient pressure `p_amb` : `p_e = p - p_amb`
         * remarks: Often, `p_amb` is chosen as a standard pressure. Gauge pressure is positive or negative.
         */
    }

    /* ISO-80000-4 item 4-15 stress */
    attribute def StressValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-15 stress (magnitude)
         * symbol(s): `σ`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> num : Real;
        attribute :>> mRef : StressUnit [1];
    }

    attribute stress : StressValue :> scalarQuantities [*] nonunique;

    attribute def StressUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def Cartesian3dStressTensor :> TensorQuantityValue {
        doc /*
         * source: item 4-15 stress (tensor)
         * symbol(s): `vec(vec(σ))`
         * application domain: generic
         * name: Stress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing state of tension of matter
         * remarks: Stress tensor is symmetric and has three normal-stress and three shear-stress (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real [9];
        attribute :>> mRef : Cartesian3dStressMeasurementReference [1];
    }

    attribute stressTensor : Cartesian3dStressTensor :> tensorQuantities;

    attribute def Cartesian3dStressMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : StressUnit [9];
    }

    /* ISO-80000-4 item 4-16.1 normal stress */
    attribute def NormalStressValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-16.1 normal stress
         * symbol(s): `σ_n`, `σ`
         * application domain: generic
         * name: NormalStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `σ_n = (d F_n)/(dA)` where `F_n` is the normal component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter normal to it, and evenly distributed, cause a constant normal stress `σ_n = F A` in the slice (layer).
         */
        attribute :>> num : Real;
        attribute :>> mRef : NormalStressUnit [1];
    }

    attribute normalStress : NormalStressValue :> scalarQuantities [*] nonunique;

    attribute def NormalStressUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-16.2 shear stress */
    attribute def ShearStressValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-16.2 shear stress
         * symbol(s): `τ_s`, `τ`
         * application domain: generic
         * name: ShearStress
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity describing surface action of a force into a body equal to: `τ_s = (d F_t)/(dA)` where `F_t` is the tangential component of force (item 4-9.1) and `A` is the area (ISO 80000-3) of the surface element
         * remarks: A couple of mutually opposite forces of magnitude `F` acting on the opposite surfaces of a slice (layer) of homogenous solid matter parallel to it, and evenly distributed, cause a constant shear stress `τ = F/A` in the slice (layer).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ShearStressUnit [1];
    }

    attribute shearStress : ShearStressValue :> scalarQuantities [*] nonunique;

    attribute def ShearStressUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-17.1 strain */
    attribute def StrainValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-17.1 strain (magnitude)
         * symbol(s): `ε`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> num : Real;
        attribute :>> mRef : StrainUnit [1];
    }

    attribute strain : StrainValue :> scalarQuantities [*] nonunique;

    attribute def StrainUnit :> DimensionOneUnit { }

    attribute def Cartesian3dStrainTensor :> TensorQuantityValue {
        doc /*
         * source: item 4-17.1 strain (tensor)
         * symbol(s): `vec(vec(ε))`
         * application domain: generic
         * name: Strain
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 2
         * definition: tensor (ISO 80000-2) quantity representing the deformation of matter caused by stress
         * remarks: Strain tensor is symmetric and has three linear-strain and three shear strain (Cartesian) components.
         */
        attribute :>> isBound = false;
        attribute :>> num : Real [9];
        attribute :>> mRef : Cartesian3dStrainMeasurementReference [1];
    }

    attribute strainTensor : Cartesian3dStrainTensor :> tensorQuantities;

    attribute def Cartesian3dStrainMeasurementReference :> TensorMeasurementReference {
        attribute :>> dimensions = (3, 3);
        attribute :>> isBound = false;
        attribute :>> mRefs : StrainUnit [9];
    }

    /* ISO-80000-4 item 4-17.2 relative linear strain */
    attribute def RelativeLinearStrainValue :> DimensionOneValue {
        doc /*
         * source: item 4-17.2 relative linear strain
         * symbol(s): `ε`, `(e)`
         * application domain: generic
         * name: RelativeLinearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in length `Δl` (ISO 80000-3) of an object and its length `l` (ISO 80000-3): `ε = (Δl)/l`
         * remarks: None.
         */
    }
    attribute relativeLinearStrain : RelativeLinearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.3 shear strain */
    attribute def ShearStrainValue :> DimensionOneValue {
        doc /*
         * source: item 4-17.3 shear strain
         * symbol(s): `γ`
         * application domain: generic
         * name: ShearStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of parallel displacement `Δx` (ISO 80000-3) of two surfaces of a layer and the thickness `d` (ISO 80000-3) of the layer: `γ = (Δx)/d`
         * remarks: None.
         */
    }
    attribute shearStrain : ShearStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-17.4 relative volume strain */
    attribute def RelativeVolumeStrainValue :> DimensionOneValue {
        doc /*
         * source: item 4-17.4 relative volume strain
         * symbol(s): `θ`
         * application domain: generic
         * name: RelativeVolumeStrain (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in volume `ΔV` (ISO 80000-3) of an object and its volume `V_0` (ISO 80000-3): `θ = (ΔV)/V_0`
         * remarks: None.
         */
    }
    attribute relativeVolumeStrain : RelativeVolumeStrainValue :> scalarQuantities;

    /* ISO-80000-4 item 4-18 Poisson number */
    attribute def PoissonNumberValue :> DimensionOneValue {
        doc /*
         * source: item 4-18 Poisson number
         * symbol(s): `μ`, `(v)`
         * application domain: generic
         * name: PoissonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of change in width `Δb` (width is defined in ISO 80000-3) and change in length `Δl` (length is defined in ISO 80000-3) of an object: `μ = (Δb)/(Δl)`
         * remarks: None.
         */
    }
    attribute poissonNumber : PoissonNumberValue :> scalarQuantities;

    /* ISO-80000-4 item 4-19.1 modulus of elasticity, Young modulus */
    attribute def ModulusOfElasticityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-19.1 modulus of elasticity, Young modulus
         * symbol(s): `E`, `E_m`, `Y`
         * application domain: generic
         * name: ModulusOfElasticity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of normal stress `σ` (item 4-16.1) and relative linear strain `ε` (item 4-17.2): `E = σ/ε`
         * remarks: Conditions should be specified (e.g. adiabatic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfElasticityUnit [1];
    }

    attribute modulusOfElasticity : ModulusOfElasticityValue :> scalarQuantities [*] nonunique;

    attribute def ModulusOfElasticityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    alias YoungModulusUnit for ModulusOfElasticityUnit;
    alias YoungModulusValue for ModulusOfElasticityValue;
    alias youngModulus for modulusOfElasticity;

    /* ISO-80000-4 item 4-19.2 modulus of rigidity, shear modulus */
    attribute def ModulusOfRigidityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-19.2 modulus of rigidity, shear modulus
         * symbol(s): `G`
         * application domain: generic
         * name: ModulusOfRigidity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quotient of shear stress `τ` (item 4-16.2) and shear strain `γ` (item 4-17.3): `G = τ/γ`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfRigidityUnit [1];
    }

    attribute modulusOfRigidity : ModulusOfRigidityValue :> scalarQuantities [*] nonunique;

    attribute def ModulusOfRigidityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    alias ShearModulusUnit for ModulusOfRigidityUnit;
    alias ShearModulusValue for ModulusOfRigidityValue;
    alias shearModulus for modulusOfRigidity;

    /* ISO-80000-4 item 4-19.3 modulus of compression, bulk modulus */
    attribute def ModulusOfCompressionValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-19.3 modulus of compression, bulk modulus
         * symbol(s): `K`, `K_m`, `B`
         * application domain: generic
         * name: ModulusOfCompression
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, N*m^-2, kg*m^-1*s^-2
         * tensor order: 0
         * definition: negative of the quotient of pressure `p` (item 4-14.1) and relative volume strain `θ` (item 4-17.4): `K = -(p/θ)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process).
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfCompressionUnit [1];
    }

    attribute modulusOfCompression : ModulusOfCompressionValue :> scalarQuantities [*] nonunique;

    attribute def ModulusOfCompressionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    alias BulkModulusUnit for ModulusOfCompressionUnit;
    alias BulkModulusValue for ModulusOfCompressionValue;
    alias bulkModulus for modulusOfCompression;

    /* ISO-80000-4 item 4-20 compressibility */
    attribute def CompressibilityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-20 compressibility
         * symbol(s): `ϰ`
         * application domain: generic
         * name: Compressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume `V` (ISO 80000-3) of an object under pressure `p` (item 4-14.1) expressed by: `ϰ = -(1/V)(dV)/(dp)`
         * remarks: Conditions should be specified (e.g. isentropic or isothermal process). See also ISO 80000-5.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CompressibilityUnit [1];
    }

    attribute compressibility : CompressibilityValue :> scalarQuantities [*] nonunique;

    attribute def CompressibilityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-21.1 second axial moment of area */
    attribute def SecondAxialMomentOfAreaValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-21.1 second axial moment of area
         * symbol(s): `I_a`
         * application domain: generic
         * name: SecondAxialMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_a = int int_M r_Q^2 dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis in the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `a`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SecondAxialMomentOfAreaUnit [1];
    }

    attribute secondAxialMomentOfArea : SecondAxialMomentOfAreaValue :> scalarQuantities [*] nonunique;

    attribute def SecondAxialMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 4;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-4 item 4-21.2 second polar moment of area */
    attribute def SecondPolarMomentOfAreaValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-21.2 second polar moment of area
         * symbol(s): `I_p`
         * application domain: generic
         * name: SecondPolarMomentOfArea
         * quantity dimension: L^4
         * measurement unit(s): m^4
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `I_p = int int_M r_Q^2 * dA` where `M` is the two-dimensional domain of the cross-section of a plane and considered body, `r_Q` is radial distance (ISO 80000-3) from a Q-axis perpendicular to the plane of the surface considered and `A` is area (ISO 80000-3)
         * remarks: This quantity is often referred to wrongly as "moment of inertia" (item 4-7). The subscript, `p`, may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SecondPolarMomentOfAreaUnit [1];
    }

    attribute secondPolarMomentOfArea : SecondPolarMomentOfAreaValue :> scalarQuantities [*] nonunique;

    attribute def SecondPolarMomentOfAreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 4;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-4 item 4-22 section modulus */
    attribute def SectionModulusValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-22 section modulus
         * symbol(s): `Z`, `(W)`
         * application domain: generic
         * name: SectionModulus
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: geometrical characteristic of a shape of a body equal to: `Z = I_a/r_(Q_max)` where `I_a` is the second axial moment of area (item 4-21.1) and `r_(Q,max)` is the maximum radial distance (ISO 80000-3) of any point in the surface considered from the Q-axis with respect to which `I_a` is defined
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SectionModulusUnit [1];
    }

    attribute sectionModulus : SectionModulusValue :> scalarQuantities [*] nonunique;

    attribute def SectionModulusUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = lengthPF;
        }
    }

    /* ISO-80000-4 item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction */
    attribute def StaticFrictionCoefficientValue :> DimensionOneValue {
        doc /*
         * source: item 4-23.1 static friction coefficient, static friction factor, coefficient of static friction
         * symbol(s): `μ_s`, `(f_s)`
         * application domain: generic
         * name: StaticFrictionCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the maximum magnitude of the tangential component `F_max` of the static friction force (item 4-9.3) and the magnitude of the normal component `N` of the contact force (item 4-9.1) between two bodies at relative rest with respect to each other: `F_max = μ_s * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both.
         */
    }
    attribute staticFrictionCoefficient : StaticFrictionCoefficientValue :> scalarQuantities;

    alias staticFrictionFactor for staticFrictionCoefficient;

    alias coefficientOfStaticFriction for staticFrictionCoefficient;

    /* ISO-80000-4 item 4-23.2 kinetic friction factor, dynamic friction factor */
    attribute def KineticFrictionFactorValue :> DimensionOneValue {
        doc /*
         * source: item 4-23.2 kinetic friction factor, dynamic friction factor
         * symbol(s): `μ`, `(f)`
         * application domain: generic
         * name: KineticFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitudes of the kinetic friction force, `F_μ` (item 4-9.4) and the normal component `N` of the contact force (item 4-9.1): `F_μ = μ * N`
         * remarks: When it is not necessary to distinguish between dynamic friction factor and static friction factor, the name friction factor may be used for both. The dynamic friction factor `µ` is independent in first approximation of the contact surface.
         */
    }
    attribute kineticFrictionFactor : KineticFrictionFactorValue :> scalarQuantities;

    alias dynamicFrictionFactor for kineticFrictionFactor;

    /* ISO-80000-4 item 4-23.3 rolling resistance factor */
    attribute def RollingResistanceFactorValue :> DimensionOneValue {
        doc /*
         * source: item 4-23.3 rolling resistance factor
         * symbol(s): `C_"rr"`
         * application domain: generic
         * name: RollingResistanceFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: proportionality factor between the magnitude of the tangential component `F` and the magnitude of the normal component `N` of the force applied to a body rolling on a surface at constant speed: `F = C_(rr)*N`
         * remarks: Also known as rolling resistance coefficient, RRC.
         */
    }
    attribute rollingResistanceFactor : RollingResistanceFactorValue :> scalarQuantities;

    /* ISO-80000-4 item 4-23.4 drag coefficient, drag factor */
    attribute def DragCoefficientValue :> DimensionOneValue {
        doc /*
         * source: item 4-23.4 drag coefficient, drag factor
         * symbol(s): `C_D`
         * application domain: generic
         * name: DragCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: factor proportional to magnitude `F_D` of the drag force (item 4-9.6) of a body moving in a fluid, dependent on the shape and speed `v` (ISO 80000-3) of a body: `F_D = 1/2 * C_D * ρ * v^2 * A` where `ρ` is mass density (item 4-2) of the fluid and `A` is cross-section area (ISO 80000-3) of the body
         * remarks: None.
         */
    }
    attribute dragCoefficient : DragCoefficientValue :> scalarQuantities;

    alias dragFactor for dragCoefficient;

    /* ISO-80000-4 item 4-24 dynamic viscosity, viscosity */
    attribute def DynamicViscosityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-24 dynamic viscosity, viscosity
         * symbol(s): `η`
         * application domain: generic
         * name: DynamicViscosity
         * quantity dimension: L^-1*M^1*T^-1
         * measurement unit(s): Pa*s, kg*m^-1*s^-1
         * tensor order: 0
         * definition: for laminar flows, proportionality constant between shear stress `τ_(xz)` (item 4-16.2) in a fluid moving with a velocity `v_x` (ISO 80000-3) and gradient `(d v_x)/dz` perpendicular to the plane of shear: `τ_(xz) = η (d v_x)/(dz)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DynamicViscosityUnit [1];
    }

    attribute dynamicViscosity : DynamicViscosityValue :> scalarQuantities [*] nonunique;

    attribute def DynamicViscosityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    alias ViscosityUnit for DynamicViscosityUnit;
    alias ViscosityValue for DynamicViscosityValue;
    alias viscosity for dynamicViscosity;

    /* ISO-80000-4 item 4-25 kinematic viscosity */
    attribute def KinematicViscosityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-25 kinematic viscosity
         * symbol(s): `v`
         * application domain: generic
         * name: KinematicViscosity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of dynamic viscosity `η` (item 4-24) and mass density `ρ` (item 4-2) of a fluid: `v = η/ρ`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : KinematicViscosityUnit [1];
    }

    attribute kinematicViscosity : KinematicViscosityValue :> scalarQuantities [*] nonunique;

    attribute def KinematicViscosityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-26 surface tension */
    attribute def SurfaceTensionValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-26 surface tension
         * symbol(s): `γ`, `σ`
         * application domain: generic
         * name: SurfaceTension
         * quantity dimension: M^1*T^-2
         * measurement unit(s): N*m^-1, kg*s^-2
         * tensor order: 0
         * definition: magnitude of a force acting against the enlargement of area portion of a surface separating a liquid from its surrounding
         * remarks: The concept of surface energy is closely related to surface tension and has the same dimension.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceTensionUnit [1];
    }

    attribute surfaceTension : SurfaceTensionValue :> scalarQuantities [*] nonunique;

    attribute def SurfaceTensionUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-27.1 power */
    attribute def PowerValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-27.1 power
         * symbol(s): `P`
         * application domain: generic
         * name: Power
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: quotient of energy (ISO 80000-5) and duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PowerUnit [1];
    }

    attribute power : PowerValue :> scalarQuantities [*] nonunique;

    attribute def PowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-27 mechanical power */
    attribute mechanicalPower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 4-27 mechanical power
         * symbol(s): `P`
         * application domain: mechanics
         * name: MechanicalPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, N*m*s^-1, kg*m^2*s^-3
         * tensor order: 0
         * definition: scalar product of force `vec(F)` (item 4-9.1) acting to a body and its velocity `vec(v)` (ISO 80000-3): `P = vec(F) * vec(v)`
         * remarks: None.
         */
    }

    /* ISO-80000-4 item 4-28.1 potential energy */
    attribute potentialEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 4-28.1 potential energy
         * symbol(s): `V`, `E_p`
         * application domain: generic
         * name: PotentialEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: for conservative force `vec(F)`, scalar additive quantity obeying condition `vec(F) = -nabla F`, if it exists
         * remarks: For the definition of energy, see ISO 80000-5. A force is conservative when the force field is irrotational, i.e. `rot(F) = 0` , or `vec(F)` is perpendicular to the speed of the body to ensure `vec(F) * d vec(r) = 0` .
         */
    }

    /* ISO-80000-4 item 4-28.2 kinetic energy */
    attribute kineticEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 4-28.2 kinetic energy
         * symbol(s): `T`, `E_k`
         * application domain: generic
         * name: KineticEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing a moving body expressed by: `T = 1/2 m v^2` where `m` is mass (item 4-1) of the body and `v` is its speed (ISO 80000-3)
         * remarks: For the definition of energy, see ISO 80000-5.
         */
    }

    /* ISO-80000-4 item 4-28.3 mechanical energy */
    attribute mechanicalEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 4-28.3 mechanical energy
         * symbol(s): `E`, `W`
         * application domain: generic
         * name: MechanicalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of kinetic energy `T` (item 4-28.2) and potential energy `V` (item 4-28.1): `E = T+V`
         * remarks: The symbols `E` and `W` are also used for other kinds of energy. This definition is understood in a classical way and it does not include thermal motion.
         */
    }

    /* ISO-80000-4 item 4-28.4 mechanical work, work */
    attribute mechanicalWork : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 4-28.4 mechanical work, work
         * symbol(s): `A`, `W`
         * application domain: generic
         * name: MechanicalWork (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: process quantity describing the total action of a force `vec(F)` (item 4-9.1) along a continuous curve `Γ` in three-dimensional space with infinitesimal displacement (ISO 80000-3) `dvec(r)`, as a line integral of their scalar product: `A = int_Γ vec(F) * d vec(r)`
         * remarks: The definition covers the case `A = -int_Γ p*dV` where `Γ` is a curve in the phase space and implies that work generally depends upon `Γ`, and that type of process must be defined (e.g. isentropic or isothermic).
         */
    }

    alias work for mechanicalWork;

    /* ISO-80000-4 item 4-29 mechanical efficiency */
    attribute def MechanicalEfficiencyValue :> DimensionOneValue {
        doc /*
         * source: item 4-29 mechanical efficiency
         * symbol(s): `η`
         * application domain: mechanics
         * name: MechanicalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of output power `P_"out"` (item 4-27) from a system and input power `P_"in"` (item 4-27) to this system: `η = P_"out"/P_"in"`
         * remarks: The system must be specified. This quantity is often expressed by the unit percent, symbol %.
         */
    }
    attribute mechanicalEfficiency : MechanicalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-4 item 4-30.1 mass flow */
    attribute def MassFlowValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-30.1 mass flow (magnitude)
         * symbol(s): `j_m`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassFlowUnit [1];
    }

    attribute massFlow : MassFlowValue :> scalarQuantities [*] nonunique;

    attribute def MassFlowUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def CartesianMassFlow3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 4-30.1 mass flow (vector)
         * symbol(s): `vec(j_m)`
         * application domain: generic
         * name: MassFlow
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): kg*m^-2*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity characterizing a flowing fluid by the product of its local mass density `ρ` (item 4-2) and local velocity `vec(v)` (ISO 80000-3): `vec(j_m) = ρ vec(v)`
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMassFlow3dCoordinateFrame [1];
    }

    attribute cartesianMassFlow3dVector : CartesianMassFlow3dVector :> vectorQuantities;

    attribute def CartesianMassFlow3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MassFlowUnit [3];
    }

    /* ISO-80000-4 item 4-30.2 mass flow rate */
    attribute def MassFlowRateValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-30.2 mass flow rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassFlowRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with normal vector `vec(e)_n` of a flowing fluid with mass flow `vec(j)_m` (item 4-30.1) as an integral: `q_m = int int_A vec(j)_m * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassFlowRateUnit [1];
    }

    attribute massFlowRate : MassFlowRateValue :> scalarQuantities [*] nonunique;

    attribute def MassFlowRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-30.3 mass change rate */
    attribute def MassChangeRateValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-30.3 mass change rate
         * symbol(s): `q_m`
         * application domain: generic
         * name: MassChangeRate
         * quantity dimension: M^1*T^-1
         * measurement unit(s): kg*s^-1
         * tensor order: 0
         * definition: rate of increment of mass `m` (item 4-1): `q_m = (dm)/(dt)` where `dm` is the infinitesimal mass (item 4-1) increment and `dt` is the infinitesimal duration (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassChangeRateUnit [1];
    }

    attribute massChangeRate : MassChangeRateValue :> scalarQuantities [*] nonunique;

    attribute def MassChangeRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-31 volume flow rate */
    attribute def VolumeFlowRateValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-31 volume flow rate
         * symbol(s): `q_v`
         * application domain: generic
         * name: VolumeFlowRate
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: scalar (ISO 80000-2) quantity characterizing the total flow through the two-dimensional domain `A` with the normal vector `vec(e)_n` of a flowing fluid with velocity `vec(v)` (ISO 80000-3) as an integral: `q_v = int int_A vec(v) * vec(e)_n dA` where `dA` is the area (ISO 80000-3) of an element of the two-dimensional domain `A`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeFlowRateUnit [1];
    }

    attribute volumeFlowRate : VolumeFlowRateValue :> scalarQuantities [*] nonunique;

    attribute def VolumeFlowRateUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-4 item 4-32 action quantity */
    attribute def ActionQuantityValue :> ScalarQuantityValue {
        doc /*
         * source: item 4-32 action quantity
         * symbol(s): `S`
         * application domain: generic
         * name: ActionQuantity
         * quantity dimension: L^2*M^1*T^-1
         * measurement unit(s): J*s, kg*m^2*s^-1
         * tensor order: 0
         * definition: time integral of energy `E` over a time interval `(t_1, t_2)`: `S = int_(t_1)^(t_2) E dt`
         * remarks: The energy may be expressed by a Lagrangian or Hamiltonian function. Note for SysML: the ISQ quantity "action" has been renamed to "action quantity" to avoid the name clash with the SysML action keyword.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ActionQuantityUnit [1];
    }

    attribute actionQuantity : ActionQuantityValue :> scalarQuantities [*] nonunique;

    attribute def ActionQuantityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQMechanics"))) (name "ISQMechanics") (declared-name "ISQMechanics")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQMechanics::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQMechanics::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQMechanics::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (name "ActionQuantityUnit") (declared-name "ActionQuantityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (name "ActionQuantityValue") (declared-name "ActionQuantityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (name "AngularImpulseUnit") (declared-name "AngularImpulseUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (name "AngularImpulseValue") (declared-name "AngularImpulseValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (name "AngularMomentumUnit") (declared-name "AngularMomentumUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (name "AngularMomentumValue") (declared-name "AngularMomentumValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::BulkModulusUnit"))) (name "BulkModulusUnit") (declared-name "BulkModulusUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::BulkModulusValue"))) (name "BulkModulusValue") (declared-name "BulkModulusValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (name "Cartesian3dMomentOfInertiaMeasurementReference") (declared-name "Cartesian3dMomentOfInertiaMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (name "Cartesian3dMomentOfInertiaTensor") (declared-name "Cartesian3dMomentOfInertiaTensor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (name "Cartesian3dStrainMeasurementReference") (declared-name "Cartesian3dStrainMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (name "Cartesian3dStrainTensor") (declared-name "Cartesian3dStrainTensor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (name "Cartesian3dStressMeasurementReference") (declared-name "Cartesian3dStressMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (name "Cartesian3dStressTensor") (declared-name "Cartesian3dStressTensor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (name "CartesianAngularImpulse3dCoordinateFrame") (declared-name "CartesianAngularImpulse3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (name "CartesianAngularImpulse3dVector") (declared-name "CartesianAngularImpulse3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (name "CartesianAngularMomentum3dCoordinateFrame") (declared-name "CartesianAngularMomentum3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (name "CartesianAngularMomentum3dVector") (declared-name "CartesianAngularMomentum3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (name "CartesianDragForce3dVector") (declared-name "CartesianDragForce3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (name "CartesianForce3dCoordinateFrame") (declared-name "CartesianForce3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (name "CartesianForce3dVector") (declared-name "CartesianForce3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (name "CartesianImpulse3dCoordinateFrame") (declared-name "CartesianImpulse3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (name "CartesianImpulse3dVector") (declared-name "CartesianImpulse3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (name "CartesianKineticFrictionForce3dVector") (declared-name "CartesianKineticFrictionForce3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (name "CartesianMassFlow3dCoordinateFrame") (declared-name "CartesianMassFlow3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (name "CartesianMassFlow3dVector") (declared-name "CartesianMassFlow3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (name "CartesianMomentOfForce3dCoordinateFrame") (declared-name "CartesianMomentOfForce3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (name "CartesianMomentOfForce3dVector") (declared-name "CartesianMomentOfForce3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (name "CartesianMomentum3dCoordinateFrame") (declared-name "CartesianMomentum3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (name "CartesianMomentum3dVector") (declared-name "CartesianMomentum3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (name "CartesianRollingResistance3dVector") (declared-name "CartesianRollingResistance3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (name "CartesianStaticFrictionForce3dVector") (declared-name "CartesianStaticFrictionForce3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (name "CartesianWeight3dVector") (declared-name "CartesianWeight3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (name "CompressibilityUnit") (declared-name "CompressibilityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (name "CompressibilityValue") (declared-name "CompressibilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::DensityUnit"))) (name "DensityUnit") (declared-name "DensityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::DensityValue"))) (name "DensityValue") (declared-name "DensityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))) (name "DragCoefficientValue") (declared-name "DragCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (name "DynamicViscosityUnit") (declared-name "DynamicViscosityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (name "DynamicViscosityValue") (declared-name "DynamicViscosityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (name "ForceUnit") (declared-name "ForceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (name "ForceValue") (declared-name "ForceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ForceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (name "ImpulseUnit") (declared-name "ImpulseUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (name "ImpulseValue") (declared-name "ImpulseValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (name "KinematicViscosityUnit") (declared-name "KinematicViscosityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (name "KinematicViscosityValue") (declared-name "KinematicViscosityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))) (name "KineticFrictionFactorValue") (declared-name "KineticFrictionFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::LinearDensityUnit"))) (name "LinearDensityUnit") (declared-name "LinearDensityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::LinearDensityValue"))) (name "LinearDensityValue") (declared-name "LinearDensityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (name "LinearMassDensityUnit") (declared-name "LinearMassDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (name "LinearMassDensityValue") (declared-name "LinearMassDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (name "MassChangeRateUnit") (declared-name "MassChangeRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (name "MassChangeRateValue") (declared-name "MassChangeRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (name "MassDensityUnit") (declared-name "MassDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (name "MassDensityValue") (declared-name "MassDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (name "MassFlowRateUnit") (declared-name "MassFlowRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (name "MassFlowRateValue") (declared-name "MassFlowRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (name "MassFlowUnit") (declared-name "MassFlowUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (name "MassFlowValue") (declared-name "MassFlowValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))) (name "MechanicalEfficiencyValue") (declared-name "MechanicalEfficiencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (name "ModulusOfCompressionUnit") (declared-name "ModulusOfCompressionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (name "ModulusOfCompressionValue") (declared-name "ModulusOfCompressionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (name "ModulusOfElasticityUnit") (declared-name "ModulusOfElasticityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (name "ModulusOfElasticityValue") (declared-name "ModulusOfElasticityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (name "ModulusOfRigidityUnit") (declared-name "ModulusOfRigidityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (name "ModulusOfRigidityValue") (declared-name "ModulusOfRigidityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (name "MomentOfForceUnit") (declared-name "MomentOfForceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (name "MomentOfForceValue") (declared-name "MomentOfForceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (name "MomentOfInertiaUnit") (declared-name "MomentOfInertiaUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (name "MomentOfInertiaValue") (declared-name "MomentOfInertiaValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (name "MomentumUnit") (declared-name "MomentumUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (name "MomentumValue") (declared-name "MomentumValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::MomentumValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (name "NormalStressUnit") (declared-name "NormalStressUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (name "NormalStressValue") (declared-name "NormalStressValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))) (name "PoissonNumberValue") (declared-name "PoissonNumberValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (name "PowerUnit") (declared-name "PowerUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (name "PowerValue") (declared-name "PowerValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PowerValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (name "PressureUnit") (declared-name "PressureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (name "PressureValue") (declared-name "PressureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::PressureValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQMechanics::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))) (name "RelativeLinearStrainValue") (declared-name "RelativeLinearStrainValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))) (name "RelativeMassDensityValue") (declared-name "RelativeMassDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))) (name "RelativeVolumeStrainValue") (declared-name "RelativeVolumeStrainValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))) (name "RollingResistanceFactorValue") (declared-name "RollingResistanceFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (name "SecondAxialMomentOfAreaUnit") (declared-name "SecondAxialMomentOfAreaUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (name "SecondAxialMomentOfAreaValue") (declared-name "SecondAxialMomentOfAreaValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (name "SecondPolarMomentOfAreaUnit") (declared-name "SecondPolarMomentOfAreaUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (name "SecondPolarMomentOfAreaValue") (declared-name "SecondPolarMomentOfAreaValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (name "SectionModulusUnit") (declared-name "SectionModulusUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (name "SectionModulusValue") (declared-name "SectionModulusValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::ShearModulusUnit"))) (name "ShearModulusUnit") (declared-name "ShearModulusUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::ShearModulusValue"))) (name "ShearModulusValue") (declared-name "ShearModulusValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))) (name "ShearStrainValue") (declared-name "ShearStrainValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (name "ShearStressUnit") (declared-name "ShearStressUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (name "ShearStressValue") (declared-name "ShearStressValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (name "SpecificVolumeUnit") (declared-name "SpecificVolumeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (name "SpecificVolumeValue") (declared-name "SpecificVolumeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))) (name "StaticFrictionCoefficientValue") (declared-name "StaticFrictionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))) (name "StrainUnit") (declared-name "StrainUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (name "StrainValue") (declared-name "StrainValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StrainValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StrainValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StrainValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (name "StressUnit") (declared-name "StressUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (name "StressValue") (declared-name "StressValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::StressValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceDensityUnit"))) (name "SurfaceDensityUnit") (declared-name "SurfaceDensityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceDensityValue"))) (name "SurfaceDensityValue") (declared-name "SurfaceDensityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (name "SurfaceMassDensityUnit") (declared-name "SurfaceMassDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (name "SurfaceMassDensityValue") (declared-name "SurfaceMassDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (name "SurfaceTensionUnit") (declared-name "SurfaceTensionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (name "SurfaceTensionValue") (declared-name "SurfaceTensionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (name "TorqueUnit") (declared-name "TorqueUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (name "TorqueValue") (declared-name "TorqueValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::TorqueValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::ViscosityUnit"))) (name "ViscosityUnit") (declared-name "ViscosityUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::ViscosityValue"))) (name "ViscosityValue") (declared-name "ViscosityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (name "VolumeFlowRateUnit") (declared-name "VolumeFlowRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (name "VolumeFlowRateValue") (declared-name "VolumeFlowRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::YoungModulusUnit"))) (name "YoungModulusUnit") (declared-name "YoungModulusUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::YoungModulusValue"))) (name "YoungModulusValue") (declared-name "YoungModulusValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (name "actionQuantity") (declared-name "actionQuantity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (name "angularImpulse") (declared-name "angularImpulse") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (name "angularMomentum") (declared-name "angularMomentum") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::bulkModulus"))) (name "bulkModulus") (declared-name "bulkModulus"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (name "cartesianAngularImpulse3dVector") (declared-name "cartesianAngularImpulse3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (name "cartesianAngularMomentum3dVector") (declared-name "cartesianAngularMomentum3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (name "cartesianDragForce3dVector") (declared-name "cartesianDragForce3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianDynamicFrictionForce3dVector"))) (name "cartesianDynamicFrictionForce3dVector") (declared-name "cartesianDynamicFrictionForce3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (name "cartesianForce3dVector") (declared-name "cartesianForce3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (name "cartesianImpulse3dVector") (declared-name "cartesianImpulse3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (name "cartesianKineticFrictionForce3dVector") (declared-name "cartesianKineticFrictionForce3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (name "cartesianMassFlow3dVector") (declared-name "cartesianMassFlow3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (name "cartesianMomentOfForce3dVector") (declared-name "cartesianMomentOfForce3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (name "cartesianMomentum3dVector") (declared-name "cartesianMomentum3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingDrag3dVector"))) (name "cartesianRollingDrag3dVector") (declared-name "cartesianRollingDrag3dVector"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingFrictionForce3dVector"))) (name "cartesianRollingFrictionForce3dVector") (declared-name "cartesianRollingFrictionForce3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (name "cartesianRollingResistance3dVector") (declared-name "cartesianRollingResistance3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFriction3dVector"))) (name "cartesianStaticFriction3dVector") (declared-name "cartesianStaticFriction3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (name "cartesianStaticFrictionForce3dVector") (declared-name "cartesianStaticFrictionForce3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (name "cartesianWeight3dVector") (declared-name "cartesianWeight3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::coefficientOfStaticFriction"))) (name "coefficientOfStaticFriction") (declared-name "coefficientOfStaticFriction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (name "compressibility") (declared-name "compressibility") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::density"))) (name "density") (declared-name "density"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (name "dragCoefficient") (declared-name "dragCoefficient") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::dragFactor"))) (name "dragFactor") (declared-name "dragFactor"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::dynamicFrictionFactor"))) (name "dynamicFrictionFactor") (declared-name "dynamicFrictionFactor"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (name "dynamicViscosity") (declared-name "dynamicViscosity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::force"))) (name "force") (declared-name "force") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (name "gaugePressure") (declared-name "gaugePressure") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::gaugePressure::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::gaugePressure")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (name "impulse") (declared-name "impulse") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (name "kinematicViscosity") (declared-name "kinematicViscosity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))) (name "kineticEnergy") (declared-name "kineticEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (name "kineticFrictionFactor") (declared-name "kineticFrictionFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::linearDensity"))) (name "linearDensity") (declared-name "linearDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (name "linearMassDensity") (declared-name "linearMassDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (name "massChangeRate") (declared-name "massChangeRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (name "massDensity") (declared-name "massDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (name "massFlow") (declared-name "massFlow") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (name "massFlowRate") (declared-name "massFlowRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (name "mechanicalEfficiency") (declared-name "mechanicalEfficiency") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))) (name "mechanicalEnergy") (declared-name "mechanicalEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (name "mechanicalPower") (declared-name "mechanicalPower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))) (name "mechanicalWork") (declared-name "mechanicalWork") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (name "modulusOfCompression") (declared-name "modulusOfCompression") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (name "modulusOfElasticity") (declared-name "modulusOfElasticity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (name "modulusOfRigidity") (declared-name "modulusOfRigidity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (name "momentOfForce") (declared-name "momentOfForce") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (name "momentOfInertia") (declared-name "momentOfInertia") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (name "momentOfInertiaTensor") (declared-name "momentOfInertiaTensor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (name "momentum") (declared-name "momentum") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (name "normalStress") (declared-name "normalStress") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (name "poissonNumber") (declared-name "poissonNumber") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))) (name "potentialEnergy") (declared-name "potentialEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::power"))) (name "power") (declared-name "power") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (name "pressure") (declared-name "pressure") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::relativeDensity"))) (name "relativeDensity") (declared-name "relativeDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (name "relativeLinearStrain") (declared-name "relativeLinearStrain") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (name "relativeMassDensity") (declared-name "relativeMassDensity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (name "relativeVolumeStrain") (declared-name "relativeVolumeStrain") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (name "rollingResistanceFactor") (declared-name "rollingResistanceFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (name "secondAxialMomentOfArea") (declared-name "secondAxialMomentOfArea") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (name "secondPolarMomentOfArea") (declared-name "secondPolarMomentOfArea") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (name "sectionModulus") (declared-name "sectionModulus") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::shearModulus"))) (name "shearModulus") (declared-name "shearModulus"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (name "shearStrain") (declared-name "shearStrain") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (name "shearStress") (declared-name "shearStress") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (name "specificVolume") (declared-name "specificVolume") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (name "staticFrictionCoefficient") (declared-name "staticFrictionCoefficient") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::staticFrictionFactor"))) (name "staticFrictionFactor") (declared-name "staticFrictionFactor"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::strain"))) (name "strain") (declared-name "strain") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (name "strainTensor") (declared-name "strainTensor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::stress"))) (name "stress") (declared-name "stress") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (name "stressTensor") (declared-name "stressTensor") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::surfaceDensity"))) (name "surfaceDensity") (declared-name "surfaceDensity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (name "surfaceMassDensity") (declared-name "surfaceMassDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (name "surfaceTension") (declared-name "surfaceTension") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::torque"))) (name "torque") (declared-name "torque") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::viscosity"))) (name "viscosity") (declared-name "viscosity"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (name "volumeFlowRate") (declared-name "volumeFlowRate") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::work"))) (name "work") (declared-name "work"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQMechanics::youngModulus"))) (name "youngModulus") (declared-name "youngModulus"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ForceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::PowerValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::PressureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::StrainValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::StressValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::StressValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::gaugePressure::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (to (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (to (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (to (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (to (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (to (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (to (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::force"))) (to (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (to (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (to (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (to (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (to (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (to (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (to (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (to (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (to (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (to (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (to (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (to (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (to (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::power"))) (to (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (to (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (to (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (to (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (to (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (to (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (to (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (to (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (to (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (to (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::strain"))) (to (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::stress"))) (to (node (document "d0") (qualified-name "ISQMechanics::StressValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (to (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (to (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::torque"))) (to (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (to (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
