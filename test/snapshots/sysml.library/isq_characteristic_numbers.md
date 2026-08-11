# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQCharacteristicNumbers
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQCharacteristicNumbers {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 "Characteristic numbers"
     * see also https://www.iso.org/standard/64982.html
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

    /* ISO-80000-11 item 11-4.1 Reynolds number */
    attribute def ReynoldsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.1 Reynolds number
         * symbol(s): `Re`
         * application domain: generic
         * name: ReynoldsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.
         */
    }
    attribute reynoldsNumber: ReynoldsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.2 Euler number */
    attribute def EulerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.2 Euler number
         * symbol(s): `Eu`
         * application domain: generic
         * name: EulerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^"'" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).
         */
    }
    attribute eulerNumber: EulerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.3 Froude number */
    attribute def FroudeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.3 Froude number
         * symbol(s): `Fr`
         * application domain: generic
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)
         * remarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.
         */
    }
    attribute froudeNumber: FroudeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.4 Grashof number */
    attribute def GrashofNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.4 Grashof number
         * symbol(s): `Gr`
         * application domain: generic
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).
         */
    }
    attribute grashofNumber: GrashofNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.5 Weber number */
    attribute def WeberNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.5 Weber number
         * symbol(s): `We`
         * application domain: generic
         * name: WeberNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)
         * remarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.
         */
    }
    attribute weberNumber: WeberNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.6 Mach number */
    attribute def MachNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.6 Mach number
         * symbol(s): `Ma`
         * application domain: generic
         * name: MachNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid
         * remarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).
         */
    }
    attribute machNumber: MachNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.7 Knudsen number */
    attribute def KnudsenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.7 Knudsen number
         * symbol(s): `Kn`
         * application domain: generic
         * name: KnudsenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.
         */
    }
    attribute knudsenNumber: KnudsenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.8 Strouhal number, Thomson number */
    attribute def StrouhalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.8 Strouhal number, Thomson number
         * symbol(s): `Sr`, `Sh`
         * application domain: generic
         * name: StrouhalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow
         * remarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.
         */
    }
    attribute strouhalNumber: StrouhalNumberValue :> scalarQuantities;

    alias thomsonNumber for strouhalNumber;

    /* ISO-80000-11 item 11-4.9 drag coefficient */
    /* Refer to declaration for DragCoefficient in ISQMechanics item 4-23.4 drag coefficient */

    /* ISO-80000-11 item 11-4.10 Bagnold number */
    attribute def BagnoldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.10 Bagnold number
         * symbol(s): `Bg`
         * application domain: generic
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body
         * remarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.
         */
    }
    attribute bagnoldNumber: BagnoldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.11 Bagnold number */
    attribute def BagnoldNumberForSolidParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.11 Bagnold number
         * symbol(s): `Ba_2`
         * application domain: solid particles
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles
         * remarks: None.
         */
    }
    attribute bagnoldNumberForSolidParticles: BagnoldNumberForSolidParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.12 lift coefficient */
    attribute def LiftCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.12 lift coefficient
         * symbol(s): `c_l`, `c_A`
         * application domain: generic
         * name: LiftCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure
         * remarks: The lift coefficient is dependant on the shape of the wing.
         */
    }
    attribute liftCoefficient: LiftCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.13 thrust coefficient */
    attribute def ThrustCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.13 thrust coefficient
         * symbol(s): `c_t`
         * application domain: generic
         * name: ThrustCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller
         * remarks: The thrust coefficient is dependant on the shape of the propeller.
         */
    }
    attribute thrustCoefficient: ThrustCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.14 Dean number */
    attribute def DeanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.14 Dean number
         * symbol(s): `Dn`
         * application domain: generic
         * name: DeanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe
         * remarks: None.
         */
    }
    attribute deanNumber: DeanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.15 Bejan number */
    attribute def BejanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.15 Bejan number
         * symbol(s): `Be`
         * application domain: generic
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)
         * remarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.
         */
    }
    attribute bejanNumber: BejanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.16 Lagrange number */
    attribute def LagrangeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.16 Lagrange number
         * symbol(s): `Lg`
         * application domain: generic
         * name: LagrangeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).
         */
    }
    attribute lagrangeNumber: LagrangeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.17 Bingham number, plasticity number */
    attribute def BinghamNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.17 Bingham number, plasticity number
         * symbol(s): `Bm`, `Bn`
         * application domain: generic
         * name: BinghamNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute binghamNumber: BinghamNumberValue :> scalarQuantities;

    alias plasticityNumber for binghamNumber;

    /* ISO-80000-11 item 11-4.18 Hedström number */
    attribute def 'HedströmNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.18 Hedström number
         * symbol(s): `He`, `Hd`
         * application domain: generic
         * name: HedströmNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'hedströmNumber': 'HedströmNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-4.19 Bodenstein number */
    attribute def BodensteinNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.19 Bodenstein number
         * symbol(s): `Bd`
         * application domain: generic
         * name: BodensteinNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Bodenstein number is also given by `Bd = Pe^"*" = Re*Sc`, where `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).
         */
    }
    attribute bodensteinNumber: BodensteinNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.20 Rossby number, Kiebel number */
    attribute def RossbyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.20 Rossby number, Kiebel number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RossbyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude
         * remarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.
         */
    }
    attribute rossbyNumber: RossbyNumberValue :> scalarQuantities;

    alias kiebelNumber for rossbyNumber;

    /* ISO-80000-11 item 11-4.21 Ekman number */
    attribute def EkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.21 Ekman number
         * symbol(s): `Ek`
         * application domain: generic
         * name: EkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude
         * remarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).
         */
    }
    attribute ekmanNumber: EkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.22 elasticity number */
    attribute def ElasticityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.22 elasticity number
         * symbol(s): `El`
         * application domain: generic
         * name: ElasticityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe
         * remarks: See also Deborah number (item 11-7.8).
         */
    }
    attribute elasticityNumber: ElasticityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.23 Darcy friction factor, Moody friction factor */
    attribute def DarcyFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.23 Darcy friction factor, Moody friction factor
         * symbol(s): `f_D`
         * application domain: generic
         * name: DarcyFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute darcyFrictionFactor: DarcyFrictionFactorValue :> scalarQuantities;

    alias moodyFrictionFactor for darcyFrictionFactor;

    /* ISO-80000-11 item 11-4.24 Fanning number */
    attribute def FanningNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.24 Fanning number
         * symbol(s): `f_n`, `f`
         * application domain: generic
         * name: FanningNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe
         * remarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.
         */
    }
    attribute fanningNumber: FanningNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.25 Goertler number, Goertler parameter */
    attribute def GoertlerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.25 Goertler number, Goertler parameter
         * symbol(s): `Go`
         * application domain: generic
         * name: GoertlerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)
         * remarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.
         */
    }
    attribute goertlerNumber: GoertlerNumberValue :> scalarQuantities;

    alias goertlerParameter for goertlerNumber;

    /* ISO-80000-11 item 11-4.26 Hagen number */
    attribute def HagenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.26 Hagen number
         * symbol(s): `Hg`, `Ha`
         * application domain: generic
         * name: HagenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).
         */
    }
    attribute hagenNumber: HagenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.27 Laval number */
    attribute def LavalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.27 Laval number
         * symbol(s): `La`
         * application domain: generic
         * name: LavalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3),  `R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)
         * remarks: The Laval number is a specific kind of Mach number (item 11-4.6).
         */
    }
    attribute lavalNumber: LavalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.28 Poiseuille number */
    attribute def PoiseuilleNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.28 Poiseuille number
         * symbol(s): `Poi`
         * application domain: generic
         * name: PoiseuilleNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid
         * remarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).
         */
    }
    attribute poiseuilleNumber: PoiseuilleNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.29 power number */
    attribute def PowerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.29 power number
         * symbol(s): `Pn`
         * application domain: generic
         * name: PowerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer
         * remarks: None.
         */
    }
    attribute powerNumber: PowerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.30 Richardson number */
    attribute def RichardsonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.30 Richardson number
         * symbol(s): `Ri`
         * application domain: generic
         * name: RichardsonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)
         * remarks: In geophysics differences of these quantities are of interest.
         */
    }
    attribute richardsonNumber: RichardsonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.31 Reech number */
    attribute def ReechNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.31 Reech number
         * symbol(s): `Ree`
         * application domain: generic
         * name: ReechNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water
         * remarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .
         */
    }
    attribute reechNumber: ReechNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.32 Stokes number */
    attribute def StokesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.32 Stokes number
         * symbol(s): `Stk`
         * application domain: time-related
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence
         * remarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.
         */
    }
    attribute stokesNumber: StokesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.33 Stokes number */
    attribute def StokesNumberForVibratingParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.33 Stokes number
         * symbol(s): `Stk_1`
         * application domain: vibrating particles
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute stokesNumberForVibratingParticles: StokesNumberForVibratingParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.34 Stokes number, power coefficient */
    attribute def StokesNumberForRotameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.34 Stokes number, power coefficient
         * symbol(s): `Stk_2`
         * application domain: rotameter
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).
         */
    }
    attribute stokesNumberForRotameter: StokesNumberForRotameterValue :> scalarQuantities;

    alias powerCoefficient for stokesNumber;

    /* ISO-80000-11 item 11-4.35 Stokes number */
    attribute def StokesNumberForGravityValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.35 Stokes number
         * symbol(s): `Stk_3`
         * application domain: gravity
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall
         * remarks: None.
         */
    }
    attribute stokesNumberForGravity: StokesNumberForGravityValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.36 Stokes number */
    attribute def StokesNumberForDragValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.36 Stokes number
         * symbol(s): `Stk_4`
         * application domain: drag
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute stokesNumberForDrag: StokesNumberForDragValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.37 Laplace number, Suratman number */
    attribute def LaplaceNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.37 Laplace number, Suratman number
         * symbol(s): `La`, `Su`
         * application domain: generic
         * name: LaplaceNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid
         * remarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).
         */
    }
    attribute laplaceNumber: LaplaceNumberValue :> scalarQuantities;

    alias suratmanNumber for laplaceNumber;

    /* ISO-80000-11 item 11-4.38 Blake number */
    attribute def BlakeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.38 Blake number
         * symbol(s): `Bl`
         * application domain: generic
         * name: BlakeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)
         * remarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.
         */
    }
    attribute blakeNumber: BlakeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.39 Sommerfeld number */
    attribute def SommerfeldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.39 Sommerfeld number
         * symbol(s): `So`, `Sm`
         * application domain: generic
         * name: SommerfeldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute sommerfeldNumber: SommerfeldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.40 Taylor number */
    attribute def TaylorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.40 Taylor number
         * symbol(s): `Ta`
         * application domain: momentum transfer
         * name: TaylorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.
         */
    }
    attribute taylorNumber: TaylorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.41 Galilei number */
    attribute def GalileiNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.41 Galilei number
         * symbol(s): `Ga`
         * application domain: generic
         * name: GalileiNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).
         */
    }
    attribute galileiNumber: GalileiNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.42 Womersley number */
    attribute def WomersleyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.42 Womersley number
         * symbol(s): `Wo`, `α`
         * application domain: generic
         * name: WomersleyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Womersley number is used for pulsating flows e.g. in blood flow.
         */
    }
    attribute womersleyNumber: WomersleyNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.1 Fourier number */
    attribute def FourierNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.1 Fourier number
         * symbol(s): `Fo`
         * application domain: heat transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.
         */
    }
    attribute fourierNumber: FourierNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.2 Péclet number */
    attribute def 'PécletNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.2 Péclet number
         * symbol(s): `Pe`
         * application domain: heat transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.
         */
    }
    attribute 'pécletNumber': 'PécletNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-5.3 Rayleigh number */
    attribute def RayleighNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.3 Rayleigh number
         * symbol(s): `Ra`
         * application domain: generic
         * name: RayleighNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid
         * remarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).
         */
    }
    attribute rayleighNumber: RayleighNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.4 Froude number */
    attribute def FroudeNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.4 Froude number
         * symbol(s): `Fr^"*"`
         * application domain: heat transfer
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^"*" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)"
         * remarks: None.
         */
    }
    attribute froudeNumberForHeatTransfer: FroudeNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.5 Nusselt number */
    attribute def NusseltNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.5 Nusselt number
         * symbol(s): `Nu`
         * application domain: heat transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)
         * remarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the "Biot number for heat transfer" (item 11-5.6) is used.
         */
    }
    attribute nusseltNumber: NusseltNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.6 Biot number */
    attribute def BiotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.6 Biot number
         * symbol(s): `Bi`
         * application domain: heat transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body
         * remarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.
         */
    }
    attribute biotNumber: BiotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.7 Stanton number */
    attribute def StantonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.7 Stanton number
         * symbol(s): `St`
         * application domain: heat transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid
         * remarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.
         */
    }
    attribute stantonNumber: StantonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.8 j-factor, heat transfer factor, Colburn number */
    attribute def JFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.8 j-factor, heat transfer factor, Colburn number
         * symbol(s): `j`, `Co`, `Jq`
         * application domain: heat transfer
         * name: JFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).
         */
    }
    attribute jFactor: JFactorValue :> scalarQuantities;

    alias heatTransferFactor for jFactor;

    alias colburnNumber for jFactor;

    /* ISO-80000-11 item 11-5.9 Bejan number */
    attribute def BejanNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.9 Bejan number
         * symbol(s): `Be_1`
         * application domain: heat transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute bejanNumberForHeatTransfer: BejanNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.10 Bejan number */
    attribute def BejanNumberForEntropyValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.10 Bejan number
         * symbol(s): `Be_S`
         * application domain: entropy
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction
         * remarks: None.
         */
    }
    attribute bejanNumberForEntropy: BejanNumberForEntropyValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.11 Stefan number */
    attribute def StefanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.11 Stefan number
         * symbol(s): `Ste`, `Stf`
         * application domain: phase transition
         * name: StefanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute stefanNumber: StefanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.12 Brinkman number */
    attribute def BrinkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.12 Brinkman number
         * symbol(s): `Br`, `N_(Br)`
         * application domain: generic
         * name: BrinkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature
         * remarks: None.
         */
    }
    attribute brinkmanNumber: BrinkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.13 Clausius number */
    attribute def ClausiusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.13 Clausius number
         * symbol(s): `Cl`
         * application domain: generic
         * name: ClausiusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`
         * remarks: None.
         */
    }
    attribute clausiusNumber: ClausiusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.14 Carnot number */
    attribute def CarnotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.14 Carnot number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CarnotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively
         * remarks: None.
         */
    }
    attribute carnotNumber: CarnotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.15 Eckert number, Dulong number */
    attribute def EckertNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.15 Eckert number, Dulong number
         * symbol(s): `Ec`
         * application domain: generic
         * name: EckertNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)
         * remarks: None.
         */
    }
    attribute eckertNumber: EckertNumberValue :> scalarQuantities;

    alias dulongNumber for eckertNumber;

    /* ISO-80000-11 item 11-5.16 Graetz number */
    attribute def GraetzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.16 Graetz number
         * symbol(s): `Gz`
         * application domain: heat transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute graetzNumber: GraetzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.17 heat transfer number */
    attribute def HeatTransferNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.17 heat transfer number
         * symbol(s): `K_Q`
         * application domain: generic
         * name: HeatTransferNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute heatTransferNumber: HeatTransferNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.18 Pomerantsev number */
    attribute def PomerantsevNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.18 Pomerantsev number
         * symbol(s): `Po`, `Pov`
         * application domain: heat transfer
         * name: PomerantsevNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)
         * remarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.
         */
    }
    attribute pomerantsevNumber: PomerantsevNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.19 Boltzmann number */
    attribute def BoltzmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.19 Boltzmann number
         * symbol(s): `Bz`, `Bol`, `Bo`
         * application domain: generic
         * name: BoltzmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute boltzmannNumber: BoltzmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.20 Stark number */
    attribute def StarkNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.20 Stark number
         * symbol(s): `Sk`
         * application domain: generic
         * name: StarkNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.
         */
    }
    attribute starkNumber: StarkNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.1 Fourier number */
    attribute def FourierNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.1 Fourier number
         * symbol(s): `Fo^"*"`
         * application domain: mass transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^"*" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer"
         * remarks: The Fourier number for mass transfer is also given by `Fo^*" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1)."
         */
    }
    attribute fourierNumberForMassTransfer: FourierNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.2 Péclet number */
    attribute def 'PécletNumberForMassTransferValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.2 Péclet number
         * symbol(s): `Pe^"*"`, `Bd`, `Bod`
         * application domain: mass transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The Péclet number for mass transfer is also given by `Pe^"*" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.
         */
    }
    attribute 'pécletNumberForMassTransfer': 'PécletNumberForMassTransferValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-6.3 Grashof number */
    attribute def GrashofNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.3 Grashof number
         * symbol(s): `Gr^"*"`
         * application domain: mass transfer
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^"*" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)"
         * remarks: Instead of "amount-of-substance fraction" the "amount-of-substance concentration" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.
         */
    }
    attribute grashofNumberForMassTransfer: GrashofNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.4 Nusselt number */
    attribute def NusseltNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.4 Nusselt number
         * symbol(s): `Nu^"*"`
         * application domain: mass transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^"*" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.
         */
    }
    attribute nusseltNumberForMassTransfer: NusseltNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.5 Stanton number */
    attribute def StantonNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.5 Stanton number
         * symbol(s): `St^"*"`
         * application domain: mass transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^"*" = k^"*"
         * remarks: The Stanton number for mass transfer is also given by `St^*" = (Nu^"*")/(Pe^"*"*)`, where `Nu^"*"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer."
         */
    }
    attribute stantonNumberForMassTransfer: StantonNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.6 Graetz number */
    attribute def GraetzNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.6 Graetz number
         * symbol(s): `Gz^"*"`
         * application domain: mass transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^"*" = (v*d)/D = d/l*Pe^"*"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2)"
         * remarks: None.
         */
    }
    attribute graetzNumberForMassTransfer: GraetzNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.7 mass transfer factor */
    attribute def MassTransferFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.7 mass transfer factor
         * symbol(s): `j^"*"`
         * application domain: mass transfer
         * name: MassTransferFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The mass transfer factor is also given by `j_m = j^*" = St^"*" * (Sc)^(2/3)` where `St^"*"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17)."
         */
    }
    attribute massTransferFactor: MassTransferFactorValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.8 Atwood number */
    attribute def AtwoodNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.8 Atwood number
         * symbol(s): `At`
         * application domain: generic
         * name: AtwoodNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid
         * remarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.
         */
    }
    attribute atwoodNumber: AtwoodNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.9 Biot number */
    attribute def BiotNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.9 Biot number
         * symbol(s): `Bi^"*"`
         * application domain: mass transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*" = (k*l)/D_"int"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_"int"` is diffusion coefficient (ISO 80000-9) at the interface"
         * remarks: None.
         */
    }
    attribute biotNumberForMassTransfer: BiotNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.10 Morton number */
    attribute def MortonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.10 Morton number
         * symbol(s): `Mo`
         * application domain: generic
         * name: MortonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop
         * remarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute mortonNumber: MortonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.11 Bond number, Eötvös number */
    attribute def BondNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.11 Bond number, Eötvös number
         * symbol(s): `Bo`, `Eo`
         * application domain: generic
         * name: BondNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble
         * remarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.
         */
    }
    attribute bondNumber: BondNumberValue :> scalarQuantities;

    alias 'eötvösNumber' for bondNumber;

    /* ISO-80000-11 item 11-6.12 Archimedes number */
    attribute def ArchimedesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.12 Archimedes number
         * symbol(s): `Ar`
         * application domain: generic
         * name: ArchimedesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid
         * remarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).
         */
    }
    attribute archimedesNumber: ArchimedesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.13 expansion number */
    attribute def ExpansionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.13 expansion number
         * symbol(s): `Ex`
         * application domain: generic
         * name: ExpansionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid
         * remarks: None.
         */
    }
    attribute expansionNumber: ExpansionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.14 Marangoni number */
    attribute def MarangoniNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.14 Marangoni number
         * symbol(s): `Mg`, `Mar`
         * application domain: generic
         * name: MarangoniNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film
         * remarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.
         */
    }
    attribute marangoniNumber: MarangoniNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.15 Lockhart-Martinelli parameter */
    attribute def LockhartMartinelliParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.15 Lockhart-Martinelli parameter
         * symbol(s): `Lp`
         * application domain: generic
         * name: LockhartMartinelliParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density
         * remarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.
         */
    }
    attribute lockhartMartinelliParameter: LockhartMartinelliParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.16 Bejan number */
    attribute def BejanNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.16 Bejan number
         * symbol(s): `Be^"*"`, `Be_2`
         * application domain: mass transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity"
         * remarks: A similar quantity exists for heat transfer (item 11-5.9).
         */
    }
    attribute bejanNumberForMassTransfer: BejanNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.17 cavitation number */
    attribute def CavitationNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.17 cavitation number
         * symbol(s): `Ca`, `Cn`
         * application domain: generic
         * name: CavitationNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow
         * remarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.
         */
    }
    attribute cavitationNumber: CavitationNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.18 absorption number */
    attribute def AbsorptionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.18 absorption number
         * symbol(s): `Ab`
         * application domain: generic
         * name: AbsorptionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter
         * remarks: None.
         */
    }
    attribute absorptionNumber: AbsorptionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.19 capillary number */
    attribute def CapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.19 capillary number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid
         * remarks: None.
         */
    }
    attribute capillaryNumber: CapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.20 dynamic capillary number */
    attribute def DynamicCapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.20 dynamic capillary number
         * symbol(s): `Ca^"*"`, `Cn`
         * application domain: generic
         * name: DynamicCapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)"
         * remarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.
         */
    }
    attribute dynamicCapillaryNumber: DynamicCapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.1 Prandtl number */
    attribute def PrandtlNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.1 Prandtl number
         * symbol(s): `Pr`
         * application domain: generic
         * name: PrandtlNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute prandtlNumber: PrandtlNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.2 Schmidt number */
    attribute def SchmidtNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.2 Schmidt number
         * symbol(s): `Sc`
         * application domain: generic
         * name: SchmidtNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.
         */
    }
    attribute schmidtNumber: SchmidtNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.3 Lewis number */
    attribute def LewisNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.3 Lewis number
         * symbol(s): `Le`
         * application domain: generic
         * name: LewisNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. 
         */
    }
    attribute lewisNumber: LewisNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.4 Ohnesorge number */
    attribute def OhnesorgeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.4 Ohnesorge number
         * symbol(s): `Oh`
         * application domain: generic
         * name: OhnesorgeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.
         */
    }
    attribute ohnesorgeNumber: OhnesorgeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.5 Cauchy number, aeroelasticity parameter */
    attribute def CauchyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.5 Cauchy number, aeroelasticity parameter
         * symbol(s): `Cy`
         * application domain: generic
         * name: CauchyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute cauchyNumber: CauchyNumberValue :> scalarQuantities;

    alias aeroelasticityParameter for cauchyNumber;

    /* ISO-80000-11 item 11-7.6 Hooke number */
    attribute def HookeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.6 Hooke number
         * symbol(s): `Ho_2`
         * application domain: generic
         * name: HookeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute hookeNumber: HookeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.7 Weissenberg number */
    attribute def WeissenbergNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.7 Weissenberg number
         * symbol(s): `Wi`
         * application domain: generic
         * name: WeissenbergNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)
         * remarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.
         */
    }
    attribute weissenbergNumber: WeissenbergNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.8 Deborah number */
    attribute def DeborahNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.8 Deborah number
         * symbol(s): `De`
         * application domain: generic
         * name: DeborahNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)
         * remarks: The stress relaxation time is sometimes called the Maxwell relaxation time.
         */
    }
    attribute deborahNumber: DeborahNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.9 Lorentz number */
    attribute def LorentzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.9 Lorentz number
         * symbol(s): `Lo`
         * application domain: generic
         * name: LorentzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points
         * remarks: None.
         */
    }
    attribute lorentzNumber: LorentzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.10 compressibility number */
    attribute def CompressibilityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.10 compressibility number
         * symbol(s): `Z`
         * application domain: generic
         * name: CompressibilityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute compressibilityNumber: CompressibilityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.1 Reynolds magnetic number */
    attribute def ReynoldsMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.1 Reynolds magnetic number
         * symbol(s): `Rm`
         * application domain: generic
         * name: ReynoldsMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)
         * remarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).
         */
    }
    attribute reynoldsMagneticNumber: ReynoldsMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.2 Batchelor number */
    attribute def BatchelorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.2 Batchelor number
         * symbol(s): `Bt`
         * application domain: generic
         * name: BatchelorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute batchelorNumber: BatchelorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.3 Nusselt electric number */
    attribute def NusseltElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.3 Nusselt electric number
         * symbol(s): `Ne`
         * application domain: generic
         * name: NusseltElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^"*" = D^"+" + D^"-"`, where `D^"+"`, `D^"-"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively"
         * remarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.
         */
    }
    attribute nusseltElectricNumber: NusseltElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.4 Alfvén number, Mach magnetic number, Kárman number */
    attribute def 'AlfvénNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number
         * symbol(s): `Al`
         * application domain: generic
         * name: AlfvénNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: Often, the inverse of this number is wrongly used. The name "Alfvén Mach number" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute 'alfvénNumber': 'AlfvénNumberValue' :> scalarQuantities;

    alias machMagneticNumber for 'alfvénNumber';

    alias 'kármanNumber' for 'alfvénNumber';

    /* ISO-80000-11 item 11-8.5 Hartmann number */
    attribute def HartmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.5 Hartmann number
         * symbol(s): `Ha`
         * application domain: generic
         * name: HartmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: The Hartmann number represents also the ratio of magnetic force to viscous force.
         */
    }
    attribute hartmannNumber: HartmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.6 Cowling number, Euler magnetic number */
    attribute def CowlingNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.6 Cowling number, Euler magnetic number
         * symbol(s): `Co`
         * application domain: magnetism
         * name: CowlingNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).
         */
    }
    attribute cowlingNumber: CowlingNumberValue :> scalarQuantities;

    alias eulerMagneticNumber for cowlingNumber;

    /* ISO-80000-11 item 11-8.7 Stuart electrical number */
    attribute def StuartElectricalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.7 Stuart electrical number
         * symbol(s): `Se`
         * application domain: generic
         * name: StuartElectricalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).
         */
    }
    attribute stuartElectricalNumber: StuartElectricalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.8 magnetic pressure number */
    attribute def MagneticPressureNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.8 magnetic pressure number
         * symbol(s): `N_(mp)`
         * application domain: generic
         * name: MagneticPressureNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute magneticPressureNumber: MagneticPressureNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.9 Chandrasekhar number */
    attribute def ChandrasekharNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.9 Chandrasekhar number
         * symbol(s): `Q`, `Ch`
         * application domain: generic
         * name: ChandrasekharNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).
         */
    }
    attribute chandrasekharNumber: ChandrasekharNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.10 Prandtl magnetic number */
    attribute def PrandtlMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.10 Prandtl magnetic number
         * symbol(s): `Pr_m`
         * application domain: generic
         * name: PrandtlMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.
         */
    }
    attribute prandtlMagneticNumber: PrandtlMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.11 Roberts number */
    attribute def RobertsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.11 Roberts number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RobertsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).
         */
    }
    attribute robertsNumber: RobertsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.12 Stuart number */
    attribute def StuartNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.12 Stuart number
         * symbol(s): `Stw`
         * application domain: generic
         * name: StuartNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. 
         */
    }
    attribute stuartNumber: StuartNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.13 magnetic number */
    attribute def MagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.13 magnetic number
         * symbol(s): `N_(mg)`
         * application domain: generic
         * name: MagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute magneticNumber: MagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.14 electric field parameter */
    attribute def ElectricFieldParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.14 electric field parameter
         * symbol(s): `Ef`
         * application domain: generic
         * name: ElectricFieldParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute electricFieldParameter: ElectricFieldParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.15 Hall number */
    attribute def HallNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.15 Hall number
         * symbol(s): `Hc`, `CH`
         * application domain: generic
         * name: HallNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)
         * remarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.
         */
    }
    attribute hallNumber: HallNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.16 Lundquist number */
    attribute def LundquistNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.16 Lundquist number
         * symbol(s): `Lu`
         * application domain: generic
         * name: LundquistNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).
         */
    }
    attribute lundquistNumber: LundquistNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.17 Joule magnetic number */
    attribute def JouleMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.17 Joule magnetic number
         * symbol(s): `Jo_m`
         * application domain: generic
         * name: JouleMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: This number is also called magnetic Joule number.
         */
    }
    attribute jouleMagneticNumber: JouleMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.18 Grashof magnetic number */
    attribute def GrashofMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.18 Grashof magnetic number
         * symbol(s): `Gr_m`
         * application domain: generic
         * name: GrashofMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).
         */
    }
    attribute grashofMagneticNumber: GrashofMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.19 Naze number */
    attribute def NazeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.19 Naze number
         * symbol(s): `Na`
         * application domain: generic
         * name: NazeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.
         */
    }
    attribute nazeNumber: NazeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.20 Reynolds electric number */
    attribute def ReynoldsElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.20 Reynolds electric number
         * symbol(s): `Re_e`
         * application domain: generic
         * name: ReynoldsElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers
         * remarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.
         */
    }
    attribute reynoldsElectricNumber: ReynoldsElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.21 Ampère number */
    attribute def 'AmpèreNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.21 Ampère number
         * symbol(s): `Am`
         * application domain: generic
         * name: AmpèreNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)
         * remarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).
         */
    }
    attribute 'ampèreNumber': 'AmpèreNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-9.1 Arrhenius number */
    attribute def ArrheniusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.1 Arrhenius number
         * symbol(s): `α`
         * application domain: generic
         * name: ArrheniusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute arrheniusNumber: ArrheniusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-9.2 Landau-Ginzburg number */
    attribute def LandauGinzburgNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.2 Landau-Ginzburg number
         * symbol(s): `κ`
         * application domain: generic
         * name: LandauGinzburgNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)
         * remarks: None.
         */
    }
    attribute landauGinzburgNumber: LandauGinzburgNumberValue :> scalarQuantities;

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_characteristic_numbers.md"
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
        (range (start 20 4) (end 20 956))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 928))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 4) (end 54 818))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 1203))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 88 4) (end 88 1158))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 819))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 4) (end 122 804))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 4) (end 139 854))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 4) (end 161 923))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 4) (end 178 877))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 4) (end 195 968))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 212 4) (end 212 843))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 229 4) (end 229 755))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 246 4) (end 246 881))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 263 4) (end 263 858))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 280 4) (end 280 785))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 4) (end 299 811))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 316 4) (end 316 909))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 4) (end 333 972))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 352 4) (end 352 1000))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 369 4) (end 369 703))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 386 4) (end 386 885))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 4) (end 405 885))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 422 4) (end 422 872))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 4) (end 441 918))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 458 4) (end 458 871))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 475 4) (end 475 928))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 492 4) (end 492 744))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 509 4) (end 509 731))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 526 4) (end 526 962))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 543 4) (end 543 896))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 560 4) (end 560 817))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 577 4) (end 577 1093))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 596 4) (end 596 762))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 613 4) (end 613 692))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 630 4) (end 630 1094))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 649 4) (end 649 924))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 666 4) (end 666 865))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 683 4) (end 683 1238))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 700 4) (end 700 917))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 717 4) (end 717 787))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 734 4) (end 734 882))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 751 4) (end 751 889))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 768 4) (end 768 1194))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 785 4) (end 785 734))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 802 4) (end 802 1268))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 819 4) (end 819 843))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 836 4) (end 836 1110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 853 4) (end 853 1061))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 874 4) (end 874 763))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 891 4) (end 891 648))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 908 4) (end 908 826))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 925 4) (end 925 879))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 942 4) (end 942 853))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 959 4) (end 959 700))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 976 4) (end 976 814))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 995 4) (end 995 761))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1012 4) (end 1012 715))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1029 4) (end 1029 983))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1046 4) (end 1046 878))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1063 4) (end 1063 1170))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1080 4) (end 1080 978))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1097 4) (end 1097 1061))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1114 4) (end 1114 1104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1131 4) (end 1131 1033))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1148 4) (end 1148 898))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1165 4) (end 1165 866))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1182 4) (end 1182 1233))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1199 4) (end 1199 697))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1216 4) (end 1216 942))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1233 4) (end 1233 1139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1250 4) (end 1250 1261))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1269 4) (end 1269 974))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1286 4) (end 1286 834))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1303 4) (end 1303 1148))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1320 4) (end 1320 861))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1337 4) (end 1337 879))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1354 4) (end 1354 919))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1371 4) (end 1371 1025))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1388 4) (end 1388 746))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1405 4) (end 1405 943))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1422 4) (end 1422 963))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1439 4) (end 1439 731))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1456 4) (end 1456 860))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1473 4) (end 1473 1009))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1490 4) (end 1490 677))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1509 4) (end 1509 644))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1526 4) (end 1526 835))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1543 4) (end 1543 719))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1560 4) (end 1560 802))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1577 4) (end 1577 736))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1594 4) (end 1594 1063))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1611 4) (end 1611 832))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1628 4) (end 1628 889))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1645 4) (end 1645 1046))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1666 4) (end 1666 834))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1683 4) (end 1683 1038))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1702 4) (end 1702 816))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1719 4) (end 1719 841))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1736 4) (end 1736 913))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1753 4) (end 1753 1038))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1770 4) (end 1770 869))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1787 4) (end 1787 994))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1804 4) (end 1804 773))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1821 4) (end 1821 704))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1838 4) (end 1838 766))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1855 4) (end 1855 1246))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1872 4) (end 1872 854))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1889 4) (end 1889 1142))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1906 4) (end 1906 772))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1923 4) (end 1923 1117))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1940 4) (end 1940 1015))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1957 4) (end 1957 765))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1974 4) (end 1974 790))
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
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
RegularComment,
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
KwAlias,Ident,KwFor,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
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
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
KwAlias,UnrestrictedName,KwFor,Ident,Semicolon,
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
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
KwAlias,UnrestrictedName,KwFor,UnrestrictedName,Semicolon,
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
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,UnrestrictedName,ColonGt,Ident,Semicolon,
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
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQCharacteristicNumbers'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (attribute_def 'ReynoldsNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reynoldsNumber' : 'ReynoldsNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'EulerNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'eulerNumber' : 'EulerNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'FroudeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'froudeNumber' : 'FroudeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GrashofNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'grashofNumber' : 'GrashofNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'WeberNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'weberNumber' : 'WeberNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MachNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'machNumber' : 'MachNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'KnudsenNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'knudsenNumber' : 'KnudsenNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StrouhalNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'strouhalNumber' : 'StrouhalNumberValue' :> 'scalarQuantities')
    (alias_member 'thomsonNumber' for 'strouhalNumber')
    (comment)
    (comment)
    (comment)
    (attribute_def 'BagnoldNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bagnoldNumber' : 'BagnoldNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BagnoldNumberForSolidParticlesValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bagnoldNumberForSolidParticles' : 'BagnoldNumberForSolidParticlesValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LiftCoefficientValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'liftCoefficient' : 'LiftCoefficientValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ThrustCoefficientValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'thrustCoefficient' : 'ThrustCoefficientValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DeanNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'deanNumber' : 'DeanNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BejanNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bejanNumber' : 'BejanNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LagrangeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lagrangeNumber' : 'LagrangeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BinghamNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'binghamNumber' : 'BinghamNumberValue' :> 'scalarQuantities')
    (alias_member 'plasticityNumber' for 'binghamNumber')
    (comment)
    (attribute_def ''HedströmNumberValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''hedströmNumber'' : ''HedströmNumberValue'' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BodensteinNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bodensteinNumber' : 'BodensteinNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RossbyNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'rossbyNumber' : 'RossbyNumberValue' :> 'scalarQuantities')
    (alias_member 'kiebelNumber' for 'rossbyNumber')
    (comment)
    (attribute_def 'EkmanNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'ekmanNumber' : 'EkmanNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ElasticityNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'elasticityNumber' : 'ElasticityNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DarcyFrictionFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'darcyFrictionFactor' : 'DarcyFrictionFactorValue' :> 'scalarQuantities')
    (alias_member 'moodyFrictionFactor' for 'darcyFrictionFactor')
    (comment)
    (attribute_def 'FanningNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'fanningNumber' : 'FanningNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GoertlerNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'goertlerNumber' : 'GoertlerNumberValue' :> 'scalarQuantities')
    (alias_member 'goertlerParameter' for 'goertlerNumber')
    (comment)
    (attribute_def 'HagenNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'hagenNumber' : 'HagenNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LavalNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lavalNumber' : 'LavalNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PoiseuilleNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'poiseuilleNumber' : 'PoiseuilleNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PowerNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'powerNumber' : 'PowerNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RichardsonNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'richardsonNumber' : 'RichardsonNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ReechNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reechNumber' : 'ReechNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StokesNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stokesNumber' : 'StokesNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StokesNumberForVibratingParticlesValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stokesNumberForVibratingParticles' : 'StokesNumberForVibratingParticlesValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StokesNumberForRotameterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stokesNumberForRotameter' : 'StokesNumberForRotameterValue' :> 'scalarQuantities')
    (alias_member 'powerCoefficient' for 'stokesNumber')
    (comment)
    (attribute_def 'StokesNumberForGravityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stokesNumberForGravity' : 'StokesNumberForGravityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StokesNumberForDragValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stokesNumberForDrag' : 'StokesNumberForDragValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LaplaceNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'laplaceNumber' : 'LaplaceNumberValue' :> 'scalarQuantities')
    (alias_member 'suratmanNumber' for 'laplaceNumber')
    (comment)
    (attribute_def 'BlakeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'blakeNumber' : 'BlakeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'SommerfeldNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'sommerfeldNumber' : 'SommerfeldNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'TaylorNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'taylorNumber' : 'TaylorNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GalileiNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'galileiNumber' : 'GalileiNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'WomersleyNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'womersleyNumber' : 'WomersleyNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'FourierNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'fourierNumber' : 'FourierNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def ''PécletNumberValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''pécletNumber'' : ''PécletNumberValue'' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RayleighNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'rayleighNumber' : 'RayleighNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'FroudeNumberForHeatTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'froudeNumberForHeatTransfer' : 'FroudeNumberForHeatTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'NusseltNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'nusseltNumber' : 'NusseltNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BiotNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'biotNumber' : 'BiotNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StantonNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stantonNumber' : 'StantonNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'JFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'jFactor' : 'JFactorValue' :> 'scalarQuantities')
    (alias_member 'heatTransferFactor' for 'jFactor')
    (alias_member 'colburnNumber' for 'jFactor')
    (comment)
    (attribute_def 'BejanNumberForHeatTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bejanNumberForHeatTransfer' : 'BejanNumberForHeatTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BejanNumberForEntropyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bejanNumberForEntropy' : 'BejanNumberForEntropyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StefanNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stefanNumber' : 'StefanNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BrinkmanNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'brinkmanNumber' : 'BrinkmanNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ClausiusNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'clausiusNumber' : 'ClausiusNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CarnotNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'carnotNumber' : 'CarnotNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'EckertNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'eckertNumber' : 'EckertNumberValue' :> 'scalarQuantities')
    (alias_member 'dulongNumber' for 'eckertNumber')
    (comment)
    (attribute_def 'GraetzNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'graetzNumber' : 'GraetzNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'HeatTransferNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'heatTransferNumber' : 'HeatTransferNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PomerantsevNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'pomerantsevNumber' : 'PomerantsevNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BoltzmannNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'boltzmannNumber' : 'BoltzmannNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StarkNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'starkNumber' : 'StarkNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'FourierNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'fourierNumberForMassTransfer' : 'FourierNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def ''PécletNumberForMassTransferValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''pécletNumberForMassTransfer'' : ''PécletNumberForMassTransferValue'' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GrashofNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'grashofNumberForMassTransfer' : 'GrashofNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'NusseltNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'nusseltNumberForMassTransfer' : 'NusseltNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StantonNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stantonNumberForMassTransfer' : 'StantonNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GraetzNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'graetzNumberForMassTransfer' : 'GraetzNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MassTransferFactorValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massTransferFactor' : 'MassTransferFactorValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AtwoodNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'atwoodNumber' : 'AtwoodNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BiotNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'biotNumberForMassTransfer' : 'BiotNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MortonNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'mortonNumber' : 'MortonNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BondNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bondNumber' : 'BondNumberValue' :> 'scalarQuantities')
    (alias_member ''eötvösNumber'' for 'bondNumber')
    (comment)
    (attribute_def 'ArchimedesNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'archimedesNumber' : 'ArchimedesNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ExpansionNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'expansionNumber' : 'ExpansionNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MarangoniNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'marangoniNumber' : 'MarangoniNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LockhartMartinelliParameterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lockhartMartinelliParameter' : 'LockhartMartinelliParameterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BejanNumberForMassTransferValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'bejanNumberForMassTransfer' : 'BejanNumberForMassTransferValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CavitationNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'cavitationNumber' : 'CavitationNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'AbsorptionNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'absorptionNumber' : 'AbsorptionNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CapillaryNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'capillaryNumber' : 'CapillaryNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DynamicCapillaryNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'dynamicCapillaryNumber' : 'DynamicCapillaryNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PrandtlNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'prandtlNumber' : 'PrandtlNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'SchmidtNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'schmidtNumber' : 'SchmidtNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LewisNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lewisNumber' : 'LewisNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'OhnesorgeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'ohnesorgeNumber' : 'OhnesorgeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CauchyNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'cauchyNumber' : 'CauchyNumberValue' :> 'scalarQuantities')
    (alias_member 'aeroelasticityParameter' for 'cauchyNumber')
    (comment)
    (attribute_def 'HookeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'hookeNumber' : 'HookeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'WeissenbergNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'weissenbergNumber' : 'WeissenbergNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'DeborahNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'deborahNumber' : 'DeborahNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LorentzNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lorentzNumber' : 'LorentzNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CompressibilityNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'compressibilityNumber' : 'CompressibilityNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ReynoldsMagneticNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reynoldsMagneticNumber' : 'ReynoldsMagneticNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'BatchelorNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'batchelorNumber' : 'BatchelorNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'NusseltElectricNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'nusseltElectricNumber' : 'NusseltElectricNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def ''AlfvénNumberValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''alfvénNumber'' : ''AlfvénNumberValue'' :> 'scalarQuantities')
    (alias_member 'machMagneticNumber' for ''alfvénNumber'')
    (alias_member ''kármanNumber'' for ''alfvénNumber'')
    (comment)
    (attribute_def 'HartmannNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'hartmannNumber' : 'HartmannNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'CowlingNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'cowlingNumber' : 'CowlingNumberValue' :> 'scalarQuantities')
    (alias_member 'eulerMagneticNumber' for 'cowlingNumber')
    (comment)
    (attribute_def 'StuartElectricalNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stuartElectricalNumber' : 'StuartElectricalNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MagneticPressureNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'magneticPressureNumber' : 'MagneticPressureNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ChandrasekharNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'chandrasekharNumber' : 'ChandrasekharNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'PrandtlMagneticNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'prandtlMagneticNumber' : 'PrandtlMagneticNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RobertsNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'robertsNumber' : 'RobertsNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'StuartNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'stuartNumber' : 'StuartNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MagneticNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'magneticNumber' : 'MagneticNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ElectricFieldParameterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'electricFieldParameter' : 'ElectricFieldParameterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'HallNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'hallNumber' : 'HallNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LundquistNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'lundquistNumber' : 'LundquistNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'JouleMagneticNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'jouleMagneticNumber' : 'JouleMagneticNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'GrashofMagneticNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'grashofMagneticNumber' : 'GrashofMagneticNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'NazeNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'nazeNumber' : 'NazeNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ReynoldsElectricNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'reynoldsElectricNumber' : 'ReynoldsElectricNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def ''AmpèreNumberValue'' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage ''ampèreNumber'' : ''AmpèreNumberValue'' :> 'scalarQuantities')
    (comment)
    (attribute_def 'ArrheniusNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'arrheniusNumber' : 'ArrheniusNumberValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'LandauGinzburgNumberValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'landauGinzburgNumber' : 'LandauGinzburgNumberValue' :> 'scalarQuantities')))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'DimensionOneValue'
semantic.unresolved_name 'scalarQuantities'
~~~
# FORMAT
~~~sysml
standard library package ISQCharacteristicNumbers {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 "Characteristic numbers"
     * see also https://www.iso.org/standard/64982.html
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

    /* ISO-80000-11 item 11-4.1 Reynolds number */
    attribute def ReynoldsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.1 Reynolds number
         * symbol(s): `Re`
         * application domain: generic
         * name: ReynoldsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.
         */
    }
    attribute reynoldsNumber: ReynoldsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.2 Euler number */
    attribute def EulerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.2 Euler number
         * symbol(s): `Eu`
         * application domain: generic
         * name: EulerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^"'" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).
         */
    }
    attribute eulerNumber: EulerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.3 Froude number */
    attribute def FroudeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.3 Froude number
         * symbol(s): `Fr`
         * application domain: generic
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)
         * remarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.
         */
    }
    attribute froudeNumber: FroudeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.4 Grashof number */
    attribute def GrashofNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.4 Grashof number
         * symbol(s): `Gr`
         * application domain: generic
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).
         */
    }
    attribute grashofNumber: GrashofNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.5 Weber number */
    attribute def WeberNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.5 Weber number
         * symbol(s): `We`
         * application domain: generic
         * name: WeberNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)
         * remarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.
         */
    }
    attribute weberNumber: WeberNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.6 Mach number */
    attribute def MachNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.6 Mach number
         * symbol(s): `Ma`
         * application domain: generic
         * name: MachNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid
         * remarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).
         */
    }
    attribute machNumber: MachNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.7 Knudsen number */
    attribute def KnudsenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.7 Knudsen number
         * symbol(s): `Kn`
         * application domain: generic
         * name: KnudsenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.
         */
    }
    attribute knudsenNumber: KnudsenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.8 Strouhal number, Thomson number */
    attribute def StrouhalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.8 Strouhal number, Thomson number
         * symbol(s): `Sr`, `Sh`
         * application domain: generic
         * name: StrouhalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow
         * remarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.
         */
    }
    attribute strouhalNumber: StrouhalNumberValue :> scalarQuantities;

    alias thomsonNumber for strouhalNumber;

    /* ISO-80000-11 item 11-4.9 drag coefficient */
    /* Refer to declaration for DragCoefficient in ISQMechanics item 4-23.4 drag coefficient */

    /* ISO-80000-11 item 11-4.10 Bagnold number */
    attribute def BagnoldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.10 Bagnold number
         * symbol(s): `Bg`
         * application domain: generic
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body
         * remarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.
         */
    }
    attribute bagnoldNumber: BagnoldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.11 Bagnold number */
    attribute def BagnoldNumberForSolidParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.11 Bagnold number
         * symbol(s): `Ba_2`
         * application domain: solid particles
         * name: BagnoldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles
         * remarks: None.
         */
    }
    attribute bagnoldNumberForSolidParticles: BagnoldNumberForSolidParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.12 lift coefficient */
    attribute def LiftCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.12 lift coefficient
         * symbol(s): `c_l`, `c_A`
         * application domain: generic
         * name: LiftCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure
         * remarks: The lift coefficient is dependant on the shape of the wing.
         */
    }
    attribute liftCoefficient: LiftCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.13 thrust coefficient */
    attribute def ThrustCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.13 thrust coefficient
         * symbol(s): `c_t`
         * application domain: generic
         * name: ThrustCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller
         * remarks: The thrust coefficient is dependant on the shape of the propeller.
         */
    }
    attribute thrustCoefficient: ThrustCoefficientValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.14 Dean number */
    attribute def DeanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.14 Dean number
         * symbol(s): `Dn`
         * application domain: generic
         * name: DeanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe
         * remarks: None.
         */
    }
    attribute deanNumber: DeanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.15 Bejan number */
    attribute def BejanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.15 Bejan number
         * symbol(s): `Be`
         * application domain: generic
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)
         * remarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.
         */
    }
    attribute bejanNumber: BejanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.16 Lagrange number */
    attribute def LagrangeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.16 Lagrange number
         * symbol(s): `Lg`
         * application domain: generic
         * name: LagrangeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).
         */
    }
    attribute lagrangeNumber: LagrangeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.17 Bingham number, plasticity number */
    attribute def BinghamNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.17 Bingham number, plasticity number
         * symbol(s): `Bm`, `Bn`
         * application domain: generic
         * name: BinghamNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute binghamNumber: BinghamNumberValue :> scalarQuantities;

    alias plasticityNumber for binghamNumber;

    /* ISO-80000-11 item 11-4.18 Hedström number */
    attribute def 'HedströmNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.18 Hedström number
         * symbol(s): `He`, `Hd`
         * application domain: generic
         * name: HedströmNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute 'hedströmNumber': 'HedströmNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-4.19 Bodenstein number */
    attribute def BodensteinNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.19 Bodenstein number
         * symbol(s): `Bd`
         * application domain: generic
         * name: BodensteinNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Bodenstein number is also given by `Bd = Pe^"*" = Re*Sc`, where `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).
         */
    }
    attribute bodensteinNumber: BodensteinNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.20 Rossby number, Kiebel number */
    attribute def RossbyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.20 Rossby number, Kiebel number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RossbyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude
         * remarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.
         */
    }
    attribute rossbyNumber: RossbyNumberValue :> scalarQuantities;

    alias kiebelNumber for rossbyNumber;

    /* ISO-80000-11 item 11-4.21 Ekman number */
    attribute def EkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.21 Ekman number
         * symbol(s): `Ek`
         * application domain: generic
         * name: EkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude
         * remarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).
         */
    }
    attribute ekmanNumber: EkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.22 elasticity number */
    attribute def ElasticityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.22 elasticity number
         * symbol(s): `El`
         * application domain: generic
         * name: ElasticityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe
         * remarks: See also Deborah number (item 11-7.8).
         */
    }
    attribute elasticityNumber: ElasticityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.23 Darcy friction factor, Moody friction factor */
    attribute def DarcyFrictionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.23 Darcy friction factor, Moody friction factor
         * symbol(s): `f_D`
         * application domain: generic
         * name: DarcyFrictionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute darcyFrictionFactor: DarcyFrictionFactorValue :> scalarQuantities;

    alias moodyFrictionFactor for darcyFrictionFactor;

    /* ISO-80000-11 item 11-4.24 Fanning number */
    attribute def FanningNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.24 Fanning number
         * symbol(s): `f_n`, `f`
         * application domain: generic
         * name: FanningNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe
         * remarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.
         */
    }
    attribute fanningNumber: FanningNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.25 Goertler number, Goertler parameter */
    attribute def GoertlerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.25 Goertler number, Goertler parameter
         * symbol(s): `Go`
         * application domain: generic
         * name: GoertlerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)
         * remarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.
         */
    }
    attribute goertlerNumber: GoertlerNumberValue :> scalarQuantities;

    alias goertlerParameter for goertlerNumber;

    /* ISO-80000-11 item 11-4.26 Hagen number */
    attribute def HagenNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.26 Hagen number
         * symbol(s): `Hg`, `Ha`
         * application domain: generic
         * name: HagenNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).
         */
    }
    attribute hagenNumber: HagenNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.27 Laval number */
    attribute def LavalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.27 Laval number
         * symbol(s): `La`
         * application domain: generic
         * name: LavalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3),  `R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)
         * remarks: The Laval number is a specific kind of Mach number (item 11-4.6).
         */
    }
    attribute lavalNumber: LavalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.28 Poiseuille number */
    attribute def PoiseuilleNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.28 Poiseuille number
         * symbol(s): `Poi`
         * application domain: generic
         * name: PoiseuilleNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid
         * remarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).
         */
    }
    attribute poiseuilleNumber: PoiseuilleNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.29 power number */
    attribute def PowerNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.29 power number
         * symbol(s): `Pn`
         * application domain: generic
         * name: PowerNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer
         * remarks: None.
         */
    }
    attribute powerNumber: PowerNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.30 Richardson number */
    attribute def RichardsonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.30 Richardson number
         * symbol(s): `Ri`
         * application domain: generic
         * name: RichardsonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)
         * remarks: In geophysics differences of these quantities are of interest.
         */
    }
    attribute richardsonNumber: RichardsonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.31 Reech number */
    attribute def ReechNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.31 Reech number
         * symbol(s): `Ree`
         * application domain: generic
         * name: ReechNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water
         * remarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .
         */
    }
    attribute reechNumber: ReechNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.32 Stokes number */
    attribute def StokesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.32 Stokes number
         * symbol(s): `Stk`
         * application domain: time-related
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence
         * remarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.
         */
    }
    attribute stokesNumber: StokesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.33 Stokes number */
    attribute def StokesNumberForVibratingParticlesValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.33 Stokes number
         * symbol(s): `Stk_1`
         * application domain: vibrating particles
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute stokesNumberForVibratingParticles: StokesNumberForVibratingParticlesValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.34 Stokes number, power coefficient */
    attribute def StokesNumberForRotameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.34 Stokes number, power coefficient
         * symbol(s): `Stk_2`
         * application domain: rotameter
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).
         */
    }
    attribute stokesNumberForRotameter: StokesNumberForRotameterValue :> scalarQuantities;

    alias powerCoefficient for stokesNumber;

    /* ISO-80000-11 item 11-4.35 Stokes number */
    attribute def StokesNumberForGravityValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.35 Stokes number
         * symbol(s): `Stk_3`
         * application domain: gravity
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall
         * remarks: None.
         */
    }
    attribute stokesNumberForGravity: StokesNumberForGravityValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.36 Stokes number */
    attribute def StokesNumberForDragValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.36 Stokes number
         * symbol(s): `Stk_4`
         * application domain: drag
         * name: StokesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute stokesNumberForDrag: StokesNumberForDragValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.37 Laplace number, Suratman number */
    attribute def LaplaceNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.37 Laplace number, Suratman number
         * symbol(s): `La`, `Su`
         * application domain: generic
         * name: LaplaceNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid
         * remarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).
         */
    }
    attribute laplaceNumber: LaplaceNumberValue :> scalarQuantities;

    alias suratmanNumber for laplaceNumber;

    /* ISO-80000-11 item 11-4.38 Blake number */
    attribute def BlakeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.38 Blake number
         * symbol(s): `Bl`
         * application domain: generic
         * name: BlakeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)
         * remarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.
         */
    }
    attribute blakeNumber: BlakeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.39 Sommerfeld number */
    attribute def SommerfeldNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.39 Sommerfeld number
         * symbol(s): `So`, `Sm`
         * application domain: generic
         * name: SommerfeldNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus
         * remarks: Sometimes the inverse of this number is wrongly used.
         */
    }
    attribute sommerfeldNumber: SommerfeldNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.40 Taylor number */
    attribute def TaylorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.40 Taylor number
         * symbol(s): `Ta`
         * application domain: momentum transfer
         * name: TaylorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.
         */
    }
    attribute taylorNumber: TaylorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.41 Galilei number */
    attribute def GalileiNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.41 Galilei number
         * symbol(s): `Ga`
         * application domain: generic
         * name: GalileiNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid
         * remarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).
         */
    }
    attribute galileiNumber: GalileiNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-4.42 Womersley number */
    attribute def WomersleyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-4.42 Womersley number
         * symbol(s): `Wo`, `α`
         * application domain: generic
         * name: WomersleyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Womersley number is used for pulsating flows e.g. in blood flow.
         */
    }
    attribute womersleyNumber: WomersleyNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.1 Fourier number */
    attribute def FourierNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.1 Fourier number
         * symbol(s): `Fo`
         * application domain: heat transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)
         * remarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.
         */
    }
    attribute fourierNumber: FourierNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.2 Péclet number */
    attribute def 'PécletNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.2 Péclet number
         * symbol(s): `Pe`
         * application domain: heat transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.
         */
    }
    attribute 'pécletNumber': 'PécletNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-5.3 Rayleigh number */
    attribute def RayleighNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.3 Rayleigh number
         * symbol(s): `Ra`
         * application domain: generic
         * name: RayleighNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid
         * remarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).
         */
    }
    attribute rayleighNumber: RayleighNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.4 Froude number */
    attribute def FroudeNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.4 Froude number
         * symbol(s): `Fr^"*"`
         * application domain: heat transfer
         * name: FroudeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^"*" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)"
         * remarks: None.
         */
    }
    attribute froudeNumberForHeatTransfer: FroudeNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.5 Nusselt number */
    attribute def NusseltNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.5 Nusselt number
         * symbol(s): `Nu`
         * application domain: heat transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)
         * remarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the "Biot number for heat transfer" (item 11-5.6) is used.
         */
    }
    attribute nusseltNumber: NusseltNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.6 Biot number */
    attribute def BiotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.6 Biot number
         * symbol(s): `Bi`
         * application domain: heat transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body
         * remarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.
         */
    }
    attribute biotNumber: BiotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.7 Stanton number */
    attribute def StantonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.7 Stanton number
         * symbol(s): `St`
         * application domain: heat transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid
         * remarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.
         */
    }
    attribute stantonNumber: StantonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.8 j-factor, heat transfer factor, Colburn number */
    attribute def JFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.8 j-factor, heat transfer factor, Colburn number
         * symbol(s): `j`, `Co`, `Jq`
         * application domain: heat transfer
         * name: JFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).
         */
    }
    attribute jFactor: JFactorValue :> scalarQuantities;

    alias heatTransferFactor for jFactor;

    alias colburnNumber for jFactor;

    /* ISO-80000-11 item 11-5.9 Bejan number */
    attribute def BejanNumberForHeatTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.9 Bejan number
         * symbol(s): `Be_1`
         * application domain: heat transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute bejanNumberForHeatTransfer: BejanNumberForHeatTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.10 Bejan number */
    attribute def BejanNumberForEntropyValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.10 Bejan number
         * symbol(s): `Be_S`
         * application domain: entropy
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction
         * remarks: None.
         */
    }
    attribute bejanNumberForEntropy: BejanNumberForEntropyValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.11 Stefan number */
    attribute def StefanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.11 Stefan number
         * symbol(s): `Ste`, `Stf`
         * application domain: phase transition
         * name: StefanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute stefanNumber: StefanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.12 Brinkman number */
    attribute def BrinkmanNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.12 Brinkman number
         * symbol(s): `Br`, `N_(Br)`
         * application domain: generic
         * name: BrinkmanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature
         * remarks: None.
         */
    }
    attribute brinkmanNumber: BrinkmanNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.13 Clausius number */
    attribute def ClausiusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.13 Clausius number
         * symbol(s): `Cl`
         * application domain: generic
         * name: ClausiusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`
         * remarks: None.
         */
    }
    attribute clausiusNumber: ClausiusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.14 Carnot number */
    attribute def CarnotNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.14 Carnot number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CarnotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively
         * remarks: None.
         */
    }
    attribute carnotNumber: CarnotNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.15 Eckert number, Dulong number */
    attribute def EckertNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.15 Eckert number, Dulong number
         * symbol(s): `Ec`
         * application domain: generic
         * name: EckertNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)
         * remarks: None.
         */
    }
    attribute eckertNumber: EckertNumberValue :> scalarQuantities;

    alias dulongNumber for eckertNumber;

    /* ISO-80000-11 item 11-5.16 Graetz number */
    attribute def GraetzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.16 Graetz number
         * symbol(s): `Gz`
         * application domain: heat transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe
         * remarks: None.
         */
    }
    attribute graetzNumber: GraetzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.17 heat transfer number */
    attribute def HeatTransferNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.17 heat transfer number
         * symbol(s): `K_Q`
         * application domain: generic
         * name: HeatTransferNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute heatTransferNumber: HeatTransferNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.18 Pomerantsev number */
    attribute def PomerantsevNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.18 Pomerantsev number
         * symbol(s): `Po`, `Pov`
         * application domain: heat transfer
         * name: PomerantsevNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)
         * remarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.
         */
    }
    attribute pomerantsevNumber: PomerantsevNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.19 Boltzmann number */
    attribute def BoltzmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.19 Boltzmann number
         * symbol(s): `Bz`, `Bol`, `Bo`
         * application domain: generic
         * name: BoltzmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute boltzmannNumber: BoltzmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-5.20 Stark number */
    attribute def StarkNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-5.20 Stark number
         * symbol(s): `Sk`
         * application domain: generic
         * name: StarkNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)
         * remarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.
         */
    }
    attribute starkNumber: StarkNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.1 Fourier number */
    attribute def FourierNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.1 Fourier number
         * symbol(s): `Fo^"*"`
         * application domain: mass transfer
         * name: FourierNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^"*" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer"
         * remarks: The Fourier number for mass transfer is also given by `Fo^*" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1)."
         */
    }
    attribute fourierNumberForMassTransfer: FourierNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.2 Péclet number */
    attribute def 'PécletNumberForMassTransferValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.2 Péclet number
         * symbol(s): `Pe^"*"`, `Bd`, `Bod`
         * application domain: mass transfer
         * name: PécletNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The Péclet number for mass transfer is also given by `Pe^"*" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.
         */
    }
    attribute 'pécletNumberForMassTransfer': 'PécletNumberForMassTransferValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-6.3 Grashof number */
    attribute def GrashofNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.3 Grashof number
         * symbol(s): `Gr^"*"`
         * application domain: mass transfer
         * name: GrashofNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^"*" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)"
         * remarks: Instead of "amount-of-substance fraction" the "amount-of-substance concentration" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.
         */
    }
    attribute grashofNumberForMassTransfer: GrashofNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.4 Nusselt number */
    attribute def NusseltNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.4 Nusselt number
         * symbol(s): `Nu^"*"`
         * application domain: mass transfer
         * name: NusseltNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^"*" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.
         */
    }
    attribute nusseltNumberForMassTransfer: NusseltNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.5 Stanton number */
    attribute def StantonNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.5 Stanton number
         * symbol(s): `St^"*"`
         * application domain: mass transfer
         * name: StantonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^"*" = k^"*"
         * remarks: The Stanton number for mass transfer is also given by `St^*" = (Nu^"*")/(Pe^"*"*)`, where `Nu^"*"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer."
         */
    }
    attribute stantonNumberForMassTransfer: StantonNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.6 Graetz number */
    attribute def GraetzNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.6 Graetz number
         * symbol(s): `Gz^"*"`
         * application domain: mass transfer
         * name: GraetzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^"*" = (v*d)/D = d/l*Pe^"*"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^"*"` is the Péclet number for mass transfer (item 11-6.2)"
         * remarks: None.
         */
    }
    attribute graetzNumberForMassTransfer: GraetzNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.7 mass transfer factor */
    attribute def MassTransferFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.7 mass transfer factor
         * symbol(s): `j^"*"`
         * application domain: mass transfer
         * name: MassTransferFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)"
         * remarks: The mass transfer factor is also given by `j_m = j^*" = St^"*" * (Sc)^(2/3)` where `St^"*"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17)."
         */
    }
    attribute massTransferFactor: MassTransferFactorValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.8 Atwood number */
    attribute def AtwoodNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.8 Atwood number
         * symbol(s): `At`
         * application domain: generic
         * name: AtwoodNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid
         * remarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.
         */
    }
    attribute atwoodNumber: AtwoodNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.9 Biot number */
    attribute def BiotNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.9 Biot number
         * symbol(s): `Bi^"*"`
         * application domain: mass transfer
         * name: BiotNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*" = (k*l)/D_"int"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_"int"` is diffusion coefficient (ISO 80000-9) at the interface"
         * remarks: None.
         */
    }
    attribute biotNumberForMassTransfer: BiotNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.10 Morton number */
    attribute def MortonNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.10 Morton number
         * symbol(s): `Mo`
         * application domain: generic
         * name: MortonNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop
         * remarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute mortonNumber: MortonNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.11 Bond number, Eötvös number */
    attribute def BondNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.11 Bond number, Eötvös number
         * symbol(s): `Bo`, `Eo`
         * application domain: generic
         * name: BondNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble
         * remarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.
         */
    }
    attribute bondNumber: BondNumberValue :> scalarQuantities;

    alias 'eötvösNumber' for bondNumber;

    /* ISO-80000-11 item 11-6.12 Archimedes number */
    attribute def ArchimedesNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.12 Archimedes number
         * symbol(s): `Ar`
         * application domain: generic
         * name: ArchimedesNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid
         * remarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).
         */
    }
    attribute archimedesNumber: ArchimedesNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.13 expansion number */
    attribute def ExpansionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.13 expansion number
         * symbol(s): `Ex`
         * application domain: generic
         * name: ExpansionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid
         * remarks: None.
         */
    }
    attribute expansionNumber: ExpansionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.14 Marangoni number */
    attribute def MarangoniNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.14 Marangoni number
         * symbol(s): `Mg`, `Mar`
         * application domain: generic
         * name: MarangoniNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film
         * remarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.
         */
    }
    attribute marangoniNumber: MarangoniNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.15 Lockhart-Martinelli parameter */
    attribute def LockhartMartinelliParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.15 Lockhart-Martinelli parameter
         * symbol(s): `Lp`
         * application domain: generic
         * name: LockhartMartinelliParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density
         * remarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.
         */
    }
    attribute lockhartMartinelliParameter: LockhartMartinelliParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.16 Bejan number */
    attribute def BejanNumberForMassTransferValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.16 Bejan number
         * symbol(s): `Be^"*"`, `Be_2`
         * application domain: mass transfer
         * name: BejanNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity"
         * remarks: A similar quantity exists for heat transfer (item 11-5.9).
         */
    }
    attribute bejanNumberForMassTransfer: BejanNumberForMassTransferValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.17 cavitation number */
    attribute def CavitationNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.17 cavitation number
         * symbol(s): `Ca`, `Cn`
         * application domain: generic
         * name: CavitationNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow
         * remarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.
         */
    }
    attribute cavitationNumber: CavitationNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.18 absorption number */
    attribute def AbsorptionNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.18 absorption number
         * symbol(s): `Ab`
         * application domain: generic
         * name: AbsorptionNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter
         * remarks: None.
         */
    }
    attribute absorptionNumber: AbsorptionNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.19 capillary number */
    attribute def CapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.19 capillary number
         * symbol(s): `Ca`
         * application domain: generic
         * name: CapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid
         * remarks: None.
         */
    }
    attribute capillaryNumber: CapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-6.20 dynamic capillary number */
    attribute def DynamicCapillaryNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-6.20 dynamic capillary number
         * symbol(s): `Ca^"*"`, `Cn`
         * application domain: generic
         * name: DynamicCapillaryNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)"
         * remarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.
         */
    }
    attribute dynamicCapillaryNumber: DynamicCapillaryNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.1 Prandtl number */
    attribute def PrandtlNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.1 Prandtl number
         * symbol(s): `Pr`
         * application domain: generic
         * name: PrandtlNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)
         * remarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). 
         */
    }
    attribute prandtlNumber: PrandtlNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.2 Schmidt number */
    attribute def SchmidtNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.2 Schmidt number
         * symbol(s): `Sc`
         * application domain: generic
         * name: SchmidtNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.
         */
    }
    attribute schmidtNumber: SchmidtNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.3 Lewis number */
    attribute def LewisNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.3 Lewis number
         * symbol(s): `Le`
         * application domain: generic
         * name: LewisNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)
         * remarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. 
         */
    }
    attribute lewisNumber: LewisNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.4 Ohnesorge number */
    attribute def OhnesorgeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.4 Ohnesorge number
         * symbol(s): `Oh`
         * application domain: generic
         * name: OhnesorgeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)
         * remarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.
         */
    }
    attribute ohnesorgeNumber: OhnesorgeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.5 Cauchy number, aeroelasticity parameter */
    attribute def CauchyNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.5 Cauchy number, aeroelasticity parameter
         * symbol(s): `Cy`
         * application domain: generic
         * name: CauchyNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute cauchyNumber: CauchyNumberValue :> scalarQuantities;

    alias aeroelasticityParameter for cauchyNumber;

    /* ISO-80000-11 item 11-7.6 Hooke number */
    attribute def HookeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.6 Hooke number
         * symbol(s): `Ho_2`
         * application domain: generic
         * name: HookeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)
         * remarks: None.
         */
    }
    attribute hookeNumber: HookeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.7 Weissenberg number */
    attribute def WeissenbergNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.7 Weissenberg number
         * symbol(s): `Wi`
         * application domain: generic
         * name: WeissenbergNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)
         * remarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.
         */
    }
    attribute weissenbergNumber: WeissenbergNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.8 Deborah number */
    attribute def DeborahNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.8 Deborah number
         * symbol(s): `De`
         * application domain: generic
         * name: DeborahNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)
         * remarks: The stress relaxation time is sometimes called the Maxwell relaxation time.
         */
    }
    attribute deborahNumber: DeborahNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.9 Lorentz number */
    attribute def LorentzNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.9 Lorentz number
         * symbol(s): `Lo`
         * application domain: generic
         * name: LorentzNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points
         * remarks: None.
         */
    }
    attribute lorentzNumber: LorentzNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-7.10 compressibility number */
    attribute def CompressibilityNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-7.10 compressibility number
         * symbol(s): `Z`
         * application domain: generic
         * name: CompressibilityNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute compressibilityNumber: CompressibilityNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.1 Reynolds magnetic number */
    attribute def ReynoldsMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.1 Reynolds magnetic number
         * symbol(s): `Rm`
         * application domain: generic
         * name: ReynoldsMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)
         * remarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).
         */
    }
    attribute reynoldsMagneticNumber: ReynoldsMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.2 Batchelor number */
    attribute def BatchelorNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.2 Batchelor number
         * symbol(s): `Bt`
         * application domain: generic
         * name: BatchelorNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute batchelorNumber: BatchelorNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.3 Nusselt electric number */
    attribute def NusseltElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.3 Nusselt electric number
         * symbol(s): `Ne`
         * application domain: generic
         * name: NusseltElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^"*" = D^"+" + D^"-"`, where `D^"+"`, `D^"-"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively"
         * remarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.
         */
    }
    attribute nusseltElectricNumber: NusseltElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.4 Alfvén number, Mach magnetic number, Kárman number */
    attribute def 'AlfvénNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number
         * symbol(s): `Al`
         * application domain: generic
         * name: AlfvénNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: Often, the inverse of this number is wrongly used. The name "Alfvén Mach number" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute 'alfvénNumber': 'AlfvénNumberValue' :> scalarQuantities;

    alias machMagneticNumber for 'alfvénNumber';

    alias 'kármanNumber' for 'alfvénNumber';

    /* ISO-80000-11 item 11-8.5 Hartmann number */
    attribute def HartmannNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.5 Hartmann number
         * symbol(s): `Ha`
         * application domain: generic
         * name: HartmannNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)
         * remarks: The Hartmann number represents also the ratio of magnetic force to viscous force.
         */
    }
    attribute hartmannNumber: HartmannNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.6 Cowling number, Euler magnetic number */
    attribute def CowlingNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.6 Cowling number, Euler magnetic number
         * symbol(s): `Co`
         * application domain: magnetism
         * name: CowlingNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).
         */
    }
    attribute cowlingNumber: CowlingNumberValue :> scalarQuantities;

    alias eulerMagneticNumber for cowlingNumber;

    /* ISO-80000-11 item 11-8.7 Stuart electrical number */
    attribute def StuartElectricalNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.7 Stuart electrical number
         * symbol(s): `Se`
         * application domain: generic
         * name: StuartElectricalNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).
         */
    }
    attribute stuartElectricalNumber: StuartElectricalNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.8 magnetic pressure number */
    attribute def MagneticPressureNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.8 magnetic pressure number
         * symbol(s): `N_(mp)`
         * application domain: generic
         * name: MagneticPressureNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).
         */
    }
    attribute magneticPressureNumber: MagneticPressureNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.9 Chandrasekhar number */
    attribute def ChandrasekharNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.9 Chandrasekhar number
         * symbol(s): `Q`, `Ch`
         * application domain: generic
         * name: ChandrasekharNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).
         */
    }
    attribute chandrasekharNumber: ChandrasekharNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.10 Prandtl magnetic number */
    attribute def PrandtlMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.10 Prandtl magnetic number
         * symbol(s): `Pr_m`
         * application domain: generic
         * name: PrandtlMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.
         */
    }
    attribute prandtlMagneticNumber: PrandtlMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.11 Roberts number */
    attribute def RobertsNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.11 Roberts number
         * symbol(s): `Ro`
         * application domain: generic
         * name: RobertsNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).
         */
    }
    attribute robertsNumber: RobertsNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.12 Stuart number */
    attribute def StuartNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.12 Stuart number
         * symbol(s): `Stw`
         * application domain: generic
         * name: StuartNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)
         * remarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. 
         */
    }
    attribute stuartNumber: StuartNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.13 magnetic number */
    attribute def MagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.13 magnetic number
         * symbol(s): `N_(mg)`
         * application domain: generic
         * name: MagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute magneticNumber: MagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.14 electric field parameter */
    attribute def ElectricFieldParameterValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.14 electric field parameter
         * symbol(s): `Ef`
         * application domain: generic
         * name: ElectricFieldParameter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: None.
         */
    }
    attribute electricFieldParameter: ElectricFieldParameterValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.15 Hall number */
    attribute def HallNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.15 Hall number
         * symbol(s): `Hc`, `CH`
         * application domain: generic
         * name: HallNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)
         * remarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.
         */
    }
    attribute hallNumber: HallNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.16 Lundquist number */
    attribute def LundquistNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.16 Lundquist number
         * symbol(s): `Lu`
         * application domain: generic
         * name: LundquistNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).
         */
    }
    attribute lundquistNumber: LundquistNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.17 Joule magnetic number */
    attribute def JouleMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.17 Joule magnetic number
         * symbol(s): `Jo_m`
         * application domain: generic
         * name: JouleMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)
         * remarks: This number is also called magnetic Joule number.
         */
    }
    attribute jouleMagneticNumber: JouleMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.18 Grashof magnetic number */
    attribute def GrashofMagneticNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.18 Grashof magnetic number
         * symbol(s): `Gr_m`
         * application domain: generic
         * name: GrashofMagneticNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)
         * remarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).
         */
    }
    attribute grashofMagneticNumber: GrashofMagneticNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.19 Naze number */
    attribute def NazeNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.19 Naze number
         * symbol(s): `Na`
         * application domain: generic
         * name: NazeNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)
         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.
         */
    }
    attribute nazeNumber: NazeNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.20 Reynolds electric number */
    attribute def ReynoldsElectricNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.20 Reynolds electric number
         * symbol(s): `Re_e`
         * application domain: generic
         * name: ReynoldsElectricNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers
         * remarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.
         */
    }
    attribute reynoldsElectricNumber: ReynoldsElectricNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-8.21 Ampère number */
    attribute def 'AmpèreNumberValue' :> DimensionOneValue {
        doc
        /*
         * source: item 11-8.21 Ampère number
         * symbol(s): `Am`
         * application domain: generic
         * name: AmpèreNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)
         * remarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).
         */
    }
    attribute 'ampèreNumber': 'AmpèreNumberValue' :> scalarQuantities;

    /* ISO-80000-11 item 11-9.1 Arrhenius number */
    attribute def ArrheniusNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.1 Arrhenius number
         * symbol(s): `α`
         * application domain: generic
         * name: ArrheniusNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute arrheniusNumber: ArrheniusNumberValue :> scalarQuantities;

    /* ISO-80000-11 item 11-9.2 Landau-Ginzburg number */
    attribute def LandauGinzburgNumberValue :> DimensionOneValue {
        doc
        /*
         * source: item 11-9.2 Landau-Ginzburg number
         * symbol(s): `κ`
         * application domain: generic
         * name: LandauGinzburgNumber (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)
         * remarks: None.
         */
    }
    attribute landauGinzburgNumber: LandauGinzburgNumberValue :> scalarQuantities;

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "615ec10ea2833ea8b7abbe52b9c07d74493f407844b0aa7c7844ed505014e0e4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (kind "package") (name "ISQCharacteristicNumbers") (declared-name "ISQCharacteristicNumbers") (range (start (line 0) (character 0)) (end (line 0) (character 120114))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (kind "attribute def") (name "AbsorptionNumberValue") (declared-name "AbsorptionNumberValue") (range (start (line 1371) (character 4)) (end (line 1371) (character 1025))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1371) (character 4)) (end (line 1371) (character 1025))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (kind "attribute def") (name "AlfvénNumberValue") (declared-name "AlfvénNumberValue") (range (start (line 1645) (character 4)) (end (line 1645) (character 1046))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1645) (character 4)) (end (line 1645) (character 1046))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (kind "attribute def") (name "AmpèreNumberValue") (declared-name "AmpèreNumberValue") (range (start (line 1940) (character 4)) (end (line 1940) (character 1015))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1940) (character 4)) (end (line 1940) (character 1015))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (kind "attribute def") (name "ArchimedesNumberValue") (declared-name "ArchimedesNumberValue") (range (start (line 1269) (character 4)) (end (line 1269) (character 974))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1269) (character 4)) (end (line 1269) (character 974))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (kind "attribute def") (name "ArrheniusNumberValue") (declared-name "ArrheniusNumberValue") (range (start (line 1957) (character 4)) (end (line 1957) (character 765))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1957) (character 4)) (end (line 1957) (character 765))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (kind "attribute def") (name "AtwoodNumberValue") (declared-name "AtwoodNumberValue") (range (start (line 1199) (character 4)) (end (line 1199) (character 697))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1199) (character 4)) (end (line 1199) (character 697))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (kind "attribute def") (name "BagnoldNumberForSolidParticlesValue") (declared-name "BagnoldNumberForSolidParticlesValue") (range (start (line 178) (character 4)) (end (line 178) (character 877))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 178) (character 4)) (end (line 178) (character 877))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (kind "attribute def") (name "BagnoldNumberValue") (declared-name "BagnoldNumberValue") (range (start (line 161) (character 4)) (end (line 161) (character 923))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 161) (character 4)) (end (line 161) (character 923))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (kind "attribute def") (name "BatchelorNumberValue") (declared-name "BatchelorNumberValue") (range (start (line 1611) (character 4)) (end (line 1611) (character 832))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1611) (character 4)) (end (line 1611) (character 832))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (kind "attribute def") (name "BejanNumberForEntropyValue") (declared-name "BejanNumberForEntropyValue") (range (start (line 891) (character 4)) (end (line 891) (character 648))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 891) (character 4)) (end (line 891) (character 648))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (kind "attribute def") (name "BejanNumberForHeatTransferValue") (declared-name "BejanNumberForHeatTransferValue") (range (start (line 874) (character 4)) (end (line 874) (character 763))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 874) (character 4)) (end (line 874) (character 763))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (kind "attribute def") (name "BejanNumberForMassTransferValue") (declared-name "BejanNumberForMassTransferValue") (range (start (line 1337) (character 4)) (end (line 1337) (character 879))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1337) (character 4)) (end (line 1337) (character 879))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (kind "attribute def") (name "BejanNumberValue") (declared-name "BejanNumberValue") (range (start (line 246) (character 4)) (end (line 246) (character 881))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 246) (character 4)) (end (line 246) (character 881))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (kind "attribute def") (name "BinghamNumberValue") (declared-name "BinghamNumberValue") (range (start (line 280) (character 4)) (end (line 280) (character 785))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 280) (character 4)) (end (line 280) (character 785))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (kind "attribute def") (name "BiotNumberForMassTransferValue") (declared-name "BiotNumberForMassTransferValue") (range (start (line 1216) (character 4)) (end (line 1216) (character 942))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1216) (character 4)) (end (line 1216) (character 942))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (kind "attribute def") (name "BiotNumberValue") (declared-name "BiotNumberValue") (range (start (line 819) (character 4)) (end (line 819) (character 843))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 819) (character 4)) (end (line 819) (character 843))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (kind "attribute def") (name "BlakeNumberValue") (declared-name "BlakeNumberValue") (range (start (line 649) (character 4)) (end (line 649) (character 924))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 649) (character 4)) (end (line 649) (character 924))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (kind "attribute def") (name "BodensteinNumberValue") (declared-name "BodensteinNumberValue") (range (start (line 316) (character 4)) (end (line 316) (character 909))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 316) (character 4)) (end (line 316) (character 909))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (kind "attribute def") (name "BoltzmannNumberValue") (declared-name "BoltzmannNumberValue") (range (start (line 1046) (character 4)) (end (line 1046) (character 878))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1046) (character 4)) (end (line 1046) (character 878))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (kind "attribute def") (name "BondNumberValue") (declared-name "BondNumberValue") (range (start (line 1250) (character 4)) (end (line 1250) (character 1261))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1250) (character 4)) (end (line 1250) (character 1261))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (kind "attribute def") (name "BrinkmanNumberValue") (declared-name "BrinkmanNumberValue") (range (start (line 925) (character 4)) (end (line 925) (character 879))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 925) (character 4)) (end (line 925) (character 879))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (kind "attribute def") (name "CapillaryNumberValue") (declared-name "CapillaryNumberValue") (range (start (line 1388) (character 4)) (end (line 1388) (character 746))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1388) (character 4)) (end (line 1388) (character 746))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (kind "attribute def") (name "CarnotNumberValue") (declared-name "CarnotNumberValue") (range (start (line 959) (character 4)) (end (line 959) (character 700))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 959) (character 4)) (end (line 959) (character 700))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (kind "attribute def") (name "CauchyNumberValue") (declared-name "CauchyNumberValue") (range (start (line 1490) (character 4)) (end (line 1490) (character 677))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1490) (character 4)) (end (line 1490) (character 677))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (kind "attribute def") (name "CavitationNumberValue") (declared-name "CavitationNumberValue") (range (start (line 1354) (character 4)) (end (line 1354) (character 919))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1354) (character 4)) (end (line 1354) (character 919))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (kind "attribute def") (name "ChandrasekharNumberValue") (declared-name "ChandrasekharNumberValue") (range (start (line 1736) (character 4)) (end (line 1736) (character 913))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1736) (character 4)) (end (line 1736) (character 913))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (kind "attribute def") (name "ClausiusNumberValue") (declared-name "ClausiusNumberValue") (range (start (line 942) (character 4)) (end (line 942) (character 853))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 942) (character 4)) (end (line 942) (character 853))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (kind "attribute def") (name "CompressibilityNumberValue") (declared-name "CompressibilityNumberValue") (range (start (line 1577) (character 4)) (end (line 1577) (character 736))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1577) (character 4)) (end (line 1577) (character 736))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (kind "attribute def") (name "CowlingNumberValue") (declared-name "CowlingNumberValue") (range (start (line 1683) (character 4)) (end (line 1683) (character 1038))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1683) (character 4)) (end (line 1683) (character 1038))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (kind "attribute def") (name "DarcyFrictionFactorValue") (declared-name "DarcyFrictionFactorValue") (range (start (line 386) (character 4)) (end (line 386) (character 885))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 386) (character 4)) (end (line 386) (character 885))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (kind "attribute def") (name "DeanNumberValue") (declared-name "DeanNumberValue") (range (start (line 229) (character 4)) (end (line 229) (character 755))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 229) (character 4)) (end (line 229) (character 755))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (kind "attribute def") (name "DeborahNumberValue") (declared-name "DeborahNumberValue") (range (start (line 1543) (character 4)) (end (line 1543) (character 719))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1543) (character 4)) (end (line 1543) (character 719))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (kind "attribute def") (name "DynamicCapillaryNumberValue") (declared-name "DynamicCapillaryNumberValue") (range (start (line 1405) (character 4)) (end (line 1405) (character 943))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1405) (character 4)) (end (line 1405) (character 943))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (kind "attribute def") (name "EckertNumberValue") (declared-name "EckertNumberValue") (range (start (line 976) (character 4)) (end (line 976) (character 814))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 976) (character 4)) (end (line 976) (character 814))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (kind "attribute def") (name "EkmanNumberValue") (declared-name "EkmanNumberValue") (range (start (line 352) (character 4)) (end (line 352) (character 1000))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 352) (character 4)) (end (line 352) (character 1000))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (kind "attribute def") (name "ElasticityNumberValue") (declared-name "ElasticityNumberValue") (range (start (line 369) (character 4)) (end (line 369) (character 703))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 369) (character 4)) (end (line 369) (character 703))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (kind "attribute def") (name "ElectricFieldParameterValue") (declared-name "ElectricFieldParameterValue") (range (start (line 1821) (character 4)) (end (line 1821) (character 704))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1821) (character 4)) (end (line 1821) (character 704))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (kind "attribute def") (name "EulerNumberValue") (declared-name "EulerNumberValue") (range (start (line 37) (character 4)) (end (line 37) (character 928))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 37) (character 4)) (end (line 37) (character 928))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (kind "attribute def") (name "ExpansionNumberValue") (declared-name "ExpansionNumberValue") (range (start (line 1286) (character 4)) (end (line 1286) (character 834))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1286) (character 4)) (end (line 1286) (character 834))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (kind "attribute def") (name "FanningNumberValue") (declared-name "FanningNumberValue") (range (start (line 405) (character 4)) (end (line 405) (character 885))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 405) (character 4)) (end (line 405) (character 885))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (kind "attribute def") (name "FourierNumberForMassTransferValue") (declared-name "FourierNumberForMassTransferValue") (range (start (line 1080) (character 4)) (end (line 1080) (character 978))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1080) (character 4)) (end (line 1080) (character 978))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (kind "attribute def") (name "FourierNumberValue") (declared-name "FourierNumberValue") (range (start (line 734) (character 4)) (end (line 734) (character 882))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 734) (character 4)) (end (line 734) (character 882))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (kind "attribute def") (name "FroudeNumberForHeatTransferValue") (declared-name "FroudeNumberForHeatTransferValue") (range (start (line 785) (character 4)) (end (line 785) (character 734))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 785) (character 4)) (end (line 785) (character 734))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (kind "attribute def") (name "FroudeNumberValue") (declared-name "FroudeNumberValue") (range (start (line 54) (character 4)) (end (line 54) (character 818))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 54) (character 4)) (end (line 54) (character 818))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (kind "attribute def") (name "GalileiNumberValue") (declared-name "GalileiNumberValue") (range (start (line 700) (character 4)) (end (line 700) (character 917))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 700) (character 4)) (end (line 700) (character 917))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (kind "attribute def") (name "GoertlerNumberValue") (declared-name "GoertlerNumberValue") (range (start (line 422) (character 4)) (end (line 422) (character 872))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 422) (character 4)) (end (line 422) (character 872))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (kind "attribute def") (name "GraetzNumberForMassTransferValue") (declared-name "GraetzNumberForMassTransferValue") (range (start (line 1165) (character 4)) (end (line 1165) (character 866))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1165) (character 4)) (end (line 1165) (character 866))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (kind "attribute def") (name "GraetzNumberValue") (declared-name "GraetzNumberValue") (range (start (line 995) (character 4)) (end (line 995) (character 761))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 995) (character 4)) (end (line 995) (character 761))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (kind "attribute def") (name "GrashofMagneticNumberValue") (declared-name "GrashofMagneticNumberValue") (range (start (line 1889) (character 4)) (end (line 1889) (character 1142))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1889) (character 4)) (end (line 1889) (character 1142))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (kind "attribute def") (name "GrashofNumberForMassTransferValue") (declared-name "GrashofNumberForMassTransferValue") (range (start (line 1114) (character 4)) (end (line 1114) (character 1104))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1114) (character 4)) (end (line 1114) (character 1104))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (kind "attribute def") (name "GrashofNumberValue") (declared-name "GrashofNumberValue") (range (start (line 71) (character 4)) (end (line 71) (character 1203))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 71) (character 4)) (end (line 71) (character 1203))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (kind "attribute def") (name "HagenNumberValue") (declared-name "HagenNumberValue") (range (start (line 441) (character 4)) (end (line 441) (character 918))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 441) (character 4)) (end (line 441) (character 918))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (kind "attribute def") (name "HallNumberValue") (declared-name "HallNumberValue") (range (start (line 1838) (character 4)) (end (line 1838) (character 766))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1838) (character 4)) (end (line 1838) (character 766))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (kind "attribute def") (name "HartmannNumberValue") (declared-name "HartmannNumberValue") (range (start (line 1666) (character 4)) (end (line 1666) (character 834))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1666) (character 4)) (end (line 1666) (character 834))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (kind "attribute def") (name "HeatTransferNumberValue") (declared-name "HeatTransferNumberValue") (range (start (line 1012) (character 4)) (end (line 1012) (character 715))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1012) (character 4)) (end (line 1012) (character 715))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (kind "attribute def") (name "HedströmNumberValue") (declared-name "HedströmNumberValue") (range (start (line 299) (character 4)) (end (line 299) (character 811))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 299) (character 4)) (end (line 299) (character 811))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (kind "attribute def") (name "HookeNumberValue") (declared-name "HookeNumberValue") (range (start (line 1509) (character 4)) (end (line 1509) (character 644))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1509) (character 4)) (end (line 1509) (character 644))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (kind "attribute def") (name "JFactorValue") (declared-name "JFactorValue") (range (start (line 853) (character 4)) (end (line 853) (character 1061))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 853) (character 4)) (end (line 853) (character 1061))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (kind "attribute def") (name "JouleMagneticNumberValue") (declared-name "JouleMagneticNumberValue") (range (start (line 1872) (character 4)) (end (line 1872) (character 854))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1872) (character 4)) (end (line 1872) (character 854))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (kind "attribute def") (name "KnudsenNumberValue") (declared-name "KnudsenNumberValue") (range (start (line 122) (character 4)) (end (line 122) (character 804))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 122) (character 4)) (end (line 122) (character 804))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (kind "attribute def") (name "LagrangeNumberValue") (declared-name "LagrangeNumberValue") (range (start (line 263) (character 4)) (end (line 263) (character 858))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 263) (character 4)) (end (line 263) (character 858))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (kind "attribute def") (name "LandauGinzburgNumberValue") (declared-name "LandauGinzburgNumberValue") (range (start (line 1974) (character 4)) (end (line 1974) (character 790))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1974) (character 4)) (end (line 1974) (character 790))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (kind "attribute def") (name "LaplaceNumberValue") (declared-name "LaplaceNumberValue") (range (start (line 630) (character 4)) (end (line 630) (character 1094))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 630) (character 4)) (end (line 630) (character 1094))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (kind "attribute def") (name "LavalNumberValue") (declared-name "LavalNumberValue") (range (start (line 458) (character 4)) (end (line 458) (character 871))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 458) (character 4)) (end (line 458) (character 871))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (kind "attribute def") (name "LewisNumberValue") (declared-name "LewisNumberValue") (range (start (line 1456) (character 4)) (end (line 1456) (character 860))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1456) (character 4)) (end (line 1456) (character 860))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (kind "attribute def") (name "LiftCoefficientValue") (declared-name "LiftCoefficientValue") (range (start (line 195) (character 4)) (end (line 195) (character 968))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 195) (character 4)) (end (line 195) (character 968))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (kind "attribute def") (name "LockhartMartinelliParameterValue") (declared-name "LockhartMartinelliParameterValue") (range (start (line 1320) (character 4)) (end (line 1320) (character 861))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1320) (character 4)) (end (line 1320) (character 861))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (kind "attribute def") (name "LorentzNumberValue") (declared-name "LorentzNumberValue") (range (start (line 1560) (character 4)) (end (line 1560) (character 802))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1560) (character 4)) (end (line 1560) (character 802))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (kind "attribute def") (name "LundquistNumberValue") (declared-name "LundquistNumberValue") (range (start (line 1855) (character 4)) (end (line 1855) (character 1246))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1855) (character 4)) (end (line 1855) (character 1246))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (kind "attribute def") (name "MachNumberValue") (declared-name "MachNumberValue") (range (start (line 105) (character 4)) (end (line 105) (character 819))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 105) (character 4)) (end (line 105) (character 819))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (kind "attribute def") (name "MagneticNumberValue") (declared-name "MagneticNumberValue") (range (start (line 1804) (character 4)) (end (line 1804) (character 773))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1804) (character 4)) (end (line 1804) (character 773))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (kind "attribute def") (name "MagneticPressureNumberValue") (declared-name "MagneticPressureNumberValue") (range (start (line 1719) (character 4)) (end (line 1719) (character 841))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1719) (character 4)) (end (line 1719) (character 841))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (kind "attribute def") (name "MarangoniNumberValue") (declared-name "MarangoniNumberValue") (range (start (line 1303) (character 4)) (end (line 1303) (character 1148))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1303) (character 4)) (end (line 1303) (character 1148))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (kind "attribute def") (name "MassTransferFactorValue") (declared-name "MassTransferFactorValue") (range (start (line 1182) (character 4)) (end (line 1182) (character 1233))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1182) (character 4)) (end (line 1182) (character 1233))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (kind "attribute def") (name "MortonNumberValue") (declared-name "MortonNumberValue") (range (start (line 1233) (character 4)) (end (line 1233) (character 1139))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1233) (character 4)) (end (line 1233) (character 1139))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (kind "attribute def") (name "NazeNumberValue") (declared-name "NazeNumberValue") (range (start (line 1906) (character 4)) (end (line 1906) (character 772))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1906) (character 4)) (end (line 1906) (character 772))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (kind "attribute def") (name "NusseltElectricNumberValue") (declared-name "NusseltElectricNumberValue") (range (start (line 1628) (character 4)) (end (line 1628) (character 889))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1628) (character 4)) (end (line 1628) (character 889))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (kind "attribute def") (name "NusseltNumberForMassTransferValue") (declared-name "NusseltNumberForMassTransferValue") (range (start (line 1131) (character 4)) (end (line 1131) (character 1033))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1131) (character 4)) (end (line 1131) (character 1033))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (kind "attribute def") (name "NusseltNumberValue") (declared-name "NusseltNumberValue") (range (start (line 802) (character 4)) (end (line 802) (character 1268))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 802) (character 4)) (end (line 802) (character 1268))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (kind "attribute def") (name "OhnesorgeNumberValue") (declared-name "OhnesorgeNumberValue") (range (start (line 1473) (character 4)) (end (line 1473) (character 1009))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1473) (character 4)) (end (line 1473) (character 1009))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (kind "attribute def") (name "PoiseuilleNumberValue") (declared-name "PoiseuilleNumberValue") (range (start (line 475) (character 4)) (end (line 475) (character 928))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 475) (character 4)) (end (line 475) (character 928))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (kind "attribute def") (name "PomerantsevNumberValue") (declared-name "PomerantsevNumberValue") (range (start (line 1029) (character 4)) (end (line 1029) (character 983))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1029) (character 4)) (end (line 1029) (character 983))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (kind "attribute def") (name "PowerNumberValue") (declared-name "PowerNumberValue") (range (start (line 492) (character 4)) (end (line 492) (character 744))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 492) (character 4)) (end (line 492) (character 744))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (kind "attribute def") (name "PrandtlMagneticNumberValue") (declared-name "PrandtlMagneticNumberValue") (range (start (line 1753) (character 4)) (end (line 1753) (character 1038))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1753) (character 4)) (end (line 1753) (character 1038))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (kind "attribute def") (name "PrandtlNumberValue") (declared-name "PrandtlNumberValue") (range (start (line 1422) (character 4)) (end (line 1422) (character 963))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1422) (character 4)) (end (line 1422) (character 963))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (kind "attribute def") (name "PécletNumberForMassTransferValue") (declared-name "PécletNumberForMassTransferValue") (range (start (line 1097) (character 4)) (end (line 1097) (character 1061))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1097) (character 4)) (end (line 1097) (character 1061))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (kind "attribute def") (name "PécletNumberValue") (declared-name "PécletNumberValue") (range (start (line 751) (character 4)) (end (line 751) (character 889))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 751) (character 4)) (end (line 751) (character 889))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (kind "attribute def") (name "RayleighNumberValue") (declared-name "RayleighNumberValue") (range (start (line 768) (character 4)) (end (line 768) (character 1194))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 768) (character 4)) (end (line 768) (character 1194))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (kind "attribute def") (name "ReechNumberValue") (declared-name "ReechNumberValue") (range (start (line 526) (character 4)) (end (line 526) (character 962))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 526) (character 4)) (end (line 526) (character 962))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (kind "attribute def") (name "ReynoldsElectricNumberValue") (declared-name "ReynoldsElectricNumberValue") (range (start (line 1923) (character 4)) (end (line 1923) (character 1117))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1923) (character 4)) (end (line 1923) (character 1117))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (kind "attribute def") (name "ReynoldsMagneticNumberValue") (declared-name "ReynoldsMagneticNumberValue") (range (start (line 1594) (character 4)) (end (line 1594) (character 1063))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1594) (character 4)) (end (line 1594) (character 1063))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (kind "attribute def") (name "ReynoldsNumberValue") (declared-name "ReynoldsNumberValue") (range (start (line 20) (character 4)) (end (line 20) (character 956))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 20) (character 4)) (end (line 20) (character 956))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (kind "attribute def") (name "RichardsonNumberValue") (declared-name "RichardsonNumberValue") (range (start (line 509) (character 4)) (end (line 509) (character 731))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 509) (character 4)) (end (line 509) (character 731))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (kind "attribute def") (name "RobertsNumberValue") (declared-name "RobertsNumberValue") (range (start (line 1770) (character 4)) (end (line 1770) (character 869))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1770) (character 4)) (end (line 1770) (character 869))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (kind "attribute def") (name "RossbyNumberValue") (declared-name "RossbyNumberValue") (range (start (line 333) (character 4)) (end (line 333) (character 972))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 333) (character 4)) (end (line 333) (character 972))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (kind "attribute def") (name "SchmidtNumberValue") (declared-name "SchmidtNumberValue") (range (start (line 1439) (character 4)) (end (line 1439) (character 731))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1439) (character 4)) (end (line 1439) (character 731))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (kind "attribute def") (name "SommerfeldNumberValue") (declared-name "SommerfeldNumberValue") (range (start (line 666) (character 4)) (end (line 666) (character 865))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 666) (character 4)) (end (line 666) (character 865))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (kind "attribute def") (name "StantonNumberForMassTransferValue") (declared-name "StantonNumberForMassTransferValue") (range (start (line 1148) (character 4)) (end (line 1148) (character 898))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1148) (character 4)) (end (line 1148) (character 898))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (kind "attribute def") (name "StantonNumberValue") (declared-name "StantonNumberValue") (range (start (line 836) (character 4)) (end (line 836) (character 1110))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 836) (character 4)) (end (line 836) (character 1110))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (kind "attribute def") (name "StarkNumberValue") (declared-name "StarkNumberValue") (range (start (line 1063) (character 4)) (end (line 1063) (character 1170))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1063) (character 4)) (end (line 1063) (character 1170))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (kind "attribute def") (name "StefanNumberValue") (declared-name "StefanNumberValue") (range (start (line 908) (character 4)) (end (line 908) (character 826))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 908) (character 4)) (end (line 908) (character 826))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (kind "attribute def") (name "StokesNumberForDragValue") (declared-name "StokesNumberForDragValue") (range (start (line 613) (character 4)) (end (line 613) (character 692))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue::_documentation"))) (kind "documentation") (name "") (range (start (line 613) (character 4)) (end (line 613) (character 692))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (kind "attribute def") (name "StokesNumberForGravityValue") (declared-name "StokesNumberForGravityValue") (range (start (line 596) (character 4)) (end (line 596) (character 762))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 596) (character 4)) (end (line 596) (character 762))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (kind "attribute def") (name "StokesNumberForRotameterValue") (declared-name "StokesNumberForRotameterValue") (range (start (line 577) (character 4)) (end (line 577) (character 1093))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 577) (character 4)) (end (line 577) (character 1093))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (kind "attribute def") (name "StokesNumberForVibratingParticlesValue") (declared-name "StokesNumberForVibratingParticlesValue") (range (start (line 560) (character 4)) (end (line 560) (character 817))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 560) (character 4)) (end (line 560) (character 817))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (kind "attribute def") (name "StokesNumberValue") (declared-name "StokesNumberValue") (range (start (line 543) (character 4)) (end (line 543) (character 896))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 543) (character 4)) (end (line 543) (character 896))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (kind "attribute def") (name "StrouhalNumberValue") (declared-name "StrouhalNumberValue") (range (start (line 139) (character 4)) (end (line 139) (character 854))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 139) (character 4)) (end (line 139) (character 854))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (kind "attribute def") (name "StuartElectricalNumberValue") (declared-name "StuartElectricalNumberValue") (range (start (line 1702) (character 4)) (end (line 1702) (character 816))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1702) (character 4)) (end (line 1702) (character 816))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (kind "attribute def") (name "StuartNumberValue") (declared-name "StuartNumberValue") (range (start (line 1787) (character 4)) (end (line 1787) (character 994))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1787) (character 4)) (end (line 1787) (character 994))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (kind "attribute def") (name "TaylorNumberValue") (declared-name "TaylorNumberValue") (range (start (line 683) (character 4)) (end (line 683) (character 1238))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 683) (character 4)) (end (line 683) (character 1238))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (kind "attribute def") (name "ThrustCoefficientValue") (declared-name "ThrustCoefficientValue") (range (start (line 212) (character 4)) (end (line 212) (character 843))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 212) (character 4)) (end (line 212) (character 843))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (kind "attribute def") (name "WeberNumberValue") (declared-name "WeberNumberValue") (range (start (line 88) (character 4)) (end (line 88) (character 1158))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 88) (character 4)) (end (line 88) (character 1158))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (kind "attribute def") (name "WeissenbergNumberValue") (declared-name "WeissenbergNumberValue") (range (start (line 1526) (character 4)) (end (line 1526) (character 835))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1526) (character 4)) (end (line 1526) (character 835))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (kind "attribute def") (name "WomersleyNumberValue") (declared-name "WomersleyNumberValue") (range (start (line 717) (character 4)) (end (line 717) (character 787))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue::_documentation"))) (kind "documentation") (name "") (range (start (line 717) (character 4)) (end (line 717) (character 787))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 120114))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind "attribute def") (name "absorptionNumber") (declared-name "absorptionNumber") (range (start (line 1385) (character 4)) (end (line 1385) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorptionNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (kind "alias") (name "aeroelasticityParameter") (declared-name "aeroelasticityParameter") (range (start (line 1506) (character 4)) (end (line 1506) (character 51))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind "attribute def") (name "alfvénNumber") (declared-name "alfvénNumber") (range (start (line 1659) (character 4)) (end (line 1659) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "AlfvénNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind "attribute def") (name "ampèreNumber") (declared-name "ampèreNumber") (range (start (line 1954) (character 4)) (end (line 1954) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmpèreNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind "attribute def") (name "archimedesNumber") (declared-name "archimedesNumber") (range (start (line 1283) (character 4)) (end (line 1283) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ArchimedesNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind "attribute def") (name "arrheniusNumber") (declared-name "arrheniusNumber") (range (start (line 1971) (character 4)) (end (line 1971) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ArrheniusNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind "attribute def") (name "atwoodNumber") (declared-name "atwoodNumber") (range (start (line 1213) (character 4)) (end (line 1213) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "AtwoodNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind "attribute def") (name "bagnoldNumber") (declared-name "bagnoldNumber") (range (start (line 175) (character 4)) (end (line 175) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BagnoldNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind "attribute def") (name "bagnoldNumberForSolidParticles") (declared-name "bagnoldNumberForSolidParticles") (range (start (line 192) (character 4)) (end (line 192) (character 102))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BagnoldNumberForSolidParticlesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind "attribute def") (name "batchelorNumber") (declared-name "batchelorNumber") (range (start (line 1625) (character 4)) (end (line 1625) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BatchelorNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind "attribute def") (name "bejanNumber") (declared-name "bejanNumber") (range (start (line 260) (character 4)) (end (line 260) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BejanNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind "attribute def") (name "bejanNumberForEntropy") (declared-name "bejanNumberForEntropy") (range (start (line 905) (character 4)) (end (line 905) (character 84))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BejanNumberForEntropyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind "attribute def") (name "bejanNumberForHeatTransfer") (declared-name "bejanNumberForHeatTransfer") (range (start (line 888) (character 4)) (end (line 888) (character 94))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BejanNumberForHeatTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind "attribute def") (name "bejanNumberForMassTransfer") (declared-name "bejanNumberForMassTransfer") (range (start (line 1351) (character 4)) (end (line 1351) (character 94))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BejanNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind "attribute def") (name "binghamNumber") (declared-name "binghamNumber") (range (start (line 294) (character 4)) (end (line 294) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BinghamNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind "attribute def") (name "biotNumber") (declared-name "biotNumber") (range (start (line 833) (character 4)) (end (line 833) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BiotNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind "attribute def") (name "biotNumberForMassTransfer") (declared-name "biotNumberForMassTransfer") (range (start (line 1230) (character 4)) (end (line 1230) (character 92))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BiotNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind "attribute def") (name "blakeNumber") (declared-name "blakeNumber") (range (start (line 663) (character 4)) (end (line 663) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BlakeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind "attribute def") (name "bodensteinNumber") (declared-name "bodensteinNumber") (range (start (line 330) (character 4)) (end (line 330) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BodensteinNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind "attribute def") (name "boltzmannNumber") (declared-name "boltzmannNumber") (range (start (line 1060) (character 4)) (end (line 1060) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BoltzmannNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind "attribute def") (name "bondNumber") (declared-name "bondNumber") (range (start (line 1264) (character 4)) (end (line 1264) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BondNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind "attribute def") (name "brinkmanNumber") (declared-name "brinkmanNumber") (range (start (line 939) (character 4)) (end (line 939) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "BrinkmanNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind "attribute def") (name "capillaryNumber") (declared-name "capillaryNumber") (range (start (line 1402) (character 4)) (end (line 1402) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CapillaryNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind "attribute def") (name "carnotNumber") (declared-name "carnotNumber") (range (start (line 973) (character 4)) (end (line 973) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CarnotNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind "attribute def") (name "cauchyNumber") (declared-name "cauchyNumber") (range (start (line 1504) (character 4)) (end (line 1504) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CauchyNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind "attribute def") (name "cavitationNumber") (declared-name "cavitationNumber") (range (start (line 1368) (character 4)) (end (line 1368) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CavitationNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind "attribute def") (name "chandrasekharNumber") (declared-name "chandrasekharNumber") (range (start (line 1750) (character 4)) (end (line 1750) (character 80))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ChandrasekharNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind "attribute def") (name "clausiusNumber") (declared-name "clausiusNumber") (range (start (line 956) (character 4)) (end (line 956) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ClausiusNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (kind "alias") (name "colburnNumber") (declared-name "colburnNumber") (range (start (line 871) (character 4)) (end (line 871) (character 36))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind "attribute def") (name "compressibilityNumber") (declared-name "compressibilityNumber") (range (start (line 1591) (character 4)) (end (line 1591) (character 84))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind "attribute def") (name "cowlingNumber") (declared-name "cowlingNumber") (range (start (line 1697) (character 4)) (end (line 1697) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "CowlingNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind "attribute def") (name "darcyFrictionFactor") (declared-name "darcyFrictionFactor") (range (start (line 400) (character 4)) (end (line 400) (character 80))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DarcyFrictionFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind "attribute def") (name "deanNumber") (declared-name "deanNumber") (range (start (line 243) (character 4)) (end (line 243) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DeanNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind "attribute def") (name "deborahNumber") (declared-name "deborahNumber") (range (start (line 1557) (character 4)) (end (line 1557) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DeborahNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (kind "alias") (name "dulongNumber") (declared-name "dulongNumber") (range (start (line 992) (character 4)) (end (line 992) (character 40))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind "attribute def") (name "dynamicCapillaryNumber") (declared-name "dynamicCapillaryNumber") (range (start (line 1419) (character 4)) (end (line 1419) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicCapillaryNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind "attribute def") (name "eckertNumber") (declared-name "eckertNumber") (range (start (line 990) (character 4)) (end (line 990) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "EckertNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind "attribute def") (name "ekmanNumber") (declared-name "ekmanNumber") (range (start (line 366) (character 4)) (end (line 366) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "EkmanNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind "attribute def") (name "elasticityNumber") (declared-name "elasticityNumber") (range (start (line 383) (character 4)) (end (line 383) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElasticityNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind "attribute def") (name "electricFieldParameter") (declared-name "electricFieldParameter") (range (start (line 1835) (character 4)) (end (line 1835) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricFieldParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (kind "alias") (name "eulerMagneticNumber") (declared-name "eulerMagneticNumber") (range (start (line 1699) (character 4)) (end (line 1699) (character 48))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind "attribute def") (name "eulerNumber") (declared-name "eulerNumber") (range (start (line 51) (character 4)) (end (line 51) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "EulerNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind "attribute def") (name "expansionNumber") (declared-name "expansionNumber") (range (start (line 1300) (character 4)) (end (line 1300) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExpansionNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (kind "alias") (name "eötvösNumber") (declared-name "eötvösNumber") (range (start (line 1266) (character 4)) (end (line 1266) (character 42))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind "attribute def") (name "fanningNumber") (declared-name "fanningNumber") (range (start (line 419) (character 4)) (end (line 419) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "FanningNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind "attribute def") (name "fourierNumber") (declared-name "fourierNumber") (range (start (line 748) (character 4)) (end (line 748) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "FourierNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind "attribute def") (name "fourierNumberForMassTransfer") (declared-name "fourierNumberForMassTransfer") (range (start (line 1094) (character 4)) (end (line 1094) (character 98))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "FourierNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind "attribute def") (name "froudeNumber") (declared-name "froudeNumber") (range (start (line 68) (character 4)) (end (line 68) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "FroudeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind "attribute def") (name "froudeNumberForHeatTransfer") (declared-name "froudeNumberForHeatTransfer") (range (start (line 799) (character 4)) (end (line 799) (character 96))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "FroudeNumberForHeatTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind "attribute def") (name "galileiNumber") (declared-name "galileiNumber") (range (start (line 714) (character 4)) (end (line 714) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GalileiNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind "attribute def") (name "goertlerNumber") (declared-name "goertlerNumber") (range (start (line 436) (character 4)) (end (line 436) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GoertlerNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (kind "alias") (name "goertlerParameter") (declared-name "goertlerParameter") (range (start (line 438) (character 4)) (end (line 438) (character 47))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind "attribute def") (name "graetzNumber") (declared-name "graetzNumber") (range (start (line 1009) (character 4)) (end (line 1009) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GraetzNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind "attribute def") (name "graetzNumberForMassTransfer") (declared-name "graetzNumberForMassTransfer") (range (start (line 1179) (character 4)) (end (line 1179) (character 96))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GraetzNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind "attribute def") (name "grashofMagneticNumber") (declared-name "grashofMagneticNumber") (range (start (line 1903) (character 4)) (end (line 1903) (character 84))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrashofMagneticNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind "attribute def") (name "grashofNumber") (declared-name "grashofNumber") (range (start (line 85) (character 4)) (end (line 85) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrashofNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind "attribute def") (name "grashofNumberForMassTransfer") (declared-name "grashofNumberForMassTransfer") (range (start (line 1128) (character 4)) (end (line 1128) (character 98))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "GrashofNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind "attribute def") (name "hagenNumber") (declared-name "hagenNumber") (range (start (line 455) (character 4)) (end (line 455) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HagenNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind "attribute def") (name "hallNumber") (declared-name "hallNumber") (range (start (line 1852) (character 4)) (end (line 1852) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HallNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind "attribute def") (name "hartmannNumber") (declared-name "hartmannNumber") (range (start (line 1680) (character 4)) (end (line 1680) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HartmannNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (kind "alias") (name "heatTransferFactor") (declared-name "heatTransferFactor") (range (start (line 869) (character 4)) (end (line 869) (character 41))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind "attribute def") (name "heatTransferNumber") (declared-name "heatTransferNumber") (range (start (line 1026) (character 4)) (end (line 1026) (character 78))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatTransferNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind "attribute def") (name "hedströmNumber") (declared-name "hedströmNumber") (range (start (line 313) (character 4)) (end (line 313) (character 76))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HedströmNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind "attribute def") (name "hookeNumber") (declared-name "hookeNumber") (range (start (line 1523) (character 4)) (end (line 1523) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "HookeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind "attribute def") (name "jFactor") (declared-name "jFactor") (range (start (line 867) (character 4)) (end (line 867) (character 56))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "JFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind "attribute def") (name "jouleMagneticNumber") (declared-name "jouleMagneticNumber") (range (start (line 1886) (character 4)) (end (line 1886) (character 80))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleMagneticNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (kind "alias") (name "kiebelNumber") (declared-name "kiebelNumber") (range (start (line 349) (character 4)) (end (line 349) (character 40))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind "attribute def") (name "knudsenNumber") (declared-name "knudsenNumber") (range (start (line 136) (character 4)) (end (line 136) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "KnudsenNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (kind "alias") (name "kármanNumber") (declared-name "kármanNumber") (range (start (line 1663) (character 4)) (end (line 1663) (character 46))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind "attribute def") (name "lagrangeNumber") (declared-name "lagrangeNumber") (range (start (line 277) (character 4)) (end (line 277) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LagrangeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind "attribute def") (name "landauGinzburgNumber") (declared-name "landauGinzburgNumber") (range (start (line 1988) (character 4)) (end (line 1988) (character 82))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LandauGinzburgNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind "attribute def") (name "laplaceNumber") (declared-name "laplaceNumber") (range (start (line 644) (character 4)) (end (line 644) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LaplaceNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind "attribute def") (name "lavalNumber") (declared-name "lavalNumber") (range (start (line 472) (character 4)) (end (line 472) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LavalNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind "attribute def") (name "lewisNumber") (declared-name "lewisNumber") (range (start (line 1470) (character 4)) (end (line 1470) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LewisNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind "attribute def") (name "liftCoefficient") (declared-name "liftCoefficient") (range (start (line 209) (character 4)) (end (line 209) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LiftCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind "attribute def") (name "lockhartMartinelliParameter") (declared-name "lockhartMartinelliParameter") (range (start (line 1334) (character 4)) (end (line 1334) (character 96))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LockhartMartinelliParameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind "attribute def") (name "lorentzNumber") (declared-name "lorentzNumber") (range (start (line 1574) (character 4)) (end (line 1574) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorentzNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind "attribute def") (name "lundquistNumber") (declared-name "lundquistNumber") (range (start (line 1869) (character 4)) (end (line 1869) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "LundquistNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (kind "alias") (name "machMagneticNumber") (declared-name "machMagneticNumber") (range (start (line 1661) (character 4)) (end (line 1661) (character 49))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind "attribute def") (name "machNumber") (declared-name "machNumber") (range (start (line 119) (character 4)) (end (line 119) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MachNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind "attribute def") (name "magneticNumber") (declared-name "magneticNumber") (range (start (line 1818) (character 4)) (end (line 1818) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind "attribute def") (name "magneticPressureNumber") (declared-name "magneticPressureNumber") (range (start (line 1733) (character 4)) (end (line 1733) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticPressureNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind "attribute def") (name "marangoniNumber") (declared-name "marangoniNumber") (range (start (line 1317) (character 4)) (end (line 1317) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MarangoniNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind "attribute def") (name "massTransferFactor") (declared-name "massTransferFactor") (range (start (line 1196) (character 4)) (end (line 1196) (character 78))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassTransferFactorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (kind "alias") (name "moodyFrictionFactor") (declared-name "moodyFrictionFactor") (range (start (line 402) (character 4)) (end (line 402) (character 54))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind "attribute def") (name "mortonNumber") (declared-name "mortonNumber") (range (start (line 1247) (character 4)) (end (line 1247) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "MortonNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind "attribute def") (name "nazeNumber") (declared-name "nazeNumber") (range (start (line 1920) (character 4)) (end (line 1920) (character 62))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "NazeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind "attribute def") (name "nusseltElectricNumber") (declared-name "nusseltElectricNumber") (range (start (line 1642) (character 4)) (end (line 1642) (character 84))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "NusseltElectricNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind "attribute def") (name "nusseltNumber") (declared-name "nusseltNumber") (range (start (line 816) (character 4)) (end (line 816) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "NusseltNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind "attribute def") (name "nusseltNumberForMassTransfer") (declared-name "nusseltNumberForMassTransfer") (range (start (line 1145) (character 4)) (end (line 1145) (character 98))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "NusseltNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind "attribute def") (name "ohnesorgeNumber") (declared-name "ohnesorgeNumber") (range (start (line 1487) (character 4)) (end (line 1487) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "OhnesorgeNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (kind "alias") (name "plasticityNumber") (declared-name "plasticityNumber") (range (start (line 296) (character 4)) (end (line 296) (character 45))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind "attribute def") (name "poiseuilleNumber") (declared-name "poiseuilleNumber") (range (start (line 489) (character 4)) (end (line 489) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PoiseuilleNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind "attribute def") (name "pomerantsevNumber") (declared-name "pomerantsevNumber") (range (start (line 1043) (character 4)) (end (line 1043) (character 76))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PomerantsevNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (kind "alias") (name "powerCoefficient") (declared-name "powerCoefficient") (range (start (line 593) (character 4)) (end (line 593) (character 44))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind "attribute def") (name "powerNumber") (declared-name "powerNumber") (range (start (line 506) (character 4)) (end (line 506) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind "attribute def") (name "prandtlMagneticNumber") (declared-name "prandtlMagneticNumber") (range (start (line 1767) (character 4)) (end (line 1767) (character 84))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PrandtlMagneticNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind "attribute def") (name "prandtlNumber") (declared-name "prandtlNumber") (range (start (line 1436) (character 4)) (end (line 1436) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PrandtlNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind "attribute def") (name "pécletNumber") (declared-name "pécletNumber") (range (start (line 765) (character 4)) (end (line 765) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PécletNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind "attribute def") (name "pécletNumberForMassTransfer") (declared-name "pécletNumberForMassTransfer") (range (start (line 1111) (character 4)) (end (line 1111) (character 102))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "PécletNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind "attribute def") (name "rayleighNumber") (declared-name "rayleighNumber") (range (start (line 782) (character 4)) (end (line 782) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "RayleighNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind "attribute def") (name "reechNumber") (declared-name "reechNumber") (range (start (line 540) (character 4)) (end (line 540) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReechNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind "attribute def") (name "reynoldsElectricNumber") (declared-name "reynoldsElectricNumber") (range (start (line 1937) (character 4)) (end (line 1937) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReynoldsElectricNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind "attribute def") (name "reynoldsMagneticNumber") (declared-name "reynoldsMagneticNumber") (range (start (line 1608) (character 4)) (end (line 1608) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReynoldsMagneticNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind "attribute def") (name "reynoldsNumber") (declared-name "reynoldsNumber") (range (start (line 34) (character 4)) (end (line 34) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReynoldsNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind "attribute def") (name "richardsonNumber") (declared-name "richardsonNumber") (range (start (line 523) (character 4)) (end (line 523) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "RichardsonNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind "attribute def") (name "robertsNumber") (declared-name "robertsNumber") (range (start (line 1784) (character 4)) (end (line 1784) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "RobertsNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind "attribute def") (name "rossbyNumber") (declared-name "rossbyNumber") (range (start (line 347) (character 4)) (end (line 347) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "RossbyNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind "attribute def") (name "schmidtNumber") (declared-name "schmidtNumber") (range (start (line 1453) (character 4)) (end (line 1453) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "SchmidtNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind "attribute def") (name "sommerfeldNumber") (declared-name "sommerfeldNumber") (range (start (line 680) (character 4)) (end (line 680) (character 74))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "SommerfeldNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind "attribute def") (name "stantonNumber") (declared-name "stantonNumber") (range (start (line 850) (character 4)) (end (line 850) (character 68))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StantonNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind "attribute def") (name "stantonNumberForMassTransfer") (declared-name "stantonNumberForMassTransfer") (range (start (line 1162) (character 4)) (end (line 1162) (character 98))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StantonNumberForMassTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind "attribute def") (name "starkNumber") (declared-name "starkNumber") (range (start (line 1077) (character 4)) (end (line 1077) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StarkNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind "attribute def") (name "stefanNumber") (declared-name "stefanNumber") (range (start (line 922) (character 4)) (end (line 922) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StefanNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind "attribute def") (name "stokesNumber") (declared-name "stokesNumber") (range (start (line 557) (character 4)) (end (line 557) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StokesNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind "attribute def") (name "stokesNumberForDrag") (declared-name "stokesNumberForDrag") (range (start (line 627) (character 4)) (end (line 627) (character 80))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StokesNumberForDragValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind "attribute def") (name "stokesNumberForGravity") (declared-name "stokesNumberForGravity") (range (start (line 610) (character 4)) (end (line 610) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StokesNumberForGravityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind "attribute def") (name "stokesNumberForRotameter") (declared-name "stokesNumberForRotameter") (range (start (line 591) (character 4)) (end (line 591) (character 90))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StokesNumberForRotameterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind "attribute def") (name "stokesNumberForVibratingParticles") (declared-name "stokesNumberForVibratingParticles") (range (start (line 574) (character 4)) (end (line 574) (character 108))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StokesNumberForVibratingParticlesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind "attribute def") (name "strouhalNumber") (declared-name "strouhalNumber") (range (start (line 153) (character 4)) (end (line 153) (character 70))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StrouhalNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind "attribute def") (name "stuartElectricalNumber") (declared-name "stuartElectricalNumber") (range (start (line 1716) (character 4)) (end (line 1716) (character 86))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StuartElectricalNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind "attribute def") (name "stuartNumber") (declared-name "stuartNumber") (range (start (line 1801) (character 4)) (end (line 1801) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "StuartNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (kind "alias") (name "suratmanNumber") (declared-name "suratmanNumber") (range (start (line 646) (character 4)) (end (line 646) (character 43))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind "attribute def") (name "taylorNumber") (declared-name "taylorNumber") (range (start (line 697) (character 4)) (end (line 697) (character 66))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "TaylorNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (kind "alias") (name "thomsonNumber") (declared-name "thomsonNumber") (range (start (line 155) (character 4)) (end (line 155) (character 43))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind "attribute def") (name "thrustCoefficient") (declared-name "thrustCoefficient") (range (start (line 226) (character 4)) (end (line 226) (character 76))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThrustCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind "attribute def") (name "weberNumber") (declared-name "weberNumber") (range (start (line 102) (character 4)) (end (line 102) (character 64))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "WeberNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind "attribute def") (name "weissenbergNumber") (declared-name "weissenbergNumber") (range (start (line 1540) (character 4)) (end (line 1540) (character 76))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "WeissenbergNumberValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind "attribute def") (name "womersleyNumber") (declared-name "womersleyNumber") (range (start (line 731) (character 4)) (end (line 731) (character 72))) (parent (node (document "d0") (qualified-name "ISQCharacteristicNumbers"))) (authored (membership (kind Owning)) (relationships (typing (reference "WomersleyNumberValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorptionNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "AlfvénNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "AmpèreNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ArchimedesNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ArrheniusNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "AtwoodNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BagnoldNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind featureTyping) (ordinal 0)) (authored-target "BagnoldNumberForSolidParticlesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BatchelorNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BejanNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind featureTyping) (ordinal 0)) (authored-target "BejanNumberForEntropyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "BejanNumberForHeatTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "BejanNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BinghamNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BiotNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "BiotNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BlakeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BodensteinNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BoltzmannNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BondNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "BrinkmanNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CapillaryNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CarnotNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CauchyNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CavitationNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ChandrasekharNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ClausiusNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "CowlingNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "DarcyFrictionFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "DeanNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "DeborahNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicCapillaryNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "EckertNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "EkmanNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ElasticityNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricFieldParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "EulerNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ExpansionNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "FanningNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "FourierNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "FourierNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "FroudeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "FroudeNumberForHeatTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "GalileiNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "GoertlerNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "GraetzNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "GraetzNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "GrashofMagneticNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "GrashofNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "GrashofNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HagenNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HallNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HartmannNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatTransferNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HedströmNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "HookeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "JFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleMagneticNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "KnudsenNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LagrangeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LandauGinzburgNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LaplaceNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LavalNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LewisNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "LiftCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LockhartMartinelliParameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LorentzNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "LundquistNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "MachNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticPressureNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "MarangoniNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "MassTransferFactorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "MortonNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "NazeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "NusseltElectricNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "NusseltNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "NusseltNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "OhnesorgeNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PoiseuilleNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PomerantsevNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PrandtlMagneticNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PrandtlNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "PécletNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "PécletNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RayleighNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ReechNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ReynoldsElectricNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ReynoldsMagneticNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "ReynoldsNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RichardsonNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RobertsNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "RossbyNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "SchmidtNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "SommerfeldNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StantonNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "StantonNumberForMassTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StarkNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StefanNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StokesNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind featureTyping) (ordinal 0)) (authored-target "StokesNumberForDragValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind featureTyping) (ordinal 0)) (authored-target "StokesNumberForGravityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind featureTyping) (ordinal 0)) (authored-target "StokesNumberForRotameterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind featureTyping) (ordinal 0)) (authored-target "StokesNumberForVibratingParticlesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StrouhalNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StuartElectricalNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "StuartNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "TaylorNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "ThrustCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "WeberNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "WeissenbergNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "WomersleyNumberValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (target (node (document "d0") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
