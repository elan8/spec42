# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQElectromagnetism
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQElectromagnetism {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard IEC-80000-6:2008 "Electromagnetism"
     * see also https://www.iso.org/obp/ui/#iso:std:iec:80000:-6:ed-1:v1:en,fr
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
    private import ISQMechanics::PowerValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;

    /* IEC-80000-6 item 6-1 electric current */
    /* See package ISQBase for the declarations of ElectricCurrentValue and ElectricCurrentUnit */

    /* IEC-80000-6 item 6-2 electric charge */
    attribute def ElectricChargeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-2 electric charge
         * symbol(s): `Q`, `q`
         * application domain: generic
         * name: ElectricCharge
         * quantity dimension: T^1*I^1
         * measurement unit(s): C
         * tensor order: 0
         * definition: `d(Q) = I dt` where `I` is electric current (item 6-1) and `t` is time (ISO 80000-3, item 3-7)
         * remarks: Electric charge is carried by discrete particles and can be positive or negative. The sign convention is such that the elementary electric charge `e`, i.e. the charge of the proton, is positive. See IEC 60050-121, item121-11-01. To denote a point charge `q` is often used, and that is done in the present document.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricChargeUnit[1];
    }

    attribute electricCharge: ElectricChargeValue[*] nonunique :> scalarQuantities;

    attribute def ElectricChargeUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-3 electric charge density, volumic electric charge */
    attribute def ElectricChargeDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-3 electric charge density, volumic electric charge
         * symbol(s): `ρ`, `ρ_V`
         * application domain: generic
         * name: ElectricChargeDensity
         * quantity dimension: L^-3*T^1*I^1
         * measurement unit(s): C/m^3
         * tensor order: 0
         * definition: `ρ = (dQ)/(dV)` where `Q` is electric charge (item 6-2) and `V` is volume (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-07.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricChargeDensityUnit[1];
    }

    attribute electricChargeDensity: ElectricChargeDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectricChargeDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    alias VolumicElectricChargeUnit for ElectricChargeDensityUnit;
    alias VolumicElectricChargeValue for ElectricChargeDensityValue;
    alias volumicElectricCharge for electricChargeDensity;

    /* IEC-80000-6 item 6-4 surface density of electric charge, areic electric charge */
    attribute def SurfaceDensityOfElectricChargeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-4 surface density of electric charge, areic electric charge
         * symbol(s): `ρ_A`, `sigma`
         * application domain: generic
         * name: SurfaceDensityOfElectricCharge
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `ρ_A = (dQ)/(dA)` where `Q` is electric charge (item 6-2) and `A` is area (ISO 80000-3, item 3-3)`
         * remarks: See IEC 60050-121, item 121-11-08.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceDensityOfElectricChargeUnit[1];
    }

    attribute surfaceDensityOfElectricCharge: SurfaceDensityOfElectricChargeValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceDensityOfElectricChargeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    alias AreicElectricChargeUnit for SurfaceDensityOfElectricChargeUnit;
    alias AreicElectricChargeValue for SurfaceDensityOfElectricChargeValue;
    alias areicElectricCharge for surfaceDensityOfElectricCharge;

    /* IEC-80000-6 item 6-5 linear density of electric charge, lineic electric charge */
    attribute def LinearDensityOfElectricChargeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-5 linear density of electric charge, lineic electric charge
         * symbol(s): `ρ_l`, `tau`
         * application domain: generic
         * name: LinearDensityOfElectricCharge
         * quantity dimension: L^-1*T^1*I^1
         * measurement unit(s): C/m
         * tensor order: 0
         * definition: `ρ_l = (dQ)/(dl)` where `Q` is electric charge (item 6-2) and `l` is length (ISO 80000-3, item 3-1.1)
         * remarks: See IEC 60050-121, item121-11-09.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearDensityOfElectricChargeUnit[1];
    }

    attribute linearDensityOfElectricCharge: LinearDensityOfElectricChargeValue[*] nonunique :> scalarQuantities;

    attribute def LinearDensityOfElectricChargeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    alias LineicElectricChargeUnit for LinearDensityOfElectricChargeUnit;
    alias LineicElectricChargeValue for LinearDensityOfElectricChargeValue;
    alias lineicElectricCharge for linearDensityOfElectricCharge;

    /* IEC-80000-6 item 6-6 electric dipole moment */
    attribute def ElectricDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-6 electric dipole moment (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: ElectricDipoleMoment
         * quantity dimension: L^1*T^1*I^1
         * measurement unit(s): C*m
         * tensor order: 0
         * definition: `vec(p) = q (vec(r_+) - vec(r_-))` where `vec(r_+)` and `vec(r_-)` are the position vectors (ISO 80000-3, item 3-1.11) to carriers of electric charges `q` and `-q` (item 6-2), respectively
         * remarks: The electric dipole moment of a substance within a domain is the vector sum of electric dipole moments of electric dipoles included in the domain. See IEC 60050-121, items 121-11-35 and 121-11-36.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricDipoleMomentUnit[1];
    }

    attribute electricDipoleMoment: ElectricDipoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def ElectricDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianElectricDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-6 electric dipole moment (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: ElectricDipoleMoment
         * quantity dimension: L^1*T^1*I^1
         * measurement unit(s): C*m
         * tensor order: 1
         * definition: `vec(p) = q (vec(r_+) - vec(r_-))` where `vec(r_+)` and `vec(r_-)` are the position vectors (ISO 80000-3, item 3-1.11) to carriers of electric charges `q` and `-q` (item 6-2), respectively
         * remarks: The electric dipole moment of a substance within a domain is the vector sum of electric dipole moments of electric dipoles included in the domain. See IEC 60050-121, items 121-11-35 and 121-11-36.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianElectricDipoleMoment3dCoordinateFrame[1];
    }

    attribute cartesianElectricDipoleMoment3dVector: CartesianElectricDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianElectricDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ElectricDipoleMomentUnit[3];
    }

    /* IEC-80000-6 item 6-7 electric polarization */
    attribute def ElectricPolarizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-7 electric polarization (magnitude)
         * symbol(s): `P`
         * application domain: generic
         * name: ElectricPolarization
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `vec(P) = (d vec(p))/(dV)` where `vec(p)` is electric dipole moment (item 6-6) of a substance within a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-37.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricPolarizationUnit[1];
    }

    attribute electricPolarization: ElectricPolarizationValue[*] nonunique :> scalarQuantities;

    attribute def ElectricPolarizationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianElectricPolarization3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-7 electric polarization (vector)
         * symbol(s): `vec(P)`
         * application domain: generic
         * name: ElectricPolarization
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 1
         * definition: `vec(P) = (d vec(p))/(dV)` where `vec(p)` is electric dipole moment (item 6-6) of a substance within a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-37.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianElectricPolarization3dCoordinateFrame[1];
    }

    attribute cartesianElectricPolarization3dVector: CartesianElectricPolarization3dVector :> vectorQuantities;

    attribute def CartesianElectricPolarization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ElectricPolarizationUnit[3];
    }

    /* IEC-80000-6 item 6-8 electric current density, areic electric current */
    attribute def ElectricCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-8 electric current density, areic electric current (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: ElectricCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J) = ρ vec(v)` where `ρ` is electric charge density (item 6-3) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a surface `S` is `I = int_S vec(J) * vec(e_n) dA` where `vec(e_n) dA` is vector surface element. See IEC 60050-121, item 121-11-11.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricCurrentDensityUnit[1];
    }

    attribute electricCurrentDensity: ElectricCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectricCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianElectricCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-8 electric current density, areic electric current (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: ElectricCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J) = ρ vec(v)` where `ρ` is electric charge density (item 6-3) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a surface `S` is `I = int_S vec(J) * vec(e_n) dA` where `vec(e_n) dA` is vector surface element. See IEC 60050-121, item 121-11-11.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianElectricCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianElectricCurrentDensity3dVector: CartesianElectricCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianElectricCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ElectricCurrentDensityUnit[3];
    }

    alias CartesianAreicElectricCurrent3dCoordinateFrame for CartesianElectricCurrentDensity3dCoordinateFrame;
    alias cartesianAreicElectricCurrent3dVector for cartesianElectricCurrentDensity3dVector;

    /* IEC-80000-6 item 6-9 linear electric current density, lineic electric current */
    attribute def LinearElectricCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-9 linear electric current density, lineic electric current (magnitude)
         * symbol(s): `J_S`
         * application domain: generic
         * name: LinearElectricCurrentDensity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(J_S) = ρ_A vec(v)` where `ρ_A` is surface density of electric charge (item 6-4) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a curve `C` on a surface is `I = int_C vec(J_S) xx vec(e_n) * d vec(r)` where `vec(e_n)` is a unit vector perpendicular to the surface and line vector element and `d vec(r)` is the differential of position vector `vec(r)`. See IEC 60050-121, item 121-11-12.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearElectricCurrentDensityUnit[1];
    }

    attribute linearElectricCurrentDensity: LinearElectricCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def LinearElectricCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianLinearElectricCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-9 linear electric current density, lineic electric current (vector)
         * symbol(s): `vec(J_S)`
         * application domain: generic
         * name: LinearElectricCurrentDensity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(J_S) = ρ_A vec(v)` where `ρ_A` is surface density of electric charge (item 6-4) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a curve `C` on a surface is `I = int_C vec(J_S) xx vec(e_n) * d vec(r)` where `vec(e_n)` is a unit vector perpendicular to the surface and line vector element and `d vec(r)` is the differential of position vector `vec(r)`. See IEC 60050-121, item 121-11-12.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianLinearElectricCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianLinearElectricCurrentDensity3dVector: CartesianLinearElectricCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianLinearElectricCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: LinearElectricCurrentDensityUnit[3];
    }

    alias CartesianLineicElectricCurrent3dCoordinateFrame for CartesianLinearElectricCurrentDensity3dCoordinateFrame;
    alias cartesianLineicElectricCurrent3dVector for cartesianLinearElectricCurrentDensity3dVector;

    /* IEC-80000-6 item 6-10 electric field strength */
    attribute def ElectricFieldStrengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-10 electric field strength (magnitude)
         * symbol(s): `E`
         * application domain: generic
         * name: ElectricFieldStrength
         * quantity dimension: L^1*M^1*T^-3*I^-1
         * measurement unit(s): V/m
         * tensor order: 0
         * definition: `vec(E) = vec(F)/q` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `q` is electric charge (item 6-2)
         * remarks: See IEC 60050, item 121-11-18. `q` is the charge of a test particle at rest.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricFieldStrengthUnit[1];
    }

    attribute electricFieldStrength: ElectricFieldStrengthValue[*] nonunique :> scalarQuantities;

    attribute def ElectricFieldStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianElectricFieldStrength3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-10 electric field strength (vector)
         * symbol(s): `vec(E)`
         * application domain: generic
         * name: ElectricFieldStrength
         * quantity dimension: L^1*M^1*T^-3*I^-1
         * measurement unit(s): V/m
         * tensor order: 1
         * definition: `vec(E) = vec(F)/q` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `q` is electric charge (item 6-2)
         * remarks: See IEC 60050, item 121-11-18. `q` is the charge of a test particle at rest.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianElectricFieldStrength3dCoordinateFrame[1];
    }

    attribute cartesianElectricFieldStrength3dVector: CartesianElectricFieldStrength3dVector :> vectorQuantities;

    attribute def CartesianElectricFieldStrength3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ElectricFieldStrengthUnit[3];
    }

    /* IEC-80000-6 item 6-11.1 electric potential */
    attribute def ElectricPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-11.1 electric potential
         * symbol(s): `V`, `φ`
         * application domain: generic
         * name: ElectricPotential
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: `-grad(V) = vec(E) + (del A)/(del t)` where `vec(E)` is electric field strength (item 610), `A` is magnetic vector potential (item 6-32) and `t` is time (ISO 80000-3, item 3-7)
         * remarks: The electric potential is not unique, since any constant scalar field quantity can be added to it without changing its gradient. See IEC 60050-121, item 121-11-25.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricPotentialUnit[1];
    }

    attribute electricPotential: ElectricPotentialValue[*] nonunique :> scalarQuantities;

    attribute def ElectricPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-11.2 electric potential difference */
    attribute def ElectricPotentialDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-11.2 electric potential difference
         * symbol(s): `V_(ab)`
         * application domain: generic
         * name: ElectricPotentialDifference
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: `V_(ab) = int_(vec(r_a))^(vec(r_b)) (vec(E) + (del A)/(del t)) * d vec(r)` where `vec(E)` is electric field strength (item 610), `A` is magnetic vector potential (item 6-32), `t` is time (ISO 80000-3, item 3-7), and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`
         * remarks: `V_(ab) = V_a - V_b` where `V_a` and `V_b` are the potentials at points `a` and `b`, respectively. See IEC 60050-121, item 121-11-26.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricPotentialDifferenceUnit[1];
    }

    attribute electricPotentialDifference: ElectricPotentialDifferenceValue[*] nonunique :> scalarQuantities;

    attribute def ElectricPotentialDifferenceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-11.3 voltage, electric tension */
    attribute voltage: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 6-11.3 voltage, electric tension
         * symbol(s): `U`, `U_(ab)`
         * application domain: generic
         * name: Voltage (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: in electric circuit theory, `U_(ab) = V_a - V_b` where `V_a` and `V_b` are the electric potentials (item 6-11.1) at points `a` and `b`, respectively
         * remarks: For an electric field within a medium `U_(ab) = int_(vec(r_a) (C))^(vec(r_b)) vec(E) * d vec(r)` where `vec(E)` is electric field strength (item 6-10) and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`. For an irrotational electric field, the voltage is independent of the path between the two points `a` and `b`. See IEC 60050-121, item 121-11-27.
         */
    }

    alias electricTension for voltage;

    /* IEC-80000-6 item 6-12 electric flux density, electric displacement */
    attribute def ElectricFluxDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-12 electric flux density, electric displacement (magnitude)
         * symbol(s): `D`
         * application domain: generic
         * name: ElectricFluxDensity
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `vec(D) = ε_0 vec(E) + vec(P)` where `ε_0` is the electric constant (item 6-14.1 ), `vec(E)` is electric field strength (item 6-10), and `vec(P)` is electric polarization (item 6-7)
         * remarks: The electric flux density is related to electric charge density via `nabla * vec(D) = ρ` where `nabla * vec(D)` denotes the divergence of `vec(D)`. See IEC 60050-121, item 121-11-40.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricFluxDensityUnit[1];
    }

    attribute electricFluxDensity: ElectricFluxDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectricFluxDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianElectricFluxDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-12 electric flux density, electric displacement (vector)
         * symbol(s): `vec(D)`
         * application domain: generic
         * name: ElectricFluxDensity
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 1
         * definition: `vec(D) = ε_0 vec(E) + vec(P)` where `ε_0` is the electric constant (item 6-14.1 ), `vec(E)` is electric field strength (item 6-10), and `vec(P)` is electric polarization (item 6-7)
         * remarks: The electric flux density is related to electric charge density via `nabla * vec(D) = ρ` where `nabla * vec(D)` denotes the divergence of `vec(D)`. See IEC 60050-121, item 121-11-40.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianElectricFluxDensity3dCoordinateFrame[1];
    }

    attribute cartesianElectricFluxDensity3dVector: CartesianElectricFluxDensity3dVector :> vectorQuantities;

    attribute def CartesianElectricFluxDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: ElectricFluxDensityUnit[3];
    }

    alias CartesianElectricDisplacement3dCoordinateFrame for CartesianElectricFluxDensity3dCoordinateFrame;
    alias cartesianElectricDisplacement3dVector for cartesianElectricFluxDensity3dVector;

    /* IEC-80000-6 item 6-13 capacitance */
    attribute def CapacitanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-13 capacitance
         * symbol(s): `C`
         * application domain: generic
         * name: Capacitance
         * quantity dimension: L^-2*M^-1*T^4*I^2
         * measurement unit(s): F
         * tensor order: 0
         * definition: `C = Q/U` where `Q` is electric charge (item 6-2) and `U` is voltage (6-11.3)
         * remarks: See IEC 60050-131, item 131-12-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CapacitanceUnit[1];
    }

    attribute capacitance: CapacitanceValue[*] nonunique :> scalarQuantities;

    attribute def CapacitanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 4; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-14.1 electric constant, permittivity of vacuum */
    attribute def ElectricConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-14.1 electric constant, permittivity of vacuum
         * symbol(s): `ε_0`
         * application domain: generic
         * name: ElectricConstant
         * quantity dimension: L^-3*M^-1*T^4*I^2
         * measurement unit(s): F/m
         * tensor order: 0
         * definition: `ε_0 = 1 / (μ_0 * c_0^2)` where `μ_0` is the magnetic constant (item 6-26.1) and `c_0` is the speed of light (item 6-35.2)
         * remarks: `ε_0 = 8.854188 * 10^-12` F/m. See IEC 60050-121, item 121-11-03.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricConstantUnit[1];
    }

    attribute electricConstant: ElectricConstantValue[*] nonunique :> scalarQuantities;

    attribute def ElectricConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 4; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias PermittivityOfVacuumUnit for ElectricConstantUnit;
    alias PermittivityOfVacuumValue for ElectricConstantValue;
    alias permittivityOfVacuum for electricConstant;

    /* IEC-80000-6 item 6-14.2 permittivity */
    attribute def PermittivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-14.2 permittivity
         * symbol(s): `ε`
         * application domain: generic
         * name: Permittivity
         * quantity dimension: L^-3*M^-1*T^4*I^2
         * measurement unit(s): F/m
         * tensor order: 0
         * definition: `vec(D) = ε vec(E)` where `vec(D)` is electric flux density (item 6-12) and `vec(E)` is electric field strength (item 6-10)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium, permittivity is a second order tensor. See IEC 60050-121, item 121-12-12.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PermittivityUnit[1];
    }

    attribute permittivity: PermittivityValue[*] nonunique :> scalarQuantities;

    attribute def PermittivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 4; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-15 relative permittivity */
    attribute def RelativePermittivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-15 relative permittivity
         * symbol(s): `ε_r`
         * application domain: generic
         * name: RelativePermittivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `ε_r = ε / ε_0` where `ε` is permittivity (item 6-14.2) and `ε_0` is the electric constant (item 6-14.1)
         * remarks: See IEC 60050-121, item 121-12-13.
         */
    }
    attribute relativePermittivity: RelativePermittivityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-16 electric susceptibility */
    attribute def ElectricSusceptibilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-16 electric susceptibility
         * symbol(s): `χ`
         * application domain: generic
         * name: ElectricSusceptibility (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `vec(P) = ε_0 χ vec(E)` where `vec(P)` is electric polarization (item 6-7), `ε_0` is the electric constant (item 6-14. 1) and `vec(E)` is electric field strength (item 6-10)
         * remarks: `χ = ε_r - 1`. The definition applies to an isotropic medium. For an anisotropic medium, electric susceptibility is a second order tensor. See IEC 60050-121, item 121-12-19.
         */
    }
    attribute electricSusceptibility: ElectricSusceptibilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-17 electric flux */
    attribute def ElectricFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-17 electric flux
         * symbol(s): `Ψ`
         * application domain: generic
         * name: ElectricFlux
         * quantity dimension: T^1*I^1
         * measurement unit(s): C
         * tensor order: 0
         * definition: `Ψ = int_S vec(D) * vec(e_n) dA` over a surface `S`, where `vec(D)` is electric flux (item 6-12) en `vec(e_n) dA` is the vector surface element (ISO 80000-3 item 3-3)
         * remarks: See IEC 60050-121, item 121-11-41.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricFluxUnit[1];
    }

    attribute electricFlux: ElectricFluxValue[*] nonunique :> scalarQuantities;

    attribute def ElectricFluxUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-18 displacement current density */
    attribute def DisplacementCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-18 displacement current density (magnitude)
         * symbol(s): `J_D`
         * application domain: generic
         * name: DisplacementCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J_D) = (del vec(D))/(del t)` where `vec(D)` is electric flux density (item 6-12) and `t` is time (ISO 80000-3, item 3-7) 
         * remarks: See IEC 60050-121, item 121-11-42.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DisplacementCurrentDensityUnit[1];
    }

    attribute displacementCurrentDensity: DisplacementCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def DisplacementCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianDisplacementCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-18 displacement current density (vector)
         * symbol(s): `vec(J_D)`
         * application domain: generic
         * name: DisplacementCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J_D) = (del vec(D))/(del t)` where `vec(D)` is electric flux density (item 6-12) and `t` is time (ISO 80000-3, item 3-7) 
         * remarks: See IEC 60050-121, item 121-11-42.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianDisplacementCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianDisplacementCurrentDensity3dVector: CartesianDisplacementCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianDisplacementCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: DisplacementCurrentDensityUnit[3];
    }

    /* IEC-80000-6 item 6-19.1 displacement current */
    attribute displacementCurrent: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-19.1 displacement current
         * symbol(s): `I_D`
         * application domain: generic
         * name: DisplacementCurrent (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `I = int_S vec(J_D) * vec(e_n) dA` over a surface `S`, where `vec(J_D)` is displacement current density (item 6-18) en `vec(e_n) dA` is the vector surface element (ISO 80000-3 item 3-3)
         * remarks: See IEC 60050-121, item 121-11-43.
         */
    }

    /* IEC-80000-6 item 6-19.2 total current */
    attribute totalCurrent: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-19.2 total current
         * symbol(s): `I_"tot"`, `I_t`
         * application domain: generic
         * name: TotalCurrent (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `I_(tot) = I + I_D` where `I` is electric current (item 6-1) and `I_D` is displacement current (item 6-19.1)
         * remarks: See IEC 60050-121, item 121-11-45.
         */
    }

    /* IEC-80000-6 item 6-20 total current density */
    attribute def TotalCurrentDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-20 total current density (magnitude)
         * symbol(s): `J_"tot"`, `J_t`
         * application domain: generic
         * name: TotalCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J_(tot)) = vec(J) +vec(J_D)` where `vec(J)` is electric current density (item 6-8) and `vec(J_D)` is displacement current density (item 6-18)
         * remarks: See IEC 60050-121, item 121-11-44.
         */
        attribute :>> num: Real;
        attribute :>> mRef: TotalCurrentDensityUnit[1];
    }

    attribute totalCurrentDensity: TotalCurrentDensityValue[*] nonunique :> scalarQuantities;

    attribute def TotalCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianTotalCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-20 total current density (vector)
         * symbol(s): `vec(J_"tot")`, `vec(J_t)`
         * application domain: generic
         * name: TotalCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J_(tot)) = vec(J) +vec(J_D)` where `vec(J)` is electric current density (item 6-8) and `vec(J_D)` is displacement current density (item 6-18)
         * remarks: See IEC 60050-121, item 121-11-44.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianTotalCurrentDensity3dCoordinateFrame[1];
    }

    attribute cartesianTotalCurrentDensity3dVector: CartesianTotalCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianTotalCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: TotalCurrentDensityUnit[3];
    }

    /* IEC-80000-6 item 6-21 magnetic flux density */
    attribute def MagneticFluxDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-21 magnetic flux density (magnitude)
         * symbol(s): `B`
         * application domain: generic
         * name: MagneticFluxDensity
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 0
         * definition: `vec(F) = q vec(v) xx vec(B)` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1) of any test particle with electric charge `q` (item 6-2)
         * remarks: The magnetic flux density has zero divergence, `nabla * vec(B) = 0`. See IEC 60050-121, item 121-11-19.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticFluxDensityUnit[1];
    }

    attribute magneticFluxDensity: MagneticFluxDensityValue[*] nonunique :> scalarQuantities;

    attribute def MagneticFluxDensityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticFluxDensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-21 magnetic flux density (vector)
         * symbol(s): `vec(B)`
         * application domain: generic
         * name: MagneticFluxDensity
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 1
         * definition: `vec(F) = q vec(v) xx vec(B)` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1) of any test particle with electric charge `q` (item 6-2)
         * remarks: The magnetic flux density has zero divergence, `nabla * vec(B) = 0`. See IEC 60050-121, item 121-11-19.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticFluxDensity3dCoordinateFrame[1];
    }

    attribute cartesianMagneticFluxDensity3dVector: CartesianMagneticFluxDensity3dVector :> vectorQuantities;

    attribute def CartesianMagneticFluxDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticFluxDensityUnit[3];
    }

    /* IEC-80000-6 item 6-22.1 magnetic flux */
    attribute def MagneticFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-22.1 magnetic flux
         * symbol(s): `Φ`
         * application domain: generic
         * name: MagneticFlux
         * quantity dimension: L^2*M^1*T^-2*I^-1
         * measurement unit(s): Wb
         * tensor order: 0
         * definition: `Φ = int_S vec(B) * vec(e_n) dA` over a surface `S`, where `vec(B)` is magnetic flux density (item 6-21) and `vec(e_n) dA` is vector surface element (ISO 80000-3, item 3-3)
         * remarks: See IEC 60050-121, item 121-11-21.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticFluxUnit[1];
    }

    attribute magneticFlux: MagneticFluxValue[*] nonunique :> scalarQuantities;

    attribute def MagneticFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-22.2 linked flux */
    attribute def LinkedFluxValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-22.2 linked flux
         * symbol(s): `Ψ_m`, `Ψ`
         * application domain: generic
         * name: LinkedFlux
         * quantity dimension: L^2*M^1*T^-2*I^-1
         * measurement unit(s): Wb
         * tensor order: 0
         * definition: `Ψ_m = int_C vec(A) * d vec(r)` where `vec(A)` is magnetic vector potential (item 6-32) and `d vec(r)` is line vector element of the curve `C`
         * remarks: Line vector element `d vec(r)` is the differential of position vector `vec(r)` (ISO 80000-3, item 3-1.11). See IEC 60050-121, item 121-11-24.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinkedFluxUnit[1];
    }

    attribute linkedFlux: LinkedFluxValue[*] nonunique :> scalarQuantities;

    attribute def LinkedFluxUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-23 magnetic moment, magnetic area moment */
    attribute def MagneticMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-23 magnetic moment, magnetic area moment (magnitude)
         * symbol(s): `m`
         * application domain: generic
         * name: MagneticMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): A*m^2
         * tensor order: 0
         * definition: `vec(m) = I vec(e_n) A` where `I` is electric current (item 6-1) in a small closed loop, `vec(e_n)` is a unit vector perpendicular to the loop, and `A` is area (ISO 80000-3, item 3-3) of the loop
         * remarks: The magnetic moment of a substance within a domain is the vector sum of the magnetic moments of all entities included in the domain. See IEC 60050-121, items 121-11-49 and 121-11-50.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticMomentUnit[1];
    }

    attribute magneticMoment: MagneticMomentValue[*] nonunique :> scalarQuantities;

    attribute def MagneticMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-23 magnetic moment, magnetic area moment (vector)
         * symbol(s): `vec(m)`
         * application domain: generic
         * name: MagneticMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): A*m^2
         * tensor order: 1
         * definition: `vec(m) = I vec(e_n) A` where `I` is electric current (item 6-1) in a small closed loop, `vec(e_n)` is a unit vector perpendicular to the loop, and `A` is area (ISO 80000-3, item 3-3) of the loop
         * remarks: The magnetic moment of a substance within a domain is the vector sum of the magnetic moments of all entities included in the domain. See IEC 60050-121, items 121-11-49 and 121-11-50.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticMoment3dCoordinateFrame[1];
    }

    attribute cartesianMagneticMoment3dVector: CartesianMagneticMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticMomentUnit[3];
    }

    alias CartesianMagneticAreaMoment3dCoordinateFrame for CartesianMagneticMoment3dCoordinateFrame;
    alias cartesianMagneticAreaMoment3dVector for cartesianMagneticMoment3dVector;

    /* IEC-80000-6 item 6-24 magnetization */
    attribute def MagnetizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-24 magnetization (magnitude)
         * symbol(s): `M`, `H_i`
         * application domain: generic
         * name: Magnetization
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(M) = (d vec(m)) / (dV)` where `vec(m)` is magnetic moment (item 6-23) of a substance in a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-52.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagnetizationUnit[1];
    }

    attribute magnetization: MagnetizationValue[*] nonunique :> scalarQuantities;

    attribute def MagnetizationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagnetization3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-24 magnetization (vector)
         * symbol(s): `vec(M)`, `vec(H_i)`
         * application domain: generic
         * name: Magnetization
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(M) = (d vec(m)) / (dV)` where `vec(m)` is magnetic moment (item 6-23) of a substance in a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-52.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagnetization3dCoordinateFrame[1];
    }

    attribute cartesianMagnetization3dVector: CartesianMagnetization3dVector :> vectorQuantities;

    attribute def CartesianMagnetization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagnetizationUnit[3];
    }

    /* IEC-80000-6 item 6-25 magnetic field strength, magnetizing field */
    attribute def MagneticFieldStrengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-25 magnetic field strength, magnetizing field (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: MagneticFieldStrength
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(H) = vec(B)/μ_0 - vec(M)` where `vec(B)` is magnetic flux density (item 6-21), `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: The magnetic field strength is related to the total current density `vec(J_(t ot))` (item 6-20) via `rot vec(H) = vec(J_(t ot))`. See IEC 60050-121, item 121-11-56.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticFieldStrengthUnit[1];
    }

    attribute magneticFieldStrength: MagneticFieldStrengthValue[*] nonunique :> scalarQuantities;

    attribute def MagneticFieldStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticFieldStrength3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-25 magnetic field strength, magnetizing field (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: MagneticFieldStrength
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(H) = vec(B)/μ_0 - vec(M)` where `vec(B)` is magnetic flux density (item 6-21), `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: The magnetic field strength is related to the total current density `vec(J_(t ot))` (item 6-20) via `rot vec(H) = vec(J_(t ot))`. See IEC 60050-121, item 121-11-56.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticFieldStrength3dCoordinateFrame[1];
    }

    attribute cartesianMagneticFieldStrength3dVector: CartesianMagneticFieldStrength3dVector :> vectorQuantities;

    attribute def CartesianMagneticFieldStrength3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticFieldStrengthUnit[3];
    }

    alias CartesianMagnetizingField3dCoordinateFrame for CartesianMagneticFieldStrength3dCoordinateFrame;
    alias cartesianMagnetizingField3dVector for cartesianMagneticFieldStrength3dVector;

    /* IEC-80000-6 item 6-26.1 magnetic constant, permeability of vacuum */
    attribute def MagneticConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-26.1 magnetic constant, permeability of vacuum
         * symbol(s): `μ_0`
         * application domain: generic
         * name: MagneticConstant
         * quantity dimension: L^1*M^1*T^-2*I^-2
         * measurement unit(s): H/m
         * tensor order: 0
         * definition: `μ_0 = 4 π * 10^-7` H/m
         * remarks: For this definition of `μ_0` see item 6-1.a. `μ_0 ~~ 1.256637 * 10^-6` H/m. See IEC 60050-121, item 121-11-14.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticConstantUnit[1];
    }

    attribute magneticConstant: MagneticConstantValue[*] nonunique :> scalarQuantities;

    attribute def MagneticConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias PermeabilityOfVacuumUnit for MagneticConstantUnit;
    alias PermeabilityOfVacuumValue for MagneticConstantValue;
    alias permeabilityOfVacuum for magneticConstant;

    /* IEC-80000-6 item 6-26.2 permeability */
    attribute def PermeabilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-26.2 permeability
         * symbol(s): `μ`
         * application domain: generic
         * name: Permeability
         * quantity dimension: L^1*M^1*T^-2*I^-2
         * measurement unit(s): H/m
         * tensor order: 0
         * definition: `vec(B) = μ vec(H)` where `vec(B)` is magnetic flux density (item 6-21) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium permeability is a second order tensor. See IEC 60050-121, item 121-12-28.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PermeabilityUnit[1];
    }

    attribute permeability: PermeabilityValue[*] nonunique :> scalarQuantities;

    attribute def PermeabilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-27 relative permeability */
    attribute def RelativePermeabilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-27 relative permeability
         * symbol(s): `μ_r`
         * application domain: generic
         * name: RelativePermeability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `μ_r = μ / μ_0`  where `μ` is permeability (item 6-24) and `μ_0` is the magnetic constant (item 6-26.1)
         * remarks: See IEC 60050-121, item 121-12-29.
         */
    }
    attribute relativePermeability: RelativePermeabilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-28 magnetic susceptibility */
    attribute def MagneticSusceptibilityValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-28 magnetic susceptibility
         * symbol(s): `κ`, `χ_m`
         * application domain: generic
         * name: MagneticSusceptibility (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `vec(M) = κ vec(H)` where `vec(M)` is magnetization (item 6-24) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: `κ = μ_r - 1` This definition applies to an isotropic medium. For an anisotropic medium magnetic susceptibility is a second order tensor. See IEC 60050-121, item 121-12-37.
         */
    }
    attribute magneticSusceptibility: MagneticSusceptibilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-29 magnetic polarization */
    attribute def MagneticPolarizationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-29 magnetic polarization (magnitude)
         * symbol(s): `J_m`
         * application domain: generic
         * name: MagneticPolarization
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 0
         * definition: `vec(J_m) = μ_0 vec(M)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: See IEC 60050-121, item 121-11-54.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticPolarizationUnit[1];
    }

    attribute magneticPolarization: MagneticPolarizationValue[*] nonunique :> scalarQuantities;

    attribute def MagneticPolarizationUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticPolarization3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-29 magnetic polarization (vector)
         * symbol(s): `vec(J_m)`
         * application domain: generic
         * name: MagneticPolarization
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 1
         * definition: `vec(J_m) = μ_0 vec(M)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: See IEC 60050-121, item 121-11-54.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticPolarization3dCoordinateFrame[1];
    }

    attribute cartesianMagneticPolarization3dVector: CartesianMagneticPolarization3dVector :> vectorQuantities;

    attribute def CartesianMagneticPolarization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticPolarizationUnit[3];
    }

    /* IEC-80000-6 item 6-30 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-30 magnetic dipole moment (magnitude)
         * symbol(s): `j_m`, `j`
         * application domain: generic
         * name: MagneticDipoleMoment
         * quantity dimension: L^3*M^1*T^-2*I^-1
         * measurement unit(s): Wb*m
         * tensor order: 0
         * definition: `vec(j_m) = μ_0 vec(m)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(m)` is magnetic moment (item 6-23)
         * remarks: See IEC 60050-121, item 121-11-55.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticDipoleMomentUnit[1];
    }

    attribute magneticDipoleMoment: MagneticDipoleMomentValue[*] nonunique :> scalarQuantities;

    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-30 magnetic dipole moment (vector)
         * symbol(s): `vec(j_m)`, `vec(j)`
         * application domain: generic
         * name: MagneticDipoleMoment
         * quantity dimension: L^3*M^1*T^-2*I^-1
         * measurement unit(s): Wb*m
         * tensor order: 1
         * definition: `vec(j_m) = μ_0 vec(m)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(m)` is magnetic moment (item 6-23)
         * remarks: See IEC 60050-121, item 121-11-55.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianMagneticDipoleMoment3dCoordinateFrame[1];
    }

    attribute cartesianMagneticDipoleMoment3dVector: CartesianMagneticDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticDipoleMomentUnit[3];
    }

    /* IEC-80000-6 item 6-31 coercivity */
    attribute def CoercivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-31 coercivity
         * symbol(s): `H_(c,B)`
         * application domain: generic
         * name: Coercivity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: magnetic field strength (item 6-25) to be applied to bring the magnetic flux density (item 6-21) in a substance from its remaining magnetic flux density to zero
         * remarks: See IEC 60050-121, item 121-12-69. Also called coercive field strength.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CoercivityUnit[1];
    }

    attribute coercivity: CoercivityValue[*] nonunique :> scalarQuantities;

    attribute def CoercivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-32 magnetic vector potential */
    attribute def MagneticVectorPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-32 magnetic vector potential (magnitude)
         * symbol(s): `A`
         * application domain: generic
         * name: MagneticVectorPotential
         * quantity dimension: L^1*M^1*T^-2*I^-1
         * measurement unit(s): Wb/m
         * tensor order: 0
         * definition: `vec(B) = rot vec(A)` where `vec(B)` is magnetic flux density (item 6-21)
         * remarks: The magnetic vector potential is not unique since any irrotational vector field can be added to it without changing its rotation. See IEC 60050-121, item 121-11-23.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagneticVectorPotentialUnit[1];
    }

    attribute magneticVectorPotential: MagneticVectorPotentialValue[*] nonunique :> scalarQuantities;

    attribute def MagneticVectorPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    attribute def CartesianMagneticVectorPotential3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-32 magnetic vector potential (vector)
         * symbol(s): `vec(A)`
         * application domain: generic
         * name: MagneticVectorPotential
         * quantity dimension: L^1*M^1*T^-2*I^-1
         * measurement unit(s): Wb/m
         * tensor order: 1
         * definition: `vec(B) = rot vec(A)` where `vec(B)` is magnetic flux density (item 6-21)
         * remarks: The magnetic vector potential is not unique since any irrotational vector field can be added to it without changing its rotation. See IEC 60050-121, item 121-11-23.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: CartesianMagneticVectorPotential3dCoordinateFrame[1];
    }

    attribute cartesianMagneticVectorPotential3dVector: CartesianMagneticVectorPotential3dVector :> vectorQuantities;

    attribute def CartesianMagneticVectorPotential3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = true;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: MagneticVectorPotentialUnit[3];
    }

    /* IEC-80000-6 item 6-33 electromagnetic energy density, volumic electromagnetic energy */
    attribute def ElectromagneticEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-33 electromagnetic energy density, volumic electromagnetic energy
         * symbol(s): `w`
         * application domain: generic
         * name: ElectromagneticEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3
         * tensor order: 0
         * definition: `ω = 1/2*(vec(E)*vec(D) + vec(B) * vec(H))` where `vec(E)` is electric field strength (item 6-10), `vec(D)` is electric flux density (item 6-12), `vec(B)` is magnetic flux density (item 6-21), and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-65.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectromagneticEnergyDensityUnit[1];
    }

    attribute electromagneticEnergyDensity: ElectromagneticEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def ElectromagneticEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    alias VolumicElectromagneticEnergyUnit for ElectromagneticEnergyDensityUnit;
    alias VolumicElectromagneticEnergyValue for ElectromagneticEnergyDensityValue;
    alias volumicElectromagneticEnergy for electromagneticEnergyDensity;

    /* IEC-80000-6 item 6-34 Poynting vector */
    attribute def PoyntingVectorMagnitudeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-34 Poynting vector (magnitude)
         * symbol(s): `S`
         * application domain: generic
         * name: PoyntingVectorMagnitude
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2
         * tensor order: 0
         * definition: `vec(S) = vec(E) xx vec(H)` where `vec(E)` is electric field strength (item 6-10) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-66.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PoyntingVectorMagnitudeUnit[1];
    }

    attribute poyntingVectorMagnitude: PoyntingVectorMagnitudeValue[*] nonunique :> scalarQuantities;

    attribute def PoyntingVectorMagnitudeUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    attribute def CartesianPoynting3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 6-34 Poynting vector
         * symbol(s): `vec(S)`
         * application domain: generic
         * name: PoyntingVector
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2
         * tensor order: 1
         * definition: `vec(S) = vec(E) xx vec(H)` where `vec(E)` is electric field strength (item 6-10) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-66.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianPoynting3dCoordinateFrame[1];
    }

    attribute cartesianPoynting3dVector: CartesianPoynting3dVector :> vectorQuantities;

    attribute def CartesianPoynting3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: PoyntingVectorMagnitudeUnit[3];
    }

    /* IEC-80000-6 item 6-35.1 phase speed of electromagnetic waves */
    attribute def PhaseSpeedOfElectromagneticWavesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-35.1 phase speed of electromagnetic waves
         * symbol(s): `c`
         * application domain: generic
         * name: PhaseSpeedOfElectromagneticWaves
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s
         * tensor order: 0
         * definition: `c = ω/k` where `ω` is angular frequency (ISO 80000-3, item 3-16) and `k` is angular wavenumber (ISO 80000-3, item 3-19)
         * remarks: See ISO 80000-3, item 3-20.1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseSpeedOfElectromagneticWavesUnit[1];
    }

    attribute phaseSpeedOfElectromagneticWaves: PhaseSpeedOfElectromagneticWavesValue[*] nonunique :> scalarQuantities;

    attribute def PhaseSpeedOfElectromagneticWavesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* IEC-80000-6 item 6-35.2 speed of light, light speed */
    attribute def SpeedOfLightValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-35.2 speed of light, light speed
         * symbol(s): `c_0`
         * application domain: generic
         * name: SpeedOfLight
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s
         * tensor order: 0
         * definition: speed of electromagnetic waves in vacuum; `c_0 = 299792458` m/s
         * remarks: For this value of `c_0` see ISO 80000-3, item 3-1.a. `c_0 = 1/sqrt(ε_0 μ_0)`. See IEC 60050-111, item 111-13-07.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedOfLightUnit[1];
    }

    attribute speedOfLight: SpeedOfLightValue[*] nonunique :> scalarQuantities;

    attribute def SpeedOfLightUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias LightSpeedUnit for SpeedOfLightUnit;
    alias LightSpeedValue for SpeedOfLightValue;
    alias lightSpeed for speedOfLight;

    /* IEC-80000-6 item 6-36 source voltage, source tension */
    attribute def SourceVoltageValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-36 source voltage, source tension
         * symbol(s): `U_s`
         * application domain: generic
         * name: SourceVoltage
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: voltage (item 6-11.3) between the two terminals of a voltage source when there is no electric current (item 6-1) through the source
         * remarks: The name "electromotive force" with the abbreviation EMF and the symbol `E` is deprecated. See IEC 60050-131, item 131-12-22.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SourceVoltageUnit[1];
    }

    attribute sourceVoltage: SourceVoltageValue[*] nonunique :> scalarQuantities;

    attribute def SourceVoltageUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias SourceTensionUnit for SourceVoltageUnit;
    alias SourceTensionValue for SourceVoltageValue;
    alias sourceTension for sourceVoltage;

    /* IEC-80000-6 item 6-37.1 scalar magnetic potential */
    attribute scalarMagneticPotential: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-37.1 scalar magnetic potential
         * symbol(s): `V_m`, `φ`
         * application domain: generic
         * name: ScalarMagneticPotential (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: for an irrotational magnetic field strength `vec(H) =  -nabla V_m` where `vec(H)` is magnetic field strength (item 6-25)
         * remarks: The magnetic scalar potential is not unique since any constant scalar field can be added to it without changing its gradient. See IEC 60050-121, item 121-11-58.
         */
    }

    /* IEC-80000-6 item 6-37.2 magnetic tension */
    attribute magneticTension: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-37.2 magnetic tension
         * symbol(s): `U_m`
         * application domain: generic
         * name: MagneticTension (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `U_m = int_(vec(r_a) (C))^(vec(r_b)) vec(H) * d(vec(r))` where `vec(H)` is magnetic field strength (item 6-25) and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`
         * remarks: For an irrotational magnetic field strength this quantity is equal to the magnetic potential difference. See IEC 60050-121, item121-11-57.
         */
    }

    /* IEC-80000-6 item 6-37.3 magnetomotive force */
    attribute def MagnetomotiveForceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-37.3 magnetomotive force
         * symbol(s): `F_m`
         * application domain: generic
         * name: MagnetomotiveForce
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `F_m = oint_C vec(H) * d vec(r)` where `vec(H)` is magnetic field strength (item 6-25) and `vec(r)` is position vector (ISO 80000-3, item 3-1 .11) along a closed curve `C`
         * remarks: This quantity name is under consideration . Compare remark to item 6-36. See IEC 60050-121, item 121-11-60.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MagnetomotiveForceUnit[1];
    }

    attribute magnetomotiveForce: MagnetomotiveForceValue[*] nonunique :> scalarQuantities;

    attribute def MagnetomotiveForceUnit :> DerivedUnit {
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = electricCurrentPF; }
    }

    /* IEC-80000-6 item 6-37.4 current linkage */
    attribute currentLinkage: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-37.4 current linkage
         * symbol(s): `Θ`
         * application domain: generic
         * name: CurrentLinkage (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: net electric current (item 6-1) through a surface delimited by a closed loop
         * remarks: When `Θ` results from `N` (item 6-38) equal electric currents `I` (item 6-1 ), then `Θ = N I`. See IEC 60050-121 , item 121 -11-46.
         */
    }

    /* IEC-80000-6 item 6-38 number of turns in a winding */
    attribute numberOfTurnsInAWinding: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 6-38 number of turns in a winding
         * symbol(s): `N`
         * application domain: generic
         * name: NumberOfTurnsInAWinding (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of turns in a winding (same as the quantity name)
         * remarks: N may be non-integer number, see ISO 80000-3, item 3-14.
         */
    }

    /* IEC-80000-6 item 6-39 reluctance */
    attribute def ReluctanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-39 reluctance
         * symbol(s): `R_m`, `R`
         * application domain: generic
         * name: Reluctance
         * quantity dimension: L^-2*M^-1*T^2*I^2
         * measurement unit(s): H^-1
         * tensor order: 0
         * definition: `R_m = U_m/Φ` where `U_m` is magnetic tension (item 6-37.2) and `Φ` is magnetic flux (item 6-22 .1)
         * remarks: See IEC 60050-131 , item 131-12-28.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ReluctanceUnit[1];
    }

    attribute reluctance: ReluctanceValue[*] nonunique :> scalarQuantities;

    attribute def ReluctanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-40 permeance */
    attribute def PermeanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-40 permeance
         * symbol(s): `Λ`
         * application domain: generic
         * name: Permeance
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `Λ = 1/R_m` where `R_m` is reluctance (item 6-39)
         * remarks: See IEC 60050-131 , item 131-12-29.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PermeanceUnit[1];
    }

    attribute permeance: PermeanceValue[*] nonunique :> scalarQuantities;

    attribute def PermeanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-41.1 inductance, self inductance */
    attribute def InductanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-41.1 inductance, self inductance
         * symbol(s): `L`, `L_m`
         * application domain: generic
         * name: Inductance
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `L = Ψ / I` where `I` is an electric current (item 6-1) in a thin conducting loop and `Ψ` is the linked flux (item 6-22.2) caused by that electric current
         * remarks: The name "self inductance" is used for the quantity associated to mutual inductance when `n = m`. See IEC 60050-131 , items 131-12-19 and 131 -12-35.
         */
        attribute :>> num: Real;
        attribute :>> mRef: InductanceUnit[1];
    }

    attribute inductance: InductanceValue[*] nonunique :> scalarQuantities;

    attribute def InductanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias SelfInductanceUnit for InductanceUnit;
    alias SelfInductanceValue for InductanceValue;
    alias selfInductance for inductance;

    /* IEC-80000-6 item 6-41.2 mutual inductance */
    attribute mutualInductance: InductanceValue :> scalarQuantities {
        doc
        /*
         * source: item 6-41.2 mutual inductance
         * symbol(s): `L_(mn)`
         * application domain: generic
         * name: MutualInductance (specializes Inductance)
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `L_(mn) = Ψ_m / I_n` where `I_n` is an electric current (item 6-1) in a thin conducting loop `n` and `Ψ_m` is the linked flux (item 6-22.2) caused by that electric current in another loop `m`
         * remarks: `L_(mn) = L_(nm)`. For two loops , the symbol `M` is used for `L_(12)`. See IEC 60050-131, items 131-12-36.
         */
    }

    /* IEC-80000-6 item 6-42.1 coupling factor */
    attribute def CouplingFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-42.1 coupling factor
         * symbol(s): `k`
         * application domain: generic
         * name: CouplingFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for inductive coupling between two inductive elements `k = |L_(mn)| / sqrt(L_m L_n)` where `L_m` and `L_n` are their self inductances (item 6-41 .1 ), and `L_(mn)` is their mutual inductance (item 6-41.2)
         * remarks: See IEC 60050-131 , item 131-12-41.
         */
    }
    attribute couplingFactor: CouplingFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-42.2 leakage factor */
    attribute def LeakageFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-42.2 leakage factor
         * symbol(s): `σ`
         * application domain: generic
         * name: LeakageFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `σ = 1 - k^2` where `k` is the coupling factor (item 6-42 .1)
         * remarks: See IEC 60050-131 , item 131-12-42.
         */
    }
    attribute leakageFactor: LeakageFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-43 conductivity */
    attribute def ConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-43 conductivity
         * symbol(s): `σ`, `γ`
         * application domain: generic
         * name: Conductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m
         * tensor order: 0
         * definition: `vec(J) = σ vec(E)` where `vec(J)` is electric current density (item 6-8) and `vec(E)` is electric field strength (item 6-10)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium `σ` is a second order tensor. `κ` is used in electrochemistry. See IEC 60050-121 , item 121-12-03.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ConductivityUnit[1];
    }

    attribute conductivity: ConductivityValue[*] nonunique :> scalarQuantities;

    attribute def ConductivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-44 resistivity */
    attribute def ResistivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-44 resistivity
         * symbol(s): `ρ`
         * application domain: generic
         * name: Resistivity
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m
         * tensor order: 0
         * definition: `ρ = 1/σ` if is exists, where `σ` is conductivity (item 6-43)
         * remarks: See IEC 60050-121, item 121-12-04.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ResistivityUnit[1];
    }

    attribute resistivity: ResistivityValue[*] nonunique :> scalarQuantities;

    attribute def ResistivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-45 electric power, instantaneous power */
    attribute electricPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-45 electric power, instantaneous power
         * symbol(s): `p`
         * application domain: generic
         * name: ElectricPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W
         * tensor order: 0
         * definition: `p = ui` where `u` is instantaneous voltage (item 6-11 .3) and `i` is instantaneous electric current (item 6-1)
         * remarks: See IEC 60050-131 , item 131-11-30.
         */
    }

    alias instantaneousPower for electricPower;

    /* IEC-80000-6 item 6-46 resistance */
    attribute def ResistanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-46 resistance
         * symbol(s): `R`
         * application domain: generic
         * name: Resistance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: for resistive component `R = u i` where `u` is instantaneous voltage (item 6-11.3) and `i` is instantaneous electric current (item 6-1)
         * remarks: For alternating current, see item 6-51.2. See IEC 60050-131, item 131-12-04.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ResistanceUnit[1];
    }

    attribute resistance: ResistanceValue[*] nonunique :> scalarQuantities;

    attribute def ResistanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-47 conductance */
    attribute def ConductanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-47 conductance
         * symbol(s): `G`
         * application domain: generic
         * name: Conductance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: for resistive component `G = 1/R` where `R` is resistance (item 6-46)
         * remarks: For alternating current, see item 6-52.2. See IEC 60050-131, item 131-12-06.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ConductanceUnit[1];
    }

    attribute conductance: ConductanceValue[*] nonunique :> scalarQuantities;

    attribute def ConductanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-48 phase difference */
    attribute def PhaseDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-48 phase difference
         * symbol(s): `φ`
         * application domain: generic
         * name: PhaseDifference
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: `φ = φ_u - φ_i` where `φ_u` is the initial phase of the voltage (item 6-11 .3) and `φ_i` is the initial phase of the electric current (item 6-1)
         * remarks: When `u = hat(U) cos(ωt - φ_u)`, `i = hat(I) cos(ωt - φ_i)` where `u` is the voltage (item 6-11 . 3) and `i` is the electric current (item 6-1 ), `ω` is angular frequency (ISO 80000-3, item 3-16) and `t` is time (ISO 80000-3, item 3-7), then `φ` is phase difference. For phase angle, see items 6-49 and 6-50.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseDifferenceUnit[1];
    }

    attribute phaseDifference: PhaseDifferenceValue[*] nonunique :> scalarQuantities;

    attribute def PhaseDifferenceUnit :> DimensionOneUnit {
    }

    /* IEC-80000-6 item 6-49 electric current phasor */
    attribute electricCurrentPhasor: ElectricCurrentValue :> scalarQuantities {
        doc
        /*
         * source: item 6-49 electric current phasor
         * symbol(s): `underline(I)`
         * application domain: generic
         * name: ElectricCurrentPhasor (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: when `i = hat(I) cos(ωt + α)`, where `i` is the electric current (item 6-1 ), `ω` is angular frequency (ISO 80000-3, item 3-16), `t` is time (ISO 80000-3, item 3-7), and `α` is initial phase (ISO 80000-3, item 3-5), then `underline(l) = I e^(jα)`
         * remarks: `underline(l)` is the complex representation of the electric current `i = hat(I) cos(ωt + α)`. `j` is the imaginary unit.
         */
    }

    /* IEC-80000-6 item 6-50 voltage phasor */
    attribute voltagePhasor: ElectricPotentialDifferenceValue :> scalarQuantities {
        doc
        /*
         * source: item 6-50 voltage phasor
         * symbol(s): `underline(U)`
         * application domain: generic
         * name: VoltagePhasor (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: when `u = hat(U) cos(ωt + α)`, where `u` is the voltage (item 6-11.3 ), `ω` is angular frequency (ISO 80000-3, item 3-16), `t` is time (ISO 80000-3, item 3-7), and `α` is initial phase (ISO 80000-3, item 3-5), then `underline(U) = U e^(jα)`
         * remarks: `underline(U)` is the complex representation of the voltage `u = hat(U) cos(ωt + α)`. `j` is the imaginary unit.
         */
    }

    /* IEC-80000-6 item 6-51.1 impedance, complex impedance */
    attribute def ImpedanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-51.1 impedance, complex impedance
         * symbol(s): `underline(Z)`
         * application domain: generic
         * name: Impedance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `underline(Z) = underline(U)/underline(I)` where `underline(U)` is the voltage phasor (item 6-50), and `underline(I)` is the electric current phasor (item 6-49)
         * remarks: `underline(Z) = R + jX`, where `R` is resistance (item 6-51.2) and `X` is reactance (item 6-51 .3). `j` is the imaginary unit. `underline(Z) = |underline(Z)| e^(jφ)`. See IEC 60050-131 , item 131-12-43.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ImpedanceUnit[1];
    }

    attribute impedance: ImpedanceValue[*] nonunique :> scalarQuantities;

    attribute def ImpedanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias ComplexImpedanceUnit for ImpedanceUnit;
    alias ComplexImpedanceValue for ImpedanceValue;
    alias complexImpedance for impedance;

    /* IEC-80000-6 item 6-51.2 resistance to alternating current */
    attribute def ResistanceToAlternatingCurrentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-51.2 resistance to alternating current
         * symbol(s): `R`
         * application domain: generic
         * name: ResistanceToAlternatingCurrent
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `R = "Re" underline(Z)` where `underline(Z)`, is impedance (item 6-5.1) and `"Re"` denotes the real part
         * remarks: See IEC 60050-131, item 131-12-45.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ResistanceToAlternatingCurrentUnit[1];
    }

    attribute resistanceToAlternatingCurrent: ResistanceToAlternatingCurrentValue[*] nonunique :> scalarQuantities;

    attribute def ResistanceToAlternatingCurrentUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-51.3 reactance */
    attribute def ReactanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-51.3 reactance
         * symbol(s): `X`
         * application domain: generic
         * name: Reactance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `X = "Im" underline(Z)` where `underline(Z)`, is impedance (item 6-5.1) and `"Im"` denotes the imaginary part
         * remarks: `X = ωL - 1/(ωC)`. See IEC 60050-131 , item 131-12-46.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ReactanceUnit[1];
    }

    attribute reactance: ReactanceValue[*] nonunique :> scalarQuantities;

    attribute def ReactanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-51.4 modulus of impedance */
    attribute def ModulusOfImpedanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-51.4 modulus of impedance
         * symbol(s): `Z`
         * application domain: generic
         * name: ModulusOfImpedance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `Z = |underline(Z)|` where `underline(Z)` is impedance (item 6-51.1)
         * remarks: See IEC 60050-131 , item 131-12-44. Apparent impedance is defined more generally as the quotient of rms voltage and rms electric  current; it is often denoted by `Z`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfImpedanceUnit[1];
    }

    attribute modulusOfImpedance: ModulusOfImpedanceValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfImpedanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-52.1 admittance, complex admittance */
    attribute def AdmittanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-52.1 admittance, complex admittance
         * symbol(s): `underline(Y)`
         * application domain: generic
         * name: Admittance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `underline(Y) = 1/underline(Z)` where `underline(Z)` is impedance (item 6-51.1)
         * remarks: `underline(Y) = G + jB`, where `G` is conductance (item 6-52 .2) and `B` is susceptance (item 6-52 .3). `j` is the imaginary unit. `underline(Y) = |underline(Y)| e^-(jφ)`. See IEC 60050-131, item 131 -12-51.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AdmittanceUnit[1];
    }

    attribute admittance: AdmittanceValue[*] nonunique :> scalarQuantities;

    attribute def AdmittanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    alias ComplexAdmittanceUnit for AdmittanceUnit;
    alias ComplexAdmittanceValue for AdmittanceValue;
    alias complexAdmittance for admittance;

    /* IEC-80000-6 item 6-52.2 conductance for alternating current */
    attribute conductanceForAlternatingCurrent: ConductanceValue :> scalarQuantities {
        doc
        /*
         * source: item 6-52.2 conductance for alternating current
         * symbol(s): `G`
         * application domain: generic
         * name: ConductanceForAlternatingCurrent (specializes Conductance)
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `G = "Re" underline(Y)` where I is admittance (item 6-52.1)
         * remarks: See IEC 60050-131, item 131-12-53.
         */
    }

    /* IEC-80000-6 item 6-52.3 susceptance */
    attribute def SusceptanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-52.3 susceptance
         * symbol(s): `B`
         * application domain: generic
         * name: Susceptance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `B = "Im" underline(Y)` where `underline(Y)` is admittance (item 6-52.1)
         * remarks: See IEC 60050-131, item 131-12-54.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SusceptanceUnit[1];
    }

    attribute susceptance: SusceptanceValue[*] nonunique :> scalarQuantities;

    attribute def SusceptanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-52.4 modulus of admittance */
    attribute def ModulusOfAdmittanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-52.4 modulus of admittance
         * symbol(s): `Y`
         * application domain: generic
         * name: ModulusOfAdmittance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `Y = |underline(Y)|` where `underline(Y)` is admittance (item 6-52.1)
         * remarks: Apparent admittance is defined more generally as the quotient of rms electric current voltage and rms voltage; it is often denoted by `Y`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ModulusOfAdmittanceUnit[1];
    }

    attribute modulusOfAdmittance: ModulusOfAdmittanceValue[*] nonunique :> scalarQuantities;

    attribute def ModulusOfAdmittanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* IEC-80000-6 item 6-53 quality factor */
    attribute def QualityFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-53 quality factor
         * symbol(s): `Q`
         * application domain: generic
         * name: QualityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for non-radiating systems, if `underline(Z) = R + jX`, then `Q = |X|/R` where `underline(Z)` is impedance (item 6-51. 1), `R` is resistance (item 6-51 .2), and `X` is reactance (item 6-51.3)
         * remarks: None.
         */
    }
    attribute qualityFactor: QualityFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-54 loss factor */
    attribute def LossFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-54 loss factor
         * symbol(s): `d`
         * application domain: generic
         * name: LossFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `d = 1/Q` where `Q` quality factor (item 6-53)
         * remarks: It is also named dissipation factor.
         */
    }
    attribute lossFactor: LossFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-55 loss angle */
    attribute lossAngle: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 6-55 loss angle
         * symbol(s): `δ`
         * application domain: generic
         * name: LossAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: `δ = arctan d` where `d` is loss factor (item 6-54)
         * remarks: See IEC 60050-131 , item 131-12-49.
         */
    }

    /* IEC-80000-6 item 6-56 active power */
    attribute activePower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-56 active power
         * symbol(s): `P`
         * application domain: generic
         * name: ActivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W
         * tensor order: 0
         * definition: `P = 1/T int_0^T p dt` where `T` is the period (ISO 80000-3, item 3-12) and `p` is instantaneous power (item 6-45)
         * remarks: In complex notation, `P = "Re" underline(S)` where `underline(S)` is complex power (item 6-59).
         */
    }

    /* IEC-80000-6 item 6-57 apparent power */
    attribute apparentPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-57 apparent power
         * symbol(s): ``, `underline(S)`, ``
         * application domain: generic
         * name: ApparentPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `|underline(S)| = U I` where `U` is rms value of voltage (item 6-11.3 and `I` is rms value of electric current (item 6-1)
         * remarks: `U = sqrt(1/T int_0^T u^2 dt)` and `I = sqrt(1/T int_0^T i^2 dt)`. When `u = sqrt 2 U cos(ωt)` and `i = sqrt 2 I cos(ωt - φ)`, then `P = U I cos(φ)`, `Q = U I sin(φ)` and `λ = cos(φ)` . See IEC 60050-131, item 131-11-41 .
         */
    }

    /* IEC-80000-6 item 6-58 power factor */
    attribute def PowerFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 6-58 power factor
         * symbol(s): `λ`
         * application domain: generic
         * name: PowerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `λ = |P|/|S|` where `P`  is active power (item 6-56) and `S` is apparent power (item 6-57)
         * remarks: See I EC 60050-131, item 131-11-46.
         */
    }
    attribute powerFactor: PowerFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-59 complex power */
    attribute complexPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-59 complex power
         * symbol(s): `underline(S)`
         * application domain: generic
         * name: ComplexPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `underline(S) = underline(U) * underline(I)^"*"` where `underline(U)` is voltage phasor (item 6-50) and `underline(I)^"*"` is the complex conjugate of the current phasor (item 6-49)
         * remarks: `underline(S) = P + jQ` where `P` is active power (item 6-56) and `Q` is reactive power (item 6-60). See IEC 60050-131, item 131-11-39.
         */
    }

    /* IEC-80000-6 item 6-60 reactive power */
    attribute reactivePower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-60 reactive power
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A, var
         * tensor order: 0
         * definition: `Q = "Im" underline(S)` where `underline(S)` is complex power (item 6-59)
         * remarks: See IEC 60050-131, item 131-11-44.
         */
    }

    /* IEC-80000-6 item 6-61 non-active power */
    attribute nonActivePower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 6-61 non-active power
         * symbol(s): `Q'`
         * application domain: generic
         * name: NonActivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `Q' = sqrt(|underline(S)|^2 - P^2)` where `|underline(S)|` is apparent power (item 6-57) and `P` is active power (item 6-56)
         * remarks: See IEC 60050-131, item 131-11-43.
         */
    }

    /* IEC-80000-6 item 6-62 active energy */
    attribute activeEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 6-62 active energy
         * symbol(s): `W`
         * application domain: generic
         * name: ActiveEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, W*h
         * tensor order: 0
         * definition: `W = int_(t_1)^(t_2) p dt` where `p` is instantaneous power (item 6-45), and the integral interval is the time interval from `t_1` to `t_2`
         * remarks: None.
         */
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'PowerValue'
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
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'ElectricCurrentValue'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'PowerValue'
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
semantic.unresolved_name 'ElectricCurrentValue'
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'EnergyValue'
semantic.unresolved_name 'scalarQuantities'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,UnrestrictedName,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQElectromagnetism'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (import_decl private 'ISQMechanics::PowerValue')
    (import_decl private 'ISQSpaceTime::AngularMeasureValue')
    (import_decl private 'ISQThermodynamics::EnergyValue')
    (comment)
    (comment)
    (comment)
    (attribute_def 'ElectricChargeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricChargeUnit' multiplicity))
    (attribute_usage 'electricCharge' : 'ElectricChargeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricChargeUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ElectricChargeDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricChargeDensityUnit' multiplicity))
    (attribute_usage 'electricChargeDensity' : 'ElectricChargeDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricChargeDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'VolumicElectricChargeUnit' for 'ElectricChargeDensityUnit')
    (alias_member 'VolumicElectricChargeValue' for 'ElectricChargeDensityValue')
    (alias_member 'volumicElectricCharge' for 'electricChargeDensity')
    (comment)
    (attribute_def 'SurfaceDensityOfElectricChargeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SurfaceDensityOfElectricChargeUnit' multiplicity))
    (attribute_usage 'surfaceDensityOfElectricCharge' : 'SurfaceDensityOfElectricChargeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SurfaceDensityOfElectricChargeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'AreicElectricChargeUnit' for 'SurfaceDensityOfElectricChargeUnit')
    (alias_member 'AreicElectricChargeValue' for 'SurfaceDensityOfElectricChargeValue')
    (alias_member 'areicElectricCharge' for 'surfaceDensityOfElectricCharge')
    (comment)
    (attribute_def 'LinearDensityOfElectricChargeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearDensityOfElectricChargeUnit' multiplicity))
    (attribute_usage 'linearDensityOfElectricCharge' : 'LinearDensityOfElectricChargeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearDensityOfElectricChargeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'LineicElectricChargeUnit' for 'LinearDensityOfElectricChargeUnit')
    (alias_member 'LineicElectricChargeValue' for 'LinearDensityOfElectricChargeValue')
    (alias_member 'lineicElectricCharge' for 'linearDensityOfElectricCharge')
    (comment)
    (attribute_def 'ElectricDipoleMomentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricDipoleMomentUnit' multiplicity))
    (attribute_usage 'electricDipoleMoment' : 'ElectricDipoleMomentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricDipoleMomentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianElectricDipoleMoment3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianElectricDipoleMoment3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianElectricDipoleMoment3dVector' : 'CartesianElectricDipoleMoment3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianElectricDipoleMoment3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ElectricDipoleMomentUnit' multiplicity))
    (comment)
    (attribute_def 'ElectricPolarizationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricPolarizationUnit' multiplicity))
    (attribute_usage 'electricPolarization' : 'ElectricPolarizationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricPolarizationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianElectricPolarization3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianElectricPolarization3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianElectricPolarization3dVector' : 'CartesianElectricPolarization3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianElectricPolarization3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ElectricPolarizationUnit' multiplicity))
    (comment)
    (attribute_def 'ElectricCurrentDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricCurrentDensityUnit' multiplicity))
    (attribute_usage 'electricCurrentDensity' : 'ElectricCurrentDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricCurrentDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianElectricCurrentDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianElectricCurrentDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianElectricCurrentDensity3dVector' : 'CartesianElectricCurrentDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianElectricCurrentDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ElectricCurrentDensityUnit' multiplicity))
    (alias_member 'CartesianAreicElectricCurrent3dCoordinateFrame' for 'CartesianElectricCurrentDensity3dCoordinateFrame')
    (alias_member 'cartesianAreicElectricCurrent3dVector' for 'cartesianElectricCurrentDensity3dVector')
    (comment)
    (attribute_def 'LinearElectricCurrentDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearElectricCurrentDensityUnit' multiplicity))
    (attribute_usage 'linearElectricCurrentDensity' : 'LinearElectricCurrentDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearElectricCurrentDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianLinearElectricCurrentDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianLinearElectricCurrentDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianLinearElectricCurrentDensity3dVector' : 'CartesianLinearElectricCurrentDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianLinearElectricCurrentDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'LinearElectricCurrentDensityUnit' multiplicity))
    (alias_member 'CartesianLineicElectricCurrent3dCoordinateFrame' for 'CartesianLinearElectricCurrentDensity3dCoordinateFrame')
    (alias_member 'cartesianLineicElectricCurrent3dVector' for 'cartesianLinearElectricCurrentDensity3dVector')
    (comment)
    (attribute_def 'ElectricFieldStrengthValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricFieldStrengthUnit' multiplicity))
    (attribute_usage 'electricFieldStrength' : 'ElectricFieldStrengthValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricFieldStrengthUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianElectricFieldStrength3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianElectricFieldStrength3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianElectricFieldStrength3dVector' : 'CartesianElectricFieldStrength3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianElectricFieldStrength3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ElectricFieldStrengthUnit' multiplicity))
    (comment)
    (attribute_def 'ElectricPotentialValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricPotentialUnit' multiplicity))
    (attribute_usage 'electricPotential' : 'ElectricPotentialValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricPotentialUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ElectricPotentialDifferenceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricPotentialDifferenceUnit' multiplicity))
    (attribute_usage 'electricPotentialDifference' : 'ElectricPotentialDifferenceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricPotentialDifferenceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'voltage' : 'ElectricPotentialDifferenceValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'electricTension' for 'voltage')
    (comment)
    (attribute_def 'ElectricFluxDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricFluxDensityUnit' multiplicity))
    (attribute_usage 'electricFluxDensity' : 'ElectricFluxDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricFluxDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianElectricFluxDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianElectricFluxDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianElectricFluxDensity3dVector' : 'CartesianElectricFluxDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianElectricFluxDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'ElectricFluxDensityUnit' multiplicity))
    (alias_member 'CartesianElectricDisplacement3dCoordinateFrame' for 'CartesianElectricFluxDensity3dCoordinateFrame')
    (alias_member 'cartesianElectricDisplacement3dVector' for 'cartesianElectricFluxDensity3dVector')
    (comment)
    (attribute_def 'CapacitanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CapacitanceUnit' multiplicity))
    (attribute_usage 'capacitance' : 'CapacitanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CapacitanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ElectricConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricConstantUnit' multiplicity))
    (attribute_usage 'electricConstant' : 'ElectricConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'PermittivityOfVacuumUnit' for 'ElectricConstantUnit')
    (alias_member 'PermittivityOfVacuumValue' for 'ElectricConstantValue')
    (alias_member 'permittivityOfVacuum' for 'electricConstant')
    (comment)
    (attribute_def 'PermittivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PermittivityUnit' multiplicity))
    (attribute_usage 'permittivity' : 'PermittivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PermittivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RelativePermittivityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativePermittivity' : 'RelativePermittivityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ElectricSusceptibilityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'electricSusceptibility' : 'ElectricSusceptibilityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ElectricFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectricFluxUnit' multiplicity))
    (attribute_usage 'electricFlux' : 'ElectricFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectricFluxUnit' :> 'DerivedUnit'
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'DisplacementCurrentDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DisplacementCurrentDensityUnit' multiplicity))
    (attribute_usage 'displacementCurrentDensity' : 'DisplacementCurrentDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DisplacementCurrentDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianDisplacementCurrentDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianDisplacementCurrentDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianDisplacementCurrentDensity3dVector' : 'CartesianDisplacementCurrentDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianDisplacementCurrentDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'DisplacementCurrentDensityUnit' multiplicity))
    (comment)
    (attribute_usage 'displacementCurrent' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'totalCurrent' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'TotalCurrentDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TotalCurrentDensityUnit' multiplicity))
    (attribute_usage 'totalCurrentDensity' : 'TotalCurrentDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'TotalCurrentDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianTotalCurrentDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianTotalCurrentDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianTotalCurrentDensity3dVector' : 'CartesianTotalCurrentDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianTotalCurrentDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'TotalCurrentDensityUnit' multiplicity))
    (comment)
    (attribute_def 'MagneticFluxDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticFluxDensityUnit' multiplicity))
    (attribute_usage 'magneticFluxDensity' : 'MagneticFluxDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticFluxDensityUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticFluxDensity3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticFluxDensity3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticFluxDensity3dVector' : 'CartesianMagneticFluxDensity3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticFluxDensity3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticFluxDensityUnit' multiplicity))
    (comment)
    (attribute_def 'MagneticFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticFluxUnit' multiplicity))
    (attribute_usage 'magneticFlux' : 'MagneticFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticFluxUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LinkedFluxValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinkedFluxUnit' multiplicity))
    (attribute_usage 'linkedFlux' : 'LinkedFluxValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinkedFluxUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MagneticMomentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticMomentUnit' multiplicity))
    (attribute_usage 'magneticMoment' : 'MagneticMomentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticMomentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticMoment3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticMoment3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticMoment3dVector' : 'CartesianMagneticMoment3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticMoment3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticMomentUnit' multiplicity))
    (alias_member 'CartesianMagneticAreaMoment3dCoordinateFrame' for 'CartesianMagneticMoment3dCoordinateFrame')
    (alias_member 'cartesianMagneticAreaMoment3dVector' for 'cartesianMagneticMoment3dVector')
    (comment)
    (attribute_def 'MagnetizationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagnetizationUnit' multiplicity))
    (attribute_usage 'magnetization' : 'MagnetizationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagnetizationUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagnetization3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagnetization3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagnetization3dVector' : 'CartesianMagnetization3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagnetization3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagnetizationUnit' multiplicity))
    (comment)
    (attribute_def 'MagneticFieldStrengthValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticFieldStrengthUnit' multiplicity))
    (attribute_usage 'magneticFieldStrength' : 'MagneticFieldStrengthValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticFieldStrengthUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticFieldStrength3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticFieldStrength3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticFieldStrength3dVector' : 'CartesianMagneticFieldStrength3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticFieldStrength3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticFieldStrengthUnit' multiplicity))
    (alias_member 'CartesianMagnetizingField3dCoordinateFrame' for 'CartesianMagneticFieldStrength3dCoordinateFrame')
    (alias_member 'cartesianMagnetizingField3dVector' for 'cartesianMagneticFieldStrength3dVector')
    (comment)
    (attribute_def 'MagneticConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticConstantUnit' multiplicity))
    (attribute_usage 'magneticConstant' : 'MagneticConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'PermeabilityOfVacuumUnit' for 'MagneticConstantUnit')
    (alias_member 'PermeabilityOfVacuumValue' for 'MagneticConstantValue')
    (alias_member 'permeabilityOfVacuum' for 'magneticConstant')
    (comment)
    (attribute_def 'PermeabilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PermeabilityUnit' multiplicity))
    (attribute_usage 'permeability' : 'PermeabilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PermeabilityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RelativePermeabilityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativePermeability' : 'RelativePermeabilityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MagneticSusceptibilityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'magneticSusceptibility' : 'MagneticSusceptibilityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MagneticPolarizationValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticPolarizationUnit' multiplicity))
    (attribute_usage 'magneticPolarization' : 'MagneticPolarizationValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticPolarizationUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticPolarization3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticPolarization3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticPolarization3dVector' : 'CartesianMagneticPolarization3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticPolarization3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticPolarizationUnit' multiplicity))
    (comment)
    (attribute_def 'MagneticDipoleMomentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticDipoleMomentUnit' multiplicity))
    (attribute_usage 'magneticDipoleMoment' : 'MagneticDipoleMomentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticDipoleMomentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticDipoleMoment3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticDipoleMoment3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticDipoleMoment3dVector' : 'CartesianMagneticDipoleMoment3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticDipoleMoment3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticDipoleMomentUnit' multiplicity))
    (comment)
    (attribute_def 'CoercivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CoercivityUnit' multiplicity))
    (attribute_usage 'coercivity' : 'CoercivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CoercivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MagneticVectorPotentialValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagneticVectorPotentialUnit' multiplicity))
    (attribute_usage 'magneticVectorPotential' : 'MagneticVectorPotentialValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagneticVectorPotentialUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianMagneticVectorPotential3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianMagneticVectorPotential3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianMagneticVectorPotential3dVector' : 'CartesianMagneticVectorPotential3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianMagneticVectorPotential3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'MagneticVectorPotentialUnit' multiplicity))
    (comment)
    (attribute_def 'ElectromagneticEnergyDensityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ElectromagneticEnergyDensityUnit' multiplicity))
    (attribute_usage 'electromagneticEnergyDensity' : 'ElectromagneticEnergyDensityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ElectromagneticEnergyDensityUnit' :> 'DerivedUnit'
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
    (alias_member 'VolumicElectromagneticEnergyUnit' for 'ElectromagneticEnergyDensityUnit')
    (alias_member 'VolumicElectromagneticEnergyValue' for 'ElectromagneticEnergyDensityValue')
    (alias_member 'volumicElectromagneticEnergy' for 'electromagneticEnergyDensity')
    (comment)
    (attribute_def 'PoyntingVectorMagnitudeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PoyntingVectorMagnitudeUnit' multiplicity))
    (attribute_usage 'poyntingVectorMagnitude' : 'PoyntingVectorMagnitudeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PoyntingVectorMagnitudeUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'CartesianPoynting3dVector' :> ''3dVectorQuantityValue''
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'mRef' : 'CartesianPoynting3dCoordinateFrame' multiplicity))
    (attribute_usage 'cartesianPoynting3dVector' : 'CartesianPoynting3dVector' :> 'vectorQuantities')
    (attribute_def 'CartesianPoynting3dCoordinateFrame' :> ''3dCoordinateFrame''
      (attribute_usage :>> 'isBound' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' : 'PoyntingVectorMagnitudeUnit' multiplicity))
    (comment)
    (attribute_def 'PhaseSpeedOfElectromagneticWavesValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhaseSpeedOfElectromagneticWavesUnit' multiplicity))
    (attribute_usage 'phaseSpeedOfElectromagneticWaves' : 'PhaseSpeedOfElectromagneticWavesValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhaseSpeedOfElectromagneticWavesUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpeedOfLightValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpeedOfLightUnit' multiplicity))
    (attribute_usage 'speedOfLight' : 'SpeedOfLightValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpeedOfLightUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'LightSpeedUnit' for 'SpeedOfLightUnit')
    (alias_member 'LightSpeedValue' for 'SpeedOfLightValue')
    (alias_member 'lightSpeed' for 'speedOfLight')
    (comment)
    (attribute_def 'SourceVoltageValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SourceVoltageUnit' multiplicity))
    (attribute_usage 'sourceVoltage' : 'SourceVoltageValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SourceVoltageUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'SourceTensionUnit' for 'SourceVoltageUnit')
    (alias_member 'SourceTensionValue' for 'SourceVoltageValue')
    (alias_member 'sourceTension' for 'sourceVoltage')
    (comment)
    (attribute_usage 'scalarMagneticPotential' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'magneticTension' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'MagnetomotiveForceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MagnetomotiveForceUnit' multiplicity))
    (attribute_usage 'magnetomotiveForce' : 'MagnetomotiveForceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MagnetomotiveForceUnit' :> 'DerivedUnit'
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'currentLinkage' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'numberOfTurnsInAWinding' : 'CountValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ReluctanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ReluctanceUnit' multiplicity))
    (attribute_usage 'reluctance' : 'ReluctanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ReluctanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PermeanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PermeanceUnit' multiplicity))
    (attribute_usage 'permeance' : 'PermeanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PermeanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'InductanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'InductanceUnit' multiplicity))
    (attribute_usage 'inductance' : 'InductanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'InductanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'SelfInductanceUnit' for 'InductanceUnit')
    (alias_member 'SelfInductanceValue' for 'InductanceValue')
    (alias_member 'selfInductance' for 'inductance')
    (comment)
    (attribute_usage 'mutualInductance' : 'InductanceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'CouplingFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'couplingFactor' : 'CouplingFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LeakageFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'leakageFactor' : 'LeakageFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ConductivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ConductivityUnit' multiplicity))
    (attribute_usage 'conductivity' : 'ConductivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ConductivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ResistivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ResistivityUnit' multiplicity))
    (attribute_usage 'resistivity' : 'ResistivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ResistivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'electricPower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'instantaneousPower' for 'electricPower')
    (comment)
    (attribute_def 'ResistanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ResistanceUnit' multiplicity))
    (attribute_usage 'resistance' : 'ResistanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ResistanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ConductanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ConductanceUnit' multiplicity))
    (attribute_usage 'conductance' : 'ConductanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ConductanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PhaseDifferenceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PhaseDifferenceUnit' multiplicity))
    (attribute_usage 'phaseDifference' : 'PhaseDifferenceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PhaseDifferenceUnit' :> 'DimensionOneUnit')
    (comment)
    (attribute_usage 'electricCurrentPhasor' : 'ElectricCurrentValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'voltagePhasor' : 'ElectricPotentialDifferenceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'ImpedanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ImpedanceUnit' multiplicity))
    (attribute_usage 'impedance' : 'ImpedanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ImpedanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'ComplexImpedanceUnit' for 'ImpedanceUnit')
    (alias_member 'ComplexImpedanceValue' for 'ImpedanceValue')
    (alias_member 'complexImpedance' for 'impedance')
    (comment)
    (attribute_def 'ResistanceToAlternatingCurrentValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ResistanceToAlternatingCurrentUnit' multiplicity))
    (attribute_usage 'resistanceToAlternatingCurrent' : 'ResistanceToAlternatingCurrentValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ResistanceToAlternatingCurrentUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ReactanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ReactanceUnit' multiplicity))
    (attribute_usage 'reactance' : 'ReactanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ReactanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ModulusOfImpedanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ModulusOfImpedanceUnit' multiplicity))
    (attribute_usage 'modulusOfImpedance' : 'ModulusOfImpedanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ModulusOfImpedanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'AdmittanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'AdmittanceUnit' multiplicity))
    (attribute_usage 'admittance' : 'AdmittanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'AdmittanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'ComplexAdmittanceUnit' for 'AdmittanceUnit')
    (alias_member 'ComplexAdmittanceValue' for 'AdmittanceValue')
    (alias_member 'complexAdmittance' for 'admittance')
    (comment)
    (attribute_usage 'conductanceForAlternatingCurrent' : 'ConductanceValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'SusceptanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SusceptanceUnit' multiplicity))
    (attribute_usage 'susceptance' : 'SusceptanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SusceptanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ModulusOfAdmittanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ModulusOfAdmittanceUnit' multiplicity))
    (attribute_usage 'modulusOfAdmittance' : 'ModulusOfAdmittanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ModulusOfAdmittanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'electricCurrentPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'QualityFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'qualityFactor' : 'QualityFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LossFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lossFactor' : 'LossFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'lossAngle' : 'AngularMeasureValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'activePower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'apparentPower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'PowerFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'powerFactor' : 'PowerFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'complexPower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'reactivePower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'nonActivePower' : 'PowerValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'activeEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package ISQElectromagnetism {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard IEC-80000-6:2008 "Electromagnetism"
     * see also https://www.iso.org/obp/ui/#iso:std:iec:80000:-6:ed-1:v1:en,fr
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
    private import ISQMechanics::PowerValue;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;

    /* IEC-80000-6 item 6-1 electric current */
    /* See package ISQBase for the declarations of ElectricCurrentValue and ElectricCurrentUnit */

    /* IEC-80000-6 item 6-2 electric charge */
    attribute def ElectricChargeValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-2 electric charge
         * symbol(s): `Q`, `q`
         * application domain: generic
         * name: ElectricCharge
         * quantity dimension: T^1*I^1
         * measurement unit(s): C
         * tensor order: 0
         * definition: `d(Q) = I dt` where `I` is electric current (item 6-1) and `t` is time (ISO 80000-3, item 3-7)
         * remarks: Electric charge is carried by discrete particles and can be positive or negative. The sign convention is such that the elementary electric charge `e`, i.e. the charge of the proton, is positive. See IEC 60050-121, item121-11-01. To denote a point charge `q` is often used, and that is done in the present document.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricChargeUnit [1];
    }

    attribute electricCharge : ElectricChargeValue :> scalarQuantities [*] nonunique;

    attribute def ElectricChargeUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-3 electric charge density, volumic electric charge */
    attribute def ElectricChargeDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-3 electric charge density, volumic electric charge
         * symbol(s): `ρ`, `ρ_V`
         * application domain: generic
         * name: ElectricChargeDensity
         * quantity dimension: L^-3*T^1*I^1
         * measurement unit(s): C/m^3
         * tensor order: 0
         * definition: `ρ = (dQ)/(dV)` where `Q` is electric charge (item 6-2) and `V` is volume (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-07.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricChargeDensityUnit [1];
    }

    attribute electricChargeDensity : ElectricChargeDensityValue :> scalarQuantities [*] nonunique;

    attribute def ElectricChargeDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    alias VolumicElectricChargeUnit for ElectricChargeDensityUnit;
    alias VolumicElectricChargeValue for ElectricChargeDensityValue;
    alias volumicElectricCharge for electricChargeDensity;

    /* IEC-80000-6 item 6-4 surface density of electric charge, areic electric charge */
    attribute def SurfaceDensityOfElectricChargeValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-4 surface density of electric charge, areic electric charge
         * symbol(s): `ρ_A`, `sigma`
         * application domain: generic
         * name: SurfaceDensityOfElectricCharge
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `ρ_A = (dQ)/(dA)` where `Q` is electric charge (item 6-2) and `A` is area (ISO 80000-3, item 3-3)`
         * remarks: See IEC 60050-121, item 121-11-08.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceDensityOfElectricChargeUnit [1];
    }

    attribute surfaceDensityOfElectricCharge : SurfaceDensityOfElectricChargeValue :> scalarQuantities [*] nonunique;

    attribute def SurfaceDensityOfElectricChargeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    alias AreicElectricChargeUnit for SurfaceDensityOfElectricChargeUnit;
    alias AreicElectricChargeValue for SurfaceDensityOfElectricChargeValue;
    alias areicElectricCharge for surfaceDensityOfElectricCharge;

    /* IEC-80000-6 item 6-5 linear density of electric charge, lineic electric charge */
    attribute def LinearDensityOfElectricChargeValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-5 linear density of electric charge, lineic electric charge
         * symbol(s): `ρ_l`, `tau`
         * application domain: generic
         * name: LinearDensityOfElectricCharge
         * quantity dimension: L^-1*T^1*I^1
         * measurement unit(s): C/m
         * tensor order: 0
         * definition: `ρ_l = (dQ)/(dl)` where `Q` is electric charge (item 6-2) and `l` is length (ISO 80000-3, item 3-1.1)
         * remarks: See IEC 60050-121, item121-11-09.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearDensityOfElectricChargeUnit [1];
    }

    attribute linearDensityOfElectricCharge : LinearDensityOfElectricChargeValue :> scalarQuantities [*] nonunique;

    attribute def LinearDensityOfElectricChargeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    alias LineicElectricChargeUnit for LinearDensityOfElectricChargeUnit;
    alias LineicElectricChargeValue for LinearDensityOfElectricChargeValue;
    alias lineicElectricCharge for linearDensityOfElectricCharge;

    /* IEC-80000-6 item 6-6 electric dipole moment */
    attribute def ElectricDipoleMomentValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-6 electric dipole moment (magnitude)
         * symbol(s): `p`
         * application domain: generic
         * name: ElectricDipoleMoment
         * quantity dimension: L^1*T^1*I^1
         * measurement unit(s): C*m
         * tensor order: 0
         * definition: `vec(p) = q (vec(r_+) - vec(r_-))` where `vec(r_+)` and `vec(r_-)` are the position vectors (ISO 80000-3, item 3-1.11) to carriers of electric charges `q` and `-q` (item 6-2), respectively
         * remarks: The electric dipole moment of a substance within a domain is the vector sum of electric dipole moments of electric dipoles included in the domain. See IEC 60050-121, items 121-11-35 and 121-11-36.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricDipoleMomentUnit [1];
    }

    attribute electricDipoleMoment : ElectricDipoleMomentValue :> scalarQuantities [*] nonunique;

    attribute def ElectricDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianElectricDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-6 electric dipole moment (vector)
         * symbol(s): `vec(p)`
         * application domain: generic
         * name: ElectricDipoleMoment
         * quantity dimension: L^1*T^1*I^1
         * measurement unit(s): C*m
         * tensor order: 1
         * definition: `vec(p) = q (vec(r_+) - vec(r_-))` where `vec(r_+)` and `vec(r_-)` are the position vectors (ISO 80000-3, item 3-1.11) to carriers of electric charges `q` and `-q` (item 6-2), respectively
         * remarks: The electric dipole moment of a substance within a domain is the vector sum of electric dipole moments of electric dipoles included in the domain. See IEC 60050-121, items 121-11-35 and 121-11-36.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianElectricDipoleMoment3dCoordinateFrame [1];
    }

    attribute cartesianElectricDipoleMoment3dVector : CartesianElectricDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianElectricDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ElectricDipoleMomentUnit [3];
    }

    /* IEC-80000-6 item 6-7 electric polarization */
    attribute def ElectricPolarizationValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-7 electric polarization (magnitude)
         * symbol(s): `P`
         * application domain: generic
         * name: ElectricPolarization
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `vec(P) = (d vec(p))/(dV)` where `vec(p)` is electric dipole moment (item 6-6) of a substance within a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-37.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricPolarizationUnit [1];
    }

    attribute electricPolarization : ElectricPolarizationValue :> scalarQuantities [*] nonunique;

    attribute def ElectricPolarizationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianElectricPolarization3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-7 electric polarization (vector)
         * symbol(s): `vec(P)`
         * application domain: generic
         * name: ElectricPolarization
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 1
         * definition: `vec(P) = (d vec(p))/(dV)` where `vec(p)` is electric dipole moment (item 6-6) of a substance within a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-37.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianElectricPolarization3dCoordinateFrame [1];
    }

    attribute cartesianElectricPolarization3dVector : CartesianElectricPolarization3dVector :> vectorQuantities;

    attribute def CartesianElectricPolarization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ElectricPolarizationUnit [3];
    }

    /* IEC-80000-6 item 6-8 electric current density, areic electric current */
    attribute def ElectricCurrentDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-8 electric current density, areic electric current (magnitude)
         * symbol(s): `J`
         * application domain: generic
         * name: ElectricCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J) = ρ vec(v)` where `ρ` is electric charge density (item 6-3) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a surface `S` is `I = int_S vec(J) * vec(e_n) dA` where `vec(e_n) dA` is vector surface element. See IEC 60050-121, item 121-11-11.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricCurrentDensityUnit [1];
    }

    attribute electricCurrentDensity : ElectricCurrentDensityValue :> scalarQuantities [*] nonunique;

    attribute def ElectricCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianElectricCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-8 electric current density, areic electric current (vector)
         * symbol(s): `vec(J)`
         * application domain: generic
         * name: ElectricCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J) = ρ vec(v)` where `ρ` is electric charge density (item 6-3) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a surface `S` is `I = int_S vec(J) * vec(e_n) dA` where `vec(e_n) dA` is vector surface element. See IEC 60050-121, item 121-11-11.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianElectricCurrentDensity3dCoordinateFrame [1];
    }

    attribute cartesianElectricCurrentDensity3dVector : CartesianElectricCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianElectricCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ElectricCurrentDensityUnit [3];
    }

    alias CartesianAreicElectricCurrent3dCoordinateFrame for CartesianElectricCurrentDensity3dCoordinateFrame;
    alias cartesianAreicElectricCurrent3dVector for cartesianElectricCurrentDensity3dVector;

    /* IEC-80000-6 item 6-9 linear electric current density, lineic electric current */
    attribute def LinearElectricCurrentDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-9 linear electric current density, lineic electric current (magnitude)
         * symbol(s): `J_S`
         * application domain: generic
         * name: LinearElectricCurrentDensity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(J_S) = ρ_A vec(v)` where `ρ_A` is surface density of electric charge (item 6-4) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a curve `C` on a surface is `I = int_C vec(J_S) xx vec(e_n) * d vec(r)` where `vec(e_n)` is a unit vector perpendicular to the surface and line vector element and `d vec(r)` is the differential of position vector `vec(r)`. See IEC 60050-121, item 121-11-12.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearElectricCurrentDensityUnit [1];
    }

    attribute linearElectricCurrentDensity : LinearElectricCurrentDensityValue :> scalarQuantities [*] nonunique;

    attribute def LinearElectricCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianLinearElectricCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-9 linear electric current density, lineic electric current (vector)
         * symbol(s): `vec(J_S)`
         * application domain: generic
         * name: LinearElectricCurrentDensity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(J_S) = ρ_A vec(v)` where `ρ_A` is surface density of electric charge (item 6-4) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1)
         * remarks: Electric current `I` (item 6-1) through a curve `C` on a surface is `I = int_C vec(J_S) xx vec(e_n) * d vec(r)` where `vec(e_n)` is a unit vector perpendicular to the surface and line vector element and `d vec(r)` is the differential of position vector `vec(r)`. See IEC 60050-121, item 121-11-12.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianLinearElectricCurrentDensity3dCoordinateFrame [1];
    }

    attribute cartesianLinearElectricCurrentDensity3dVector : CartesianLinearElectricCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianLinearElectricCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : LinearElectricCurrentDensityUnit [3];
    }

    alias CartesianLineicElectricCurrent3dCoordinateFrame for CartesianLinearElectricCurrentDensity3dCoordinateFrame;
    alias cartesianLineicElectricCurrent3dVector for cartesianLinearElectricCurrentDensity3dVector;

    /* IEC-80000-6 item 6-10 electric field strength */
    attribute def ElectricFieldStrengthValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-10 electric field strength (magnitude)
         * symbol(s): `E`
         * application domain: generic
         * name: ElectricFieldStrength
         * quantity dimension: L^1*M^1*T^-3*I^-1
         * measurement unit(s): V/m
         * tensor order: 0
         * definition: `vec(E) = vec(F)/q` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `q` is electric charge (item 6-2)
         * remarks: See IEC 60050, item 121-11-18. `q` is the charge of a test particle at rest.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricFieldStrengthUnit [1];
    }

    attribute electricFieldStrength : ElectricFieldStrengthValue :> scalarQuantities [*] nonunique;

    attribute def ElectricFieldStrengthUnit :> DerivedUnit {
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
            :>> exponent = -3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianElectricFieldStrength3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-10 electric field strength (vector)
         * symbol(s): `vec(E)`
         * application domain: generic
         * name: ElectricFieldStrength
         * quantity dimension: L^1*M^1*T^-3*I^-1
         * measurement unit(s): V/m
         * tensor order: 1
         * definition: `vec(E) = vec(F)/q` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `q` is electric charge (item 6-2)
         * remarks: See IEC 60050, item 121-11-18. `q` is the charge of a test particle at rest.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianElectricFieldStrength3dCoordinateFrame [1];
    }

    attribute cartesianElectricFieldStrength3dVector : CartesianElectricFieldStrength3dVector :> vectorQuantities;

    attribute def CartesianElectricFieldStrength3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ElectricFieldStrengthUnit [3];
    }

    /* IEC-80000-6 item 6-11.1 electric potential */
    attribute def ElectricPotentialValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-11.1 electric potential
         * symbol(s): `V`, `φ`
         * application domain: generic
         * name: ElectricPotential
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: `-grad(V) = vec(E) + (del A)/(del t)` where `vec(E)` is electric field strength (item 610), `A` is magnetic vector potential (item 6-32) and `t` is time (ISO 80000-3, item 3-7)
         * remarks: The electric potential is not unique, since any constant scalar field quantity can be added to it without changing its gradient. See IEC 60050-121, item 121-11-25.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricPotentialUnit [1];
    }

    attribute electricPotential : ElectricPotentialValue :> scalarQuantities [*] nonunique;

    attribute def ElectricPotentialUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-11.2 electric potential difference */
    attribute def ElectricPotentialDifferenceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-11.2 electric potential difference
         * symbol(s): `V_(ab)`
         * application domain: generic
         * name: ElectricPotentialDifference
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: `V_(ab) = int_(vec(r_a))^(vec(r_b)) (vec(E) + (del A)/(del t)) * d vec(r)` where `vec(E)` is electric field strength (item 610), `A` is magnetic vector potential (item 6-32), `t` is time (ISO 80000-3, item 3-7), and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`
         * remarks: `V_(ab) = V_a - V_b` where `V_a` and `V_b` are the potentials at points `a` and `b`, respectively. See IEC 60050-121, item 121-11-26.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricPotentialDifferenceUnit [1];
    }

    attribute electricPotentialDifference : ElectricPotentialDifferenceValue :> scalarQuantities [*] nonunique;

    attribute def ElectricPotentialDifferenceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-11.3 voltage, electric tension */
    attribute voltage : ElectricPotentialDifferenceValue :> scalarQuantities {
        doc /*
         * source: item 6-11.3 voltage, electric tension
         * symbol(s): `U`, `U_(ab)`
         * application domain: generic
         * name: Voltage (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: in electric circuit theory, `U_(ab) = V_a - V_b` where `V_a` and `V_b` are the electric potentials (item 6-11.1) at points `a` and `b`, respectively
         * remarks: For an electric field within a medium `U_(ab) = int_(vec(r_a) (C))^(vec(r_b)) vec(E) * d vec(r)` where `vec(E)` is electric field strength (item 6-10) and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`. For an irrotational electric field, the voltage is independent of the path between the two points `a` and `b`. See IEC 60050-121, item 121-11-27.
         */
    }

    alias electricTension for voltage;

    /* IEC-80000-6 item 6-12 electric flux density, electric displacement */
    attribute def ElectricFluxDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-12 electric flux density, electric displacement (magnitude)
         * symbol(s): `D`
         * application domain: generic
         * name: ElectricFluxDensity
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 0
         * definition: `vec(D) = ε_0 vec(E) + vec(P)` where `ε_0` is the electric constant (item 6-14.1 ), `vec(E)` is electric field strength (item 6-10), and `vec(P)` is electric polarization (item 6-7)
         * remarks: The electric flux density is related to electric charge density via `nabla * vec(D) = ρ` where `nabla * vec(D)` denotes the divergence of `vec(D)`. See IEC 60050-121, item 121-11-40.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricFluxDensityUnit [1];
    }

    attribute electricFluxDensity : ElectricFluxDensityValue :> scalarQuantities [*] nonunique;

    attribute def ElectricFluxDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianElectricFluxDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-12 electric flux density, electric displacement (vector)
         * symbol(s): `vec(D)`
         * application domain: generic
         * name: ElectricFluxDensity
         * quantity dimension: L^-2*T^1*I^1
         * measurement unit(s): C/m^2
         * tensor order: 1
         * definition: `vec(D) = ε_0 vec(E) + vec(P)` where `ε_0` is the electric constant (item 6-14.1 ), `vec(E)` is electric field strength (item 6-10), and `vec(P)` is electric polarization (item 6-7)
         * remarks: The electric flux density is related to electric charge density via `nabla * vec(D) = ρ` where `nabla * vec(D)` denotes the divergence of `vec(D)`. See IEC 60050-121, item 121-11-40.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianElectricFluxDensity3dCoordinateFrame [1];
    }

    attribute cartesianElectricFluxDensity3dVector : CartesianElectricFluxDensity3dVector :> vectorQuantities;

    attribute def CartesianElectricFluxDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : ElectricFluxDensityUnit [3];
    }

    alias CartesianElectricDisplacement3dCoordinateFrame for CartesianElectricFluxDensity3dCoordinateFrame;
    alias cartesianElectricDisplacement3dVector for cartesianElectricFluxDensity3dVector;

    /* IEC-80000-6 item 6-13 capacitance */
    attribute def CapacitanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-13 capacitance
         * symbol(s): `C`
         * application domain: generic
         * name: Capacitance
         * quantity dimension: L^-2*M^-1*T^4*I^2
         * measurement unit(s): F
         * tensor order: 0
         * definition: `C = Q/U` where `Q` is electric charge (item 6-2) and `U` is voltage (6-11.3)
         * remarks: See IEC 60050-131, item 131-12-13.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CapacitanceUnit [1];
    }

    attribute capacitance : CapacitanceValue :> scalarQuantities [*] nonunique;

    attribute def CapacitanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 4;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-14.1 electric constant, permittivity of vacuum */
    attribute def ElectricConstantValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-14.1 electric constant, permittivity of vacuum
         * symbol(s): `ε_0`
         * application domain: generic
         * name: ElectricConstant
         * quantity dimension: L^-3*M^-1*T^4*I^2
         * measurement unit(s): F/m
         * tensor order: 0
         * definition: `ε_0 = 1 / (μ_0 * c_0^2)` where `μ_0` is the magnetic constant (item 6-26.1) and `c_0` is the speed of light (item 6-35.2)
         * remarks: `ε_0 = 8.854188 * 10^-12` F/m. See IEC 60050-121, item 121-11-03.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricConstantUnit [1];
    }

    attribute electricConstant : ElectricConstantValue :> scalarQuantities [*] nonunique;

    attribute def ElectricConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 4;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias PermittivityOfVacuumUnit for ElectricConstantUnit;
    alias PermittivityOfVacuumValue for ElectricConstantValue;
    alias permittivityOfVacuum for electricConstant;

    /* IEC-80000-6 item 6-14.2 permittivity */
    attribute def PermittivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-14.2 permittivity
         * symbol(s): `ε`
         * application domain: generic
         * name: Permittivity
         * quantity dimension: L^-3*M^-1*T^4*I^2
         * measurement unit(s): F/m
         * tensor order: 0
         * definition: `vec(D) = ε vec(E)` where `vec(D)` is electric flux density (item 6-12) and `vec(E)` is electric field strength (item 6-10)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium, permittivity is a second order tensor. See IEC 60050-121, item 121-12-12.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PermittivityUnit [1];
    }

    attribute permittivity : PermittivityValue :> scalarQuantities [*] nonunique;

    attribute def PermittivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 4;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-15 relative permittivity */
    attribute def RelativePermittivityValue :> DimensionOneValue {
        doc /*
         * source: item 6-15 relative permittivity
         * symbol(s): `ε_r`
         * application domain: generic
         * name: RelativePermittivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `ε_r = ε / ε_0` where `ε` is permittivity (item 6-14.2) and `ε_0` is the electric constant (item 6-14.1)
         * remarks: See IEC 60050-121, item 121-12-13.
         */
    }
    attribute relativePermittivity : RelativePermittivityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-16 electric susceptibility */
    attribute def ElectricSusceptibilityValue :> DimensionOneValue {
        doc /*
         * source: item 6-16 electric susceptibility
         * symbol(s): `χ`
         * application domain: generic
         * name: ElectricSusceptibility (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `vec(P) = ε_0 χ vec(E)` where `vec(P)` is electric polarization (item 6-7), `ε_0` is the electric constant (item 6-14. 1) and `vec(E)` is electric field strength (item 6-10)
         * remarks: `χ = ε_r - 1`. The definition applies to an isotropic medium. For an anisotropic medium, electric susceptibility is a second order tensor. See IEC 60050-121, item 121-12-19.
         */
    }
    attribute electricSusceptibility : ElectricSusceptibilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-17 electric flux */
    attribute def ElectricFluxValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-17 electric flux
         * symbol(s): `Ψ`
         * application domain: generic
         * name: ElectricFlux
         * quantity dimension: T^1*I^1
         * measurement unit(s): C
         * tensor order: 0
         * definition: `Ψ = int_S vec(D) * vec(e_n) dA` over a surface `S`, where `vec(D)` is electric flux (item 6-12) en `vec(e_n) dA` is the vector surface element (ISO 80000-3 item 3-3)
         * remarks: See IEC 60050-121, item 121-11-41.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricFluxUnit [1];
    }

    attribute electricFlux : ElectricFluxValue :> scalarQuantities [*] nonunique;

    attribute def ElectricFluxUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-18 displacement current density */
    attribute def DisplacementCurrentDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-18 displacement current density (magnitude)
         * symbol(s): `J_D`
         * application domain: generic
         * name: DisplacementCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J_D) = (del vec(D))/(del t)` where `vec(D)` is electric flux density (item 6-12) and `t` is time (ISO 80000-3, item 3-7) 
         * remarks: See IEC 60050-121, item 121-11-42.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DisplacementCurrentDensityUnit [1];
    }

    attribute displacementCurrentDensity : DisplacementCurrentDensityValue :> scalarQuantities [*] nonunique;

    attribute def DisplacementCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianDisplacementCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-18 displacement current density (vector)
         * symbol(s): `vec(J_D)`
         * application domain: generic
         * name: DisplacementCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J_D) = (del vec(D))/(del t)` where `vec(D)` is electric flux density (item 6-12) and `t` is time (ISO 80000-3, item 3-7) 
         * remarks: See IEC 60050-121, item 121-11-42.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianDisplacementCurrentDensity3dCoordinateFrame [1];
    }

    attribute cartesianDisplacementCurrentDensity3dVector : CartesianDisplacementCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianDisplacementCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : DisplacementCurrentDensityUnit [3];
    }

    /* IEC-80000-6 item 6-19.1 displacement current */
    attribute displacementCurrent : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-19.1 displacement current
         * symbol(s): `I_D`
         * application domain: generic
         * name: DisplacementCurrent (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `I = int_S vec(J_D) * vec(e_n) dA` over a surface `S`, where `vec(J_D)` is displacement current density (item 6-18) en `vec(e_n) dA` is the vector surface element (ISO 80000-3 item 3-3)
         * remarks: See IEC 60050-121, item 121-11-43.
         */
    }

    /* IEC-80000-6 item 6-19.2 total current */
    attribute totalCurrent : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-19.2 total current
         * symbol(s): `I_"tot"`, `I_t`
         * application domain: generic
         * name: TotalCurrent (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `I_(tot) = I + I_D` where `I` is electric current (item 6-1) and `I_D` is displacement current (item 6-19.1)
         * remarks: See IEC 60050-121, item 121-11-45.
         */
    }

    /* IEC-80000-6 item 6-20 total current density */
    attribute def TotalCurrentDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-20 total current density (magnitude)
         * symbol(s): `J_"tot"`, `J_t`
         * application domain: generic
         * name: TotalCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 0
         * definition: `vec(J_(tot)) = vec(J) +vec(J_D)` where `vec(J)` is electric current density (item 6-8) and `vec(J_D)` is displacement current density (item 6-18)
         * remarks: See IEC 60050-121, item 121-11-44.
         */
        attribute :>> num : Real;
        attribute :>> mRef : TotalCurrentDensityUnit [1];
    }

    attribute totalCurrentDensity : TotalCurrentDensityValue :> scalarQuantities [*] nonunique;

    attribute def TotalCurrentDensityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianTotalCurrentDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-20 total current density (vector)
         * symbol(s): `vec(J_"tot")`, `vec(J_t)`
         * application domain: generic
         * name: TotalCurrentDensity
         * quantity dimension: L^-2*I^1
         * measurement unit(s): A/m^2
         * tensor order: 1
         * definition: `vec(J_(tot)) = vec(J) +vec(J_D)` where `vec(J)` is electric current density (item 6-8) and `vec(J_D)` is displacement current density (item 6-18)
         * remarks: See IEC 60050-121, item 121-11-44.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianTotalCurrentDensity3dCoordinateFrame [1];
    }

    attribute cartesianTotalCurrentDensity3dVector : CartesianTotalCurrentDensity3dVector :> vectorQuantities;

    attribute def CartesianTotalCurrentDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : TotalCurrentDensityUnit [3];
    }

    /* IEC-80000-6 item 6-21 magnetic flux density */
    attribute def MagneticFluxDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-21 magnetic flux density (magnitude)
         * symbol(s): `B`
         * application domain: generic
         * name: MagneticFluxDensity
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 0
         * definition: `vec(F) = q vec(v) xx vec(B)` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1) of any test particle with electric charge `q` (item 6-2)
         * remarks: The magnetic flux density has zero divergence, `nabla * vec(B) = 0`. See IEC 60050-121, item 121-11-19.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticFluxDensityUnit [1];
    }

    attribute magneticFluxDensity : MagneticFluxDensityValue :> scalarQuantities [*] nonunique;

    attribute def MagneticFluxDensityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticFluxDensity3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-21 magnetic flux density (vector)
         * symbol(s): `vec(B)`
         * application domain: generic
         * name: MagneticFluxDensity
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 1
         * definition: `vec(F) = q vec(v) xx vec(B)` where `vec(F)` is force (ISO 80000-4, item 4-9.1) and `vec(v)` is velocity (ISO 80000-3, item 3-8.1) of any test particle with electric charge `q` (item 6-2)
         * remarks: The magnetic flux density has zero divergence, `nabla * vec(B) = 0`. See IEC 60050-121, item 121-11-19.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticFluxDensity3dCoordinateFrame [1];
    }

    attribute cartesianMagneticFluxDensity3dVector : CartesianMagneticFluxDensity3dVector :> vectorQuantities;

    attribute def CartesianMagneticFluxDensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticFluxDensityUnit [3];
    }

    /* IEC-80000-6 item 6-22.1 magnetic flux */
    attribute def MagneticFluxValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-22.1 magnetic flux
         * symbol(s): `Φ`
         * application domain: generic
         * name: MagneticFlux
         * quantity dimension: L^2*M^1*T^-2*I^-1
         * measurement unit(s): Wb
         * tensor order: 0
         * definition: `Φ = int_S vec(B) * vec(e_n) dA` over a surface `S`, where `vec(B)` is magnetic flux density (item 6-21) and `vec(e_n) dA` is vector surface element (ISO 80000-3, item 3-3)
         * remarks: See IEC 60050-121, item 121-11-21.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticFluxUnit [1];
    }

    attribute magneticFlux : MagneticFluxValue :> scalarQuantities [*] nonunique;

    attribute def MagneticFluxUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-22.2 linked flux */
    attribute def LinkedFluxValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-22.2 linked flux
         * symbol(s): `Ψ_m`, `Ψ`
         * application domain: generic
         * name: LinkedFlux
         * quantity dimension: L^2*M^1*T^-2*I^-1
         * measurement unit(s): Wb
         * tensor order: 0
         * definition: `Ψ_m = int_C vec(A) * d vec(r)` where `vec(A)` is magnetic vector potential (item 6-32) and `d vec(r)` is line vector element of the curve `C`
         * remarks: Line vector element `d vec(r)` is the differential of position vector `vec(r)` (ISO 80000-3, item 3-1.11). See IEC 60050-121, item 121-11-24.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinkedFluxUnit [1];
    }

    attribute linkedFlux : LinkedFluxValue :> scalarQuantities [*] nonunique;

    attribute def LinkedFluxUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-23 magnetic moment, magnetic area moment */
    attribute def MagneticMomentValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-23 magnetic moment, magnetic area moment (magnitude)
         * symbol(s): `m`
         * application domain: generic
         * name: MagneticMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): A*m^2
         * tensor order: 0
         * definition: `vec(m) = I vec(e_n) A` where `I` is electric current (item 6-1) in a small closed loop, `vec(e_n)` is a unit vector perpendicular to the loop, and `A` is area (ISO 80000-3, item 3-3) of the loop
         * remarks: The magnetic moment of a substance within a domain is the vector sum of the magnetic moments of all entities included in the domain. See IEC 60050-121, items 121-11-49 and 121-11-50.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticMomentUnit [1];
    }

    attribute magneticMoment : MagneticMomentValue :> scalarQuantities [*] nonunique;

    attribute def MagneticMomentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticMoment3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-23 magnetic moment, magnetic area moment (vector)
         * symbol(s): `vec(m)`
         * application domain: generic
         * name: MagneticMoment
         * quantity dimension: L^2*I^1
         * measurement unit(s): A*m^2
         * tensor order: 1
         * definition: `vec(m) = I vec(e_n) A` where `I` is electric current (item 6-1) in a small closed loop, `vec(e_n)` is a unit vector perpendicular to the loop, and `A` is area (ISO 80000-3, item 3-3) of the loop
         * remarks: The magnetic moment of a substance within a domain is the vector sum of the magnetic moments of all entities included in the domain. See IEC 60050-121, items 121-11-49 and 121-11-50.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticMoment3dCoordinateFrame [1];
    }

    attribute cartesianMagneticMoment3dVector : CartesianMagneticMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticMomentUnit [3];
    }

    alias CartesianMagneticAreaMoment3dCoordinateFrame for CartesianMagneticMoment3dCoordinateFrame;
    alias cartesianMagneticAreaMoment3dVector for cartesianMagneticMoment3dVector;

    /* IEC-80000-6 item 6-24 magnetization */
    attribute def MagnetizationValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-24 magnetization (magnitude)
         * symbol(s): `M`, `H_i`
         * application domain: generic
         * name: Magnetization
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(M) = (d vec(m)) / (dV)` where `vec(m)` is magnetic moment (item 6-23) of a substance in a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-52.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagnetizationUnit [1];
    }

    attribute magnetization : MagnetizationValue :> scalarQuantities [*] nonunique;

    attribute def MagnetizationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagnetization3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-24 magnetization (vector)
         * symbol(s): `vec(M)`, `vec(H_i)`
         * application domain: generic
         * name: Magnetization
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(M) = (d vec(m)) / (dV)` where `vec(m)` is magnetic moment (item 6-23) of a substance in a domain with volume `V` (ISO 80000-3, item 3-4)
         * remarks: See IEC 60050-121, item 121-11-52.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagnetization3dCoordinateFrame [1];
    }

    attribute cartesianMagnetization3dVector : CartesianMagnetization3dVector :> vectorQuantities;

    attribute def CartesianMagnetization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagnetizationUnit [3];
    }

    /* IEC-80000-6 item 6-25 magnetic field strength, magnetizing field */
    attribute def MagneticFieldStrengthValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-25 magnetic field strength, magnetizing field (magnitude)
         * symbol(s): `H`
         * application domain: generic
         * name: MagneticFieldStrength
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: `vec(H) = vec(B)/μ_0 - vec(M)` where `vec(B)` is magnetic flux density (item 6-21), `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: The magnetic field strength is related to the total current density `vec(J_(t ot))` (item 6-20) via `rot vec(H) = vec(J_(t ot))`. See IEC 60050-121, item 121-11-56.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticFieldStrengthUnit [1];
    }

    attribute magneticFieldStrength : MagneticFieldStrengthValue :> scalarQuantities [*] nonunique;

    attribute def MagneticFieldStrengthUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticFieldStrength3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-25 magnetic field strength, magnetizing field (vector)
         * symbol(s): `vec(H)`
         * application domain: generic
         * name: MagneticFieldStrength
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 1
         * definition: `vec(H) = vec(B)/μ_0 - vec(M)` where `vec(B)` is magnetic flux density (item 6-21), `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: The magnetic field strength is related to the total current density `vec(J_(t ot))` (item 6-20) via `rot vec(H) = vec(J_(t ot))`. See IEC 60050-121, item 121-11-56.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticFieldStrength3dCoordinateFrame [1];
    }

    attribute cartesianMagneticFieldStrength3dVector : CartesianMagneticFieldStrength3dVector :> vectorQuantities;

    attribute def CartesianMagneticFieldStrength3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticFieldStrengthUnit [3];
    }

    alias CartesianMagnetizingField3dCoordinateFrame for CartesianMagneticFieldStrength3dCoordinateFrame;
    alias cartesianMagnetizingField3dVector for cartesianMagneticFieldStrength3dVector;

    /* IEC-80000-6 item 6-26.1 magnetic constant, permeability of vacuum */
    attribute def MagneticConstantValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-26.1 magnetic constant, permeability of vacuum
         * symbol(s): `μ_0`
         * application domain: generic
         * name: MagneticConstant
         * quantity dimension: L^1*M^1*T^-2*I^-2
         * measurement unit(s): H/m
         * tensor order: 0
         * definition: `μ_0 = 4 π * 10^-7` H/m
         * remarks: For this definition of `μ_0` see item 6-1.a. `μ_0 ~~ 1.256637 * 10^-6` H/m. See IEC 60050-121, item 121-11-14.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticConstantUnit [1];
    }

    attribute magneticConstant : MagneticConstantValue :> scalarQuantities [*] nonunique;

    attribute def MagneticConstantUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias PermeabilityOfVacuumUnit for MagneticConstantUnit;
    alias PermeabilityOfVacuumValue for MagneticConstantValue;
    alias permeabilityOfVacuum for magneticConstant;

    /* IEC-80000-6 item 6-26.2 permeability */
    attribute def PermeabilityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-26.2 permeability
         * symbol(s): `μ`
         * application domain: generic
         * name: Permeability
         * quantity dimension: L^1*M^1*T^-2*I^-2
         * measurement unit(s): H/m
         * tensor order: 0
         * definition: `vec(B) = μ vec(H)` where `vec(B)` is magnetic flux density (item 6-21) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium permeability is a second order tensor. See IEC 60050-121, item 121-12-28.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PermeabilityUnit [1];
    }

    attribute permeability : PermeabilityValue :> scalarQuantities [*] nonunique;

    attribute def PermeabilityUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-27 relative permeability */
    attribute def RelativePermeabilityValue :> DimensionOneValue {
        doc /*
         * source: item 6-27 relative permeability
         * symbol(s): `μ_r`
         * application domain: generic
         * name: RelativePermeability (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `μ_r = μ / μ_0`  where `μ` is permeability (item 6-24) and `μ_0` is the magnetic constant (item 6-26.1)
         * remarks: See IEC 60050-121, item 121-12-29.
         */
    }
    attribute relativePermeability : RelativePermeabilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-28 magnetic susceptibility */
    attribute def MagneticSusceptibilityValue :> DimensionOneValue {
        doc /*
         * source: item 6-28 magnetic susceptibility
         * symbol(s): `κ`, `χ_m`
         * application domain: generic
         * name: MagneticSusceptibility (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `vec(M) = κ vec(H)` where `vec(M)` is magnetization (item 6-24) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: `κ = μ_r - 1` This definition applies to an isotropic medium. For an anisotropic medium magnetic susceptibility is a second order tensor. See IEC 60050-121, item 121-12-37.
         */
    }
    attribute magneticSusceptibility : MagneticSusceptibilityValue :> scalarQuantities;

    /* IEC-80000-6 item 6-29 magnetic polarization */
    attribute def MagneticPolarizationValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-29 magnetic polarization (magnitude)
         * symbol(s): `J_m`
         * application domain: generic
         * name: MagneticPolarization
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 0
         * definition: `vec(J_m) = μ_0 vec(M)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: See IEC 60050-121, item 121-11-54.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticPolarizationUnit [1];
    }

    attribute magneticPolarization : MagneticPolarizationValue :> scalarQuantities [*] nonunique;

    attribute def MagneticPolarizationUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticPolarization3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-29 magnetic polarization (vector)
         * symbol(s): `vec(J_m)`
         * application domain: generic
         * name: MagneticPolarization
         * quantity dimension: M^1*T^-2*I^-1
         * measurement unit(s): T
         * tensor order: 1
         * definition: `vec(J_m) = μ_0 vec(M)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(M)` is magnetization (item 6-24)
         * remarks: See IEC 60050-121, item 121-11-54.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticPolarization3dCoordinateFrame [1];
    }

    attribute cartesianMagneticPolarization3dVector : CartesianMagneticPolarization3dVector :> vectorQuantities;

    attribute def CartesianMagneticPolarization3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticPolarizationUnit [3];
    }

    /* IEC-80000-6 item 6-30 magnetic dipole moment */
    attribute def MagneticDipoleMomentValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-30 magnetic dipole moment (magnitude)
         * symbol(s): `j_m`, `j`
         * application domain: generic
         * name: MagneticDipoleMoment
         * quantity dimension: L^3*M^1*T^-2*I^-1
         * measurement unit(s): Wb*m
         * tensor order: 0
         * definition: `vec(j_m) = μ_0 vec(m)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(m)` is magnetic moment (item 6-23)
         * remarks: See IEC 60050-121, item 121-11-55.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticDipoleMomentUnit [1];
    }

    attribute magneticDipoleMoment : MagneticDipoleMomentValue :> scalarQuantities [*] nonunique;

    attribute def MagneticDipoleMomentUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticDipoleMoment3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-30 magnetic dipole moment (vector)
         * symbol(s): `vec(j_m)`, `vec(j)`
         * application domain: generic
         * name: MagneticDipoleMoment
         * quantity dimension: L^3*M^1*T^-2*I^-1
         * measurement unit(s): Wb*m
         * tensor order: 1
         * definition: `vec(j_m) = μ_0 vec(m)` where `μ_0` is the magnetic constant (item 6-26.1), and `vec(m)` is magnetic moment (item 6-23)
         * remarks: See IEC 60050-121, item 121-11-55.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianMagneticDipoleMoment3dCoordinateFrame [1];
    }

    attribute cartesianMagneticDipoleMoment3dVector : CartesianMagneticDipoleMoment3dVector :> vectorQuantities;

    attribute def CartesianMagneticDipoleMoment3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticDipoleMomentUnit [3];
    }

    /* IEC-80000-6 item 6-31 coercivity */
    attribute def CoercivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-31 coercivity
         * symbol(s): `H_(c,B)`
         * application domain: generic
         * name: Coercivity
         * quantity dimension: L^-1*I^1
         * measurement unit(s): A/m
         * tensor order: 0
         * definition: magnetic field strength (item 6-25) to be applied to bring the magnetic flux density (item 6-21) in a substance from its remaining magnetic flux density to zero
         * remarks: See IEC 60050-121, item 121-12-69. Also called coercive field strength.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CoercivityUnit [1];
    }

    attribute coercivity : CoercivityValue :> scalarQuantities [*] nonunique;

    attribute def CoercivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-32 magnetic vector potential */
    attribute def MagneticVectorPotentialValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-32 magnetic vector potential (magnitude)
         * symbol(s): `A`
         * application domain: generic
         * name: MagneticVectorPotential
         * quantity dimension: L^1*M^1*T^-2*I^-1
         * measurement unit(s): Wb/m
         * tensor order: 0
         * definition: `vec(B) = rot vec(A)` where `vec(B)` is magnetic flux density (item 6-21)
         * remarks: The magnetic vector potential is not unique since any irrotational vector field can be added to it without changing its rotation. See IEC 60050-121, item 121-11-23.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagneticVectorPotentialUnit [1];
    }

    attribute magneticVectorPotential : MagneticVectorPotentialValue :> scalarQuantities [*] nonunique;

    attribute def MagneticVectorPotentialUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    attribute def CartesianMagneticVectorPotential3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-32 magnetic vector potential (vector)
         * symbol(s): `vec(A)`
         * application domain: generic
         * name: MagneticVectorPotential
         * quantity dimension: L^1*M^1*T^-2*I^-1
         * measurement unit(s): Wb/m
         * tensor order: 1
         * definition: `vec(B) = rot vec(A)` where `vec(B)` is magnetic flux density (item 6-21)
         * remarks: The magnetic vector potential is not unique since any irrotational vector field can be added to it without changing its rotation. See IEC 60050-121, item 121-11-23.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : CartesianMagneticVectorPotential3dCoordinateFrame [1];
    }

    attribute cartesianMagneticVectorPotential3dVector : CartesianMagneticVectorPotential3dVector :> vectorQuantities;

    attribute def CartesianMagneticVectorPotential3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = true;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : MagneticVectorPotentialUnit [3];
    }

    /* IEC-80000-6 item 6-33 electromagnetic energy density, volumic electromagnetic energy */
    attribute def ElectromagneticEnergyDensityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-33 electromagnetic energy density, volumic electromagnetic energy
         * symbol(s): `w`
         * application domain: generic
         * name: ElectromagneticEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3
         * tensor order: 0
         * definition: `ω = 1/2*(vec(E)*vec(D) + vec(B) * vec(H))` where `vec(E)` is electric field strength (item 6-10), `vec(D)` is electric flux density (item 6-12), `vec(B)` is magnetic flux density (item 6-21), and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-65.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectromagneticEnergyDensityUnit [1];
    }

    attribute electromagneticEnergyDensity : ElectromagneticEnergyDensityValue :> scalarQuantities [*] nonunique;

    attribute def ElectromagneticEnergyDensityUnit :> DerivedUnit {
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

    alias VolumicElectromagneticEnergyUnit for ElectromagneticEnergyDensityUnit;
    alias VolumicElectromagneticEnergyValue for ElectromagneticEnergyDensityValue;
    alias volumicElectromagneticEnergy for electromagneticEnergyDensity;

    /* IEC-80000-6 item 6-34 Poynting vector */
    attribute def PoyntingVectorMagnitudeValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-34 Poynting vector (magnitude)
         * symbol(s): `S`
         * application domain: generic
         * name: PoyntingVectorMagnitude
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2
         * tensor order: 0
         * definition: `vec(S) = vec(E) xx vec(H)` where `vec(E)` is electric field strength (item 6-10) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-66.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PoyntingVectorMagnitudeUnit [1];
    }

    attribute poyntingVectorMagnitude : PoyntingVectorMagnitudeValue :> scalarQuantities [*] nonunique;

    attribute def PoyntingVectorMagnitudeUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF);
        }
    }

    attribute def CartesianPoynting3dVector :> '3dVectorQuantityValue' {
        doc /*
         * source: item 6-34 Poynting vector
         * symbol(s): `vec(S)`
         * application domain: generic
         * name: PoyntingVector
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2
         * tensor order: 1
         * definition: `vec(S) = vec(E) xx vec(H)` where `vec(E)` is electric field strength (item 6-10) and `vec(H)` is magnetic field strength (item 6-25)
         * remarks: See IEC 60050-121, item 121-11-66.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianPoynting3dCoordinateFrame [1];
    }

    attribute cartesianPoynting3dVector : CartesianPoynting3dVector :> vectorQuantities;

    attribute def CartesianPoynting3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : PoyntingVectorMagnitudeUnit [3];
    }

    /* IEC-80000-6 item 6-35.1 phase speed of electromagnetic waves */
    attribute def PhaseSpeedOfElectromagneticWavesValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-35.1 phase speed of electromagnetic waves
         * symbol(s): `c`
         * application domain: generic
         * name: PhaseSpeedOfElectromagneticWaves
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s
         * tensor order: 0
         * definition: `c = ω/k` where `ω` is angular frequency (ISO 80000-3, item 3-16) and `k` is angular wavenumber (ISO 80000-3, item 3-19)
         * remarks: See ISO 80000-3, item 3-20.1.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseSpeedOfElectromagneticWavesUnit [1];
    }

    attribute phaseSpeedOfElectromagneticWaves : PhaseSpeedOfElectromagneticWavesValue :> scalarQuantities [*] nonunique;

    attribute def PhaseSpeedOfElectromagneticWavesUnit :> DerivedUnit {
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

    /* IEC-80000-6 item 6-35.2 speed of light, light speed */
    attribute def SpeedOfLightValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-35.2 speed of light, light speed
         * symbol(s): `c_0`
         * application domain: generic
         * name: SpeedOfLight
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s
         * tensor order: 0
         * definition: speed of electromagnetic waves in vacuum; `c_0 = 299792458` m/s
         * remarks: For this value of `c_0` see ISO 80000-3, item 3-1.a. `c_0 = 1/sqrt(ε_0 μ_0)`. See IEC 60050-111, item 111-13-07.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpeedOfLightUnit [1];
    }

    attribute speedOfLight : SpeedOfLightValue :> scalarQuantities [*] nonunique;

    attribute def SpeedOfLightUnit :> DerivedUnit {
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

    alias LightSpeedUnit for SpeedOfLightUnit;
    alias LightSpeedValue for SpeedOfLightValue;
    alias lightSpeed for speedOfLight;

    /* IEC-80000-6 item 6-36 source voltage, source tension */
    attribute def SourceVoltageValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-36 source voltage, source tension
         * symbol(s): `U_s`
         * application domain: generic
         * name: SourceVoltage
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: voltage (item 6-11.3) between the two terminals of a voltage source when there is no electric current (item 6-1) through the source
         * remarks: The name "electromotive force" with the abbreviation EMF and the symbol `E` is deprecated. See IEC 60050-131, item 131-12-22.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SourceVoltageUnit [1];
    }

    attribute sourceVoltage : SourceVoltageValue :> scalarQuantities [*] nonunique;

    attribute def SourceVoltageUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias SourceTensionUnit for SourceVoltageUnit;
    alias SourceTensionValue for SourceVoltageValue;
    alias sourceTension for sourceVoltage;

    /* IEC-80000-6 item 6-37.1 scalar magnetic potential */
    attribute scalarMagneticPotential : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-37.1 scalar magnetic potential
         * symbol(s): `V_m`, `φ`
         * application domain: generic
         * name: ScalarMagneticPotential (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: for an irrotational magnetic field strength `vec(H) =  -nabla V_m` where `vec(H)` is magnetic field strength (item 6-25)
         * remarks: The magnetic scalar potential is not unique since any constant scalar field can be added to it without changing its gradient. See IEC 60050-121, item 121-11-58.
         */
    }

    /* IEC-80000-6 item 6-37.2 magnetic tension */
    attribute magneticTension : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-37.2 magnetic tension
         * symbol(s): `U_m`
         * application domain: generic
         * name: MagneticTension (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `U_m = int_(vec(r_a) (C))^(vec(r_b)) vec(H) * d(vec(r))` where `vec(H)` is magnetic field strength (item 6-25) and `vec(r)` is position vector (ISO 80000-3, item 3-1.11) along a given curve `C` from point `a` to point `b`
         * remarks: For an irrotational magnetic field strength this quantity is equal to the magnetic potential difference. See IEC 60050-121, item121-11-57.
         */
    }

    /* IEC-80000-6 item 6-37.3 magnetomotive force */
    attribute def MagnetomotiveForceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-37.3 magnetomotive force
         * symbol(s): `F_m`
         * application domain: generic
         * name: MagnetomotiveForce
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: `F_m = oint_C vec(H) * d vec(r)` where `vec(H)` is magnetic field strength (item 6-25) and `vec(r)` is position vector (ISO 80000-3, item 3-1 .11) along a closed curve `C`
         * remarks: This quantity name is under consideration . Compare remark to item 6-36. See IEC 60050-121, item 121-11-60.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MagnetomotiveForceUnit [1];
    }

    attribute magnetomotiveForce : MagnetomotiveForceValue :> scalarQuantities [*] nonunique;

    attribute def MagnetomotiveForceUnit :> DerivedUnit {
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = electricCurrentPF;
        }
    }

    /* IEC-80000-6 item 6-37.4 current linkage */
    attribute currentLinkage : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-37.4 current linkage
         * symbol(s): `Θ`
         * application domain: generic
         * name: CurrentLinkage (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: net electric current (item 6-1) through a surface delimited by a closed loop
         * remarks: When `Θ` results from `N` (item 6-38) equal electric currents `I` (item 6-1 ), then `Θ = N I`. See IEC 60050-121 , item 121 -11-46.
         */
    }

    /* IEC-80000-6 item 6-38 number of turns in a winding */
    attribute numberOfTurnsInAWinding : CountValue :> scalarQuantities {
        doc /*
         * source: item 6-38 number of turns in a winding
         * symbol(s): `N`
         * application domain: generic
         * name: NumberOfTurnsInAWinding (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of turns in a winding (same as the quantity name)
         * remarks: N may be non-integer number, see ISO 80000-3, item 3-14.
         */
    }

    /* IEC-80000-6 item 6-39 reluctance */
    attribute def ReluctanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-39 reluctance
         * symbol(s): `R_m`, `R`
         * application domain: generic
         * name: Reluctance
         * quantity dimension: L^-2*M^-1*T^2*I^2
         * measurement unit(s): H^-1
         * tensor order: 0
         * definition: `R_m = U_m/Φ` where `U_m` is magnetic tension (item 6-37.2) and `Φ` is magnetic flux (item 6-22 .1)
         * remarks: See IEC 60050-131 , item 131-12-28.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ReluctanceUnit [1];
    }

    attribute reluctance : ReluctanceValue :> scalarQuantities [*] nonunique;

    attribute def ReluctanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 2;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-40 permeance */
    attribute def PermeanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-40 permeance
         * symbol(s): `Λ`
         * application domain: generic
         * name: Permeance
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `Λ = 1/R_m` where `R_m` is reluctance (item 6-39)
         * remarks: See IEC 60050-131 , item 131-12-29.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PermeanceUnit [1];
    }

    attribute permeance : PermeanceValue :> scalarQuantities [*] nonunique;

    attribute def PermeanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-41.1 inductance, self inductance */
    attribute def InductanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-41.1 inductance, self inductance
         * symbol(s): `L`, `L_m`
         * application domain: generic
         * name: Inductance
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `L = Ψ / I` where `I` is an electric current (item 6-1) in a thin conducting loop and `Ψ` is the linked flux (item 6-22.2) caused by that electric current
         * remarks: The name "self inductance" is used for the quantity associated to mutual inductance when `n = m`. See IEC 60050-131 , items 131-12-19 and 131 -12-35.
         */
        attribute :>> num : Real;
        attribute :>> mRef : InductanceUnit [1];
    }

    attribute inductance : InductanceValue :> scalarQuantities [*] nonunique;

    attribute def InductanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias SelfInductanceUnit for InductanceUnit;
    alias SelfInductanceValue for InductanceValue;
    alias selfInductance for inductance;

    /* IEC-80000-6 item 6-41.2 mutual inductance */
    attribute mutualInductance : InductanceValue :> scalarQuantities {
        doc /*
         * source: item 6-41.2 mutual inductance
         * symbol(s): `L_(mn)`
         * application domain: generic
         * name: MutualInductance (specializes Inductance)
         * quantity dimension: L^2*M^1*T^-2*I^-2
         * measurement unit(s): H
         * tensor order: 0
         * definition: `L_(mn) = Ψ_m / I_n` where `I_n` is an electric current (item 6-1) in a thin conducting loop `n` and `Ψ_m` is the linked flux (item 6-22.2) caused by that electric current in another loop `m`
         * remarks: `L_(mn) = L_(nm)`. For two loops , the symbol `M` is used for `L_(12)`. See IEC 60050-131, items 131-12-36.
         */
    }

    /* IEC-80000-6 item 6-42.1 coupling factor */
    attribute def CouplingFactorValue :> DimensionOneValue {
        doc /*
         * source: item 6-42.1 coupling factor
         * symbol(s): `k`
         * application domain: generic
         * name: CouplingFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for inductive coupling between two inductive elements `k = |L_(mn)| / sqrt(L_m L_n)` where `L_m` and `L_n` are their self inductances (item 6-41 .1 ), and `L_(mn)` is their mutual inductance (item 6-41.2)
         * remarks: See IEC 60050-131 , item 131-12-41.
         */
    }
    attribute couplingFactor : CouplingFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-42.2 leakage factor */
    attribute def LeakageFactorValue :> DimensionOneValue {
        doc /*
         * source: item 6-42.2 leakage factor
         * symbol(s): `σ`
         * application domain: generic
         * name: LeakageFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `σ = 1 - k^2` where `k` is the coupling factor (item 6-42 .1)
         * remarks: See IEC 60050-131 , item 131-12-42.
         */
    }
    attribute leakageFactor : LeakageFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-43 conductivity */
    attribute def ConductivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-43 conductivity
         * symbol(s): `σ`, `γ`
         * application domain: generic
         * name: Conductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m
         * tensor order: 0
         * definition: `vec(J) = σ vec(E)` where `vec(J)` is electric current density (item 6-8) and `vec(E)` is electric field strength (item 6-10)
         * remarks: This definition applies to an isotropic medium. For an anisotropic medium `σ` is a second order tensor. `κ` is used in electrochemistry. See IEC 60050-121 , item 121-12-03.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ConductivityUnit [1];
    }

    attribute conductivity : ConductivityValue :> scalarQuantities [*] nonunique;

    attribute def ConductivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-44 resistivity */
    attribute def ResistivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-44 resistivity
         * symbol(s): `ρ`
         * application domain: generic
         * name: Resistivity
         * quantity dimension: L^3*M^1*T^-3*I^-2
         * measurement unit(s): Ω*m
         * tensor order: 0
         * definition: `ρ = 1/σ` if is exists, where `σ` is conductivity (item 6-43)
         * remarks: See IEC 60050-121, item 121-12-04.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ResistivityUnit [1];
    }

    attribute resistivity : ResistivityValue :> scalarQuantities [*] nonunique;

    attribute def ResistivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 3;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-45 electric power, instantaneous power */
    attribute electricPower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-45 electric power, instantaneous power
         * symbol(s): `p`
         * application domain: generic
         * name: ElectricPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W
         * tensor order: 0
         * definition: `p = ui` where `u` is instantaneous voltage (item 6-11 .3) and `i` is instantaneous electric current (item 6-1)
         * remarks: See IEC 60050-131 , item 131-11-30.
         */
    }

    alias instantaneousPower for electricPower;

    /* IEC-80000-6 item 6-46 resistance */
    attribute def ResistanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-46 resistance
         * symbol(s): `R`
         * application domain: generic
         * name: Resistance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: for resistive component `R = u i` where `u` is instantaneous voltage (item 6-11.3) and `i` is instantaneous electric current (item 6-1)
         * remarks: For alternating current, see item 6-51.2. See IEC 60050-131, item 131-12-04.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ResistanceUnit [1];
    }

    attribute resistance : ResistanceValue :> scalarQuantities [*] nonunique;

    attribute def ResistanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-47 conductance */
    attribute def ConductanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-47 conductance
         * symbol(s): `G`
         * application domain: generic
         * name: Conductance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: for resistive component `G = 1/R` where `R` is resistance (item 6-46)
         * remarks: For alternating current, see item 6-52.2. See IEC 60050-131, item 131-12-06.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ConductanceUnit [1];
    }

    attribute conductance : ConductanceValue :> scalarQuantities [*] nonunique;

    attribute def ConductanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-48 phase difference */
    attribute def PhaseDifferenceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-48 phase difference
         * symbol(s): `φ`
         * application domain: generic
         * name: PhaseDifference
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: `φ = φ_u - φ_i` where `φ_u` is the initial phase of the voltage (item 6-11 .3) and `φ_i` is the initial phase of the electric current (item 6-1)
         * remarks: When `u = hat(U) cos(ωt - φ_u)`, `i = hat(I) cos(ωt - φ_i)` where `u` is the voltage (item 6-11 . 3) and `i` is the electric current (item 6-1 ), `ω` is angular frequency (ISO 80000-3, item 3-16) and `t` is time (ISO 80000-3, item 3-7), then `φ` is phase difference. For phase angle, see items 6-49 and 6-50.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseDifferenceUnit [1];
    }

    attribute phaseDifference : PhaseDifferenceValue :> scalarQuantities [*] nonunique;

    attribute def PhaseDifferenceUnit :> DimensionOneUnit { }

    /* IEC-80000-6 item 6-49 electric current phasor */
    attribute electricCurrentPhasor : ElectricCurrentValue :> scalarQuantities {
        doc /*
         * source: item 6-49 electric current phasor
         * symbol(s): `underline(I)`
         * application domain: generic
         * name: ElectricCurrentPhasor (specializes ElectricCurrent)
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: when `i = hat(I) cos(ωt + α)`, where `i` is the electric current (item 6-1 ), `ω` is angular frequency (ISO 80000-3, item 3-16), `t` is time (ISO 80000-3, item 3-7), and `α` is initial phase (ISO 80000-3, item 3-5), then `underline(l) = I e^(jα)`
         * remarks: `underline(l)` is the complex representation of the electric current `i = hat(I) cos(ωt + α)`. `j` is the imaginary unit.
         */
    }

    /* IEC-80000-6 item 6-50 voltage phasor */
    attribute voltagePhasor : ElectricPotentialDifferenceValue :> scalarQuantities {
        doc /*
         * source: item 6-50 voltage phasor
         * symbol(s): `underline(U)`
         * application domain: generic
         * name: VoltagePhasor (specializes ElectricPotentialDifference)
         * quantity dimension: L^2*M^1*T^-3*I^-1
         * measurement unit(s): V
         * tensor order: 0
         * definition: when `u = hat(U) cos(ωt + α)`, where `u` is the voltage (item 6-11.3 ), `ω` is angular frequency (ISO 80000-3, item 3-16), `t` is time (ISO 80000-3, item 3-7), and `α` is initial phase (ISO 80000-3, item 3-5), then `underline(U) = U e^(jα)`
         * remarks: `underline(U)` is the complex representation of the voltage `u = hat(U) cos(ωt + α)`. `j` is the imaginary unit.
         */
    }

    /* IEC-80000-6 item 6-51.1 impedance, complex impedance */
    attribute def ImpedanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-51.1 impedance, complex impedance
         * symbol(s): `underline(Z)`
         * application domain: generic
         * name: Impedance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `underline(Z) = underline(U)/underline(I)` where `underline(U)` is the voltage phasor (item 6-50), and `underline(I)` is the electric current phasor (item 6-49)
         * remarks: `underline(Z) = R + jX`, where `R` is resistance (item 6-51.2) and `X` is reactance (item 6-51 .3). `j` is the imaginary unit. `underline(Z) = |underline(Z)| e^(jφ)`. See IEC 60050-131 , item 131-12-43.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ImpedanceUnit [1];
    }

    attribute impedance : ImpedanceValue :> scalarQuantities [*] nonunique;

    attribute def ImpedanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias ComplexImpedanceUnit for ImpedanceUnit;
    alias ComplexImpedanceValue for ImpedanceValue;
    alias complexImpedance for impedance;

    /* IEC-80000-6 item 6-51.2 resistance to alternating current */
    attribute def ResistanceToAlternatingCurrentValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-51.2 resistance to alternating current
         * symbol(s): `R`
         * application domain: generic
         * name: ResistanceToAlternatingCurrent
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `R = "Re" underline(Z)` where `underline(Z)`, is impedance (item 6-5.1) and `"Re"` denotes the real part
         * remarks: See IEC 60050-131, item 131-12-45.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ResistanceToAlternatingCurrentUnit [1];
    }

    attribute resistanceToAlternatingCurrent : ResistanceToAlternatingCurrentValue :> scalarQuantities [*] nonunique;

    attribute def ResistanceToAlternatingCurrentUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-51.3 reactance */
    attribute def ReactanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-51.3 reactance
         * symbol(s): `X`
         * application domain: generic
         * name: Reactance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `X = "Im" underline(Z)` where `underline(Z)`, is impedance (item 6-5.1) and `"Im"` denotes the imaginary part
         * remarks: `X = ωL - 1/(ωC)`. See IEC 60050-131 , item 131-12-46.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ReactanceUnit [1];
    }

    attribute reactance : ReactanceValue :> scalarQuantities [*] nonunique;

    attribute def ReactanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-51.4 modulus of impedance */
    attribute def ModulusOfImpedanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-51.4 modulus of impedance
         * symbol(s): `Z`
         * application domain: generic
         * name: ModulusOfImpedance
         * quantity dimension: L^2*M^1*T^-3*I^-2
         * measurement unit(s): Ω
         * tensor order: 0
         * definition: `Z = |underline(Z)|` where `underline(Z)` is impedance (item 6-51.1)
         * remarks: See IEC 60050-131 , item 131-12-44. Apparent impedance is defined more generally as the quotient of rms voltage and rms electric  current; it is often denoted by `Z`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfImpedanceUnit [1];
    }

    attribute modulusOfImpedance : ModulusOfImpedanceValue :> scalarQuantities [*] nonunique;

    attribute def ModulusOfImpedanceUnit :> DerivedUnit {
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
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-52.1 admittance, complex admittance */
    attribute def AdmittanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-52.1 admittance, complex admittance
         * symbol(s): `underline(Y)`
         * application domain: generic
         * name: Admittance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `underline(Y) = 1/underline(Z)` where `underline(Z)` is impedance (item 6-51.1)
         * remarks: `underline(Y) = G + jB`, where `G` is conductance (item 6-52 .2) and `B` is susceptance (item 6-52 .3). `j` is the imaginary unit. `underline(Y) = |underline(Y)| e^-(jφ)`. See IEC 60050-131, item 131 -12-51.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AdmittanceUnit [1];
    }

    attribute admittance : AdmittanceValue :> scalarQuantities [*] nonunique;

    attribute def AdmittanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    alias ComplexAdmittanceUnit for AdmittanceUnit;
    alias ComplexAdmittanceValue for AdmittanceValue;
    alias complexAdmittance for admittance;

    /* IEC-80000-6 item 6-52.2 conductance for alternating current */
    attribute conductanceForAlternatingCurrent : ConductanceValue :> scalarQuantities {
        doc /*
         * source: item 6-52.2 conductance for alternating current
         * symbol(s): `G`
         * application domain: generic
         * name: ConductanceForAlternatingCurrent (specializes Conductance)
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `G = "Re" underline(Y)` where I is admittance (item 6-52.1)
         * remarks: See IEC 60050-131, item 131-12-53.
         */
    }

    /* IEC-80000-6 item 6-52.3 susceptance */
    attribute def SusceptanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-52.3 susceptance
         * symbol(s): `B`
         * application domain: generic
         * name: Susceptance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `B = "Im" underline(Y)` where `underline(Y)` is admittance (item 6-52.1)
         * remarks: See IEC 60050-131, item 131-12-54.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SusceptanceUnit [1];
    }

    attribute susceptance : SusceptanceValue :> scalarQuantities [*] nonunique;

    attribute def SusceptanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-52.4 modulus of admittance */
    attribute def ModulusOfAdmittanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 6-52.4 modulus of admittance
         * symbol(s): `Y`
         * application domain: generic
         * name: ModulusOfAdmittance
         * quantity dimension: L^-2*M^-1*T^3*I^2
         * measurement unit(s): S
         * tensor order: 0
         * definition: `Y = |underline(Y)|` where `underline(Y)` is admittance (item 6-52.1)
         * remarks: Apparent admittance is defined more generally as the quotient of rms electric current voltage and rms voltage; it is often denoted by `Y`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ModulusOfAdmittanceUnit [1];
    }

    attribute modulusOfAdmittance : ModulusOfAdmittanceValue :> scalarQuantities [*] nonunique;

    attribute def ModulusOfAdmittanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor [1] {
            :>> quantity = isq.I;
            :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }

    /* IEC-80000-6 item 6-53 quality factor */
    attribute def QualityFactorValue :> DimensionOneValue {
        doc /*
         * source: item 6-53 quality factor
         * symbol(s): `Q`
         * application domain: generic
         * name: QualityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for non-radiating systems, if `underline(Z) = R + jX`, then `Q = |X|/R` where `underline(Z)` is impedance (item 6-51. 1), `R` is resistance (item 6-51 .2), and `X` is reactance (item 6-51.3)
         * remarks: None.
         */
    }
    attribute qualityFactor : QualityFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-54 loss factor */
    attribute def LossFactorValue :> DimensionOneValue {
        doc /*
         * source: item 6-54 loss factor
         * symbol(s): `d`
         * application domain: generic
         * name: LossFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `d = 1/Q` where `Q` quality factor (item 6-53)
         * remarks: It is also named dissipation factor.
         */
    }
    attribute lossFactor : LossFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-55 loss angle */
    attribute lossAngle : AngularMeasureValue :> scalarQuantities {
        doc /*
         * source: item 6-55 loss angle
         * symbol(s): `δ`
         * application domain: generic
         * name: LossAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: `δ = arctan d` where `d` is loss factor (item 6-54)
         * remarks: See IEC 60050-131 , item 131-12-49.
         */
    }

    /* IEC-80000-6 item 6-56 active power */
    attribute activePower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-56 active power
         * symbol(s): `P`
         * application domain: generic
         * name: ActivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W
         * tensor order: 0
         * definition: `P = 1/T int_0^T p dt` where `T` is the period (ISO 80000-3, item 3-12) and `p` is instantaneous power (item 6-45)
         * remarks: In complex notation, `P = "Re" underline(S)` where `underline(S)` is complex power (item 6-59).
         */
    }

    /* IEC-80000-6 item 6-57 apparent power */
    attribute apparentPower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-57 apparent power
         * symbol(s): ``, `underline(S)`, ``
         * application domain: generic
         * name: ApparentPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `|underline(S)| = U I` where `U` is rms value of voltage (item 6-11.3 and `I` is rms value of electric current (item 6-1)
         * remarks: `U = sqrt(1/T int_0^T u^2 dt)` and `I = sqrt(1/T int_0^T i^2 dt)`. When `u = sqrt 2 U cos(ωt)` and `i = sqrt 2 I cos(ωt - φ)`, then `P = U I cos(φ)`, `Q = U I sin(φ)` and `λ = cos(φ)` . See IEC 60050-131, item 131-11-41 .
         */
    }

    /* IEC-80000-6 item 6-58 power factor */
    attribute def PowerFactorValue :> DimensionOneValue {
        doc /*
         * source: item 6-58 power factor
         * symbol(s): `λ`
         * application domain: generic
         * name: PowerFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: `λ = |P|/|S|` where `P`  is active power (item 6-56) and `S` is apparent power (item 6-57)
         * remarks: See I EC 60050-131, item 131-11-46.
         */
    }
    attribute powerFactor : PowerFactorValue :> scalarQuantities;

    /* IEC-80000-6 item 6-59 complex power */
    attribute complexPower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-59 complex power
         * symbol(s): `underline(S)`
         * application domain: generic
         * name: ComplexPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `underline(S) = underline(U) * underline(I)^"*"` where `underline(U)` is voltage phasor (item 6-50) and `underline(I)^"*"` is the complex conjugate of the current phasor (item 6-49)
         * remarks: `underline(S) = P + jQ` where `P` is active power (item 6-56) and `Q` is reactive power (item 6-60). See IEC 60050-131, item 131-11-39.
         */
    }

    /* IEC-80000-6 item 6-60 reactive power */
    attribute reactivePower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-60 reactive power
         * symbol(s): `Q`
         * application domain: generic
         * name: ReactivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A, var
         * tensor order: 0
         * definition: `Q = "Im" underline(S)` where `underline(S)` is complex power (item 6-59)
         * remarks: See IEC 60050-131, item 131-11-44.
         */
    }

    /* IEC-80000-6 item 6-61 non-active power */
    attribute nonActivePower : PowerValue :> scalarQuantities {
        doc /*
         * source: item 6-61 non-active power
         * symbol(s): `Q'`
         * application domain: generic
         * name: NonActivePower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): V*A
         * tensor order: 0
         * definition: `Q' = sqrt(|underline(S)|^2 - P^2)` where `|underline(S)|` is apparent power (item 6-57) and `P` is active power (item 6-56)
         * remarks: See IEC 60050-131, item 131-11-43.
         */
    }

    /* IEC-80000-6 item 6-62 active energy */
    attribute activeEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 6-62 active energy
         * symbol(s): `W`
         * application domain: generic
         * name: ActiveEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, W*h
         * tensor order: 0
         * definition: `W = int_(t_1)^(t_2) p dt` where `p` is instantaneous power (item 6-45), and the integral interval is the time interval from `t_1` to `t_2`
         * remarks: None.
         */
    }
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQElectromagnetism"))) (name "ISQElectromagnetism") (declared-name "ISQElectromagnetism")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit"))) (name "AdmittanceUnit") (declared-name "AdmittanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue"))) (name "AdmittanceValue") (declared-name "AdmittanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AngularMeasureValue"))) (name "AngularMeasureValue") (declared-name "AngularMeasureValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AreicElectricChargeUnit"))) (name "AreicElectricChargeUnit") (declared-name "AreicElectricChargeUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::AreicElectricChargeValue"))) (name "AreicElectricChargeValue") (declared-name "AreicElectricChargeValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit"))) (name "CapacitanceUnit") (declared-name "CapacitanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue"))) (name "CapacitanceValue") (declared-name "CapacitanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianAreicElectricCurrent3dCoordinateFrame"))) (name "CartesianAreicElectricCurrent3dCoordinateFrame") (declared-name "CartesianAreicElectricCurrent3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame"))) (name "CartesianDisplacementCurrentDensity3dCoordinateFrame") (declared-name "CartesianDisplacementCurrentDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector"))) (name "CartesianDisplacementCurrentDensity3dVector") (declared-name "CartesianDisplacementCurrentDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame"))) (name "CartesianElectricCurrentDensity3dCoordinateFrame") (declared-name "CartesianElectricCurrentDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector"))) (name "CartesianElectricCurrentDensity3dVector") (declared-name "CartesianElectricCurrentDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame"))) (name "CartesianElectricDipoleMoment3dCoordinateFrame") (declared-name "CartesianElectricDipoleMoment3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector"))) (name "CartesianElectricDipoleMoment3dVector") (declared-name "CartesianElectricDipoleMoment3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDisplacement3dCoordinateFrame"))) (name "CartesianElectricDisplacement3dCoordinateFrame") (declared-name "CartesianElectricDisplacement3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame"))) (name "CartesianElectricFieldStrength3dCoordinateFrame") (declared-name "CartesianElectricFieldStrength3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector"))) (name "CartesianElectricFieldStrength3dVector") (declared-name "CartesianElectricFieldStrength3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame"))) (name "CartesianElectricFluxDensity3dCoordinateFrame") (declared-name "CartesianElectricFluxDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector"))) (name "CartesianElectricFluxDensity3dVector") (declared-name "CartesianElectricFluxDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame"))) (name "CartesianElectricPolarization3dCoordinateFrame") (declared-name "CartesianElectricPolarization3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector"))) (name "CartesianElectricPolarization3dVector") (declared-name "CartesianElectricPolarization3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame"))) (name "CartesianLinearElectricCurrentDensity3dCoordinateFrame") (declared-name "CartesianLinearElectricCurrentDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector"))) (name "CartesianLinearElectricCurrentDensity3dVector") (declared-name "CartesianLinearElectricCurrentDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLineicElectricCurrent3dCoordinateFrame"))) (name "CartesianLineicElectricCurrent3dCoordinateFrame") (declared-name "CartesianLineicElectricCurrent3dCoordinateFrame"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticAreaMoment3dCoordinateFrame"))) (name "CartesianMagneticAreaMoment3dCoordinateFrame") (declared-name "CartesianMagneticAreaMoment3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame"))) (name "CartesianMagneticDipoleMoment3dCoordinateFrame") (declared-name "CartesianMagneticDipoleMoment3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector"))) (name "CartesianMagneticDipoleMoment3dVector") (declared-name "CartesianMagneticDipoleMoment3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame"))) (name "CartesianMagneticFieldStrength3dCoordinateFrame") (declared-name "CartesianMagneticFieldStrength3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector"))) (name "CartesianMagneticFieldStrength3dVector") (declared-name "CartesianMagneticFieldStrength3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame"))) (name "CartesianMagneticFluxDensity3dCoordinateFrame") (declared-name "CartesianMagneticFluxDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector"))) (name "CartesianMagneticFluxDensity3dVector") (declared-name "CartesianMagneticFluxDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame"))) (name "CartesianMagneticMoment3dCoordinateFrame") (declared-name "CartesianMagneticMoment3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector"))) (name "CartesianMagneticMoment3dVector") (declared-name "CartesianMagneticMoment3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame"))) (name "CartesianMagneticPolarization3dCoordinateFrame") (declared-name "CartesianMagneticPolarization3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector"))) (name "CartesianMagneticPolarization3dVector") (declared-name "CartesianMagneticPolarization3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame"))) (name "CartesianMagneticVectorPotential3dCoordinateFrame") (declared-name "CartesianMagneticVectorPotential3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector"))) (name "CartesianMagneticVectorPotential3dVector") (declared-name "CartesianMagneticVectorPotential3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame"))) (name "CartesianMagnetization3dCoordinateFrame") (declared-name "CartesianMagnetization3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector"))) (name "CartesianMagnetization3dVector") (declared-name "CartesianMagnetization3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetizingField3dCoordinateFrame"))) (name "CartesianMagnetizingField3dCoordinateFrame") (declared-name "CartesianMagnetizingField3dCoordinateFrame"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame"))) (name "CartesianPoynting3dCoordinateFrame") (declared-name "CartesianPoynting3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector"))) (name "CartesianPoynting3dVector") (declared-name "CartesianPoynting3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame"))) (name "CartesianTotalCurrentDensity3dCoordinateFrame") (declared-name "CartesianTotalCurrentDensity3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector"))) (name "CartesianTotalCurrentDensity3dVector") (declared-name "CartesianTotalCurrentDensity3dVector") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit"))) (name "CoercivityUnit") (declared-name "CoercivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue"))) (name "CoercivityValue") (declared-name "CoercivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ComplexAdmittanceUnit"))) (name "ComplexAdmittanceUnit") (declared-name "ComplexAdmittanceUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ComplexAdmittanceValue"))) (name "ComplexAdmittanceValue") (declared-name "ComplexAdmittanceValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ComplexImpedanceUnit"))) (name "ComplexImpedanceUnit") (declared-name "ComplexImpedanceUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ComplexImpedanceValue"))) (name "ComplexImpedanceValue") (declared-name "ComplexImpedanceValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit"))) (name "ConductanceUnit") (declared-name "ConductanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue"))) (name "ConductanceValue") (declared-name "ConductanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit"))) (name "ConductivityUnit") (declared-name "ConductivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue"))) (name "ConductivityValue") (declared-name "ConductivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue"))) (name "CouplingFactorValue") (declared-name "CouplingFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit"))) (name "DisplacementCurrentDensityUnit") (declared-name "DisplacementCurrentDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue"))) (name "DisplacementCurrentDensityValue") (declared-name "DisplacementCurrentDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit"))) (name "ElectricChargeDensityUnit") (declared-name "ElectricChargeDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue"))) (name "ElectricChargeDensityValue") (declared-name "ElectricChargeDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit"))) (name "ElectricChargeUnit") (declared-name "ElectricChargeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue"))) (name "ElectricChargeValue") (declared-name "ElectricChargeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit"))) (name "ElectricConstantUnit") (declared-name "ElectricConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue"))) (name "ElectricConstantValue") (declared-name "ElectricConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit"))) (name "ElectricCurrentDensityUnit") (declared-name "ElectricCurrentDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue"))) (name "ElectricCurrentDensityValue") (declared-name "ElectricCurrentDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit"))) (name "ElectricDipoleMomentUnit") (declared-name "ElectricDipoleMomentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue"))) (name "ElectricDipoleMomentValue") (declared-name "ElectricDipoleMomentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit"))) (name "ElectricFieldStrengthUnit") (declared-name "ElectricFieldStrengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue"))) (name "ElectricFieldStrengthValue") (declared-name "ElectricFieldStrengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit"))) (name "ElectricFluxDensityUnit") (declared-name "ElectricFluxDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue"))) (name "ElectricFluxDensityValue") (declared-name "ElectricFluxDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit"))) (name "ElectricFluxUnit") (declared-name "ElectricFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue"))) (name "ElectricFluxValue") (declared-name "ElectricFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit"))) (name "ElectricPolarizationUnit") (declared-name "ElectricPolarizationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue"))) (name "ElectricPolarizationValue") (declared-name "ElectricPolarizationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit"))) (name "ElectricPotentialDifferenceUnit") (declared-name "ElectricPotentialDifferenceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))) (name "ElectricPotentialDifferenceValue") (declared-name "ElectricPotentialDifferenceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit"))) (name "ElectricPotentialUnit") (declared-name "ElectricPotentialUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue"))) (name "ElectricPotentialValue") (declared-name "ElectricPotentialValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue"))) (name "ElectricSusceptibilityValue") (declared-name "ElectricSusceptibilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit"))) (name "ElectromagneticEnergyDensityUnit") (declared-name "ElectromagneticEnergyDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue"))) (name "ElectromagneticEnergyDensityValue") (declared-name "ElectromagneticEnergyDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit"))) (name "ImpedanceUnit") (declared-name "ImpedanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue"))) (name "ImpedanceValue") (declared-name "ImpedanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit"))) (name "InductanceUnit") (declared-name "InductanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue"))) (name "InductanceValue") (declared-name "InductanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue"))) (name "LeakageFactorValue") (declared-name "LeakageFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LightSpeedUnit"))) (name "LightSpeedUnit") (declared-name "LightSpeedUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LightSpeedValue"))) (name "LightSpeedValue") (declared-name "LightSpeedValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit"))) (name "LinearDensityOfElectricChargeUnit") (declared-name "LinearDensityOfElectricChargeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue"))) (name "LinearDensityOfElectricChargeValue") (declared-name "LinearDensityOfElectricChargeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit"))) (name "LinearElectricCurrentDensityUnit") (declared-name "LinearElectricCurrentDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue"))) (name "LinearElectricCurrentDensityValue") (declared-name "LinearElectricCurrentDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LineicElectricChargeUnit"))) (name "LineicElectricChargeUnit") (declared-name "LineicElectricChargeUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LineicElectricChargeValue"))) (name "LineicElectricChargeValue") (declared-name "LineicElectricChargeValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit"))) (name "LinkedFluxUnit") (declared-name "LinkedFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue"))) (name "LinkedFluxValue") (declared-name "LinkedFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue"))) (name "LossFactorValue") (declared-name "LossFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit"))) (name "MagneticConstantUnit") (declared-name "MagneticConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue"))) (name "MagneticConstantValue") (declared-name "MagneticConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit"))) (name "MagneticDipoleMomentUnit") (declared-name "MagneticDipoleMomentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue"))) (name "MagneticDipoleMomentValue") (declared-name "MagneticDipoleMomentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit"))) (name "MagneticFieldStrengthUnit") (declared-name "MagneticFieldStrengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue"))) (name "MagneticFieldStrengthValue") (declared-name "MagneticFieldStrengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit"))) (name "MagneticFluxDensityUnit") (declared-name "MagneticFluxDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue"))) (name "MagneticFluxDensityValue") (declared-name "MagneticFluxDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit"))) (name "MagneticFluxUnit") (declared-name "MagneticFluxUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue"))) (name "MagneticFluxValue") (declared-name "MagneticFluxValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit"))) (name "MagneticMomentUnit") (declared-name "MagneticMomentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue"))) (name "MagneticMomentValue") (declared-name "MagneticMomentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit"))) (name "MagneticPolarizationUnit") (declared-name "MagneticPolarizationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue"))) (name "MagneticPolarizationValue") (declared-name "MagneticPolarizationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue"))) (name "MagneticSusceptibilityValue") (declared-name "MagneticSusceptibilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit"))) (name "MagneticVectorPotentialUnit") (declared-name "MagneticVectorPotentialUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue"))) (name "MagneticVectorPotentialValue") (declared-name "MagneticVectorPotentialValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit"))) (name "MagnetizationUnit") (declared-name "MagnetizationUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue"))) (name "MagnetizationValue") (declared-name "MagnetizationValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit"))) (name "MagnetomotiveForceUnit") (declared-name "MagnetomotiveForceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue"))) (name "MagnetomotiveForceValue") (declared-name "MagnetomotiveForceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit"))) (name "ModulusOfAdmittanceUnit") (declared-name "ModulusOfAdmittanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue"))) (name "ModulusOfAdmittanceValue") (declared-name "ModulusOfAdmittanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit"))) (name "ModulusOfImpedanceUnit") (declared-name "ModulusOfImpedanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue"))) (name "ModulusOfImpedanceValue") (declared-name "ModulusOfImpedanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityOfVacuumUnit"))) (name "PermeabilityOfVacuumUnit") (declared-name "PermeabilityOfVacuumUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityOfVacuumValue"))) (name "PermeabilityOfVacuumValue") (declared-name "PermeabilityOfVacuumValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit"))) (name "PermeabilityUnit") (declared-name "PermeabilityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue"))) (name "PermeabilityValue") (declared-name "PermeabilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit"))) (name "PermeanceUnit") (declared-name "PermeanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue"))) (name "PermeanceValue") (declared-name "PermeanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityOfVacuumUnit"))) (name "PermittivityOfVacuumUnit") (declared-name "PermittivityOfVacuumUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityOfVacuumValue"))) (name "PermittivityOfVacuumValue") (declared-name "PermittivityOfVacuumValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit"))) (name "PermittivityUnit") (declared-name "PermittivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue"))) (name "PermittivityValue") (declared-name "PermittivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceUnit"))) (name "PhaseDifferenceUnit") (declared-name "PhaseDifferenceUnit") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue"))) (name "PhaseDifferenceValue") (declared-name "PhaseDifferenceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit"))) (name "PhaseSpeedOfElectromagneticWavesUnit") (declared-name "PhaseSpeedOfElectromagneticWavesUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue"))) (name "PhaseSpeedOfElectromagneticWavesValue") (declared-name "PhaseSpeedOfElectromagneticWavesValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue"))) (name "PowerFactorValue") (declared-name "PowerFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PowerValue"))) (name "PowerValue") (declared-name "PowerValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit"))) (name "PoyntingVectorMagnitudeUnit") (declared-name "PoyntingVectorMagnitudeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue"))) (name "PoyntingVectorMagnitudeValue") (declared-name "PoyntingVectorMagnitudeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue"))) (name "QualityFactorValue") (declared-name "QualityFactorValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit"))) (name "ReactanceUnit") (declared-name "ReactanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue"))) (name "ReactanceValue") (declared-name "ReactanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQElectromagnetism::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue"))) (name "RelativePermeabilityValue") (declared-name "RelativePermeabilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue"))) (name "RelativePermittivityValue") (declared-name "RelativePermittivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit"))) (name "ReluctanceUnit") (declared-name "ReluctanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue"))) (name "ReluctanceValue") (declared-name "ReluctanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit"))) (name "ResistanceToAlternatingCurrentUnit") (declared-name "ResistanceToAlternatingCurrentUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue"))) (name "ResistanceToAlternatingCurrentValue") (declared-name "ResistanceToAlternatingCurrentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit"))) (name "ResistanceUnit") (declared-name "ResistanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue"))) (name "ResistanceValue") (declared-name "ResistanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit"))) (name "ResistivityUnit") (declared-name "ResistivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue"))) (name "ResistivityValue") (declared-name "ResistivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SelfInductanceUnit"))) (name "SelfInductanceUnit") (declared-name "SelfInductanceUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SelfInductanceValue"))) (name "SelfInductanceValue") (declared-name "SelfInductanceValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceTensionUnit"))) (name "SourceTensionUnit") (declared-name "SourceTensionUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceTensionValue"))) (name "SourceTensionValue") (declared-name "SourceTensionValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit"))) (name "SourceVoltageUnit") (declared-name "SourceVoltageUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue"))) (name "SourceVoltageValue") (declared-name "SourceVoltageValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit"))) (name "SpeedOfLightUnit") (declared-name "SpeedOfLightUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue"))) (name "SpeedOfLightValue") (declared-name "SpeedOfLightValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit"))) (name "SurfaceDensityOfElectricChargeUnit") (declared-name "SurfaceDensityOfElectricChargeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue"))) (name "SurfaceDensityOfElectricChargeValue") (declared-name "SurfaceDensityOfElectricChargeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit"))) (name "SusceptanceUnit") (declared-name "SusceptanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue"))) (name "SusceptanceValue") (declared-name "SusceptanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit"))) (name "TotalCurrentDensityUnit") (declared-name "TotalCurrentDensityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit::electricCurrentPF"))) (name "electricCurrentPF") (declared-name "electricCurrentPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue"))) (name "TotalCurrentDensityValue") (declared-name "TotalCurrentDensityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::VolumicElectricChargeUnit"))) (name "VolumicElectricChargeUnit") (declared-name "VolumicElectricChargeUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::VolumicElectricChargeValue"))) (name "VolumicElectricChargeValue") (declared-name "VolumicElectricChargeValue"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::VolumicElectromagneticEnergyUnit"))) (name "VolumicElectromagneticEnergyUnit") (declared-name "VolumicElectromagneticEnergyUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::VolumicElectromagneticEnergyValue"))) (name "VolumicElectromagneticEnergyValue") (declared-name "VolumicElectromagneticEnergyValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::activeEnergy"))) (name "activeEnergy") (declared-name "activeEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::activeEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::activeEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::activePower"))) (name "activePower") (declared-name "activePower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::activePower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::activePower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::admittance"))) (name "admittance") (declared-name "admittance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::apparentPower"))) (name "apparentPower") (declared-name "apparentPower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::apparentPower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::apparentPower")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::areicElectricCharge"))) (name "areicElectricCharge") (declared-name "areicElectricCharge"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::capacitance"))) (name "capacitance") (declared-name "capacitance") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianAreicElectricCurrent3dVector"))) (name "cartesianAreicElectricCurrent3dVector") (declared-name "cartesianAreicElectricCurrent3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianDisplacementCurrentDensity3dVector"))) (name "cartesianDisplacementCurrentDensity3dVector") (declared-name "cartesianDisplacementCurrentDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricCurrentDensity3dVector"))) (name "cartesianElectricCurrentDensity3dVector") (declared-name "cartesianElectricCurrentDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricDipoleMoment3dVector"))) (name "cartesianElectricDipoleMoment3dVector") (declared-name "cartesianElectricDipoleMoment3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricDisplacement3dVector"))) (name "cartesianElectricDisplacement3dVector") (declared-name "cartesianElectricDisplacement3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricFieldStrength3dVector"))) (name "cartesianElectricFieldStrength3dVector") (declared-name "cartesianElectricFieldStrength3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricFluxDensity3dVector"))) (name "cartesianElectricFluxDensity3dVector") (declared-name "cartesianElectricFluxDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricPolarization3dVector"))) (name "cartesianElectricPolarization3dVector") (declared-name "cartesianElectricPolarization3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianLinearElectricCurrentDensity3dVector"))) (name "cartesianLinearElectricCurrentDensity3dVector") (declared-name "cartesianLinearElectricCurrentDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianLineicElectricCurrent3dVector"))) (name "cartesianLineicElectricCurrent3dVector") (declared-name "cartesianLineicElectricCurrent3dVector"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticAreaMoment3dVector"))) (name "cartesianMagneticAreaMoment3dVector") (declared-name "cartesianMagneticAreaMoment3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticDipoleMoment3dVector"))) (name "cartesianMagneticDipoleMoment3dVector") (declared-name "cartesianMagneticDipoleMoment3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticFieldStrength3dVector"))) (name "cartesianMagneticFieldStrength3dVector") (declared-name "cartesianMagneticFieldStrength3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticFluxDensity3dVector"))) (name "cartesianMagneticFluxDensity3dVector") (declared-name "cartesianMagneticFluxDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticMoment3dVector"))) (name "cartesianMagneticMoment3dVector") (declared-name "cartesianMagneticMoment3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticPolarization3dVector"))) (name "cartesianMagneticPolarization3dVector") (declared-name "cartesianMagneticPolarization3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticVectorPotential3dVector"))) (name "cartesianMagneticVectorPotential3dVector") (declared-name "cartesianMagneticVectorPotential3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagnetization3dVector"))) (name "cartesianMagnetization3dVector") (declared-name "cartesianMagnetization3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagnetizingField3dVector"))) (name "cartesianMagnetizingField3dVector") (declared-name "cartesianMagnetizingField3dVector"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianPoynting3dVector"))) (name "cartesianPoynting3dVector") (declared-name "cartesianPoynting3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianTotalCurrentDensity3dVector"))) (name "cartesianTotalCurrentDensity3dVector") (declared-name "cartesianTotalCurrentDensity3dVector") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::coercivity"))) (name "coercivity") (declared-name "coercivity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::complexAdmittance"))) (name "complexAdmittance") (declared-name "complexAdmittance"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::complexImpedance"))) (name "complexImpedance") (declared-name "complexImpedance"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::complexPower"))) (name "complexPower") (declared-name "complexPower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::complexPower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::complexPower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::conductance"))) (name "conductance") (declared-name "conductance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent"))) (name "conductanceForAlternatingCurrent") (declared-name "conductanceForAlternatingCurrent") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::conductivity"))) (name "conductivity") (declared-name "conductivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::couplingFactor"))) (name "couplingFactor") (declared-name "couplingFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::currentLinkage"))) (name "currentLinkage") (declared-name "currentLinkage") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::currentLinkage::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::currentLinkage")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrent"))) (name "displacementCurrent") (declared-name "displacementCurrent") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrent::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrent")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrentDensity"))) (name "displacementCurrentDensity") (declared-name "displacementCurrentDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricCharge"))) (name "electricCharge") (declared-name "electricCharge") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricChargeDensity"))) (name "electricChargeDensity") (declared-name "electricChargeDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricConstant"))) (name "electricConstant") (declared-name "electricConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentDensity"))) (name "electricCurrentDensity") (declared-name "electricCurrentDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentPhasor"))) (name "electricCurrentPhasor") (declared-name "electricCurrentPhasor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentPhasor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentPhasor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricDipoleMoment"))) (name "electricDipoleMoment") (declared-name "electricDipoleMoment") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricFieldStrength"))) (name "electricFieldStrength") (declared-name "electricFieldStrength") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricFlux"))) (name "electricFlux") (declared-name "electricFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricFluxDensity"))) (name "electricFluxDensity") (declared-name "electricFluxDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricPolarization"))) (name "electricPolarization") (declared-name "electricPolarization") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricPotential"))) (name "electricPotential") (declared-name "electricPotential") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricPotentialDifference"))) (name "electricPotentialDifference") (declared-name "electricPotentialDifference") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricPower"))) (name "electricPower") (declared-name "electricPower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricPower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::electricPower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricSusceptibility"))) (name "electricSusceptibility") (declared-name "electricSusceptibility") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electricTension"))) (name "electricTension") (declared-name "electricTension"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::electromagneticEnergyDensity"))) (name "electromagneticEnergyDensity") (declared-name "electromagneticEnergyDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::impedance"))) (name "impedance") (declared-name "impedance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::inductance"))) (name "inductance") (declared-name "inductance") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::instantaneousPower"))) (name "instantaneousPower") (declared-name "instantaneousPower"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::leakageFactor"))) (name "leakageFactor") (declared-name "leakageFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::lightSpeed"))) (name "lightSpeed") (declared-name "lightSpeed"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::linearDensityOfElectricCharge"))) (name "linearDensityOfElectricCharge") (declared-name "linearDensityOfElectricCharge") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::linearElectricCurrentDensity"))) (name "linearElectricCurrentDensity") (declared-name "linearElectricCurrentDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::lineicElectricCharge"))) (name "lineicElectricCharge") (declared-name "lineicElectricCharge"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::linkedFlux"))) (name "linkedFlux") (declared-name "linkedFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::lossAngle"))) (name "lossAngle") (declared-name "lossAngle") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::lossAngle::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::lossAngle")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::lossFactor"))) (name "lossFactor") (declared-name "lossFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticConstant"))) (name "magneticConstant") (declared-name "magneticConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticDipoleMoment"))) (name "magneticDipoleMoment") (declared-name "magneticDipoleMoment") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFieldStrength"))) (name "magneticFieldStrength") (declared-name "magneticFieldStrength") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFlux"))) (name "magneticFlux") (declared-name "magneticFlux") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFluxDensity"))) (name "magneticFluxDensity") (declared-name "magneticFluxDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticMoment"))) (name "magneticMoment") (declared-name "magneticMoment") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticPolarization"))) (name "magneticPolarization") (declared-name "magneticPolarization") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticSusceptibility"))) (name "magneticSusceptibility") (declared-name "magneticSusceptibility") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticTension"))) (name "magneticTension") (declared-name "magneticTension") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticTension::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::magneticTension")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magneticVectorPotential"))) (name "magneticVectorPotential") (declared-name "magneticVectorPotential") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magnetization"))) (name "magnetization") (declared-name "magnetization") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::magnetomotiveForce"))) (name "magnetomotiveForce") (declared-name "magnetomotiveForce") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::modulusOfAdmittance"))) (name "modulusOfAdmittance") (declared-name "modulusOfAdmittance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::modulusOfImpedance"))) (name "modulusOfImpedance") (declared-name "modulusOfImpedance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance"))) (name "mutualInductance") (declared-name "mutualInductance") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::nonActivePower"))) (name "nonActivePower") (declared-name "nonActivePower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::nonActivePower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::nonActivePower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::numberOfTurnsInAWinding"))) (name "numberOfTurnsInAWinding") (declared-name "numberOfTurnsInAWinding") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::numberOfTurnsInAWinding::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::numberOfTurnsInAWinding")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::permeability"))) (name "permeability") (declared-name "permeability") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::permeabilityOfVacuum"))) (name "permeabilityOfVacuum") (declared-name "permeabilityOfVacuum"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::permeance"))) (name "permeance") (declared-name "permeance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::permittivity"))) (name "permittivity") (declared-name "permittivity") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::permittivityOfVacuum"))) (name "permittivityOfVacuum") (declared-name "permittivityOfVacuum"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::phaseDifference"))) (name "phaseDifference") (declared-name "phaseDifference") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::phaseSpeedOfElectromagneticWaves"))) (name "phaseSpeedOfElectromagneticWaves") (declared-name "phaseSpeedOfElectromagneticWaves") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::powerFactor"))) (name "powerFactor") (declared-name "powerFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::poyntingVectorMagnitude"))) (name "poyntingVectorMagnitude") (declared-name "poyntingVectorMagnitude") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::qualityFactor"))) (name "qualityFactor") (declared-name "qualityFactor") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::reactance"))) (name "reactance") (declared-name "reactance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::reactivePower"))) (name "reactivePower") (declared-name "reactivePower") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::reactivePower::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::reactivePower")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::relativePermeability"))) (name "relativePermeability") (declared-name "relativePermeability") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::relativePermittivity"))) (name "relativePermittivity") (declared-name "relativePermittivity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::reluctance"))) (name "reluctance") (declared-name "reluctance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::resistance"))) (name "resistance") (declared-name "resistance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::resistanceToAlternatingCurrent"))) (name "resistanceToAlternatingCurrent") (declared-name "resistanceToAlternatingCurrent") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::resistivity"))) (name "resistivity") (declared-name "resistivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::scalarMagneticPotential"))) (name "scalarMagneticPotential") (declared-name "scalarMagneticPotential") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::scalarMagneticPotential::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::scalarMagneticPotential")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::selfInductance"))) (name "selfInductance") (declared-name "selfInductance"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::sourceTension"))) (name "sourceTension") (declared-name "sourceTension"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::sourceVoltage"))) (name "sourceVoltage") (declared-name "sourceVoltage") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::speedOfLight"))) (name "speedOfLight") (declared-name "speedOfLight") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::surfaceDensityOfElectricCharge"))) (name "surfaceDensityOfElectricCharge") (declared-name "surfaceDensityOfElectricCharge") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::susceptance"))) (name "susceptance") (declared-name "susceptance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrent"))) (name "totalCurrent") (declared-name "totalCurrent") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrent::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrent")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrentDensity"))) (name "totalCurrentDensity") (declared-name "totalCurrentDensity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::voltage"))) (name "voltage") (declared-name "voltage") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::voltage::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::voltage")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor"))) (name "voltagePhasor") (declared-name "voltagePhasor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::volumicElectricCharge"))) (name "volumicElectricCharge") (declared-name "volumicElectricCharge"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQElectromagnetism::volumicElectromagneticEnergy"))) (name "volumicElectromagneticEnergy") (declared-name "volumicElectromagneticEnergy"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::activeEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::activeEnergy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::activePower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::activePower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::apparentPower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::apparentPower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::complexPower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::complexPower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::currentLinkage::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::currentLinkage"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrent::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrent"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentPhasor::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentPhasor"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricPower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::electricPower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::lossAngle::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::lossAngle"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticTension::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::magneticTension"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::nonActivePower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::nonActivePower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::numberOfTurnsInAWinding::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::numberOfTurnsInAWinding"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::reactivePower::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::reactivePower"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::scalarMagneticPotential::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::scalarMagneticPotential"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrent::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrent"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::voltage::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::voltage"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor::_documentation"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame::mRefs"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dCoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::admittance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::AdmittanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::capacitance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CapacitanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianDisplacementCurrentDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianDisplacementCurrentDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricCurrentDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricCurrentDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricDipoleMoment3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricDipoleMoment3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricFieldStrength3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFieldStrength3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricFluxDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricFluxDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianElectricPolarization3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianElectricPolarization3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianLinearElectricCurrentDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianLinearElectricCurrentDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticDipoleMoment3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticDipoleMoment3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticFieldStrength3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFieldStrength3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticFluxDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticFluxDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticMoment3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticMoment3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticPolarization3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticPolarization3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagneticVectorPotential3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagneticVectorPotential3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianMagnetization3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianMagnetization3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianPoynting3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianPoynting3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::cartesianTotalCurrentDensity3dVector"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CartesianTotalCurrentDensity3dVector"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::coercivity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CoercivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::conductance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::conductanceForAlternatingCurrent"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::conductivity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ConductivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::couplingFactor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::CouplingFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::displacementCurrentDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::DisplacementCurrentDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricCharge"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricChargeDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricChargeDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricConstant"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricConstantValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricCurrentDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricCurrentDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricDipoleMoment"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricDipoleMomentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricFieldStrength"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFieldStrengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricFlux"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricFluxDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricFluxDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricPolarization"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPolarizationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricPotential"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricPotentialDifference"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electricSusceptibility"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricSusceptibilityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::electromagneticEnergyDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectromagneticEnergyDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::impedance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ImpedanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::inductance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::leakageFactor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LeakageFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::linearDensityOfElectricCharge"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearDensityOfElectricChargeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::linearElectricCurrentDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinearElectricCurrentDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::linkedFlux"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LinkedFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::lossFactor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::LossFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticConstant"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticConstantValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticDipoleMoment"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticDipoleMomentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFieldStrength"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFieldStrengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFlux"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticFluxDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticFluxDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticMoment"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticMomentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticPolarization"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticPolarizationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticSusceptibility"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticSusceptibilityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magneticVectorPotential"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagneticVectorPotentialValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magnetization"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetizationValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::magnetomotiveForce"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::MagnetomotiveForceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::modulusOfAdmittance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfAdmittanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::modulusOfImpedance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ModulusOfImpedanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::mutualInductance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::InductanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::permeability"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeabilityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::permeance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermeanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::permittivity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PermittivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::phaseDifference"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseDifferenceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::phaseSpeedOfElectromagneticWaves"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PhaseSpeedOfElectromagneticWavesValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::powerFactor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PowerFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::poyntingVectorMagnitude"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::PoyntingVectorMagnitudeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::qualityFactor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::QualityFactorValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::reactance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReactanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::relativePermeability"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermeabilityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::relativePermittivity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::RelativePermittivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::reluctance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ReluctanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::resistance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::resistanceToAlternatingCurrent"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistanceToAlternatingCurrentValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::resistivity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ResistivityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::sourceVoltage"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SourceVoltageValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::speedOfLight"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SpeedOfLightValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::surfaceDensityOfElectricCharge"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SurfaceDensityOfElectricChargeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::susceptance"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::SusceptanceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::totalCurrentDensity"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::TotalCurrentDensityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::voltage"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQElectromagnetism::voltagePhasor"))) (to (node (document "d0") (qualified-name "ISQElectromagnetism::ElectricPotentialDifferenceValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
