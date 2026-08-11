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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_mechanics.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 19) (end 20 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 4) (end 26 718))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 45 4) (end 45 352))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 46 8) (end 46 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 8) (end 47 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 541))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 4) (end 75 355))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 8) (end 76 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 8) (end 77 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 4) (end 82 631))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 4) (end 101 754))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 4) (end 120 359))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 8) (end 121 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 8) (end 122 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 4) (end 131 700))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 4) (end 150 358))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 8) (end 151 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 8) (end 152 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 4) (end 161 871))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 4) (end 180 355))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 181 8) (end 181 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 182 8) (end 182 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 186 4) (end 186 959))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 4) (end 206 237))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 4) (end 213 598))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 4) (end 232 466))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 233 8) (end 233 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 234 8) (end 234 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 235 8) (end 235 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 239 4) (end 239 644))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 258 4) (end 258 211))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 265 4) (end 265 551))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 284 4) (end 284 463))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 285 8) (end 285 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 8) (end 286 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 287 8) (end 287 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 291 4) (end 291 597))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 310 4) (end 310 205))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 317 4) (end 317 1025))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 337 4) (end 337 721))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 359 4) (end 359 717))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 381 4) (end 381 724))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 4) (end 405 640))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 425 4) (end 425 842))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 444 4) (end 444 465))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 445 8) (end 445 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 446 8) (end 446 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 447 8) (end 447 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 451 4) (end 451 888))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 470 4) (end 470 209))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 477 4) (end 477 721))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 496 4) (end 496 473))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 497 8) (end 497 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 498 8) (end 498 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 499 8) (end 499 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 503 4) (end 503 767))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 522 4) (end 522 225))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 529 4) (end 529 768))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 548 4) (end 548 471))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 549 8) (end 549 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 550 8) (end 550 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 551 8) (end 551 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 555 4) (end 555 814))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 574 4) (end 574 221))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 581 4) (end 581 780))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 600 4) (end 600 464))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 601 8) (end 601 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 602 8) (end 602 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 603 8) (end 603 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 608 4) (end 608 896))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 627 4) (end 627 472))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 628 8) (end 628 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 629 8) (end 629 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 630 8) (end 630 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 634 4) (end 634 942))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 653 4) (end 653 223))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 660 4) (end 660 701))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 679 4) (end 679 467))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 680 8) (end 680 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 681 8) (end 681 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 682 8) (end 682 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 703 4) (end 703 653))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 722 4) (end 722 465))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 723 8) (end 723 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 724 8) (end 724 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 725 8) (end 725 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 729 4) (end 729 741))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 749 4) (end 749 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 756 4) (end 756 966))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 775 4) (end 775 471))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 776 8) (end 776 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 777 8) (end 777 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 778 8) (end 778 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 783 4) (end 783 965))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 802 4) (end 802 470))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 803 8) (end 803 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 804 8) (end 804 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 805 8) (end 805 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 810 4) (end 810 636))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 829 4) (end 829 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 832 4) (end 832 724))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 852 4) (end 852 219))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 859 4) (end 859 561))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 876 4) (end 876 561))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 893 4) (end 893 558))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 910 4) (end 910 589))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 927 4) (end 927 726))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 946 4) (end 946 478))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 947 8) (end 947 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 948 8) (end 948 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 949 8) (end 949 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 958 4) (end 958 696))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 977 4) (end 977 476))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 978 8) (end 978 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 979 8) (end 979 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 980 8) (end 980 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 989 4) (end 989 742))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1008 4) (end 1008 479))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1009 8) (end 1009 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1010 8) (end 1010 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1011 8) (end 1011 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1020 4) (end 1020 728))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1039 4) (end 1039 473))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1040 8) (end 1040 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1041 8) (end 1041 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1042 8) (end 1042 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1047 4) (end 1047 962))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1066 4) (end 1066 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1067 8) (end 1067 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1072 4) (end 1072 978))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1091 4) (end 1091 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1092 8) (end 1092 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1097 4) (end 1097 777))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1116 4) (end 1116 243))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1117 8) (end 1117 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1122 4) (end 1122 960))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1143 4) (end 1143 891))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1162 4) (end 1162 710))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1179 4) (end 1179 742))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1198 4) (end 1198 755))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1217 4) (end 1217 475))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1218 8) (end 1218 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1219 8) (end 1219 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1220 8) (end 1220 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1229 4) (end 1229 606))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1248 4) (end 1248 367))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1249 8) (end 1249 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1250 8) (end 1250 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1255 4) (end 1255 712))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1274 4) (end 1274 359))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1275 8) (end 1275 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1276 8) (end 1276 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1281 4) (end 1281 529))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1300 4) (end 1300 463))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1301 8) (end 1301 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1302 8) (end 1302 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1303 8) (end 1303 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1390 4) (end 1390 666))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1407 4) (end 1407 671))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1426 4) (end 1426 467))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1427 8) (end 1427 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1428 8) (end 1428 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1429 8) (end 1429 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1433 4) (end 1433 717))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1452 4) (end 1452 211))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1459 4) (end 1459 812))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1478 4) (end 1478 357))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1479 8) (end 1479 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1480 8) (end 1480 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1485 4) (end 1485 661))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1504 4) (end 1504 359))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1505 8) (end 1505 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1506 8) (end 1506 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1511 4) (end 1511 818))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1530 4) (end 1530 363))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1531 8) (end 1531 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1532 8) (end 1532 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1537 4) (end 1537 786))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1556 4) (end 1556 472))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1557 8) (end 1557 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1558 8) (end 1558 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1559 8) (end 1559 105))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8f066cce5362184af521631e27efa39be2935fb945782f2343a96343b2b7b2cd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQMechanics"))) (kind "package") (name "ISQMechanics") (declared-name "ISQMechanics") (range (start (line 0) (character 0)) (end (line 0) (character 73848))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (kind "attribute def") (name "ActionQuantityUnit") (declared-name "ActionQuantityUnit") (range (start (line 1556) (character 4)) (end (line 1556) (character 472))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1559) (character 8)) (end (line 1559) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1557) (character 8)) (end (line 1557) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1558) (character 8)) (end (line 1558) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1560) (character 8)) (end (line 1560) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1560) (character 22)) (end (line 1560) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (kind "attribute def") (name "ActionQuantityValue") (declared-name "ActionQuantityValue") (range (start (line 1537) (character 4)) (end (line 1537) (character 786))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1537) (character 4)) (end (line 1537) (character 786))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1551) (character 8)) (end (line 1551) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ActionQuantityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1551) (character 22)) (end (line 1551) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1550) (character 8)) (end (line 1550) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1550) (character 22)) (end (line 1550) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (kind "attribute def") (name "AngularImpulseUnit") (declared-name "AngularImpulseUnit") (range (start (line 627) (character 4)) (end (line 627) (character 472))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 630) (character 8)) (end (line 630) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 628) (character 8)) (end (line 628) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 629) (character 8)) (end (line 629) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 631) (character 8)) (end (line 631) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 631) (character 22)) (end (line 631) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (kind "attribute def") (name "AngularImpulseValue") (declared-name "AngularImpulseValue") (range (start (line 608) (character 4)) (end (line 608) (character 896))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::_documentation"))) (kind "documentation") (name "") (range (start (line 608) (character 4)) (end (line 608) (character 896))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 622) (character 8)) (end (line 622) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularImpulseUnit") (range none)) (redefinition (reference "mRef") (range (start (line 622) (character 22)) (end (line 622) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 621) (character 8)) (end (line 621) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 621) (character 22)) (end (line 621) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (kind "attribute def") (name "AngularMomentumUnit") (declared-name "AngularMomentumUnit") (range (start (line 496) (character 4)) (end (line 496) (character 473))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 499) (character 8)) (end (line 499) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 497) (character 8)) (end (line 497) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 498) (character 8)) (end (line 498) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 500) (character 8)) (end (line 500) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 500) (character 22)) (end (line 500) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (kind "attribute def") (name "AngularMomentumValue") (declared-name "AngularMomentumValue") (range (start (line 477) (character 4)) (end (line 477) (character 721))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::_documentation"))) (kind "documentation") (name "") (range (start (line 477) (character 4)) (end (line 477) (character 721))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 491) (character 8)) (end (line 491) (character 51))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMomentumUnit") (range none)) (redefinition (reference "mRef") (range (start (line 491) (character 22)) (end (line 491) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 490) (character 8)) (end (line 490) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 490) (character 22)) (end (line 490) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::BulkModulusUnit"))) (kind "alias") (name "BulkModulusUnit") (declared-name "BulkModulusUnit") (range (start (line 1015) (character 4)) (end (line 1015) (character 55))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::BulkModulusValue"))) (kind "alias") (name "BulkModulusValue") (declared-name "BulkModulusValue") (range (start (line 1016) (character 4)) (end (line 1016) (character 57))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (kind "attribute def") (name "Cartesian3dMomentOfInertiaMeasurementReference") (declared-name "Cartesian3dMomentOfInertiaMeasurementReference") (range (start (line 206) (character 4)) (end (line 206) (character 237))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorMeasurementReference") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (range (start (line 207) (character 8)) (end (line 207) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions") (range (start (line 207) (character 22)) (end (line 207) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 208) (character 8)) (end (line 208) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 208) (character 22)) (end (line 208) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 209) (character 8)) (end (line 209) (character 52))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentOfInertiaUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 209) (character 22)) (end (line 209) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (kind "attribute def") (name "Cartesian3dMomentOfInertiaTensor") (declared-name "Cartesian3dMomentOfInertiaTensor") (range (start (line 186) (character 4)) (end (line 186) (character 959))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::_documentation"))) (kind "documentation") (name "") (range (start (line 186) (character 4)) (end (line 186) (character 959))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 199) (character 8)) (end (line 199) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 199) (character 22)) (end (line 199) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 201) (character 8)) (end (line 201) (character 78))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cartesian3dMomentOfInertiaMeasurementReference") (range none)) (redefinition (reference "mRef") (range (start (line 201) (character 22)) (end (line 201) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 200) (character 8)) (end (line 200) (character 35))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 200) (character 22)) (end (line 200) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (kind "attribute def") (name "Cartesian3dStrainMeasurementReference") (declared-name "Cartesian3dStrainMeasurementReference") (range (start (line 852) (character 4)) (end (line 852) (character 219))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorMeasurementReference") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (range (start (line 853) (character 8)) (end (line 853) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions") (range (start (line 853) (character 22)) (end (line 853) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 854) (character 8)) (end (line 854) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 854) (character 22)) (end (line 854) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 855) (character 8)) (end (line 855) (character 43))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "StrainUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 855) (character 22)) (end (line 855) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (kind "attribute def") (name "Cartesian3dStrainTensor") (declared-name "Cartesian3dStrainTensor") (range (start (line 832) (character 4)) (end (line 832) (character 724))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::_documentation"))) (kind "documentation") (name "") (range (start (line 832) (character 4)) (end (line 832) (character 724))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 845) (character 8)) (end (line 845) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 845) (character 22)) (end (line 845) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 847) (character 8)) (end (line 847) (character 69))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cartesian3dStrainMeasurementReference") (range none)) (redefinition (reference "mRef") (range (start (line 847) (character 22)) (end (line 847) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 846) (character 8)) (end (line 846) (character 35))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 846) (character 22)) (end (line 846) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (kind "attribute def") (name "Cartesian3dStressMeasurementReference") (declared-name "Cartesian3dStressMeasurementReference") (range (start (line 749) (character 4)) (end (line 749) (character 219))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorMeasurementReference") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (range (start (line 750) (character 8)) (end (line 750) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions") (range (start (line 750) (character 22)) (end (line 750) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 751) (character 8)) (end (line 751) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 751) (character 22)) (end (line 751) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 752) (character 8)) (end (line 752) (character 43))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "StressUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 752) (character 22)) (end (line 752) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (kind "attribute def") (name "Cartesian3dStressTensor") (declared-name "Cartesian3dStressTensor") (range (start (line 729) (character 4)) (end (line 729) (character 741))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::_documentation"))) (kind "documentation") (name "") (range (start (line 729) (character 4)) (end (line 729) (character 741))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 742) (character 8)) (end (line 742) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 742) (character 22)) (end (line 742) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 744) (character 8)) (end (line 744) (character 69))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cartesian3dStressMeasurementReference") (range none)) (redefinition (reference "mRef") (range (start (line 744) (character 22)) (end (line 744) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 743) (character 8)) (end (line 743) (character 35))) (parent (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 743) (character 22)) (end (line 743) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularImpulse3dCoordinateFrame") (declared-name "CartesianAngularImpulse3dCoordinateFrame") (range (start (line 653) (character 4)) (end (line 653) (character 223))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 654) (character 8)) (end (line 654) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 654) (character 22)) (end (line 654) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 655) (character 8)) (end (line 655) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 655) (character 22)) (end (line 655) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 656) (character 8)) (end (line 656) (character 51))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularImpulseUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 656) (character 22)) (end (line 656) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (kind "attribute def") (name "CartesianAngularImpulse3dVector") (declared-name "CartesianAngularImpulse3dVector") (range (start (line 634) (character 4)) (end (line 634) (character 942))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 634) (character 4)) (end (line 634) (character 942))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 647) (character 8)) (end (line 647) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 647) (character 22)) (end (line 647) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 648) (character 8)) (end (line 648) (character 72))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularImpulse3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 648) (character 22)) (end (line 648) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (kind "attribute def") (name "CartesianAngularMomentum3dCoordinateFrame") (declared-name "CartesianAngularMomentum3dCoordinateFrame") (range (start (line 522) (character 4)) (end (line 522) (character 225))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 523) (character 8)) (end (line 523) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 523) (character 22)) (end (line 523) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 524) (character 8)) (end (line 524) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 524) (character 22)) (end (line 524) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 525) (character 8)) (end (line 525) (character 52))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMomentumUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 525) (character 22)) (end (line 525) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (kind "attribute def") (name "CartesianAngularMomentum3dVector") (declared-name "CartesianAngularMomentum3dVector") (range (start (line 503) (character 4)) (end (line 503) (character 767))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 503) (character 4)) (end (line 503) (character 767))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 516) (character 8)) (end (line 516) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 516) (character 22)) (end (line 516) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 517) (character 8)) (end (line 517) (character 73))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianAngularMomentum3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 517) (character 22)) (end (line 517) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (kind "attribute def") (name "CartesianDragForce3dVector") (declared-name "CartesianDragForce3dVector") (range (start (line 405) (character 4)) (end (line 405) (character 640))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 405) (character 4)) (end (line 405) (character 640))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 418) (character 8)) (end (line 418) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 418) (character 22)) (end (line 418) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 419) (character 8)) (end (line 419) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 419) (character 22)) (end (line 419) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (kind "attribute def") (name "CartesianForce3dCoordinateFrame") (declared-name "CartesianForce3dCoordinateFrame") (range (start (line 310) (character 4)) (end (line 310) (character 205))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 311) (character 8)) (end (line 311) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 311) (character 22)) (end (line 311) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 312) (character 8)) (end (line 312) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 312) (character 22)) (end (line 312) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 313) (character 8)) (end (line 313) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForceUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 313) (character 22)) (end (line 313) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (kind "attribute def") (name "CartesianForce3dVector") (declared-name "CartesianForce3dVector") (range (start (line 291) (character 4)) (end (line 291) (character 597))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 291) (character 4)) (end (line 291) (character 597))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 304) (character 8)) (end (line 304) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 304) (character 22)) (end (line 304) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 305) (character 8)) (end (line 305) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 305) (character 22)) (end (line 305) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (kind "attribute def") (name "CartesianImpulse3dCoordinateFrame") (declared-name "CartesianImpulse3dCoordinateFrame") (range (start (line 470) (character 4)) (end (line 470) (character 209))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 471) (character 8)) (end (line 471) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 471) (character 22)) (end (line 471) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 472) (character 8)) (end (line 472) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 472) (character 22)) (end (line 472) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 473) (character 8)) (end (line 473) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "ImpulseUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 473) (character 22)) (end (line 473) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (kind "attribute def") (name "CartesianImpulse3dVector") (declared-name "CartesianImpulse3dVector") (range (start (line 451) (character 4)) (end (line 451) (character 888))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 451) (character 4)) (end (line 451) (character 888))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 464) (character 8)) (end (line 464) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 464) (character 22)) (end (line 464) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 465) (character 8)) (end (line 465) (character 65))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianImpulse3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 465) (character 22)) (end (line 465) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (kind "attribute def") (name "CartesianKineticFrictionForce3dVector") (declared-name "CartesianKineticFrictionForce3dVector") (range (start (line 359) (character 4)) (end (line 359) (character 717))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 359) (character 4)) (end (line 359) (character 717))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 372) (character 8)) (end (line 372) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 372) (character 22)) (end (line 372) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 373) (character 8)) (end (line 373) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 373) (character 22)) (end (line 373) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (kind "attribute def") (name "CartesianMassFlow3dCoordinateFrame") (declared-name "CartesianMassFlow3dCoordinateFrame") (range (start (line 1452) (character 4)) (end (line 1452) (character 211))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 1453) (character 8)) (end (line 1453) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 1453) (character 22)) (end (line 1453) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 1454) (character 8)) (end (line 1454) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 1454) (character 22)) (end (line 1454) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 1455) (character 8)) (end (line 1455) (character 45))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFlowUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 1455) (character 22)) (end (line 1455) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (kind "attribute def") (name "CartesianMassFlow3dVector") (declared-name "CartesianMassFlow3dVector") (range (start (line 1433) (character 4)) (end (line 1433) (character 717))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 1433) (character 4)) (end (line 1433) (character 717))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 1446) (character 8)) (end (line 1446) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 1446) (character 22)) (end (line 1446) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1447) (character 8)) (end (line 1447) (character 66))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianMassFlow3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 1447) (character 22)) (end (line 1447) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (kind "attribute def") (name "CartesianMomentOfForce3dCoordinateFrame") (declared-name "CartesianMomentOfForce3dCoordinateFrame") (range (start (line 574) (character 4)) (end (line 574) (character 221))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 575) (character 8)) (end (line 575) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 575) (character 22)) (end (line 575) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 576) (character 8)) (end (line 576) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 576) (character 22)) (end (line 576) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 577) (character 8)) (end (line 577) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentOfForceUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 577) (character 22)) (end (line 577) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (kind "attribute def") (name "CartesianMomentOfForce3dVector") (declared-name "CartesianMomentOfForce3dVector") (range (start (line 555) (character 4)) (end (line 555) (character 814))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 555) (character 4)) (end (line 555) (character 814))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 568) (character 8)) (end (line 568) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 568) (character 22)) (end (line 568) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 569) (character 8)) (end (line 569) (character 71))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianMomentOfForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 569) (character 22)) (end (line 569) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (kind "attribute def") (name "CartesianMomentum3dCoordinateFrame") (declared-name "CartesianMomentum3dCoordinateFrame") (range (start (line 258) (character 4)) (end (line 258) (character 211))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dCoordinateFrame") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 259) (character 8)) (end (line 259) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 259) (character 22)) (end (line 259) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (range (start (line 260) (character 8)) (end (line 260) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal") (range (start (line 260) (character 22)) (end (line 260) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (range (start (line 261) (character 8)) (end (line 261) (character 45))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentumUnit") (range none)) (redefinition (reference "mRefs") (range (start (line 261) (character 22)) (end (line 261) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (kind "attribute def") (name "CartesianMomentum3dVector") (declared-name "CartesianMomentum3dVector") (range (start (line 239) (character 4)) (end (line 239) (character 644))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 239) (character 4)) (end (line 239) (character 644))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 252) (character 8)) (end (line 252) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 252) (character 22)) (end (line 252) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 253) (character 8)) (end (line 253) (character 66))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianMomentum3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 253) (character 22)) (end (line 253) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (kind "attribute def") (name "CartesianRollingResistance3dVector") (declared-name "CartesianRollingResistance3dVector") (range (start (line 381) (character 4)) (end (line 381) (character 724))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 381) (character 4)) (end (line 381) (character 724))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 394) (character 8)) (end (line 394) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 394) (character 22)) (end (line 394) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 395) (character 8)) (end (line 395) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 395) (character 22)) (end (line 395) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (kind "attribute def") (name "CartesianStaticFrictionForce3dVector") (declared-name "CartesianStaticFrictionForce3dVector") (range (start (line 337) (character 4)) (end (line 337) (character 721))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 337) (character 4)) (end (line 337) (character 721))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 350) (character 8)) (end (line 350) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 350) (character 22)) (end (line 350) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 351) (character 8)) (end (line 351) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 351) (character 22)) (end (line 351) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (kind "attribute def") (name "CartesianWeight3dVector") (declared-name "CartesianWeight3dVector") (range (start (line 317) (character 4)) (end (line 317) (character 1025))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "3dVectorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::_documentation"))) (kind "documentation") (name "") (range (start (line 317) (character 4)) (end (line 317) (character 1025))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 330) (character 8)) (end (line 330) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound") (range (start (line 330) (character 22)) (end (line 330) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 331) (character 8)) (end (line 331) (character 63))) (parent (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (authored (membership (kind Feature)) (relationships (typing (reference "CartesianForce3dCoordinateFrame") (range none)) (redefinition (reference "mRef") (range (start (line 331) (character 22)) (end (line 331) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (kind "attribute def") (name "CompressibilityUnit") (declared-name "CompressibilityUnit") (range (start (line 1039) (character 4)) (end (line 1039) (character 473))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1042) (character 8)) (end (line 1042) (character 104))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1040) (character 8)) (end (line 1040) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1041) (character 8)) (end (line 1041) (character 101))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1043) (character 8)) (end (line 1043) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1043) (character 22)) (end (line 1043) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (kind "attribute def") (name "CompressibilityValue") (declared-name "CompressibilityValue") (range (start (line 1020) (character 4)) (end (line 1020) (character 728))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1020) (character 4)) (end (line 1020) (character 728))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1034) (character 8)) (end (line 1034) (character 51))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CompressibilityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1034) (character 22)) (end (line 1034) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1033) (character 8)) (end (line 1033) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1033) (character 22)) (end (line 1033) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DensityUnit"))) (kind "alias") (name "DensityUnit") (declared-name "DensityUnit") (range (start (line 51) (character 4)) (end (line 51) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DensityValue"))) (kind "alias") (name "DensityValue") (declared-name "DensityValue") (range (start (line 52) (character 4)) (end (line 52) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))) (kind "attribute def") (name "DragCoefficientValue") (declared-name "DragCoefficientValue") (range (start (line 1179) (character 4)) (end (line 1179) (character 742))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1179) (character 4)) (end (line 1179) (character 742))) (parent (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (kind "attribute def") (name "DynamicViscosityUnit") (declared-name "DynamicViscosityUnit") (range (start (line 1217) (character 4)) (end (line 1217) (character 475))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1220) (character 8)) (end (line 1220) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1218) (character 8)) (end (line 1218) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1219) (character 8)) (end (line 1219) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1221) (character 8)) (end (line 1221) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1221) (character 22)) (end (line 1221) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (kind "attribute def") (name "DynamicViscosityValue") (declared-name "DynamicViscosityValue") (range (start (line 1198) (character 4)) (end (line 1198) (character 755))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1198) (character 4)) (end (line 1198) (character 755))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1212) (character 8)) (end (line 1212) (character 52))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DynamicViscosityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1212) (character 22)) (end (line 1212) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1211) (character 8)) (end (line 1211) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1211) (character 22)) (end (line 1211) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (kind "import") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 20) (character 4)) (end (line 20) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQThermodynamics::EnergyValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 19)) (end (line 20) (character 49))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (kind "attribute def") (name "ForceUnit") (declared-name "ForceUnit") (range (start (line 284) (character 4)) (end (line 284) (character 463))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 287) (character 8)) (end (line 287) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 285) (character 8)) (end (line 285) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 286) (character 8)) (end (line 286) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 288) (character 8)) (end (line 288) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 288) (character 22)) (end (line 288) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (kind "attribute def") (name "ForceValue") (declared-name "ForceValue") (range (start (line 265) (character 4)) (end (line 265) (character 551))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 265) (character 4)) (end (line 265) (character 551))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 279) (character 8)) (end (line 279) (character 41))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 279) (character 22)) (end (line 279) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 278) (character 8)) (end (line 278) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 278) (character 22)) (end (line 278) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (kind "attribute def") (name "ImpulseUnit") (declared-name "ImpulseUnit") (range (start (line 444) (character 4)) (end (line 444) (character 465))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 447) (character 8)) (end (line 447) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 445) (character 8)) (end (line 445) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 446) (character 8)) (end (line 446) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 448) (character 8)) (end (line 448) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 448) (character 22)) (end (line 448) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (kind "attribute def") (name "ImpulseValue") (declared-name "ImpulseValue") (range (start (line 425) (character 4)) (end (line 425) (character 842))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::_documentation"))) (kind "documentation") (name "") (range (start (line 425) (character 4)) (end (line 425) (character 842))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 439) (character 8)) (end (line 439) (character 43))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ImpulseUnit") (range none)) (redefinition (reference "mRef") (range (start (line 439) (character 22)) (end (line 439) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 438) (character 8)) (end (line 438) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 438) (character 22)) (end (line 438) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (kind "attribute def") (name "KinematicViscosityUnit") (declared-name "KinematicViscosityUnit") (range (start (line 1248) (character 4)) (end (line 1248) (character 367))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1250) (character 8)) (end (line 1250) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1249) (character 8)) (end (line 1249) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1251) (character 8)) (end (line 1251) (character 94))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1251) (character 22)) (end (line 1251) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (kind "attribute def") (name "KinematicViscosityValue") (declared-name "KinematicViscosityValue") (range (start (line 1229) (character 4)) (end (line 1229) (character 606))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1229) (character 4)) (end (line 1229) (character 606))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1243) (character 8)) (end (line 1243) (character 54))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "KinematicViscosityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1243) (character 22)) (end (line 1243) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1242) (character 8)) (end (line 1242) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1242) (character 22)) (end (line 1242) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))) (kind "attribute def") (name "KineticFrictionFactorValue") (declared-name "KineticFrictionFactorValue") (range (start (line 1143) (character 4)) (end (line 1143) (character 891))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1143) (character 4)) (end (line 1143) (character 891))) (parent (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearDensityUnit"))) (kind "alias") (name "LinearDensityUnit") (declared-name "LinearDensityUnit") (range (start (line 156) (character 4)) (end (line 156) (character 54))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearDensityValue"))) (kind "alias") (name "LinearDensityValue") (declared-name "LinearDensityValue") (range (start (line 157) (character 4)) (end (line 157) (character 56))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (kind "attribute def") (name "LinearMassDensityUnit") (declared-name "LinearMassDensityUnit") (range (start (line 150) (character 4)) (end (line 150) (character 358))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 151) (character 8)) (end (line 151) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 152) (character 8)) (end (line 152) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 153) (character 8)) (end (line 153) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 153) (character 22)) (end (line 153) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (kind "attribute def") (name "LinearMassDensityValue") (declared-name "LinearMassDensityValue") (range (start (line 131) (character 4)) (end (line 131) (character 700))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 131) (character 4)) (end (line 131) (character 700))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 145) (character 8)) (end (line 145) (character 53))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LinearMassDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 145) (character 22)) (end (line 145) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 144) (character 8)) (end (line 144) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 144) (character 22)) (end (line 144) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (kind "attribute def") (name "MassChangeRateUnit") (declared-name "MassChangeRateUnit") (range (start (line 1504) (character 4)) (end (line 1504) (character 359))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1506) (character 8)) (end (line 1506) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1505) (character 8)) (end (line 1505) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1507) (character 8)) (end (line 1507) (character 92))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1507) (character 22)) (end (line 1507) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (kind "attribute def") (name "MassChangeRateValue") (declared-name "MassChangeRateValue") (range (start (line 1485) (character 4)) (end (line 1485) (character 661))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1485) (character 4)) (end (line 1485) (character 661))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1499) (character 8)) (end (line 1499) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassChangeRateUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1499) (character 22)) (end (line 1499) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1498) (character 8)) (end (line 1498) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1498) (character 22)) (end (line 1498) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (kind "attribute def") (name "MassDensityUnit") (declared-name "MassDensityUnit") (range (start (line 45) (character 4)) (end (line 45) (character 352))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 46) (character 8)) (end (line 46) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 47) (character 8)) (end (line 47) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 48) (character 8)) (end (line 48) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 48) (character 22)) (end (line 48) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (kind "attribute def") (name "MassDensityValue") (declared-name "MassDensityValue") (range (start (line 26) (character 4)) (end (line 26) (character 718))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 4)) (end (line 26) (character 718))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 40) (character 8)) (end (line 40) (character 47))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 40) (character 22)) (end (line 40) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 39) (character 8)) (end (line 39) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 39) (character 22)) (end (line 39) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (kind "attribute def") (name "MassFlowRateUnit") (declared-name "MassFlowRateUnit") (range (start (line 1478) (character 4)) (end (line 1478) (character 357))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1480) (character 8)) (end (line 1480) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1479) (character 8)) (end (line 1479) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1481) (character 8)) (end (line 1481) (character 92))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1481) (character 22)) (end (line 1481) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (kind "attribute def") (name "MassFlowRateValue") (declared-name "MassFlowRateValue") (range (start (line 1459) (character 4)) (end (line 1459) (character 812))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1459) (character 4)) (end (line 1459) (character 812))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1473) (character 8)) (end (line 1473) (character 48))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFlowRateUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1473) (character 22)) (end (line 1473) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1472) (character 8)) (end (line 1472) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1472) (character 22)) (end (line 1472) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (kind "attribute def") (name "MassFlowUnit") (declared-name "MassFlowUnit") (range (start (line 1426) (character 4)) (end (line 1426) (character 467))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1429) (character 8)) (end (line 1429) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1427) (character 8)) (end (line 1427) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1428) (character 8)) (end (line 1428) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1430) (character 8)) (end (line 1430) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1430) (character 22)) (end (line 1430) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (kind "attribute def") (name "MassFlowValue") (declared-name "MassFlowValue") (range (start (line 1407) (character 4)) (end (line 1407) (character 671))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1407) (character 4)) (end (line 1407) (character 671))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1421) (character 8)) (end (line 1421) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFlowUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1421) (character 22)) (end (line 1421) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1420) (character 8)) (end (line 1420) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1420) (character 22)) (end (line 1420) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))) (kind "attribute def") (name "MechanicalEfficiencyValue") (declared-name "MechanicalEfficiencyValue") (range (start (line 1390) (character 4)) (end (line 1390) (character 666))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1390) (character 4)) (end (line 1390) (character 666))) (parent (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (kind "attribute def") (name "ModulusOfCompressionUnit") (declared-name "ModulusOfCompressionUnit") (range (start (line 1008) (character 4)) (end (line 1008) (character 479))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1011) (character 8)) (end (line 1011) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1009) (character 8)) (end (line 1009) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1010) (character 8)) (end (line 1010) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1012) (character 8)) (end (line 1012) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1012) (character 22)) (end (line 1012) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (kind "attribute def") (name "ModulusOfCompressionValue") (declared-name "ModulusOfCompressionValue") (range (start (line 989) (character 4)) (end (line 989) (character 742))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 989) (character 4)) (end (line 989) (character 742))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1003) (character 8)) (end (line 1003) (character 56))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ModulusOfCompressionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1003) (character 22)) (end (line 1003) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1002) (character 8)) (end (line 1002) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1002) (character 22)) (end (line 1002) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (kind "attribute def") (name "ModulusOfElasticityUnit") (declared-name "ModulusOfElasticityUnit") (range (start (line 946) (character 4)) (end (line 946) (character 478))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 949) (character 8)) (end (line 949) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 947) (character 8)) (end (line 947) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 948) (character 8)) (end (line 948) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 950) (character 8)) (end (line 950) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 950) (character 22)) (end (line 950) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (kind "attribute def") (name "ModulusOfElasticityValue") (declared-name "ModulusOfElasticityValue") (range (start (line 927) (character 4)) (end (line 927) (character 726))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 927) (character 4)) (end (line 927) (character 726))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 941) (character 8)) (end (line 941) (character 55))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ModulusOfElasticityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 941) (character 22)) (end (line 941) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 940) (character 8)) (end (line 940) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 940) (character 22)) (end (line 940) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (kind "attribute def") (name "ModulusOfRigidityUnit") (declared-name "ModulusOfRigidityUnit") (range (start (line 977) (character 4)) (end (line 977) (character 476))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 980) (character 8)) (end (line 980) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 978) (character 8)) (end (line 978) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 979) (character 8)) (end (line 979) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 981) (character 8)) (end (line 981) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 981) (character 22)) (end (line 981) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (kind "attribute def") (name "ModulusOfRigidityValue") (declared-name "ModulusOfRigidityValue") (range (start (line 958) (character 4)) (end (line 958) (character 696))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 958) (character 4)) (end (line 958) (character 696))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 972) (character 8)) (end (line 972) (character 53))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ModulusOfRigidityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 972) (character 22)) (end (line 972) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 971) (character 8)) (end (line 971) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 971) (character 22)) (end (line 971) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (kind "attribute def") (name "MomentOfForceUnit") (declared-name "MomentOfForceUnit") (range (start (line 548) (character 4)) (end (line 548) (character 471))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 551) (character 8)) (end (line 551) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 549) (character 8)) (end (line 549) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 550) (character 8)) (end (line 550) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 552) (character 8)) (end (line 552) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 552) (character 22)) (end (line 552) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (kind "attribute def") (name "MomentOfForceValue") (declared-name "MomentOfForceValue") (range (start (line 529) (character 4)) (end (line 529) (character 768))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 529) (character 4)) (end (line 529) (character 768))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 543) (character 8)) (end (line 543) (character 49))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentOfForceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 543) (character 22)) (end (line 543) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 542) (character 8)) (end (line 542) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 542) (character 22)) (end (line 542) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (kind "attribute def") (name "MomentOfInertiaUnit") (declared-name "MomentOfInertiaUnit") (range (start (line 180) (character 4)) (end (line 180) (character 355))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 181) (character 8)) (end (line 181) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 182) (character 8)) (end (line 182) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 183) (character 8)) (end (line 183) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 183) (character 22)) (end (line 183) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (kind "attribute def") (name "MomentOfInertiaValue") (declared-name "MomentOfInertiaValue") (range (start (line 161) (character 4)) (end (line 161) (character 871))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::_documentation"))) (kind "documentation") (name "") (range (start (line 161) (character 4)) (end (line 161) (character 871))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 175) (character 8)) (end (line 175) (character 51))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentOfInertiaUnit") (range none)) (redefinition (reference "mRef") (range (start (line 175) (character 22)) (end (line 175) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 174) (character 8)) (end (line 174) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 174) (character 22)) (end (line 174) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (kind "attribute def") (name "MomentumUnit") (declared-name "MomentumUnit") (range (start (line 232) (character 4)) (end (line 232) (character 466))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 235) (character 8)) (end (line 235) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 233) (character 8)) (end (line 233) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 234) (character 8)) (end (line 234) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 236) (character 8)) (end (line 236) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 236) (character 22)) (end (line 236) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (kind "attribute def") (name "MomentumValue") (declared-name "MomentumValue") (range (start (line 213) (character 4)) (end (line 213) (character 598))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::_documentation"))) (kind "documentation") (name "") (range (start (line 213) (character 4)) (end (line 213) (character 598))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 227) (character 8)) (end (line 227) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MomentumUnit") (range none)) (redefinition (reference "mRef") (range (start (line 227) (character 22)) (end (line 227) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 226) (character 8)) (end (line 226) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 226) (character 22)) (end (line 226) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (kind "attribute def") (name "NormalStressUnit") (declared-name "NormalStressUnit") (range (start (line 775) (character 4)) (end (line 775) (character 471))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 778) (character 8)) (end (line 778) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 776) (character 8)) (end (line 776) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 777) (character 8)) (end (line 777) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 779) (character 8)) (end (line 779) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 779) (character 22)) (end (line 779) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (kind "attribute def") (name "NormalStressValue") (declared-name "NormalStressValue") (range (start (line 756) (character 4)) (end (line 756) (character 966))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::_documentation"))) (kind "documentation") (name "") (range (start (line 756) (character 4)) (end (line 756) (character 966))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 770) (character 8)) (end (line 770) (character 48))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "NormalStressUnit") (range none)) (redefinition (reference "mRef") (range (start (line 770) (character 22)) (end (line 770) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 769) (character 8)) (end (line 769) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 769) (character 22)) (end (line 769) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))) (kind "attribute def") (name "PoissonNumberValue") (declared-name "PoissonNumberValue") (range (start (line 910) (character 4)) (end (line 910) (character 589))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 910) (character 4)) (end (line 910) (character 589))) (parent (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (kind "attribute def") (name "PowerUnit") (declared-name "PowerUnit") (range (start (line 1300) (character 4)) (end (line 1300) (character 463))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1303) (character 8)) (end (line 1303) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1301) (character 8)) (end (line 1301) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1302) (character 8)) (end (line 1302) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1304) (character 8)) (end (line 1304) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1304) (character 22)) (end (line 1304) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (kind "attribute def") (name "PowerValue") (declared-name "PowerValue") (range (start (line 1281) (character 4)) (end (line 1281) (character 529))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1281) (character 4)) (end (line 1281) (character 529))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1295) (character 8)) (end (line 1295) (character 41))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1295) (character 22)) (end (line 1295) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1294) (character 8)) (end (line 1294) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1294) (character 22)) (end (line 1294) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (kind "attribute def") (name "PressureUnit") (declared-name "PressureUnit") (range (start (line 679) (character 4)) (end (line 679) (character 467))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 682) (character 8)) (end (line 682) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 680) (character 8)) (end (line 680) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 681) (character 8)) (end (line 681) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 683) (character 8)) (end (line 683) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 683) (character 22)) (end (line 683) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (kind "attribute def") (name "PressureValue") (declared-name "PressureValue") (range (start (line 660) (character 4)) (end (line 660) (character 701))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 660) (character 4)) (end (line 660) (character 701))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 674) (character 8)) (end (line 674) (character 44))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PressureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 674) (character 22)) (end (line 674) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 673) (character 8)) (end (line 673) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 673) (character 22)) (end (line 673) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))) (kind "attribute def") (name "RelativeLinearStrainValue") (declared-name "RelativeLinearStrainValue") (range (start (line 859) (character 4)) (end (line 859) (character 561))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue::_documentation"))) (kind "documentation") (name "") (range (start (line 859) (character 4)) (end (line 859) (character 561))) (parent (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))) (kind "attribute def") (name "RelativeMassDensityValue") (declared-name "RelativeMassDensityValue") (range (start (line 82) (character 4)) (end (line 82) (character 631))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 82) (character 4)) (end (line 82) (character 631))) (parent (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))) (kind "attribute def") (name "RelativeVolumeStrainValue") (declared-name "RelativeVolumeStrainValue") (range (start (line 893) (character 4)) (end (line 893) (character 558))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue::_documentation"))) (kind "documentation") (name "") (range (start (line 893) (character 4)) (end (line 893) (character 558))) (parent (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))) (kind "attribute def") (name "RollingResistanceFactorValue") (declared-name "RollingResistanceFactorValue") (range (start (line 1162) (character 4)) (end (line 1162) (character 710))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1162) (character 4)) (end (line 1162) (character 710))) (parent (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (kind "attribute def") (name "SecondAxialMomentOfAreaUnit") (declared-name "SecondAxialMomentOfAreaUnit") (range (start (line 1066) (character 4)) (end (line 1066) (character 252))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1067) (character 8)) (end (line 1067) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1068) (character 8)) (end (line 1068) (character 80))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1068) (character 22)) (end (line 1068) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (kind "attribute def") (name "SecondAxialMomentOfAreaValue") (declared-name "SecondAxialMomentOfAreaValue") (range (start (line 1047) (character 4)) (end (line 1047) (character 962))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1047) (character 4)) (end (line 1047) (character 962))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1061) (character 8)) (end (line 1061) (character 59))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SecondAxialMomentOfAreaUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1061) (character 22)) (end (line 1061) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1060) (character 8)) (end (line 1060) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1060) (character 22)) (end (line 1060) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (kind "attribute def") (name "SecondPolarMomentOfAreaUnit") (declared-name "SecondPolarMomentOfAreaUnit") (range (start (line 1091) (character 4)) (end (line 1091) (character 252))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1092) (character 8)) (end (line 1092) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1093) (character 8)) (end (line 1093) (character 80))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1093) (character 22)) (end (line 1093) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (kind "attribute def") (name "SecondPolarMomentOfAreaValue") (declared-name "SecondPolarMomentOfAreaValue") (range (start (line 1072) (character 4)) (end (line 1072) (character 978))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1072) (character 4)) (end (line 1072) (character 978))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1086) (character 8)) (end (line 1086) (character 59))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SecondPolarMomentOfAreaUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1086) (character 22)) (end (line 1086) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1085) (character 8)) (end (line 1085) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1085) (character 22)) (end (line 1085) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (kind "attribute def") (name "SectionModulusUnit") (declared-name "SectionModulusUnit") (range (start (line 1116) (character 4)) (end (line 1116) (character 243))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1117) (character 8)) (end (line 1117) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1118) (character 8)) (end (line 1118) (character 80))) (parent (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1118) (character 22)) (end (line 1118) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (kind "attribute def") (name "SectionModulusValue") (declared-name "SectionModulusValue") (range (start (line 1097) (character 4)) (end (line 1097) (character 777))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1097) (character 4)) (end (line 1097) (character 777))) (parent (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1111) (character 8)) (end (line 1111) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SectionModulusUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1111) (character 22)) (end (line 1111) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1110) (character 8)) (end (line 1110) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1110) (character 22)) (end (line 1110) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearModulusUnit"))) (kind "alias") (name "ShearModulusUnit") (declared-name "ShearModulusUnit") (range (start (line 984) (character 4)) (end (line 984) (character 53))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearModulusValue"))) (kind "alias") (name "ShearModulusValue") (declared-name "ShearModulusValue") (range (start (line 985) (character 4)) (end (line 985) (character 55))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))) (kind "attribute def") (name "ShearStrainValue") (declared-name "ShearStrainValue") (range (start (line 876) (character 4)) (end (line 876) (character 561))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue::_documentation"))) (kind "documentation") (name "") (range (start (line 876) (character 4)) (end (line 876) (character 561))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (kind "attribute def") (name "ShearStressUnit") (declared-name "ShearStressUnit") (range (start (line 802) (character 4)) (end (line 802) (character 470))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 805) (character 8)) (end (line 805) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 803) (character 8)) (end (line 803) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 804) (character 8)) (end (line 804) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 806) (character 8)) (end (line 806) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 806) (character 22)) (end (line 806) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (kind "attribute def") (name "ShearStressValue") (declared-name "ShearStressValue") (range (start (line 783) (character 4)) (end (line 783) (character 965))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::_documentation"))) (kind "documentation") (name "") (range (start (line 783) (character 4)) (end (line 783) (character 965))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 797) (character 8)) (end (line 797) (character 47))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShearStressUnit") (range none)) (redefinition (reference "mRef") (range (start (line 797) (character 22)) (end (line 797) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 796) (character 8)) (end (line 796) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 796) (character 22)) (end (line 796) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (kind "attribute def") (name "SpecificVolumeUnit") (declared-name "SpecificVolumeUnit") (range (start (line 75) (character 4)) (end (line 75) (character 355))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 76) (character 8)) (end (line 76) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 77) (character 8)) (end (line 77) (character 101))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 78) (character 8)) (end (line 78) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 78) (character 22)) (end (line 78) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (kind "attribute def") (name "SpecificVolumeValue") (declared-name "SpecificVolumeValue") (range (start (line 56) (character 4)) (end (line 56) (character 541))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 56) (character 4)) (end (line 56) (character 541))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 70) (character 8)) (end (line 70) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificVolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 70) (character 22)) (end (line 70) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 69) (character 8)) (end (line 69) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 69) (character 22)) (end (line 69) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))) (kind "attribute def") (name "StaticFrictionCoefficientValue") (declared-name "StaticFrictionCoefficientValue") (range (start (line 1122) (character 4)) (end (line 1122) (character 960))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1122) (character 4)) (end (line 1122) (character 960))) (parent (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))) (kind "attribute def") (name "StrainUnit") (declared-name "StrainUnit") (range (start (line 829) (character 4)) (end (line 829) (character 56))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (kind "attribute def") (name "StrainValue") (declared-name "StrainValue") (range (start (line 810) (character 4)) (end (line 810) (character 636))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::_documentation"))) (kind "documentation") (name "") (range (start (line 810) (character 4)) (end (line 810) (character 636))) (parent (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 824) (character 8)) (end (line 824) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "StrainUnit") (range none)) (redefinition (reference "mRef") (range (start (line 824) (character 22)) (end (line 824) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 823) (character 8)) (end (line 823) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 823) (character 22)) (end (line 823) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (kind "attribute def") (name "StressUnit") (declared-name "StressUnit") (range (start (line 722) (character 4)) (end (line 722) (character 465))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 725) (character 8)) (end (line 725) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 723) (character 8)) (end (line 723) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 724) (character 8)) (end (line 724) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 726) (character 8)) (end (line 726) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 726) (character 22)) (end (line 726) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (kind "attribute def") (name "StressValue") (declared-name "StressValue") (range (start (line 703) (character 4)) (end (line 703) (character 653))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::_documentation"))) (kind "documentation") (name "") (range (start (line 703) (character 4)) (end (line 703) (character 653))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 717) (character 8)) (end (line 717) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "StressUnit") (range none)) (redefinition (reference "mRef") (range (start (line 717) (character 22)) (end (line 717) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 716) (character 8)) (end (line 716) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 716) (character 22)) (end (line 716) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceDensityUnit"))) (kind "alias") (name "SurfaceDensityUnit") (declared-name "SurfaceDensityUnit") (range (start (line 126) (character 4)) (end (line 126) (character 56))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceDensityValue"))) (kind "alias") (name "SurfaceDensityValue") (declared-name "SurfaceDensityValue") (range (start (line 127) (character 4)) (end (line 127) (character 58))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (kind "attribute def") (name "SurfaceMassDensityUnit") (declared-name "SurfaceMassDensityUnit") (range (start (line 120) (character 4)) (end (line 120) (character 359))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 121) (character 8)) (end (line 121) (character 103))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 122) (character 8)) (end (line 122) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 123) (character 8)) (end (line 123) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 123) (character 22)) (end (line 123) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (kind "attribute def") (name "SurfaceMassDensityValue") (declared-name "SurfaceMassDensityValue") (range (start (line 101) (character 4)) (end (line 101) (character 754))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 101) (character 4)) (end (line 101) (character 754))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 115) (character 8)) (end (line 115) (character 54))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 115) (character 22)) (end (line 115) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 114) (character 8)) (end (line 114) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 114) (character 22)) (end (line 114) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (kind "attribute def") (name "SurfaceTensionUnit") (declared-name "SurfaceTensionUnit") (range (start (line 1274) (character 4)) (end (line 1274) (character 359))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1276) (character 8)) (end (line 1276) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1275) (character 8)) (end (line 1275) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1277) (character 8)) (end (line 1277) (character 92))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1277) (character 22)) (end (line 1277) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (kind "attribute def") (name "SurfaceTensionValue") (declared-name "SurfaceTensionValue") (range (start (line 1255) (character 4)) (end (line 1255) (character 712))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1255) (character 4)) (end (line 1255) (character 712))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1269) (character 8)) (end (line 1269) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SurfaceTensionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1269) (character 22)) (end (line 1269) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1268) (character 8)) (end (line 1268) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1268) (character 22)) (end (line 1268) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (kind "attribute def") (name "TorqueUnit") (declared-name "TorqueUnit") (range (start (line 600) (character 4)) (end (line 600) (character 464))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 603) (character 8)) (end (line 603) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 601) (character 8)) (end (line 601) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 602) (character 8)) (end (line 602) (character 100))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 604) (character 8)) (end (line 604) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 604) (character 22)) (end (line 604) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (kind "attribute def") (name "TorqueValue") (declared-name "TorqueValue") (range (start (line 581) (character 4)) (end (line 581) (character 780))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::_documentation"))) (kind "documentation") (name "") (range (start (line 581) (character 4)) (end (line 581) (character 780))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 595) (character 8)) (end (line 595) (character 42))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueUnit") (range none)) (redefinition (reference "mRef") (range (start (line 595) (character 22)) (end (line 595) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 594) (character 8)) (end (line 594) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 594) (character 22)) (end (line 594) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ViscosityUnit"))) (kind "alias") (name "ViscosityUnit") (declared-name "ViscosityUnit") (range (start (line 1224) (character 4)) (end (line 1224) (character 49))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::ViscosityValue"))) (kind "alias") (name "ViscosityValue") (declared-name "ViscosityValue") (range (start (line 1225) (character 4)) (end (line 1225) (character 51))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (kind "attribute def") (name "VolumeFlowRateUnit") (declared-name "VolumeFlowRateUnit") (range (start (line 1530) (character 4)) (end (line 1530) (character 363))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1532) (character 8)) (end (line 1532) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1531) (character 8)) (end (line 1531) (character 102))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1533) (character 8)) (end (line 1533) (character 94))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1533) (character 22)) (end (line 1533) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (kind "attribute def") (name "VolumeFlowRateValue") (declared-name "VolumeFlowRateValue") (range (start (line 1511) (character 4)) (end (line 1511) (character 818))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1511) (character 4)) (end (line 1511) (character 818))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1525) (character 8)) (end (line 1525) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1525) (character 22)) (end (line 1525) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1524) (character 8)) (end (line 1524) (character 32))) (parent (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1524) (character 22)) (end (line 1524) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::YoungModulusUnit"))) (kind "alias") (name "YoungModulusUnit") (declared-name "YoungModulusUnit") (range (start (line 953) (character 4)) (end (line 953) (character 55))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::YoungModulusValue"))) (kind "alias") (name "YoungModulusValue") (declared-name "YoungModulusValue") (range (start (line 954) (character 4)) (end (line 954) (character 57))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 73848))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (kind "attribute def") (name "actionQuantity") (declared-name "actionQuantity") (range (start (line 1554) (character 4)) (end (line 1554) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActionQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (kind "attribute def") (name "angularImpulse") (declared-name "angularImpulse") (range (start (line 625) (character 4)) (end (line 625) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularImpulseValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (kind "attribute def") (name "angularMomentum") (declared-name "angularMomentum") (range (start (line 494) (character 4)) (end (line 494) (character 85))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMomentumValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::bulkModulus"))) (kind "alias") (name "bulkModulus") (declared-name "bulkModulus") (range (start (line 1017) (character 4)) (end (line 1017) (character 47))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (kind "attribute def") (name "cartesianAngularImpulse3dVector") (declared-name "cartesianAngularImpulse3dVector") (range (start (line 651) (character 4)) (end (line 651) (character 99))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularImpulse3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (kind "attribute def") (name "cartesianAngularMomentum3dVector") (declared-name "cartesianAngularMomentum3dVector") (range (start (line 520) (character 4)) (end (line 520) (character 101))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianAngularMomentum3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (kind "attribute def") (name "cartesianDragForce3dVector") (declared-name "cartesianDragForce3dVector") (range (start (line 422) (character 4)) (end (line 422) (character 89))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianDragForce3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianDynamicFrictionForce3dVector"))) (kind "alias") (name "cartesianDynamicFrictionForce3dVector") (declared-name "cartesianDynamicFrictionForce3dVector") (range (start (line 378) (character 4)) (end (line 378) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (kind "attribute def") (name "cartesianForce3dVector") (declared-name "cartesianForce3dVector") (range (start (line 308) (character 4)) (end (line 308) (character 81))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianForce3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (kind "attribute def") (name "cartesianImpulse3dVector") (declared-name "cartesianImpulse3dVector") (range (start (line 468) (character 4)) (end (line 468) (character 85))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianImpulse3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (kind "attribute def") (name "cartesianKineticFrictionForce3dVector") (declared-name "cartesianKineticFrictionForce3dVector") (range (start (line 376) (character 4)) (end (line 376) (character 111))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianKineticFrictionForce3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (kind "attribute def") (name "cartesianMassFlow3dVector") (declared-name "cartesianMassFlow3dVector") (range (start (line 1450) (character 4)) (end (line 1450) (character 87))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianMassFlow3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (kind "attribute def") (name "cartesianMomentOfForce3dVector") (declared-name "cartesianMomentOfForce3dVector") (range (start (line 572) (character 4)) (end (line 572) (character 97))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianMomentOfForce3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (kind "attribute def") (name "cartesianMomentum3dVector") (declared-name "cartesianMomentum3dVector") (range (start (line 256) (character 4)) (end (line 256) (character 87))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianMomentum3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingDrag3dVector"))) (kind "alias") (name "cartesianRollingDrag3dVector") (declared-name "cartesianRollingDrag3dVector") (range (start (line 400) (character 4)) (end (line 400) (character 78))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingFrictionForce3dVector"))) (kind "alias") (name "cartesianRollingFrictionForce3dVector") (declared-name "cartesianRollingFrictionForce3dVector") (range (start (line 402) (character 4)) (end (line 402) (character 87))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (kind "attribute def") (name "cartesianRollingResistance3dVector") (declared-name "cartesianRollingResistance3dVector") (range (start (line 398) (character 4)) (end (line 398) (character 105))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianRollingResistance3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFriction3dVector"))) (kind "alias") (name "cartesianStaticFriction3dVector") (declared-name "cartesianStaticFriction3dVector") (range (start (line 356) (character 4)) (end (line 356) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (kind "attribute def") (name "cartesianStaticFrictionForce3dVector") (declared-name "cartesianStaticFrictionForce3dVector") (range (start (line 354) (character 4)) (end (line 354) (character 109))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianStaticFrictionForce3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (kind "attribute def") (name "cartesianWeight3dVector") (declared-name "cartesianWeight3dVector") (range (start (line 334) (character 4)) (end (line 334) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CartesianWeight3dVector") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::coefficientOfStaticFriction"))) (kind "alias") (name "coefficientOfStaticFriction") (declared-name "coefficientOfStaticFriction") (range (start (line 1140) (character 4)) (end (line 1140) (character 68))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (kind "attribute def") (name "compressibility") (declared-name "compressibility") (range (start (line 1037) (character 4)) (end (line 1037) (character 85))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::density"))) (kind "alias") (name "density") (declared-name "density") (range (start (line 53) (character 4)) (end (line 53) (character 34))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (kind "attribute def") (name "dragCoefficient") (declared-name "dragCoefficient") (range (start (line 1193) (character 4)) (end (line 1193) (character 72))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DragCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::dragFactor"))) (kind "alias") (name "dragFactor") (declared-name "dragFactor") (range (start (line 1195) (character 4)) (end (line 1195) (character 41))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::dynamicFrictionFactor"))) (kind "alias") (name "dynamicFrictionFactor") (declared-name "dynamicFrictionFactor") (range (start (line 1159) (character 4)) (end (line 1159) (character 58))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (kind "attribute def") (name "dynamicViscosity") (declared-name "dynamicViscosity") (range (start (line 1215) (character 4)) (end (line 1215) (character 87))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::force"))) (kind "attribute def") (name "force") (declared-name "force") (range (start (line 282) (character 4)) (end (line 282) (character 65))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (kind "attribute def") (name "gaugePressure") (declared-name "gaugePressure") (range (start (line 687) (character 4)) (end (line 687) (character 620))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::gaugePressure::_documentation"))) (kind "documentation") (name "") (range (start (line 687) (character 4)) (end (line 687) (character 620))) (parent (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (kind "attribute def") (name "impulse") (declared-name "impulse") (range (start (line 442) (character 4)) (end (line 442) (character 69))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ImpulseValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (kind "attribute def") (name "kinematicViscosity") (declared-name "kinematicViscosity") (range (start (line 1246) (character 4)) (end (line 1246) (character 91))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "KinematicViscosityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))) (kind "attribute def") (name "kineticEnergy") (declared-name "kineticEnergy") (range (start (line 1340) (character 4)) (end (line 1340) (character 646))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 1340) (character 4)) (end (line 1340) (character 646))) (parent (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (kind "attribute def") (name "kineticFrictionFactor") (declared-name "kineticFrictionFactor") (range (start (line 1157) (character 4)) (end (line 1157) (character 84))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "KineticFrictionFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::linearDensity"))) (kind "alias") (name "linearDensity") (declared-name "linearDensity") (range (start (line 158) (character 4)) (end (line 158) (character 46))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (kind "attribute def") (name "linearMassDensity") (declared-name "linearMassDensity") (range (start (line 148) (character 4)) (end (line 148) (character 89))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (kind "attribute def") (name "massChangeRate") (declared-name "massChangeRate") (range (start (line 1502) (character 4)) (end (line 1502) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassChangeRateValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (kind "attribute def") (name "massDensity") (declared-name "massDensity") (range (start (line 43) (character 4)) (end (line 43) (character 77))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (kind "attribute def") (name "massFlow") (declared-name "massFlow") (range (start (line 1424) (character 4)) (end (line 1424) (character 71))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (kind "attribute def") (name "massFlowRate") (declared-name "massFlowRate") (range (start (line 1476) (character 4)) (end (line 1476) (character 79))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (kind "attribute def") (name "mechanicalEfficiency") (declared-name "mechanicalEfficiency") (range (start (line 1404) (character 4)) (end (line 1404) (character 82))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MechanicalEfficiencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))) (kind "attribute def") (name "mechanicalEnergy") (declared-name "mechanicalEnergy") (range (start (line 1356) (character 4)) (end (line 1356) (character 685))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 1356) (character 4)) (end (line 1356) (character 685))) (parent (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (kind "attribute def") (name "mechanicalPower") (declared-name "mechanicalPower") (range (start (line 1308) (character 4)) (end (line 1308) (character 573))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower::_documentation"))) (kind "documentation") (name "") (range (start (line 1308) (character 4)) (end (line 1308) (character 573))) (parent (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))) (kind "attribute def") (name "mechanicalWork") (declared-name "mechanicalWork") (range (start (line 1372) (character 4)) (end (line 1372) (character 924))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork::_documentation"))) (kind "documentation") (name "") (range (start (line 1372) (character 4)) (end (line 1372) (character 924))) (parent (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (kind "attribute def") (name "modulusOfCompression") (declared-name "modulusOfCompression") (range (start (line 1006) (character 4)) (end (line 1006) (character 95))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ModulusOfCompressionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (kind "attribute def") (name "modulusOfElasticity") (declared-name "modulusOfElasticity") (range (start (line 944) (character 4)) (end (line 944) (character 93))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ModulusOfElasticityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (kind "attribute def") (name "modulusOfRigidity") (declared-name "modulusOfRigidity") (range (start (line 975) (character 4)) (end (line 975) (character 89))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ModulusOfRigidityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (kind "attribute def") (name "momentOfForce") (declared-name "momentOfForce") (range (start (line 546) (character 4)) (end (line 546) (character 81))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (kind "attribute def") (name "momentOfInertia") (declared-name "momentOfInertia") (range (start (line 178) (character 4)) (end (line 178) (character 85))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfInertiaValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (kind "attribute def") (name "momentOfInertiaTensor") (declared-name "momentOfInertiaTensor") (range (start (line 204) (character 4)) (end (line 204) (character 90))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Cartesian3dMomentOfInertiaTensor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (kind "attribute def") (name "momentum") (declared-name "momentum") (range (start (line 230) (character 4)) (end (line 230) (character 71))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentumValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (kind "attribute def") (name "normalStress") (declared-name "normalStress") (range (start (line 773) (character 4)) (end (line 773) (character 79))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "NormalStressValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (kind "attribute def") (name "poissonNumber") (declared-name "poissonNumber") (range (start (line 924) (character 4)) (end (line 924) (character 68))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PoissonNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))) (kind "attribute def") (name "potentialEnergy") (declared-name "potentialEnergy") (range (start (line 1324) (character 4)) (end (line 1324) (character 769))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 1324) (character 4)) (end (line 1324) (character 769))) (parent (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::power"))) (kind "attribute def") (name "power") (declared-name "power") (range (start (line 1298) (character 4)) (end (line 1298) (character 65))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (kind "attribute def") (name "pressure") (declared-name "pressure") (range (start (line 677) (character 4)) (end (line 677) (character 71))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::relativeDensity"))) (kind "alias") (name "relativeDensity") (declared-name "relativeDensity") (range (start (line 98) (character 4)) (end (line 98) (character 50))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (kind "attribute def") (name "relativeLinearStrain") (declared-name "relativeLinearStrain") (range (start (line 873) (character 4)) (end (line 873) (character 82))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeLinearStrainValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (kind "attribute def") (name "relativeMassDensity") (declared-name "relativeMassDensity") (range (start (line 96) (character 4)) (end (line 96) (character 80))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeMassDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (kind "attribute def") (name "relativeVolumeStrain") (declared-name "relativeVolumeStrain") (range (start (line 907) (character 4)) (end (line 907) (character 82))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeVolumeStrainValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (kind "attribute def") (name "rollingResistanceFactor") (declared-name "rollingResistanceFactor") (range (start (line 1176) (character 4)) (end (line 1176) (character 88))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RollingResistanceFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (kind "attribute def") (name "secondAxialMomentOfArea") (declared-name "secondAxialMomentOfArea") (range (start (line 1064) (character 4)) (end (line 1064) (character 101))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondAxialMomentOfAreaValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (kind "attribute def") (name "secondPolarMomentOfArea") (declared-name "secondPolarMomentOfArea") (range (start (line 1089) (character 4)) (end (line 1089) (character 101))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondPolarMomentOfAreaValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (kind "attribute def") (name "sectionModulus") (declared-name "sectionModulus") (range (start (line 1114) (character 4)) (end (line 1114) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SectionModulusValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::shearModulus"))) (kind "alias") (name "shearModulus") (declared-name "shearModulus") (range (start (line 986) (character 4)) (end (line 986) (character 45))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (kind "attribute def") (name "shearStrain") (declared-name "shearStrain") (range (start (line 890) (character 4)) (end (line 890) (character 64))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ShearStrainValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (kind "attribute def") (name "shearStress") (declared-name "shearStress") (range (start (line 800) (character 4)) (end (line 800) (character 77))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ShearStressValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (kind "attribute def") (name "specificVolume") (declared-name "specificVolume") (range (start (line 73) (character 4)) (end (line 73) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (kind "attribute def") (name "staticFrictionCoefficient") (declared-name "staticFrictionCoefficient") (range (start (line 1136) (character 4)) (end (line 1136) (character 92))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "StaticFrictionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::staticFrictionFactor"))) (kind "alias") (name "staticFrictionFactor") (declared-name "staticFrictionFactor") (range (start (line 1138) (character 4)) (end (line 1138) (character 61))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::strain"))) (kind "attribute def") (name "strain") (declared-name "strain") (range (start (line 827) (character 4)) (end (line 827) (character 67))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "StrainValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (kind "attribute def") (name "strainTensor") (declared-name "strainTensor") (range (start (line 850) (character 4)) (end (line 850) (character 72))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Cartesian3dStrainTensor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::stress"))) (kind "attribute def") (name "stress") (declared-name "stress") (range (start (line 720) (character 4)) (end (line 720) (character 67))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "StressValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (kind "attribute def") (name "stressTensor") (declared-name "stressTensor") (range (start (line 747) (character 4)) (end (line 747) (character 72))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Cartesian3dStressTensor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::surfaceDensity"))) (kind "alias") (name "surfaceDensity") (declared-name "surfaceDensity") (range (start (line 128) (character 4)) (end (line 128) (character 48))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (kind "attribute def") (name "surfaceMassDensity") (declared-name "surfaceMassDensity") (range (start (line 118) (character 4)) (end (line 118) (character 91))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (kind "attribute def") (name "surfaceTension") (declared-name "surfaceTension") (range (start (line 1272) (character 4)) (end (line 1272) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::torque"))) (kind "attribute def") (name "torque") (declared-name "torque") (range (start (line 598) (character 4)) (end (line 598) (character 67))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "TorqueValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::viscosity"))) (kind "alias") (name "viscosity") (declared-name "viscosity") (range (start (line 1226) (character 4)) (end (line 1226) (character 41))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (kind "attribute def") (name "volumeFlowRate") (declared-name "volumeFlowRate") (range (start (line 1528) (character 4)) (end (line 1528) (character 83))) (parent (node (document "d0") (qualified-name "ISQMechanics"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::work"))) (kind "alias") (name "work") (declared-name "work") (range (start (line 1387) (character 4)) (end (line 1387) (character 34))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
    (element (id (node (document "d0") (qualified-name "ISQMechanics::youngModulus"))) (kind "alias") (name "youngModulus") (declared-name "youngModulus") (range (start (line 955) (character 4)) (end (line 955) (character 47))) (parent (node (document "d0") (qualified-name "ISQMechanics"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1560) (character 22)) (end (line 1560) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ActionQuantityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1551) (character 22)) (end (line 1551) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1550) (character 22)) (end (line 1550) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 631) (character 22)) (end (line 631) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularImpulseUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 622) (character 22)) (end (line 622) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 621) (character 22)) (end (line 621) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 500) (character 22)) (end (line 500) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMomentumUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 491) (character 22)) (end (line 491) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 490) (character 22)) (end (line 490) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (range (start (line 207) (character 22)) (end (line 207) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 208) (character 22)) (end (line 208) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 209) (character 22)) (end (line 209) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 199) (character 22)) (end (line 199) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dMomentOfInertiaMeasurementReference") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 201) (character 22)) (end (line 201) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 200) (character 22)) (end (line 200) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (range (start (line 853) (character 22)) (end (line 853) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 854) (character 22)) (end (line 854) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "StrainUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StrainUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 855) (character 22)) (end (line 855) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 845) (character 22)) (end (line 845) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dStrainMeasurementReference") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 847) (character 22)) (end (line 847) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 846) (character 22)) (end (line 846) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (range (start (line 750) (character 22)) (end (line 750) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 751) (character 22)) (end (line 751) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "StressUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 752) (character 22)) (end (line 752) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 742) (character 22)) (end (line 742) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dStressMeasurementReference") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 744) (character 22)) (end (line 744) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 743) (character 22)) (end (line 743) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 654) (character 22)) (end (line 654) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 655) (character 22)) (end (line 655) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularImpulseUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 656) (character 22)) (end (line 656) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 647) (character 22)) (end (line 647) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularImpulse3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 648) (character 22)) (end (line 648) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 523) (character 22)) (end (line 523) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 524) (character 22)) (end (line 524) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMomentumUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 525) (character 22)) (end (line 525) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 516) (character 22)) (end (line 516) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularMomentum3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 517) (character 22)) (end (line 517) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 418) (character 22)) (end (line 418) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 419) (character 22)) (end (line 419) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 311) (character 22)) (end (line 311) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 312) (character 22)) (end (line 312) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 313) (character 22)) (end (line 313) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 304) (character 22)) (end (line 304) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 305) (character 22)) (end (line 305) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 471) (character 22)) (end (line 471) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 472) (character 22)) (end (line 472) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "ImpulseUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 473) (character 22)) (end (line 473) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 464) (character 22)) (end (line 464) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianImpulse3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 465) (character 22)) (end (line 465) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 372) (character 22)) (end (line 372) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 373) (character 22)) (end (line 373) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 1453) (character 22)) (end (line 1453) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 1454) (character 22)) (end (line 1454) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 1455) (character 22)) (end (line 1455) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 1446) (character 22)) (end (line 1446) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMassFlow3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1447) (character 22)) (end (line 1447) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 575) (character 22)) (end (line 575) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 576) (character 22)) (end (line 576) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 577) (character 22)) (end (line 577) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 568) (character 22)) (end (line 568) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMomentOfForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 569) (character 22)) (end (line 569) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "3dCoordinateFrame") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 259) (character 22)) (end (line 259) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (range (start (line 260) (character 22)) (end (line 260) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentumUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (range (start (line 261) (character 22)) (end (line 261) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 252) (character 22)) (end (line 252) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMomentum3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 253) (character 22)) (end (line 253) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 394) (character 22)) (end (line 394) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 395) (character 22)) (end (line 395) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 350) (character 22)) (end (line 350) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 351) (character 22)) (end (line 351) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "3dVectorQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (range (start (line 330) (character 22)) (end (line 330) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dCoordinateFrame") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 331) (character 22)) (end (line 331) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1043) (character 22)) (end (line 1043) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1034) (character 22)) (end (line 1034) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1033) (character 22)) (end (line 1033) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1221) (character 22)) (end (line 1221) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1212) (character 22)) (end (line 1212) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1211) (character 22)) (end (line 1211) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQThermodynamics::EnergyValue") (range (start (line 20) (character 19)) (end (line 20) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 288) (character 22)) (end (line 288) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 279) (character 22)) (end (line 279) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 278) (character 22)) (end (line 278) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 448) (character 22)) (end (line 448) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ImpulseUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 439) (character 22)) (end (line 439) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 438) (character 22)) (end (line 438) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1251) (character 22)) (end (line 1251) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1243) (character 22)) (end (line 1243) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1242) (character 22)) (end (line 1242) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 153) (character 22)) (end (line 153) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 145) (character 22)) (end (line 145) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 144) (character 22)) (end (line 144) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1507) (character 22)) (end (line 1507) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassChangeRateUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1499) (character 22)) (end (line 1499) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1498) (character 22)) (end (line 1498) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 48) (character 22)) (end (line 48) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 40) (character 22)) (end (line 40) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 39) (character 22)) (end (line 39) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1481) (character 22)) (end (line 1481) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1473) (character 22)) (end (line 1473) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1472) (character 22)) (end (line 1472) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1430) (character 22)) (end (line 1430) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1421) (character 22)) (end (line 1421) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1420) (character 22)) (end (line 1420) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1012) (character 22)) (end (line 1012) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfCompressionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1003) (character 22)) (end (line 1003) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1002) (character 22)) (end (line 1002) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 950) (character 22)) (end (line 950) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfElasticityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 941) (character 22)) (end (line 941) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 940) (character 22)) (end (line 940) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 981) (character 22)) (end (line 981) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfRigidityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 972) (character 22)) (end (line 972) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 971) (character 22)) (end (line 971) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 552) (character 22)) (end (line 552) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 543) (character 22)) (end (line 543) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 542) (character 22)) (end (line 542) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 183) (character 22)) (end (line 183) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 175) (character 22)) (end (line 175) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 174) (character 22)) (end (line 174) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 236) (character 22)) (end (line 236) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentumUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 227) (character 22)) (end (line 227) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 226) (character 22)) (end (line 226) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 779) (character 22)) (end (line 779) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "NormalStressUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 770) (character 22)) (end (line 770) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 769) (character 22)) (end (line 769) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1304) (character 22)) (end (line 1304) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1295) (character 22)) (end (line 1295) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1294) (character 22)) (end (line 1294) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 683) (character 22)) (end (line 683) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 674) (character 22)) (end (line 674) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 673) (character 22)) (end (line 673) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1068) (character 22)) (end (line 1068) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1061) (character 22)) (end (line 1061) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1060) (character 22)) (end (line 1060) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1093) (character 22)) (end (line 1093) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondPolarMomentOfAreaUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1086) (character 22)) (end (line 1086) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1085) (character 22)) (end (line 1085) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1118) (character 22)) (end (line 1118) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SectionModulusUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1111) (character 22)) (end (line 1111) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1110) (character 22)) (end (line 1110) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 806) (character 22)) (end (line 806) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ShearStressUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 797) (character 22)) (end (line 797) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 796) (character 22)) (end (line 796) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 78) (character 22)) (end (line 78) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 70) (character 22)) (end (line 70) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 69) (character 22)) (end (line 69) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "StrainUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StrainUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 824) (character 22)) (end (line 824) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 823) (character 22)) (end (line 823) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 726) (character 22)) (end (line 726) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "StressUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 717) (character 22)) (end (line 717) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 716) (character 22)) (end (line 716) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 123) (character 22)) (end (line 123) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 115) (character 22)) (end (line 115) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 114) (character 22)) (end (line 114) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1277) (character 22)) (end (line 1277) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1269) (character 22)) (end (line 1269) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1268) (character 22)) (end (line 1268) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 604) (character 22)) (end (line 604) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 595) (character 22)) (end (line 595) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 594) (character 22)) (end (line 594) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1533) (character 22)) (end (line 1533) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1525) (character 22)) (end (line 1525) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1524) (character 22)) (end (line 1524) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (kind featureTyping) (ordinal 0)) (authored-target "ActionQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularImpulseValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMomentumValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularImpulse3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianAngularMomentum3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianDragForce3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianForce3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianImpulse3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianKineticFrictionForce3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMassFlow3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMomentOfForce3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianMomentum3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianRollingResistance3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianStaticFrictionForce3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (kind featureTyping) (ordinal 0)) (authored-target "CartesianWeight3dVector") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "DragCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::force"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (kind featureTyping) (ordinal 0)) (authored-target "ImpulseValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "KineticFrictionFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (kind featureTyping) (ordinal 0)) (authored-target "MassChangeRateValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "MechanicalEfficiencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfCompressionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfElasticityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulusOfRigidityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dMomentOfInertiaTensor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentumValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (kind featureTyping) (ordinal 0)) (authored-target "NormalStressValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PoissonNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeLinearStrainValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeMassDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeVolumeStrainValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "RollingResistanceFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondPolarMomentOfAreaValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (kind featureTyping) (ordinal 0)) (authored-target "SectionModulusValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (kind featureTyping) (ordinal 0)) (authored-target "ShearStrainValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (kind featureTyping) (ordinal 0)) (authored-target "ShearStressValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "StaticFrictionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::strain"))) (kind featureTyping) (ordinal 0)) (authored-target "StrainValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dStrainTensor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::stress"))) (kind featureTyping) (ordinal 0)) (authored-target "StressValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (kind featureTyping) (ordinal 0)) (authored-target "Cartesian3dStressTensor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::torque"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ForceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ForceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::MomentumValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PowerUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PowerValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::PressureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::StrainUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StrainValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StressUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::StressValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::TorqueValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (target (node (document "d0") (qualified-name "ISQMechanics::ActionQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::actionQuantity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularImpulseValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::angularImpulse"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (target (node (document "d0") (qualified-name "ISQMechanics::AngularMomentumValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::angularMomentum"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularImpulse3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularImpulse3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianAngularMomentum3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianAngularMomentum3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianDragForce3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianDragForce3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianForce3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianForce3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianImpulse3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianImpulse3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianKineticFrictionForce3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianKineticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMassFlow3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMassFlow3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentOfForce3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentOfForce3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianMomentum3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianMomentum3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianRollingResistance3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianRollingResistance3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianStaticFrictionForce3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianStaticFrictionForce3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (target (node (document "d0") (qualified-name "ISQMechanics::CartesianWeight3dVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::cartesianWeight3dVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (target (node (document "d0") (qualified-name "ISQMechanics::CompressibilityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::compressibility"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (target (node (document "d0") (qualified-name "ISQMechanics::DragCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::dragCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (target (node (document "d0") (qualified-name "ISQMechanics::DynamicViscosityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::dynamicViscosity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::force"))) (target (node (document "d0") (qualified-name "ISQMechanics::ForceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::force"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::gaugePressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (target (node (document "d0") (qualified-name "ISQMechanics::ImpulseValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::impulse"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (target (node (document "d0") (qualified-name "ISQMechanics::KinematicViscosityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::kinematicViscosity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::kineticEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (target (node (document "d0") (qualified-name "ISQMechanics::KineticFrictionFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::kineticFrictionFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (target (node (document "d0") (qualified-name "ISQMechanics::LinearMassDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::linearMassDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassChangeRateValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::massChangeRate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::massDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::massFlow"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (target (node (document "d0") (qualified-name "ISQMechanics::MassFlowRateValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::massFlowRate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (target (node (document "d0") (qualified-name "ISQMechanics::MechanicalEfficiencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalPower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::mechanicalWork"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfCompressionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfCompression"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfElasticityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfElasticity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (target (node (document "d0") (qualified-name "ISQMechanics::ModulusOfRigidityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::modulusOfRigidity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfForceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::momentOfForce"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentOfInertiaValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertia"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dMomentOfInertiaTensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::momentOfInertiaTensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (target (node (document "d0") (qualified-name "ISQMechanics::MomentumValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::momentum"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (target (node (document "d0") (qualified-name "ISQMechanics::NormalStressValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::normalStress"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (target (node (document "d0") (qualified-name "ISQMechanics::PoissonNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::poissonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))) (target (node (document "d0") (qualified-name "ISQMechanics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::potentialEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::power"))) (target (node (document "d0") (qualified-name "ISQMechanics::PowerValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (target (node (document "d0") (qualified-name "ISQMechanics::PressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::pressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeLinearStrainValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::relativeLinearStrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeMassDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::relativeMassDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (target (node (document "d0") (qualified-name "ISQMechanics::RelativeVolumeStrainValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::relativeVolumeStrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (target (node (document "d0") (qualified-name "ISQMechanics::RollingResistanceFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::rollingResistanceFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondAxialMomentOfAreaValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::secondAxialMomentOfArea"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (target (node (document "d0") (qualified-name "ISQMechanics::SecondPolarMomentOfAreaValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::secondPolarMomentOfArea"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (target (node (document "d0") (qualified-name "ISQMechanics::SectionModulusValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::sectionModulus"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStrainValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::shearStrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (target (node (document "d0") (qualified-name "ISQMechanics::ShearStressValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::shearStress"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (target (node (document "d0") (qualified-name "ISQMechanics::SpecificVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::specificVolume"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (target (node (document "d0") (qualified-name "ISQMechanics::StaticFrictionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::staticFrictionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::strain"))) (target (node (document "d0") (qualified-name "ISQMechanics::StrainValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::strain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStrainTensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::strainTensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::stress"))) (target (node (document "d0") (qualified-name "ISQMechanics::StressValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::stress"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (target (node (document "d0") (qualified-name "ISQMechanics::Cartesian3dStressTensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::stressTensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceMassDensityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::surfaceMassDensity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (target (node (document "d0") (qualified-name "ISQMechanics::SurfaceTensionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::surfaceTension"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::torque"))) (target (node (document "d0") (qualified-name "ISQMechanics::TorqueValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (target (node (document "d0") (qualified-name "ISQMechanics::VolumeFlowRateValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQMechanics::volumeFlowRate"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
