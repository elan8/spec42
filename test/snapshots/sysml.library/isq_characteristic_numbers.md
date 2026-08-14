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
  (document "memory://snapshot/isq_characteristic_numbers.md"
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
        (range (start 15 19) (end 15 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 19) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 19) (end 17 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 41) (end 20 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 37 38) (end 37 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 54 39) (end 54 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 71 40) (end 71 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 88 38) (end 88 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 105 37) (end 105 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 122 40) (end 122 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 139 41) (end 139 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 161 40) (end 161 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 178 57) (end 178 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 195 42) (end 195 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 212 44) (end 212 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 229 37) (end 229 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 246 38) (end 246 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 263 41) (end 263 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 280 40) (end 280 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 299 44) (end 299 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 316 43) (end 316 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 333 39) (end 333 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 352 38) (end 352 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 369 43) (end 369 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 386 46) (end 386 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 405 40) (end 405 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 422 41) (end 422 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 441 38) (end 441 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 458 38) (end 458 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 475 43) (end 475 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 492 38) (end 492 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 509 43) (end 509 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 526 38) (end 526 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 543 39) (end 543 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 560 60) (end 560 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 577 51) (end 577 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 596 49) (end 596 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 613 46) (end 613 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 630 40) (end 630 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 649 38) (end 649 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 666 43) (end 666 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 683 39) (end 683 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 700 40) (end 700 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 717 42) (end 717 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 734 40) (end 734 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 751 42) (end 751 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 768 41) (end 768 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 785 54) (end 785 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 802 40) (end 802 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 819 37) (end 819 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 836 40) (end 836 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 853 34) (end 853 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 874 53) (end 874 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 891 48) (end 891 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 908 39) (end 908 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 925 41) (end 925 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 942 41) (end 942 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 959 39) (end 959 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 976 39) (end 976 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 995 39) (end 995 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1012 45) (end 1012 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1029 44) (end 1029 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1046 42) (end 1046 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1063 38) (end 1063 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1080 55) (end 1080 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1097 57) (end 1097 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1114 55) (end 1114 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1131 55) (end 1131 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1148 55) (end 1148 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1165 54) (end 1165 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1182 45) (end 1182 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1199 39) (end 1199 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1216 52) (end 1216 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1233 39) (end 1233 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1250 37) (end 1250 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1269 43) (end 1269 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1286 42) (end 1286 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1303 42) (end 1303 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1320 54) (end 1320 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1337 53) (end 1337 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1354 43) (end 1354 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1371 43) (end 1371 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1388 42) (end 1388 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1405 49) (end 1405 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1422 40) (end 1422 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1439 40) (end 1439 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1456 38) (end 1456 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1473 42) (end 1473 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1490 39) (end 1490 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1509 38) (end 1509 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1526 44) (end 1526 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1543 40) (end 1543 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1560 40) (end 1560 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1577 48) (end 1577 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1594 49) (end 1594 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1611 42) (end 1611 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1628 48) (end 1628 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1645 42) (end 1645 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1666 41) (end 1666 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1683 40) (end 1683 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1702 49) (end 1702 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1719 49) (end 1719 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1736 46) (end 1736 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1753 48) (end 1753 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1770 40) (end 1770 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1787 39) (end 1787 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1804 41) (end 1804 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1821 49) (end 1821 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1838 37) (end 1838 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1855 42) (end 1855 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1872 46) (end 1872 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1889 48) (end 1889 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1906 37) (end 1906 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1923 49) (end 1923 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1940 42) (end 1940 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1957 42) (end 1957 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 1974 47) (end 1974 64))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e45141cc4843d8e1e190337e22c0e41d1607dd6087a79d2e28e0123a1c209cb1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n     * International System of Quantities and Units\n     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-11:2019 \"Characteristic numbers\"\n     * see also https://www.iso.org/standard/64982.html\n     * \n     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\n     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\n     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \n     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \n     * or TensorMeasurementReference.\n     "))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 3)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQBase") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.18 absorption number\n         * symbol(s): `Ab`\n         * application domain: generic\n         * name: AbsorptionNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between mass flow rate and surface area for gas absorption at wetted walls, expressed by `Ab = k*sqrt((l*d)/(D*q_V))`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), and `k^'` is mass flux density through the surface, `k^' = q_m/A`, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is length (ISO 80000-3) of wetted surface, `d` is thickness (ISO 80000-3) of liquid film, `D` is diffusion coefficient (ISO 80000-9), and `q_V` is volume flow rate (ISO 80000-4) per wetted perimeter\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.4 Alfvén number, Mach magnetic number, Kárman number\n         * symbol(s): `Al`\n         * application domain: generic\n         * name: AlfvénNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between speed of a plasma and the Alfvén wave speed, expressed by `Al = v/(B/sqrt(ρ*μ))`, where `v` is speed (ISO 80000-3), `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)\n         * remarks: Often, the inverse of this number is wrongly used. The name \"Alfvén Mach number\" is used in investigations on the solar wind. The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed, where `B` is magnetic flux density (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.21 Ampère number\n         * symbol(s): `Am`\n         * application domain: generic\n         * name: AmpèreNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between electric surface current and magnetic field strength in an electrically conducting liquid, expressed by `Am = I_A/(l*H)`, where `I_A` is electric surface current, `l` is characteristic length (ISO 80000-3), and `H` is magnetic field strength (IEC 80000-6)\n         * remarks: This number is also called magnetic field number. The electric surface current is given by `I_A = ρ_A*l*µ*E`, where `ρ_A` is surface density of electric charge (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `µ` is mobility (ISO 80000-10) of charge carriers, and `E` is electric field strength (IEC 80000-6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.12 Archimedes number\n         * symbol(s): `Ar`\n         * application domain: generic\n         * name: ArchimedesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of buoyancy forces and viscous forces in fluids motion due to density differences for a body in a fluid, expressed by `Ar = (g*l^3)/v^2*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3) of the body, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ρ` is mass density (ISO 80000-4) of the fluid\n         * remarks: In this definition, the body can be replaced by an immiscible fluid. See also Stokes number <rotameter> (item 11-4.34).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-9.1 Arrhenius number\n         * symbol(s): `α`\n         * application domain: generic\n         * name: ArrheniusNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of chemical activation energy and thermal energy; in a chemical reaction it is the exponential factor of the reaction rate constant, `k`, expressed by `k ~ exp(α)`, with `α = E_0/(R*T)`, where `E_0` is activation energy (ISO 80000-5), `R` is molar gas constant (ISO 80000-9), and `T` is thermodynamic temperature (ISO 80000-5)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.8 Atwood number\n         * symbol(s): `At`\n         * application domain: generic\n         * name: AtwoodNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: scaled density difference of heavier and lighter fluids, expressed by `At = (ρ_1 - ρ_2)/(ρ_1 + ρ_2)`, where `ρ_1` is density of heavier fluid, and `ρ_2` is density of lighter fluid\n         * remarks: The Atwood number is used in the study of hydrodynamic instabilities in density stratified flows.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.11 Bagnold number\n         * symbol(s): `Ba_2`\n         * application domain: solid particles\n         * name: BagnoldNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of drag force and viscous force in a fluid transferring solid particles, expressed by `Ba_2 = (ρ_s*d^2*dot(γ))/η*sqrt(1/(f_s^(1/2) - 1))`, where `ρ_s` is mass density (ISO 80000-4) of particles, `d` is diameter (ISO 80000-3) of particles, `dot(γ) = v/d` is shear rate time-derivative of shear strain (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4) of fluid, and `f_s` is volumic fraction of solid particles\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.10 Bagnold number\n         * symbol(s): `Bg`\n         * application domain: generic\n         * name: BagnoldNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of drag force and gravitational force for a body moving in a fluid, expressed by `Bg = (c_D*ρ*v^2)/(l*g*ρ_b)`, where `c_D` is drag coefficient (item 11-4.9) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), and `ρ_b` is mass density (ISO 80000-4) of the body\n         * remarks: The characteristic length, `l`, is the body’s volume divided by its cross-sectional area.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.2 Batchelor number\n         * symbol(s): `Bt`\n         * application domain: generic\n         * name: BatchelorNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertia and magneto-dynamic diffusion in an electrically conducting liquid, expressed by `Bt = (v*l*σ*μ)/(ε_r*μ_r)`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ε_r` is relative permittivity (IEC 80000-6), and `μ_r` is relative permeability (IEC 80000-6)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.10 Bejan number\n         * symbol(s): `Be_S`\n         * application domain: entropy\n         * name: BejanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: efficiency of heat transfer by a fluid, expressed by `Be_S = (S(ΔT))/(S(ΔT)+S(Δp))`, where `S(ΔT)` is entropy generation contributed by heat transfer, and `S(Δp)` is entropy generation contributed by fluid friction\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.9 Bejan number\n         * symbol(s): `Be_1`\n         * application domain: heat transfer\n         * name: BejanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of mechanical work and frictional and thermal diffusion energy losses for a forced flow, expressed by `Be_1 = (Δp*l^2)/(η*a)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe, `l` is length (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.16 Bejan number\n         * symbol(s): `Be^\"*\"`, `Be_2`\n         * application domain: mass transfer\n         * name: BejanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of mechanical work and frictional and diffusion energy loss in viscous flow of fluids in pipes, expressed by `Be^*\" = (Δp*l^2)/(η*D)`, where `Δp` is drop of pressure (ISO 80000-4) along a pipe or channel, `l` is length (ISO 80000-3) of channel, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9), mass diffusivity\"\n         * remarks: A similar quantity exists for heat transfer (item 11-5.9).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.15 Bejan number\n         * symbol(s): `Be`\n         * application domain: generic\n         * name: BejanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Be = (Δp*ρ*l^2)/(η*ν)`, where `p` is drop of pressure (ISO 80000-4) along the pipe, `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), `ν` is kinematic viscosity (ISO 80000-4), and `ρ` is mass density (ISO 80000-4)\n         * remarks: A similar number exists for heat transfer (item 11-5.9). The kinematic viscosity is also called momentum diffusivity.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.17 Bingham number, plasticity number\n         * symbol(s): `Bm`, `Bn`\n         * application domain: generic\n         * name: BinghamNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of yield stress and viscous stress in a viscous material for flow of viscoplastic material in channels, expressed by `Bm = (τ*d)/(η*v)`, where `τ` is shear stress (ISO 80000-4), `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.9 Biot number\n         * symbol(s): `Bi^\"*\"`\n         * application domain: mass transfer\n         * name: BiotNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between mass transfer rate at the interface and mass transfer rate in the interior of a body, expressed by `Bi^*\" = (k*l)/D_\"int\"`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3) of layer, and `D_\"int\"` is diffusion coefficient (ISO 80000-9) at the interface\"\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.6 Biot number\n         * symbol(s): `Bi`\n         * application domain: heat transfer\n         * name: BiotNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: special case of the Nusselt number for heat transfer (item 11-5.5) in case of conductive heat transfer in a solid body, expressed by `Bi = (K*l)/λ`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5) of the body\n         * remarks: The characteristic length is commonly defined as the volume of the body divided by its surface area.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.38 Blake number\n         * symbol(s): `Bl`\n         * application domain: generic\n         * name: BlakeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertial forces and viscous forces in a porous material, expressed by `Bl = (v*ρ*l)/(η*(1-ε))`, where `v` is speed (ISO 80000-3) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3) defined as the volume of a particle divided by its surface area, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `ε` is porosity of the material (=void fraction)\n         * remarks: The Blake number can be interpreted as a Reynolds number for flow in porous material.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.19 Bodenstein number\n         * symbol(s): `Bd`\n         * application domain: generic\n         * name: BodensteinNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: mathematical expression of the transfer of matter by convection in reactors with respect to diffusion, `Bd = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the reactor, and `D` is diffusion coefficient (ISO 80000-9)\n         * remarks: The Bodenstein number is also given by `Bd = Pe^\"*\" = Re*Sc`, where `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2), `Re` is the Reynolds number (item 11-4.1), and `Sc = η/(ρ*D) = ν/D` is the Schmidt number (item 11-7.2).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.19 Boltzmann number\n         * symbol(s): `Bz`, `Bol`, `Bo`\n         * application domain: generic\n         * name: BoltzmannNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between convective heat and radiant heat for a fluid in a channel, expressed by `Bz = (ρ*v*c_p)/(ε*σ*T^3)`, where `ρ` is mass density (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3) of the fluid, `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ε` is emissivity (ISO 80000-7), `σ` is the Stefan-Boltzmann constant (ISO 80000-7), and `T` is thermodynamic temperature (ISO 80000-5)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.11 Bond number, Eötvös number\n         * symbol(s): `Bo`, `Eo`\n         * application domain: generic\n         * name: BondNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of inertial force and capillary force for gas bubbles or liquid drops in a fluid, expressed by `Bo = a/γ * ρ*l^2*(ρ_b/ρ - 1)`, where `a` is the acceleration of the body (ISO 80000-3), mostly acceleration of free fall, `g` (ISO 80000-3), `γ` is surface tension (ISO 80000-4) of the interface, `ρ` is density (ISO 80000-4) of the medium, `l` is characteristic length (ISO 80000-3) (radius of a drop or radius of a capillary tube), and `ρ_b` is mass density (ISO 80000-4) of the drop or bubble\n         * remarks: In the case of gravity `a = g` acceleration of free fall (ISO 80000-3), the name Eötvös number is mostly used. The Bond number is also given by `Bo = (We)/(Fr)`, where `We` is the Weber number (item 11-4.5), and `Fr` is the Froude number (item11-4.3). The Bond number is also used for capillary action driven by buoyancy.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.12 Brinkman number\n         * symbol(s): `Br`, `N_(Br)`\n         * application domain: generic\n         * name: BrinkmanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat produced by viscosity and heat conducted from a wall adjacent to a fluid moving relative to it, expressed by `Br = (η*v^2)/(λ*ΔT)`, where `η` is dynamic viscosity (ISO 80000-4), `v` is characteristic speed (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_W - T_0` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_0` is bulk fluid temperature, and `T_W` is wall temperature\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.19 capillary number\n         * symbol(s): `Ca`\n         * application domain: generic\n         * name: CapillaryNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of gravitational forces and capillary forces for fluids in narrow pipes, expressed by `Ca = (d^2*ρ*g)/γ`, where `d` is diameter (ISO 80000-3) of the pipe, `ρ` is mass density (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `γ` is surface tension (ISO 80000-4) of the fluid\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.14 Carnot number\n         * symbol(s): `Ca`\n         * application domain: generic\n         * name: CarnotNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: theoretical maximum efficiency (ISO 80000-5) of a Carnot cycle operating between temperature reservoirs `Ca = (T_2 - T_1)/T_2`, where `T` is thermodynamic temperature (ISO 80000-5), and `T_2`, `T_1` are the thermodynamic temperatures of a heat source and a heat sink, respectively\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.5 Cauchy number, aeroelasticity parameter\n         * symbol(s): `Cy`\n         * application domain: generic\n         * name: CauchyNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertia forces and compression forces in compressible fluids, expressed by `Cy = `, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `K` is modulus of compression, bulk modulus (ISO 80000-4)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.17 cavitation number\n         * symbol(s): `Ca`, `Cn`\n         * application domain: generic\n         * name: CavitationNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of the excess of local static head over vapour pressure head and velocity head for fast flow in liquids, expressed by `Ca = (p-p_v)/(1/2*ρ*v^2)`, where `p` is local static pressure (ISO 80000-4), `p_v` is vapour pressure (ISO 80000-4) of the fluid, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the flow\n         * remarks: The cavitation number represents the ratio of the excess of local static head over vapour pressure head to velocity head.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.9 Chandrasekhar number\n         * symbol(s): `Q`, `Ch`\n         * application domain: generic\n         * name: ChandrasekharNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of Lorentz force and viscous force in magnetic convection in a fluid, expressed by `Q = ((B*l)^2*σ)/(ρ*ν)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), a length scale of the system, `σ` is electrical conductivity (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: The Chandrasekhar number is also given by `Q = Ha^2` where `Ha` is the Hartmann number (item 11-8.5).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.13 Clausius number\n         * symbol(s): `Cl`\n         * application domain: generic\n         * name: ClausiusNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between energy transfer associated with fluid momentum and energy transfer by thermal conduction in forced heating, expressed by `Cl = (v^3*l*ρ)/(λ*ΔT)`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) of the path of energy transfer, `ρ` is mass density (ISO 80000-4), `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) along length `l`\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.10 compressibility number\n         * symbol(s): `Z`\n         * application domain: generic\n         * name: CompressibilityNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of isothermal compressibility (ISO 80000-5) of a gas and that of an ideal gas, expressed by `Z = p/(ρ*R_s*T)`, where `p` is pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), `R_s` is specific gas constant (ISO 80000-5), and `T` is thermodynamic temperature (ISO 80000-5)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.6 Cowling number, Euler magnetic number\n         * symbol(s): `Co`\n         * application domain: magnetism\n         * name: CowlingNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of magnetic and kinematic energy density in a plasma, expressed by `Co = B^2/(μ*ρ*v^2)`, where `B` is magnetic flux density (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: The Cowling number also represents the ratio of magnetic to dynamic pressure. This quantity is equal to the square of the inverse of the Alfvén number. This quantity is often called the second Cowling number, `Co_2`. The first Cowling number is then defined as `Co_1 = Co*Rm`, where `Rm` is the Reynolds magnetic number (item 11-8.1).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.23 Darcy friction factor, Moody friction factor\n         * symbol(s): `f_D`\n         * application domain: generic\n         * name: DarcyFrictionFactor (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: representation of pressure loss in a pipe due to friction within a laminar or turbulent flow of a fluid in a pipe, expressed by `f_D = (2*Δp)/(ρ*v^2)*d/l`, where `Δp` is drop of pressure (ISO 80000-4) due to friction, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is (average) speed (ISO 80000-3) of the fluid in the pipe, `d` is diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3) of the pipe\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.14 Dean number\n         * symbol(s): `Dn`\n         * application domain: generic\n         * name: DeanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between centrifugal force and inertial force, for flows of fluids in curved pipes, expressed by `Dn = (2*v*r)/ν*sqrt(r/R)`, where `v` is (axial) speed (ISO 80000-3), `r` is radius (ISO 80000-3) of the pipe, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `R` is radius of curvature (ISO 80000-3) of the path of the pipe\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.8 Deborah number\n         * symbol(s): `De`\n         * application domain: generic\n         * name: DeborahNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of relaxation time of viscoelastic fluids and observation duration in rheology of viscoelastic fluids, expressed by `De = t_c/t_p`, where `t_c` is stress relaxation time, and `t_p` is observation duration (ISO 80000-3)\n         * remarks: The stress relaxation time is sometimes called the Maxwell relaxation time.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.20 dynamic capillary number\n         * symbol(s): `Ca^\"*\"`, `Cn`\n         * application domain: generic\n         * name: DynamicCapillaryNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of viscous force and capillary force acting across an interface between a liquid and a gas, or between two immiscible liquids for a flow of fluid influenced by interfacial tension, expressed by `Ca^*\" = (η*v)/γ`, where `η` is dynamic viscosity (ISO 80000-4) of the fluid, `v` is characteristic speed (ISO 80000-3), and `γ` is surface or interfacial tension (ISO 80000-4)\"\n         * remarks: The dynamic capillary number is also given by the quotient of the Weber number and the Reynolds number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.15 Eckert number, Dulong number\n         * symbol(s): `Ec`\n         * application domain: generic\n         * name: EckertNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between the kinetic energy of a flow and its enthalpy change in fluid dynamics exhibiting dissipation, expressed by `Ec = v^2/(c_p*ΔT)`, where `v` is characteristic speed (ISO 80000-3), `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the flow, and `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) due to dissipation (by friction)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.21 Ekman number\n         * symbol(s): `Ek`\n         * application domain: generic\n         * name: EkmanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of viscous forces and Coriolis forces in the context of transfer of matter for the flow of a rotating fluid, expressed by `Ek = ν/(2*l^2*ω_E*sin(φ))`, where `ν` is kinematic viscosity (ISO 80000-4), `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular frequency (ISO 80000-3) of the Earth’s rotation, and `φ` is angle of latitude\n         * remarks: In plasma physics, the square root of this number is used. The Ekman number is also given by `Ek = (Ro)/(Re)`, where `Ro` is the Rossby number (item 11-4.20), and `Re` is the Reynolds number (item 11-4.1).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.22 elasticity number\n         * symbol(s): `El`\n         * application domain: generic\n         * name: ElasticityNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between relaxation time and diffusion time in viscoelastic flows, expressed by `El = (t_r*ν)/r^2`, where `t_r` is relaxation time (ISO 80000-12), `ν` is kinematic viscosity (ISO 80000-4), and `r` is radius (ISO 80000-3) of pipe\n         * remarks: See also Deborah number (item 11-7.8).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.14 electric field parameter\n         * symbol(s): `Ef`\n         * application domain: generic\n         * name: ElectricFieldParameter (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of Coulomb force and Lorentz force on moving electrically charged material or particles, expressed by `Ef = E/(v*B)`, where `E` is electric field strength (IEC 80000-6), `v` is speed (ISO 80000-3), and `B` is magnetic flux density (IEC 80000-6)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.2 Euler number\n         * symbol(s): `Eu`\n         * application domain: generic\n         * name: EulerNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relationship between pressure drop in a flow and the kinetic energy per volume for flow of fluids in a pipe, expressed by `Eu = (Δp)/(ρ*v^2)`, where `Δp` is drop of pressure (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: The Euler number is used to characterize losses in the flow. A modification of the Euler number is considering the dimensions of the containment (pipe): `Eu^\"'\" = d/l*Eu`, where `d` is inner diameter (ISO 80000-3) of the pipe, and `l` is length (ISO 80000-3).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.13 expansion number\n         * symbol(s): `Ex`\n         * application domain: generic\n         * name: ExpansionNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of buoyancy force and inertial force in moving fluids due to density differences for gas bubbles rising in a liquid, expressed by `Ex = (g*d)/v^2*(1-ρ_b/ρ)`, where `g` is acceleration of free fall (ISO 80000-3), `d` is diameter (ISO 80000-3) of bubbles, `v` is speed (ISO 80000-3) of bubbles, `ρ_b` is mass density (ISO 80000-4) of bubbles, and `ρ` is mass density (ISO 80000-4) of the liquid\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.24 Fanning number\n         * symbol(s): `f_n`, `f`\n         * application domain: generic\n         * name: FanningNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between shear stress and dynamic pressure in the flow of a fluid in a containment, expressed by `f_n = (2*τ)/(ρ*v^2)`, where `τ` is shear stress (ISO 80000-4) at the wall, `ρ` is mass density (ISO 80000-4) of the fluid, and `v` is speed (ISO 80000-3) of the fluid in the pipe\n         * remarks: The Fanning number describes the flow of fluids in a pipe with friction at the walls represented by its shear stress. Symbol `f` may be used where no conflicts are possible.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.1 Fourier number\n         * symbol(s): `Fo^\"*\"`\n         * application domain: mass transfer\n         * name: FourierNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between diffusive mass transfer within a given duration and mass storage rate in transient mass transfer, expressed by `Fo^\"*\" = (D*t)/l^2`, where `D` is diffusion coefficient (ISO 80000-9), `t` is duration (ISO 80000-3) of observation, and `l` is length (ISO 80000-3) of transfer\"\n         * remarks: The Fourier number for mass transfer is also given by `Fo^*\" = (Fo)/(Le)`, where `Fo` is the Fourier number for heat transfer (item 11-5.1), and `Le` is the Lewis number (item 11-7.3). See also the Fourier number for heat transfer (item 11-5.1).\"\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.1 Fourier number\n         * symbol(s): `Fo`\n         * application domain: heat transfer\n         * name: FourierNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat conduction rate and the rate of thermal energy storage in a body for conductive heat transfer into a body, expressed by `Fo = (a*t)/l^2`, where `a` is thermal diffusivity (ISO 80000-5), `t` is time (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)\n         * remarks: The characteristic length `l` of the body is often defined as the quotient of the body’s volume and its heated surface. Sometimes the reciprocal of this number is wrongly used.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.4 Froude number\n         * symbol(s): `Fr^\"*\"`\n         * application domain: heat transfer\n         * name: FroudeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of gravitational forces and thermodiffusion forces for heat transfer in forced convection of fluids, expressed by `Fr^\"*\" = (g*l^3)/a^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `a` is thermal diffusivity (ISO 80000-5)\"\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.3 Froude number\n         * symbol(s): `Fr`\n         * application domain: generic\n         * name: FroudeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of a body’s inertial forces and its gravitational forces for flow of fluids, expressed by `Fr = v/sqrt(l*g)`, where `v` is speed (ISO 80000-3) of flow, `l` is characteristic length (ISO 80000-3), and `g` is acceleration of free fall (ISO 80000-3)\n         * remarks: The Froude number can be modified by buoyancy. Sometimes the square and sometimes the inverse of the Froude number as defined here is wrongly used.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.41 Galilei number\n         * symbol(s): `Ga`\n         * application domain: generic\n         * name: GalileiNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between gravitational force and viscous force in fluid films flowing over walls, expressed by `Ga = (g*l^3)/ν^2`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4) of the fluid\n         * remarks: The Galilei number is also given by `Ga = Re^2*Ri` or `Ga = {:Re:}^2/{:Fr:}^2`, where `Re` is the Reynolds number (item 11-4.1), `Ri` is the Richardson number (item 11-4.30), and `Fr` is the Froude number (item 11-4.3).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.25 Goertler number, Goertler parameter\n         * symbol(s): `Go`\n         * application domain: generic\n         * name: GoertlerNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: characterization of the stability of laminar boundary layer flows in transfer of matter in a boundary layer on curved surfaces, expressed by `Go = (v*l_b)/ν * sqrt(l_b/r_c)`, where `v` is speed (ISO 80000-3), `l_b` is boundary layer thickness (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `r_c` is radius of curvature (ISO 80000-3)\n         * remarks: The Goertler number represents the ratio of centrifugal effects to viscous effects.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.6 Graetz number\n         * symbol(s): `Gz^\"*\"`\n         * application domain: mass transfer\n         * name: GraetzNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of advective mass transfer rate and radial diffusive mass transfer rate for mass transfer in pipes, expressed by `Gz^\"*\" = (v*d)/D = d/l*Pe^\"*\"`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `d` is hydraulic diameter (ISO 80000-3) of the pipe, `D` is diffusion coefficient (ISO 80000-9), `l` is length (ISO 80000-3) of the pipe, and `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2)\"\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.16 Graetz number\n         * symbol(s): `Gz`\n         * application domain: heat transfer\n         * name: GraetzNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat transferred by convection and heat transferred by conduction in a laminar flow in a pipe, expressed by `Gz = (v*d^2)/(a*l)`, where `v` is speed (ISO 80000-3) of the fluid, `d` is diameter (ISO 80000-3) of the pipe, `a` is thermal diffusivity (ISO 80000-5) of the fluid, and `l` is length (ISO 80000-3) of the pipe\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.18 Grashof magnetic number\n         * symbol(s): `Gr_m`\n         * application domain: generic\n         * name: GrashofMagneticNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: mathematical expression for the heat transfer by free thermo-magnetic convection of a paramagnetic fluid under gravity, `Gr_m = (4*π*σ_e*μ_e*g*α_V*ΔT*l^3)/ν`, where `σ_e` is electrical conductivity (IEC 80000-6), `μ_e` is magnetic permeability (IEC 80000-6), `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5), `ΔT = T_S - T_∞` is difference of thermodynamic temperature `T` (ISO 80000-5), where `T_S` is surface temperature and `T_∞` is bulk temperature, `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: This number is also called magnetic Grashof number. See also Grashof number (item 11-4.4).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.3 Grashof number\n         * symbol(s): `Gr^\"*\"`\n         * application domain: mass transfer\n         * name: GrashofNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between buoyancy forces and viscous forces in natural convection of fluids, expressed by `Gr^\"*\" = (l^3*g*β*Δx)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `β = -1/ρ*((del ρ)/(del x))_(T,p)`, where `ρ` is mass density (ISO 80000-4) of the fluid, and `x` is amount-of-substance fraction (ISO 80000-9), `Δx` is difference of amount-of-substance fraction (ISO 80000-9) along length `l`, and `ν` is kinematic viscosity (ISO 80000-4)\"\n         * remarks: Instead of \"amount-of-substance fraction\" the \"amount-of-substance concentration\" (ISO 80000-9) is used also. Compare with item 11-4.4, the Grashof number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.4 Grashof number\n         * symbol(s): `Gr`\n         * application domain: generic\n         * name: GrashofNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of buoyancy forces due to thermal expansion which results in a change of mass density and viscous forces for free convection due to temperature differences, expressed by `Gr = l^3*g*α_V*(ΔT)/ν^2`, where `l` is characteristic length (ISO 80000-3), `g` is acceleration of free fall (ISO 80000-3), `α_V` is thermal cubic expansion coefficient (ISO 80000-5), `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface of the body and the fluid far away from the body, and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: Heating can occur near hot vertical walls, in pipes, or by a bluff body. The characteristic length can be the vertical height of a hot plate, the diameter of a pipe, or the effective length of a body. See also Rayleigh number (item 11-5.3).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.26 Hagen number\n         * symbol(s): `Hg`, `Ha`\n         * application domain: generic\n         * name: HagenNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: generalization of the Grashof number for forced or free convection in laminar flow, expressed by `Hg = -1/ρ*(dp)/(dx)*l^3/ν^2`, where `ρ` is mass density (ISO 80000-4) of fluid, `(dp)/(dx)` is gradient of pressure (ISO 80000-4), `l` is characteristic length (ISO 80000-3), and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: For free thermal convection with `(dp)/(dx) = ρ*g*α_V*ΔT`, the Hagen number then coincides with the Grashof number (item 11-4.4). See also the Poiseuille number (item 11-4.28).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.15 Hall number\n         * symbol(s): `Hc`, `CH`\n         * application domain: generic\n         * name: HallNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of gyro frequency and collision frequency in a plasma, expressed by `H_c = (ω_c*λ)/(2*π*v)`, where `ω_c` is cyclotron angular frequency (ISO 80000-10), `λ` is mean free path (ISO 80000-9), and `v` is average speed (ISO 80000-3)\n         * remarks: Sometimes the inverse of this number is wrongly used. `2*π` times this quantity is called the Hall parameter.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.5 Hartmann number\n         * symbol(s): `Ha`\n         * application domain: generic\n         * name: HartmannNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between magnetically induced stress and hydrodynamic shear stress in an electrically conducting fluid, expressed by `Ha = B*l*sqrt(σ/η)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `η` is dynamic viscosity (ISO 80000-4)\n         * remarks: The Hartmann number represents also the ratio of magnetic force to viscous force.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.17 heat transfer number\n         * symbol(s): `K_Q`\n         * application domain: generic\n         * name: HeatTransferNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat transferred by a flow and its kinetic energy, expressed by `K_Q = Φ/(v^3*l^2*ρ)`, where `Φ` is heat flow rate (ISO 80000-5), `v` is characteristic speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.18 Hedström number\n         * symbol(s): `He`, `Hd`\n         * application domain: generic\n         * name: HedströmNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of yield stress and viscous stress of a viscous material at flow limit for visco-plastic material in a channel, expressed by `He = (τ_0*d^2*ρ)/η^2`, where `τ_0` is shear stress (ISO 80000-4) at flow limit, `d` is characteristic diameter (ISO 80000-3), e.g. effective channel width, `ρ` is mass density (ISO 80000-4), and `η` is dynamic viscosity (ISO 80000-4)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.6 Hooke number\n         * symbol(s): `Ho_2`\n         * application domain: generic\n         * name: HookeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertia forces and linear stress forces in elastic fluids, expressed by `Ho_2 = (ρ*v^2)/E`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `E` is modulus of elasticity (ISO 80000-4)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.8 j-factor, heat transfer factor, Colburn number\n         * symbol(s): `j`, `Co`, `Jq`\n         * application domain: heat transfer\n         * name: JFactor (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat transfer and mass transfer in a fluid, expressed by `j = K/(c_p*ρ*v)*((c_p*η)/λ)^(2/3)`, where `K` is coefficient of heat transfer (ISO 80000-5), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `λ` is thermal conductivity (ISO 80000-5)\n         * remarks: The heat transfer factor is also given by `j = St*Pr^(2/3)`, where `St` is the Stanton number for heat transfer (item 11-5.7), and `Pr` is the Prandtl number (item 11-7.1). See also mass transfer factor (item 11-6.7).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.17 Joule magnetic number\n         * symbol(s): `Jo_m`\n         * application domain: generic\n         * name: JouleMagneticNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of Joule heating energy and magnetic field energy in a plasma, expressed by `Jo_m = (2*ρ*μ*c_p*ΔT)/B^2`, where `ρ` is mass density (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `T` is thermodynamic temperature (ISO 80000-5), and `B` is magnetic flux density (IEC 80000-6)\n         * remarks: This number is also called magnetic Joule number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.7 Knudsen number\n         * symbol(s): `Kn`\n         * application domain: generic\n         * name: KnudsenNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of free path length of a particle and a characteristic length, expressed by `Kn = λ/l`, where `λ` is mean free path (ISO 80000-9), and `l` is characteristic length (ISO 80000-3)\n         * remarks: The Knudsen number is a measure to estimate whether the gas in flow behaves like a continuum. The characteristic length, `l`, can be a characteristic size of the gas flow region like a pipe diameter.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.16 Lagrange number\n         * symbol(s): `Lg`\n         * application domain: generic\n         * name: LagrangeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of mechanical work and frictional energy loss in fluid dynamics in a pipe, expressed by `Lg = (l*Δp)/(η*v)`, where `l` is length (ISO 80000-3) of the pipe, `Δp` is drop of pressure (ISO 80000-4) along the pipe, `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: The Lagrange number is also given by `Lg = Re*Eu`, where `Re` is the Reynolds number (item 11-4.1), and `Eu` is the Euler number (item 11-4.2).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-9.2 Landau-Ginzburg number\n         * symbol(s): `κ`\n         * application domain: generic\n         * name: LandauGinzburgNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of penetration depth of a magnetic field into a superconductor and the coherence length of thermodynamic fluctuations within a superconducting phase in a material at zero thermodynamic temperature, expressed by `κ = λ_L/(ξ*sqrt(2))`, where `λ_L` is London penetration depth (ISO 80000-12), and `ξ` is coherence length (ISO 80000-12)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.37 Laplace number, Suratman number\n         * symbol(s): `La`, `Su`\n         * application domain: generic\n         * name: LaplaceNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between capillary forces and viscous forces when characterizing free surface flow, expressed by `La = Su = (γ*ρ*l)/η^2`, where `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4) of the fluid, `l` is characteristic length (ISO 80000-3), and `η` is dynamic viscosity (ISO 80000-4) of the fluid\n         * remarks: The Laplace number is also the ratio of surface tension to momentum transfer, especially dissipation, inside a fluid. The Laplace number is also given by `La = Su = 1/(Oh)^2 = (Re)^2/(We)`, where `Oh` is the Ohnesorge number (item 11-7.4), `Re` is the Reynolds number (item 11-4.1), and `We` is the Weber number (item 11-4.5).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.27 Laval number\n         * symbol(s): `La`\n         * application domain: generic\n         * name: LavalNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of speed and the (critical) sound speed at the throat of a nozzle, expressed by `La = v/sqrt((R_s*T*2*γ)/(γ+1))`, where `v` is speed (ISO 80000-3), \u{a0}`R_s = R/M` is specific gas constant, where `R` is molar gas constant (ISO 80000-9), and `M` is molar mass (ISO 80000-9), `T` is thermodynamic temperature (ISO 80000-5), and `γ` is ratio of the specific heat capacities (ISO 80000-5)\n         * remarks: The Laval number is a specific kind of Mach number (item 11-4.6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.3 Lewis number\n         * symbol(s): `Le`\n         * application domain: generic\n         * name: LewisNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of thermal diffusivity and diffusion coefficient for heat transfer in a fluid, expressed by `Le = a/D`, where `a` is thermal diffusivity (ISO 80000-5), and `D` is diffusion coefficient (ISO 80000-9)\n         * remarks: The Lewis number is also given by `Le = (Sc)/(Pr)`, where `Sc` is the Schmidt number (item 11-7.2), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-5.2. The Lewis number is sometimes defined as reciprocal of this quantity. \n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.12 lift coefficient\n         * symbol(s): `c_l`, `c_A`\n         * application domain: generic\n         * name: LiftCoefficient (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of the lift force available from a wing at a given angle and the inertial force for a wing shaped body moving in a fluid, expressed by `c_l = ( 2*F_l)/(ρ*v^2*S) = F_l/(q*S)`, where `F_l` is lift force (ISO 80000-4) on the wing, `ρ` is mass density (ISO 80000-4) of the fluid, `v` is speed (ISO 80000-3) of the body, `S = A*cos(α)` is effective area (ISO 80000-3) when `α` is the angle of attack and `A` is area of the wing, and `q = 1/2*ρ*v^2` is dynamic pressure\n         * remarks: The lift coefficient is dependant on the shape of the wing.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.15 Lockhart-Martinelli parameter\n         * symbol(s): `Lp`\n         * application domain: generic\n         * name: LockhartMartinelliParameter (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of mass flow rates multiplied by the square root of density in a two-phase flow, expressed by `Lp = dot(m)_l/dot(m)_g*sqrt(ρ_m/ρ_l)`, where `dot(m)_l = q_m` is liquid phase mass flow rate (ISO 80000-4), `dot(m)_g` is gas phase mass flow rate, `ρ_g` is gas density (ISO 80000-4), and `ρ_l` is liquid density\n         * remarks: The Lockhart-Martinelli parameter is used, for example, in boiling or condensing.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.9 Lorentz number\n         * symbol(s): `Lo`\n         * application domain: generic\n         * name: LorentzNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of electrical conductivity and thermal conductivity, expressed by `Lo = (σ*(ΔU)^2)/(λ*ΔT)`, where `σ` is electrical conductivity (IEC 80000-6), `ΔU` is difference of voltage `U` (ISO 80000-6) between two reference points, `λ` is thermal conductivity (ISO 80000-5), and `ΔT` is difference in thermodynamic temperature `T` (ISO 80000-5) between the reference points\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.16 Lundquist number\n         * symbol(s): `Lu`\n         * application domain: generic\n         * name: LundquistNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of Alfvén speed and magneto-dynamic speed in a plasma, expressed by `Lu = B*l*σ*sqrt(μ/ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `μ` is magnetic permeability (IEC 80000-6), and `ρ` is mass density (ISO 80000-4)\n         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4. The quantity `v_m = 1/(l*σ*μ)` is called magneto dynamic speed, where `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6). The Lundquist number is also given by `Lu = (Rm)/(Al)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Al` is the Alfvén number (item 11-8.4). See also Hartmann number (item 11-8.5).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.6 Mach number\n         * symbol(s): `Ma`\n         * application domain: generic\n         * name: MachNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of the speed of flow and the speed of sound, expressed by `Ma = v/c`, where `v` is speed (ISO 80000-3) of the body, and `c` is speed of sound (ISO 80000-8) in the fluid\n         * remarks: The Mach number represents the relationship of inertial forces compared to compression forces. For an ideal gas `c = sqrt(γ p/rho) = sqrt(γ (RT)/M) = sqrt(γ (kT)/m)`, where `γ` is ratio of the specific heat capacity (ISO 80000-5).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.13 magnetic number\n         * symbol(s): `N_(mg)`\n         * application domain: generic\n         * name: MagneticNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of magnetic forces and viscous forces in an electrically conducting fluid, expressed by `N_(mg) = B*sqrt((l*σ)/(η*v))`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `η` is dynamic viscosity (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.8 magnetic pressure number\n         * symbol(s): `N_(mp)`\n         * application domain: generic\n         * name: MagneticPressureNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of gas pressure and magnetic pressure in a gas or plasma, expressed by `N_(mp) = p*(2*μ)/B^2`, where `p` is pressure (ISO 80000-4), `μ` is magnetic permeability (IEC 80000-6), and `B` is magnetic flux density (IEC 80000-6)\n         * remarks: The quantity `p_m = B^2/(2*μ)` is called magnetic pressure, where `B` is magnetic flux density (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.14 Marangoni number\n         * symbol(s): `Mg`, `Mar`\n         * application domain: generic\n         * name: MarangoniNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of heat transferred by Marangoni convection and heat transferred by thermal diffusivity in thermo-capillary convection of liquid films on a free surface, expressed by `Mg = l*ΔT/(η*a)*((dγ)/(dT))`, where `l` is characteristic thickness (ISO 80000-3) of the film, `ΔT` is difference of thermodynamic temperature `T` (ISO 80000-5) between surface and outer surface of the film, `η` is dynamic viscosity (ISO 80000-4) of the liquid, `a` is thermal diffusivity (ISO 80000-5) of the liquid, and `γ` is surface tension (ISO 80000-4) of the film\n         * remarks: The Marangoni convection is free surface flow due to different surface tensions caused by a temperature gradient. This quantity is sometimes called Thompson number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.7 mass transfer factor\n         * symbol(s): `j^\"*\"`\n         * application domain: mass transfer\n         * name: MassTransferFactor (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between mass transfer perpendicular to the surface of a fluid and mass transfer parallel to the surface in an open flow of fluids, expressed by `j^*\" = k/v * (ν/D)^(2/3)`, where `k` is the mass transfer coefficient through the surface, `k = k^'/ρ`, where `ρ` is mass density (ISO 80000-4), `k^'` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `v` is speed (ISO 80000-3), `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)\"\n         * remarks: The mass transfer factor is also given by `j_m = j^*\" = St^\"*\" * (Sc)^(2/3)` where `St^\"*\"` is the Stanton number for mass transfer (item 11-6.5), and `Sc` is the Schmidt number (item 11-7.2). See also heat transfer factor (item 11-5.17).\"\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.10 Morton number\n         * symbol(s): `Mo`\n         * application domain: generic\n         * name: MortonNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of gravitational forces and viscous forces for gas bubbles in a liquid, or liquid drops in a gas, expressed by `Mo = (g*η^4)/(ρ*γ^3)*(ρ_b/ρ - 1)`, where `g` is acceleration of free fall (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4) of the surrounding fluid, `ρ` is mass density (ISO 80000-4) of the surrounding fluid, `γ` is surface tension (ISO 80000-4) of the interface, and `ρ_b` is mass density (ISO 80000-4) of the bubble or drop\n         * remarks: The Morton number is used to determine the shape of bubbles or drops. The Morton number is also given by `Mo = We^3*Fr^-2*Re^-4`, where `We` is the Weber number (item 11-4.5), `Fr` is the Froude number (item 11-4.3), and `Re` is the Reynolds number (item 11-4.1). \n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.19 Naze number\n         * symbol(s): `Na`\n         * application domain: generic\n         * name: NazeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of velocity of Alfvén waves and velocity of sound in a plasma, expressed by `Na = B/(c*sqrt(ρ*μ))`, where `B` is magnetic flux density (IEC 80000-6), `c` is speed of sound (ISO 80000-8), `ρ` is mass density (ISO 80000-4), and `μ` is magnetic permeability (IEC 80000-6)\n         * remarks: The quantity `v_A = B/sqrt(ρ*μ)` is called Alfvén wave speed. See item 11-8.4.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.3 Nusselt electric number\n         * symbol(s): `Ne`\n         * application domain: generic\n         * name: NusseltElectricNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between convective current and diffusive current of ions in electrochemistry, expressed by `Ne = (v*l)/D^*\"`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D^\"*\" = D^\"+\" + D^\"-\"`, where `D^\"+\"`, `D^\"-\"` are diffusion coefficients (ISO 80000-9) of positive or negative ions respectively\"\n         * remarks: This number is also called electric Nusselt number. Sometimes this quantity is called the Reynolds electric number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.4 Nusselt number\n         * symbol(s): `Nu^\"*\"`\n         * application domain: mass transfer\n         * name: NusseltNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between mass flux at an interface and specific flux by pure molecular diffusion in a layer of thickness `l` for mass transfer at the boundary of a fluid, expressed by `Nu^\"*\" = (k’*l)/(ρ*D)`, where `k’` is mass flux density `q_m/A` through the surface, where `q_m` is mass flow rate (ISO 80000-4), and `A` is area (ISO 80000-3), `l` is thickness (ISO 80000-3), `ρ` is mass density (ISO 80000-4) of the fluid, and `D` is diffusion coefficient (ISO 80000-9)\"\n         * remarks: Sometimes this quantity is called the Sherwood number, `Sh`. Compare with item 11-5.5, Nusselt number for heat transfer.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.5 Nusselt number\n         * symbol(s): `Nu`\n         * application domain: heat transfer\n         * name: NusseltNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between the internal thermal resistance of a body and its surface thermal resistance in a body transferring heat from a surface into its interior or vice versa, expressed by `Nu = (K*l)/λ = (K*l)/(a*ρ*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `l` is length (ISO 80000-3) of the body in direction of heat flow, `λ` is thermal conductivity (ISO 80000-5) of the surface, `a` is thermal diffusivity (ISO 80000-5), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5)\n         * remarks: The body under consideration can be a solid body, a fluid, or their combination, and additional heat transfer due to convective motion can occur. In case of merely conductive heat transfer especially in a solid body, the \"Biot number for heat transfer\" (item 11-5.6) is used.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.4 Ohnesorge number\n         * symbol(s): `Oh`\n         * application domain: generic\n         * name: OhnesorgeNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between viscous force and the square root of the product of inertia force and capillary force for atomization of liquids, expressed by `Oh = η/sqrt(γ*ρ*l)`, where `η` is dynamic viscosity (ISO 80000-4), `γ` is surface tension (ISO 80000-4), `ρ` is mass density (ISO 80000-4), and `l` is characteristic length (ISO 80000-3)\n         * remarks: The Ohnesorge number is also given by `Oh = sqrt(We)/(Re)` where `We` is the Weber number (item 11-4.5), and `Re` is the Reynolds number (item 11-4.1). See also Laplace number (item 11-4.37). The characteristic length typically is the drop diameter.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.28 Poiseuille number\n         * symbol(s): `Poi`\n         * application domain: generic\n         * name: PoiseuilleNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of propulsive force by pressure and viscous force for a flow of fluids in a pipe, expressed by `Poi = -(Δp)/l*d^2/(η*v)`, where `Δp` is drop of pressure (ISO 80000-4) along the pipe, `l` is length (ISO 80000-3) of the pipe, `d` is diameter (ISO 80000-3) of the pipe, `η` is dynamic viscosity (ISO 80000-4) of the fluid, and `v` is characteristic speed (ISO 80000-3) of the fluid\n         * remarks: The Poiseuille number is `Poi=32` for laminar flow in a round pipe. See also the Hagen number (item 11-4.26).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.18 Pomerantsev number\n         * symbol(s): `Po`, `Pov`\n         * application domain: heat transfer\n         * name: PomerantsevNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat generated in a body and conducted heat in the body, expressed by `Po = (Q_m*l^2)/(λ*ΔT)`, where `Q_m` is (constant) volumic heat generation rate, `l` is characteristic length (ISO 80000-3), `λ` is thermal conductivity (ISO 80000-5), and `ΔT = T_m - T_0` is difference of thermodynamic temperature (ISO 80000-5) between that of the medium (T_m) and the initial temperature of the body (T_0)\n         * remarks: Similar numbers are known for areic, lineic and point sources of heat, each with decreasing power of length `l` respectively.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.29 power number\n         * symbol(s): `Pn`\n         * application domain: generic\n         * name: PowerNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of power consumption by agitators due to drag and rotational inertial power in fluids, expressed by `Pn = P/(ρ*n^3*d^5)`, where `P` is active power (IEC 80000-6) consumed by a stirrer, `ρ` is mass density (ISO 80000-4) of fluid, `n` is rotational frequency (ISO 80000-3), and `d` is diameter (ISO 80000-3) of stirrer\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.10 Prandtl magnetic number\n         * symbol(s): `Pr_m`\n         * application domain: generic\n         * name: PrandtlMagneticNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of kinematic viscosity and magnetic viscosity in an electrically conducting liquid, expressed by `Pr_m = ν*σ*μ`, where `ν` is kinematic viscosity (ISO 80000-4), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)\n         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity. See item 11-8.11. The Prandtl magnetic number is also given by `Pr_m = (Rm)/(Re)`, where `Rm` is the Reynolds magnetic number (item 11-8.1), and `Re` is the Reynolds number (item 11-4.1). This number is also called magnetic Prandtl number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.1 Prandtl number\n         * symbol(s): `Pr`\n         * application domain: generic\n         * name: PrandtlNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of kinematic viscosity and thermal diffusivity for a fluid, expressed by `Pr = ν/a`, where `ν` is kinematic viscosity (ISO 80000-4), and `a` is thermal diffusivity (ISO 80000-5)\n         * remarks: The Prandtl number also represents the quotient of heat produced by viscosity and heat transferred by thermal diffusivity. The mass transfer analogue of the Prandtl number is the Schmidt number (item 11-7.2). The Prandtl number is also given by `Pr = (Pe)/(Re)`; where `Pe` is the Péclet number (item 11-5.2), and `Re` is the Reynolds number (item 11-4.1). \n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.2 Péclet number\n         * symbol(s): `Pe^\"*\"`, `Bd`, `Bod`\n         * application domain: mass transfer\n         * name: PécletNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between advective mass transfer rate and longitudinal diffusive mass transfer rate for mass transfer in reactors, expressed by `Pe^*\" = (v*l)/D`, where `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `D` is diffusion coefficient (ISO 80000-9)\"\n         * remarks: The Péclet number for mass transfer is also given by `Pe^\"*\" = Pe*Le = Re*Sc`, where `Pe` is the Péclet number for heat transfer, `Le` is the Lewis number (item 11-7.3), `Re` is the Reynolds number (item 11-4.1), and `Sc` is the Schmidt number (item 11-7.2). Compare with item 11-5.2, the Péclet number for heat transfer.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.2 Péclet number\n         * symbol(s): `Pe`\n         * application domain: heat transfer\n         * name: PécletNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between convective heat transfer rate and conductive heat transfer rate, expressed by `Pe = (v*l)/a`, where `v` is speed (ISO 80000-3), `l` is length (ISO 80000-3) in the direction of heat transfer, and `a` is thermal diffusivity (ISO 80000-5)\n         * remarks: The thermal Péclet number is also given by `Pe = Re*Pr`, where `Re` is the Reynolds number (item 11-4.1), and `Pr` is the Prandtl number (item 11-7.1). Compare with item 11-6.2, Péclet number for mass transfer.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.3 Rayleigh number\n         * symbol(s): `Ra`\n         * application domain: generic\n         * name: RayleighNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between buoyancy forces due to thermal expansion and viscous forces in free convection in buoyancy driven flow near a heated surface perpendicular to the gravity force, expressed by `Ra = (l^3*g*α_V*ΔT)/(ν*a)`, where `l` is distance (ISO 80000-3) from the wall, `g` is acceleration of free fall (ISO 80000-3), `α_V` is cubic expansion coefficient (ISO 80000-5) of the fluid, `ΔT` is difference of thermodynamic temperature (ISO 80000-5) between surface of the wall and the fluid far away from the wall, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, and `a` is thermal diffusivity (ISO 80000-5) of the fluid\n         * remarks: The Rayleigh number is also given by `Ra = Gr*Pr`, where `Gr` is the Grashof number (item 11-4.4), and `Pr` is the Prandtl number (item 11-7.1).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.31 Reech number\n         * symbol(s): `Ree`\n         * application domain: generic\n         * name: ReechNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between the speed of an object submerged in water relative to the water, and wave propagation speed, expressed by `Ree = (g*l)/v`, where `g` is acceleration of free fall (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of the object relative to the water\n         * remarks: The Reech number can be used to determine the resistance of a partially submerged object (e.g. a ship) of length `l` (in direction of the motion) moving through water. A similar quantity is defined as the Boussinesq number `Bs = v/sqrt(2*g*l)` .\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.20 Reynolds electric number\n         * symbol(s): `Re_e`\n         * application domain: generic\n         * name: ReynoldsElectricNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of speed of a fluid and average drift speed of the charged particles in an electrically conducting fluid, expressed by `Re_e = (v*ε_e)/(ρ_e*l*μ)`, where `v` is characteristic speed (ISO 80000-3) of the fluid, `ε_e` is electric permittivity (IEC 80000-6), `ρ_e` is electric charge density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), and `μ` is mobility (ISO 80000-10) of charge carriers\n         * remarks: This number is also called electrical Reynolds number. The drift speed of the charged particles in an electric field is given by `v_d = 1/(μ*E)`, where `E` is electric field strength (IEC 80000-6), and `μ` is mobility (ISO 80000-10) of charge carriers.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.1 Reynolds magnetic number\n         * symbol(s): `Rm`\n         * application domain: generic\n         * name: ReynoldsMagneticNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertial force and magneto-dynamic viscous force in an electrically conducting fluid, expressed by `Rm = v*l*μ*σ = (v*l)/ν_m`, where `v` is speed (ISO 80000-3) of the fluid, `l` is characteristic length (ISO 80000-3), `μ` is magnetic permeability (IEC 80000-6), `σ` is electrical conductivity (IEC 80000-6), and `ν_m = 1/(μ*σ)` is magnetic viscosity (magnetic diffusivity)\n         * remarks: This number is also called magnetic Reynolds number. The Reynolds magnetic number is also given by `Rm = Re*Pr_m`, where `Re` is the Reynolds number (item 11-4.1), and `Pr_m` is the Prandtl magnetic number (item 11-8.10).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.1 Reynolds number\n         * symbol(s): `Re`\n         * application domain: generic\n         * name: ReynoldsNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of inertial forces and viscous forces in a fluid flow, expressed by `Re = (ρ*v*l)/η = (v*l)/ν`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), `η` is dynamic viscosity (ISO 80000-4), and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: The value of the Reynolds number gives an estimate on the flow state: laminar flow or turbulent flow. In rotating movement, the speed `v = ω*l`, where `l` is the distance from the rotation axis and `ω` is the angular velocity.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.30 Richardson number\n         * symbol(s): `Ri`\n         * application domain: generic\n         * name: RichardsonNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of potential energy and kinetic energy for a falling body, expressed by `Ri = (g*h)/v^2`, where `g` is acceleration of free fall (ISO 80000-3), `h` is characteristic height (ISO 80000-3), and `v` is characteristic speed (ISO 80000-3)\n         * remarks: In geophysics differences of these quantities are of interest.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.11 Roberts number\n         * symbol(s): `Ro`\n         * application domain: generic\n         * name: RobertsNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of thermal diffusivity and magnetic viscosity in an electrically conducting liquid, expressed by `Ro = a*σ*μ`, where `a` is thermal diffusivity (ISO 80000-5), `σ` is electrical conductivity (IEC 80000-6), and `μ` is magnetic permeability (IEC 80000-6)\n         * remarks: The quantity `ν_m = 1/(μ*σ)` is called magnetic viscosity or magnetic diffusivity; where `μ` is magnetic permeability (IEC 80000-6), and `σ` is electrical conductivity (IEC 80000-6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.20 Rossby number, Kiebel number\n         * symbol(s): `Ro`\n         * application domain: generic\n         * name: RossbyNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of inertial forces and Coriolis forces in the context of transfer of matter in geophysics, expressed by `Ro = v/(2*l*ω_E*sin(φ)`, where `v` is speed (ISO 80000-3) of motion, `l` is characteristic length (ISO 80000-3), the scale of the phenomenon, `ω_E` is angular velocity (ISO 80000-3) of the Earth's rotation, and `φ` is angle (ISO 80000-3) of latitude\n         * remarks: The Rossby number represents the effect of Earth's rotation on flow in pipes, rivers, ocean currents, tornadoes, etc. The quantity `ω_E*sin(φ)` is called Coriolis frequency.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.2 Schmidt number\n         * symbol(s): `Sc`\n         * application domain: generic\n         * name: SchmidtNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of kinematic viscosity and diffusion coefficient for a fluid, expressed by `Sc = ν/D`, where `ν` is kinematic viscosity (ISO 80000-4), and `D` is diffusion coefficient (ISO 80000-9)\n         * remarks: The heat transfer analogue of the Schmidt number is the Prandtl number (item 11-7.1). A deprecated name is Colburn number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.39 Sommerfeld number\n         * symbol(s): `So`, `Sm`\n         * application domain: generic\n         * name: SommerfeldNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between viscous force and load force in a lubrication boundary, expressed by `So = (η*n)/p*(r/c)^2`, where `η` is dynamic viscosity (ISO 80000-4) of the lubricant, `n` is rotational frequency (ISO 80000-3), `p` is mean bearing pressure (ISO 80000-4), `r` is radius (ISO 80000-3) of the shaft, and `c` is radial distance (ISO 80000-3) between rotating shaft and annulus\n         * remarks: Sometimes the inverse of this number is wrongly used.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-6.5 Stanton number\n         * symbol(s): `St^\"*\"`\n         * application domain: mass transfer\n         * name: StantonNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between mass transfer perpendicular to the surface of a fluid flow and mass transfer parallel to the surface in a free surface flow, expressed by `St^\"*\" = k^\"*\"\n         * remarks: The Stanton number for mass transfer is also given by `St^*\" = (Nu^\"*\")/(Pe^\"*\"*)`, where `Nu^\"*\"` is the Nusselt number for mass transfer (item 11-6.4), and `Pe^\"*\"` is the Péclet number for mass transfer (item 11-6.2). Compare with item 11-5.7, the Stanton number for heat transfer.\"\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.7 Stanton number\n         * symbol(s): `St`\n         * application domain: heat transfer\n         * name: StantonNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat transfer into a fluid from a surface and its heat transfer by convection, expressed by `St = K/(ρ*v*c_p)`, where `K` is coefficient of heat transfer (ISO 80000-5) through the surface, `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), and `c_p` is specific heat capacity at constant pressure (ISO 80000-5) of the fluid\n         * remarks: The Stanton number is also given by `St = (Nu)/(Re*Pr) = (Nu)/(Pe)`, where `Nu` is Nusselt number for heat transfer (item 11-5.5), `Re` is the Reynolds number (item 11-4.1), `Pr` is the Prandtl number (item 11-7.1), and Pe  is the Péclet number (item 11-5.2). Sometimes this quantity is called Margoulis number, symbol `Ms` or `Mg`.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.20 Stark number\n         * symbol(s): `Sk`\n         * application domain: generic\n         * name: StarkNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between radiant heat and conductive heat multiplied by the relative temperature difference for a body, expressed by `Sk = (ε*σ*T^3*l)/λ`, where `ε` is emissivity (ISO 80000-7) of the surface, `σ` is the Stefan-Boltzmann constant (ISO 80000-7), `T` is thermodynamic temperature (ISO 80000-5), `l` is characteristic length (ISO 80000-3), and `λ` is thermal conductivity (ISO 80000-5)\n         * remarks: The relative temperature difference is defined by `(ΔT)/T`, where `ΔT = T_s - T_l` is the difference of the temperature at the surface, `T_s`, and the temperature at a layer at a distance `l` from the surface, `T_l`. Sometimes this characteristic number is wrongly defined without the factor `ε`. Deprecated names are: Stefan number and Biot radiation number.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-5.11 Stefan number\n         * symbol(s): `Ste`, `Stf`\n         * application domain: phase transition\n         * name: StefanNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between heat content and latent heat content in a binary mixture undergoing a phase transition, expressed by `Ste = (c_p*ΔT)/Q`, where `c_p` is specific heat capacity at constant pressure (ISO 80000-5), `ΔT` is difference of thermodynamic temperature T (ISO 80000-5) between the phases, and `Q` is quotient of latent heat of phase transition (ISO 80000-5) and mass (ISO 80000-4)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.36 Stokes number\n         * symbol(s): `Stk_4`\n         * application domain: drag\n         * name: StokesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of drag force and internal friction forces for particles dragged in a fluid `Stk_4 = F_D/(η*v*l)`, where `F_D` is drag force (ISO 80000-4), `η` is dynamic viscosity (ISO 80000-4), `v` is speed (ISO 80000-3), and `l` is characteristic length (ISO 80000-3)\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.35 Stokes number\n         * symbol(s): `Stk_3`\n         * application domain: gravity\n         * name: StokesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between viscous forces and gravity forces for particles falling in a fluid, expressed by `Stk_3 = (v*ν)/(g*l^2)`, where `v` is characteristic speed (ISO 80000-3) of particles, `ν` is kinematic viscosity (ISO 80000-4) of the fluid, `g` is acceleration of free fall (ISO 80000-3), and `l` is length (ISO 80000-3) of fall\n         * remarks: None.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.34 Stokes number, power coefficient\n         * symbol(s): `Stk_2`\n         * application domain: rotameter\n         * name: StokesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: Stokes number for calibration of rotameters metering vertical flows of fluids by means of a floating body, expressed by `Stk_2 = (r^3*g*m*ρ)/(η^2) * (ρ_b-ρ)/(ρ_b) = (r^3*g*m)/ν^2 * (1/ρ-1/ρ_b)`, where `r` is ratio of pipe and float radii, `g` is acceleration of free fall (ISO 80000-3), `m` is mass (ISO 80000-4) of the body, `ρ` is mass density (ISO 80000-4) of the fluid, `η` is dynamic viscosity (ISO 80000-4) of the fluid, `ρ_b` is mass density (ISO 80000-4) of the body, and `ν` is kinematic viscosity (ISO 80000-4) of the fluid\n         * remarks: In general use, this value is multiplied by 1,042. See also the Archimedes number (item 11-6.12).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.33 Stokes number\n         * symbol(s): `Stk_1`\n         * application domain: vibrating particles\n         * name: StokesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of friction and inertia forces for the special case of particles vibrating in a fluid or plasma, expressed by `Stk_1 = ν/(d^2*f)`, where `ν` is kinematic viscosity (ISO 80000-4) of the fluid or plasma, `d` is diameter (ISO 80000-3) of particle, and `f` is frequency (ISO 80000-3) of particle vibrations\n         * remarks: Sometimes the inverse of this number is wrongly used.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.32 Stokes number\n         * symbol(s): `Stk`\n         * application domain: time-related\n         * name: StokesNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of friction and inertia forces for particles in a fluid or in a plasma, expressed by `Stk = t_r/t_a`, where `t_r` is relaxation time (ISO 80000-12) of particles to achieve fluid’s velocity due to friction (viscosity), and `t_a` is time (ISO 80000-3) of fluid to alter its velocity under external influence\n         * remarks: In most cases `t_r = l/v`, where `l` is characteristic length, and `v` is speed of fluid. The characteristic length can be the diameter of an obstacle or hole.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.8 Strouhal number, Thomson number\n         * symbol(s): `Sr`, `Sh`\n         * application domain: generic\n         * name: StrouhalNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between a characteristic frequency and a characteristic speed for unsteady flow with periodic behaviour, expressed by `Sr = f*l/v`, where `f` is frequency (ISO 80000-3) of vortex shedding, `l` is characteristic length (ISO 80000-3), and `v` is speed (ISO 80000-3) of flow\n         * remarks: The characteristic length, `l`, can be the diameter of an obstacle in the flow which can cause vortex shedding, or the length of it.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.7 Stuart electrical number\n         * symbol(s): `Se`\n         * application domain: generic\n         * name: StuartElectricalNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of electric energy density and kinematic energy density in a plasma, expressed by `Se = (ε*E^2)/(ρ*v^2)`, where `ε` is electric permittivity (IEC 80000-6), E is electric field strength (IEC 80000-6), ρ is mass density (ISO 80000-4), and `v` is speed (ISO 80000-3)\n         * remarks: The Stuart electrical number is the electrical counterpart of the Cowling number (item 11-8.6).\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-8.12 Stuart number\n         * symbol(s): `Stw`\n         * application domain: generic\n         * name: StuartNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of magnetic forces and inertia forces in an electrically conducting liquid, expressed by `Stw = (B^2*l*σ)/(v*ρ)`, where `B` is magnetic flux density (IEC 80000-6), `l` is characteristic length (ISO 80000-3), `σ` is electrical conductivity (IEC 80000-6), `v` is characteristic speed (ISO 80000-3), and `ρ` is mass density (ISO 80000-4)\n         * remarks: The Stuart number sometimes is called magnetic force parameter. Sometimes the square root is wrongly used. The Stuart number is also given by `Stw = (Ha^2)/(Re)`, where `Ha` is the Hartmann number, and `Re` is the Reynolds number. \n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.40 Taylor number\n         * symbol(s): `Ta`\n         * application domain: momentum transfer\n         * name: TaylorNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between centrifugal force and viscous force of a rotating shaft, expressed by `Ta = (4*ω^2*l^4)/ν^2`, where `ω` is angular velocity (ISO 80000-3) of rotation, `l` is length (ISO 80000-3) perpendicular to the rotation axis, and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: Sometimes the square root of this quantity is wrongly used. The Taylor number for a rotating shaft relative to an annulus is given by `Ta_a = (ω/nu)^2*r*a^3`, where `ω` is angular velocity (ISO 80000-3) of the shaft, `nu` is kinematic viscosity (ISO 80000-4), `r = (r_2+r_1)/2` is mean radius (ISO 80000-3) of the annulus, and `a = (r_2 - r_1)` is width of the annulus, where `r_1` is inner radius of the annulus, and `r_2` is outer radius of the annulus. Sometimes the square root of this quantity is used; this use is deprecated.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.13 thrust coefficient\n         * symbol(s): `c_t`\n         * application domain: generic\n         * name: ThrustCoefficient (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: quotient of the effective thrust force available from a propeller and the inertial force in a fluid, expressed by `c_t = F_T/(ρ*n^2*d^4)`, where `F_T` is thrust force (ISO 80000-4) of the propeller, `ρ` is mass density (ISO 80000-4) of the fluid, `n` is rotational frequency (ISO 80000-3), and `d` is tip diameter (ISO 80000-3) of the propeller\n         * remarks: The thrust coefficient is dependant on the shape of the propeller.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.5 Weber number\n         * symbol(s): `We`\n         * application domain: generic\n         * name: WeberNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertial forces and capillary forces due to surface tension at the interface between two different fluids, expressed by `We = (ρ*v^2*l)/γ`, where `ρ` is mass density (ISO 80000-4), `v` is speed (ISO 80000-3), `l` is characteristic length (ISO 80000-3), and `γ` is surface tension (ISO 80000-4)\n         * remarks: The fluids can be gases or liquids. The different fluids often are drops moving in a gas or bubbles in a liquid. The characteristic length is commonly the diameter of bubbles or drops. The square root of the Weber number is called Rayleigh number. Sometimes the square root of the Weber number as defined here is called the Weber number. That definition is deprecated. Interfaces only exist between two fluids which are not miscible.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-7.7 Weissenberg number\n         * symbol(s): `Wi`\n         * application domain: generic\n         * name: WeissenbergNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: product of time derivative of shear rate and relaxation time in viscoelastic flows, expressed by `Wi = dot(γ)*t_r`, where `dot(γ)` is time derivative of shear strain (ISO 80000-4), and `t_r` is relaxation time (ISO 80000-12)\n         * remarks: The Weissenberg number represents the relative importance of viscous forces when compared to elastic forces. The time derivative of shear strain is sometimes called the shear rate.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n         * source: item 11-4.42 Womersley number\n         * symbol(s): `Wo`, `α`\n         * application domain: generic\n         * name: WomersleyNumber (specializes DimensionOneQuantity)\n         * quantity dimension: 1\n         * measurement unit(s): 1\n         * tensor order: 0\n         * definition: relation between inertial forces and viscous forces in oscillating flows of fluids in pipes, expressed by `Wo = R*sqrt(ω/ν)`, where `R` is (effective) radius (ISO 80000-3) of the pipe, `ω` is angular frequency (ISO 80000-3) of oscillations, and `ν` is kinematic viscosity (ISO 80000-4)\n         * remarks: The Womersley number is used for pulsating flows e.g. in blood flow.\n         "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AbsorptionNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "cauchyNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AlfvénNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AmpèreNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ArchimedesNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ArrheniusNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "AtwoodNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BagnoldNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BagnoldNumberForSolidParticlesValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BatchelorNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BejanNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BejanNumberForEntropyValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BejanNumberForHeatTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BejanNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BinghamNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BiotNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BiotNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BlakeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BodensteinNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BoltzmannNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BondNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "BrinkmanNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CapillaryNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CarnotNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CauchyNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CavitationNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ChandrasekharNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ClausiusNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "jFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CompressibilityNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CowlingNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DarcyFrictionFactorValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DeanNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DeborahNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "eckertNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DynamicCapillaryNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EckertNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EkmanNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ElasticityNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ElectricFieldParameterValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "cowlingNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "EulerNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ExpansionNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "bondNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FanningNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FourierNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FourierNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FroudeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FroudeNumberForHeatTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GalileiNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GoertlerNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "goertlerNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GraetzNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GraetzNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GrashofMagneticNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GrashofNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "GrashofNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HagenNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HallNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HartmannNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "jFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HeatTransferNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HedströmNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "HookeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "JFactorValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "JouleMagneticNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "rossbyNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "KnudsenNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "alfvénNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LagrangeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LandauGinzburgNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LaplaceNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LavalNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LewisNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LiftCoefficientValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LockhartMartinelliParameterValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LorentzNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "LundquistNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "alfvénNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MachNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MagneticNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MagneticPressureNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MarangoniNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassTransferFactorValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "darcyFrictionFactor"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MortonNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NazeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NusseltElectricNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NusseltNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NusseltNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "OhnesorgeNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "binghamNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PoiseuilleNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PomerantsevNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "stokesNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PowerNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PrandtlMagneticNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PrandtlNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PécletNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PécletNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "RayleighNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ReechNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ReynoldsElectricNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ReynoldsMagneticNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ReynoldsNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "RichardsonNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "RobertsNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "RossbyNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SchmidtNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "SommerfeldNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StantonNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StantonNumberForMassTransferValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StarkNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StefanNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StokesNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StokesNumberForDragValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StokesNumberForGravityValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StokesNumberForRotameterValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StokesNumberForVibratingParticlesValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StrouhalNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StuartElectricalNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "StuartNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "laplaceNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TaylorNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "strouhalNumber"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ThrustCoefficientValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "WeberNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "WeissenbergNumberValue"))))
    (declaration (id (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "WomersleyNumberValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQBase")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "AbsorptionNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (kind aliasBinding) (ordinal 0))
      (authored-target "cauchyNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "AlfvénNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "AmpèreNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchimedesNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArrheniusNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "AtwoodNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BagnoldNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind featureTyping) (ordinal 0))
      (authored-target "BagnoldNumberForSolidParticlesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatchelorNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BejanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind featureTyping) (ordinal 0))
      (authored-target "BejanNumberForEntropyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "BejanNumberForHeatTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "BejanNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BinghamNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BiotNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "BiotNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BlakeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BodensteinNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BoltzmannNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BondNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "BrinkmanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CapillaryNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarnotNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CauchyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CavitationNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChandrasekharNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClausiusNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "jFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CompressibilityNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "CowlingNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "DarcyFrictionFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "DeanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "DeborahNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "eckertNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "DynamicCapillaryNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "EckertNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "EkmanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElasticityNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricFieldParameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "cowlingNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "EulerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ExpansionNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "bondNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "FanningNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "FourierNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "FourierNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "FroudeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "FroudeNumberForHeatTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "GalileiNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "GoertlerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (kind aliasBinding) (ordinal 0))
      (authored-target "goertlerNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "GraetzNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "GraetzNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "GrashofMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "GrashofNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "GrashofNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HagenNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HallNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HartmannNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (kind aliasBinding) (ordinal 0))
      (authored-target "jFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HeatTransferNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HedströmNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "HookeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "JFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "JouleMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "rossbyNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "KnudsenNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "alfvénNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LagrangeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LandauGinzburgNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LaplaceNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LavalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LewisNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "LiftCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LockhartMartinelliParameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LorentzNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "LundquistNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "alfvénNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "MachNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "MagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "MagneticPressureNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "MarangoniNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassTransferFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (kind aliasBinding) (ordinal 0))
      (authored-target "darcyFrictionFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "MortonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "NazeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "NusseltElectricNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "NusseltNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "NusseltNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "OhnesorgeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "binghamNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PoiseuilleNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PomerantsevNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (kind aliasBinding) (ordinal 0))
      (authored-target "stokesNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PrandtlMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PrandtlNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "PécletNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "PécletNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "RayleighNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ReechNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ReynoldsElectricNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ReynoldsMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "ReynoldsNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "RichardsonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "RobertsNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "RossbyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "SchmidtNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "SommerfeldNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StantonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "StantonNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StarkNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StefanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StokesNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind featureTyping) (ordinal 0))
      (authored-target "StokesNumberForDragValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind featureTyping) (ordinal 0))
      (authored-target "StokesNumberForGravityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "StokesNumberForRotameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind featureTyping) (ordinal 0))
      (authored-target "StokesNumberForVibratingParticlesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StrouhalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StuartElectricalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "StuartNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "laplaceNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "TaylorNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (kind aliasBinding) (ordinal 0))
      (authored-target "strouhalNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThrustCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "WeberNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "WeissenbergNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue")))))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "WomersleyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 15 19) (end 15 32)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 16 19) (end 16 43)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 17 19) (end 17 29)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQBase")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 14 19) (end 14 37)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (path (named (kind library-package) (name "ISQCharacteristicNumbers")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1371 43) (end 1371 60)) (probe (position 1371 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1645 42) (end 1645 59)) (probe (position 1645 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1940 42) (end 1940 59)) (probe (position 1940 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1269 43) (end 1269 60)) (probe (position 1269 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1957 42) (end 1957 59)) (probe (position 1957 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1199 39) (end 1199 56)) (probe (position 1199 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 178 57) (end 178 74)) (probe (position 178 57))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 161 40) (end 161 57)) (probe (position 161 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1611 42) (end 1611 59)) (probe (position 1611 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 891 48) (end 891 65)) (probe (position 891 48))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 874 53) (end 874 70)) (probe (position 874 53))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1337 53) (end 1337 70)) (probe (position 1337 53))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 246 38) (end 246 55)) (probe (position 246 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 280 40) (end 280 57)) (probe (position 280 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1216 52) (end 1216 69)) (probe (position 1216 52))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 819 37) (end 819 54)) (probe (position 819 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 649 38) (end 649 55)) (probe (position 649 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 316 43) (end 316 60)) (probe (position 316 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1046 42) (end 1046 59)) (probe (position 1046 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1250 37) (end 1250 54)) (probe (position 1250 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 925 41) (end 925 58)) (probe (position 925 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1388 42) (end 1388 59)) (probe (position 1388 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 959 39) (end 959 56)) (probe (position 959 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1490 39) (end 1490 56)) (probe (position 1490 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1354 43) (end 1354 60)) (probe (position 1354 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1736 46) (end 1736 63)) (probe (position 1736 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 942 41) (end 942 58)) (probe (position 942 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1577 48) (end 1577 65)) (probe (position 1577 48))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1683 40) (end 1683 57)) (probe (position 1683 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 386 46) (end 386 63)) (probe (position 386 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 229 37) (end 229 54)) (probe (position 229 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1543 40) (end 1543 57)) (probe (position 1543 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1405 49) (end 1405 66)) (probe (position 1405 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 976 39) (end 976 56)) (probe (position 976 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 352 38) (end 352 55)) (probe (position 352 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 369 43) (end 369 60)) (probe (position 369 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1821 49) (end 1821 66)) (probe (position 1821 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 37 38) (end 37 55)) (probe (position 37 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1286 42) (end 1286 59)) (probe (position 1286 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 405 40) (end 405 57)) (probe (position 405 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1080 55) (end 1080 72)) (probe (position 1080 55))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 734 40) (end 734 57)) (probe (position 734 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 785 54) (end 785 71)) (probe (position 785 54))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 54 39) (end 54 56)) (probe (position 54 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 700 40) (end 700 57)) (probe (position 700 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 422 41) (end 422 58)) (probe (position 422 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1165 54) (end 1165 71)) (probe (position 1165 54))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 995 39) (end 995 56)) (probe (position 995 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1889 48) (end 1889 65)) (probe (position 1889 48))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1114 55) (end 1114 72)) (probe (position 1114 55))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 71 40) (end 71 57)) (probe (position 71 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 441 38) (end 441 55)) (probe (position 441 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1838 37) (end 1838 54)) (probe (position 1838 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1666 41) (end 1666 58)) (probe (position 1666 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1012 45) (end 1012 62)) (probe (position 1012 45))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 299 44) (end 299 61)) (probe (position 299 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1509 38) (end 1509 55)) (probe (position 1509 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 853 34) (end 853 51)) (probe (position 853 34))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1872 46) (end 1872 63)) (probe (position 1872 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 122 40) (end 122 57)) (probe (position 122 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 263 41) (end 263 58)) (probe (position 263 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1974 47) (end 1974 64)) (probe (position 1974 47))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 630 40) (end 630 57)) (probe (position 630 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 458 38) (end 458 55)) (probe (position 458 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1456 38) (end 1456 55)) (probe (position 1456 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 195 42) (end 195 59)) (probe (position 195 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1320 54) (end 1320 71)) (probe (position 1320 54))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1560 40) (end 1560 57)) (probe (position 1560 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1855 42) (end 1855 59)) (probe (position 1855 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 105 37) (end 105 54)) (probe (position 105 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1804 41) (end 1804 58)) (probe (position 1804 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1719 49) (end 1719 66)) (probe (position 1719 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1303 42) (end 1303 59)) (probe (position 1303 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1182 45) (end 1182 62)) (probe (position 1182 45))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1233 39) (end 1233 56)) (probe (position 1233 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1906 37) (end 1906 54)) (probe (position 1906 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1628 48) (end 1628 65)) (probe (position 1628 48))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1131 55) (end 1131 72)) (probe (position 1131 55))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 802 40) (end 802 57)) (probe (position 802 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1473 42) (end 1473 59)) (probe (position 1473 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 475 43) (end 475 60)) (probe (position 475 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1029 44) (end 1029 61)) (probe (position 1029 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 492 38) (end 492 55)) (probe (position 492 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1753 48) (end 1753 65)) (probe (position 1753 48))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1422 40) (end 1422 57)) (probe (position 1422 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1097 57) (end 1097 74)) (probe (position 1097 57))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 751 42) (end 751 59)) (probe (position 751 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 768 41) (end 768 58)) (probe (position 768 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 526 38) (end 526 55)) (probe (position 526 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1923 49) (end 1923 66)) (probe (position 1923 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1594 49) (end 1594 66)) (probe (position 1594 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 20 41) (end 20 58)) (probe (position 20 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 509 43) (end 509 60)) (probe (position 509 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1770 40) (end 1770 57)) (probe (position 1770 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 333 39) (end 333 56)) (probe (position 333 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1439 40) (end 1439 57)) (probe (position 1439 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 666 43) (end 666 60)) (probe (position 666 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1148 55) (end 1148 72)) (probe (position 1148 55))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 836 40) (end 836 57)) (probe (position 836 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1063 38) (end 1063 55)) (probe (position 1063 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 908 39) (end 908 56)) (probe (position 908 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 613 46) (end 613 63)) (probe (position 613 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 596 49) (end 596 66)) (probe (position 596 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 577 51) (end 577 68)) (probe (position 577 51))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 560 60) (end 560 77)) (probe (position 560 60))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 543 39) (end 543 56)) (probe (position 543 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 139 41) (end 139 58)) (probe (position 139 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1702 49) (end 1702 66)) (probe (position 1702 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1787 39) (end 1787 56)) (probe (position 1787 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 683 39) (end 683 56)) (probe (position 683 39))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 212 44) (end 212 61)) (probe (position 212 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 88 38) (end 88 55)) (probe (position 88 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1526 44) (end 1526 61)) (probe (position 1526 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 717 42) (end 717 59)) (probe (position 717 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1385 32) (end 1385 53)) (probe (position 1385 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::absorptionNumber"))) (kind featureTyping) (ordinal 0) (authored-target "AbsorptionNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AbsorptionNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1506 38) (end 1506 50)) (probe (position 1506 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::aeroelasticityParameter"))) (kind aliasBinding) (ordinal 0) (authored-target "cauchyNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1659 31) (end 1659 51)) (probe (position 1659 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber"))) (kind featureTyping) (ordinal 0) (authored-target "AlfvénNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AlfvénNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1954 31) (end 1954 51)) (probe (position 1954 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ampèreNumber"))) (kind featureTyping) (ordinal 0) (authored-target "AmpèreNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AmpèreNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1283 32) (end 1283 53)) (probe (position 1283 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::archimedesNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ArchimedesNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArchimedesNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1971 31) (end 1971 51)) (probe (position 1971 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::arrheniusNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ArrheniusNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ArrheniusNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1213 28) (end 1213 45)) (probe (position 1213 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::atwoodNumber"))) (kind featureTyping) (ordinal 0) (authored-target "AtwoodNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::AtwoodNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 175 29) (end 175 47)) (probe (position 175 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BagnoldNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 192 46) (end 192 81)) (probe (position 192 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bagnoldNumberForSolidParticles"))) (kind featureTyping) (ordinal 0) (authored-target "BagnoldNumberForSolidParticlesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BagnoldNumberForSolidParticlesValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1625 31) (end 1625 51)) (probe (position 1625 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::batchelorNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BatchelorNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BatchelorNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 260 27) (end 260 43)) (probe (position 260 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BejanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 905 37) (end 905 63)) (probe (position 905 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForEntropy"))) (kind featureTyping) (ordinal 0) (authored-target "BejanNumberForEntropyValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForEntropyValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 888 42) (end 888 73)) (probe (position 888 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "BejanNumberForHeatTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForHeatTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1351 42) (end 1351 73)) (probe (position 1351 42))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bejanNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "BejanNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BejanNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 294 29) (end 294 47)) (probe (position 294 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BinghamNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BinghamNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 833 26) (end 833 41)) (probe (position 833 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BiotNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1230 41) (end 1230 71)) (probe (position 1230 41))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::biotNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "BiotNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BiotNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 663 27) (end 663 43)) (probe (position 663 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::blakeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BlakeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BlakeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 330 32) (end 330 53)) (probe (position 330 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bodensteinNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BodensteinNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BodensteinNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1060 31) (end 1060 51)) (probe (position 1060 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::boltzmannNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BoltzmannNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BoltzmannNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1264 26) (end 1264 41)) (probe (position 1264 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BondNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BondNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 939 30) (end 939 49)) (probe (position 939 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::brinkmanNumber"))) (kind featureTyping) (ordinal 0) (authored-target "BrinkmanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::BrinkmanNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1402 31) (end 1402 51)) (probe (position 1402 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::capillaryNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CapillaryNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CapillaryNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 973 28) (end 973 45)) (probe (position 973 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::carnotNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CarnotNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CarnotNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1504 28) (end 1504 45)) (probe (position 1504 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cauchyNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CauchyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CauchyNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1368 32) (end 1368 53)) (probe (position 1368 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cavitationNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CavitationNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CavitationNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1750 35) (end 1750 59)) (probe (position 1750 35))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::chandrasekharNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ChandrasekharNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ChandrasekharNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 956 30) (end 956 49)) (probe (position 956 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::clausiusNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ClausiusNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ClausiusNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 871 28) (end 871 35)) (probe (position 871 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::colburnNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "jFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1591 37) (end 1591 63)) (probe (position 1591 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::compressibilityNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CompressibilityNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CompressibilityNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1697 29) (end 1697 47)) (probe (position 1697 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber"))) (kind featureTyping) (ordinal 0) (authored-target "CowlingNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::CowlingNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 400 35) (end 400 59)) (probe (position 400 35))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor"))) (kind featureTyping) (ordinal 0) (authored-target "DarcyFrictionFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DarcyFrictionFactorValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 243 26) (end 243 41)) (probe (position 243 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deanNumber"))) (kind featureTyping) (ordinal 0) (authored-target "DeanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeanNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1557 29) (end 1557 47)) (probe (position 1557 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::deborahNumber"))) (kind featureTyping) (ordinal 0) (authored-target "DeborahNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DeborahNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 992 27) (end 992 39)) (probe (position 992 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dulongNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "eckertNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1419 38) (end 1419 65)) (probe (position 1419 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::dynamicCapillaryNumber"))) (kind featureTyping) (ordinal 0) (authored-target "DynamicCapillaryNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::DynamicCapillaryNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 990 28) (end 990 45)) (probe (position 990 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eckertNumber"))) (kind featureTyping) (ordinal 0) (authored-target "EckertNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EckertNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 366 27) (end 366 43)) (probe (position 366 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ekmanNumber"))) (kind featureTyping) (ordinal 0) (authored-target "EkmanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EkmanNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 383 32) (end 383 53)) (probe (position 383 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::elasticityNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ElasticityNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElasticityNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1835 38) (end 1835 65)) (probe (position 1835 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::electricFieldParameter"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricFieldParameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ElectricFieldParameterValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1699 34) (end 1699 47)) (probe (position 1699 34))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerMagneticNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "cowlingNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::cowlingNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 51 27) (end 51 43)) (probe (position 51 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eulerNumber"))) (kind featureTyping) (ordinal 0) (authored-target "EulerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::EulerNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1300 31) (end 1300 51)) (probe (position 1300 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::expansionNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ExpansionNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ExpansionNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1266 31) (end 1266 41)) (probe (position 1266 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::eötvösNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "bondNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::bondNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 419 29) (end 419 47)) (probe (position 419 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fanningNumber"))) (kind featureTyping) (ordinal 0) (authored-target "FanningNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FanningNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 748 29) (end 748 47)) (probe (position 748 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumber"))) (kind featureTyping) (ordinal 0) (authored-target "FourierNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1094 44) (end 1094 77)) (probe (position 1094 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::fourierNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "FourierNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FourierNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 68 28) (end 68 45)) (probe (position 68 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "FroudeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 799 43) (end 799 75)) (probe (position 799 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::froudeNumberForHeatTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "FroudeNumberForHeatTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::FroudeNumberForHeatTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 714 29) (end 714 47)) (probe (position 714 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::galileiNumber"))) (kind featureTyping) (ordinal 0) (authored-target "GalileiNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GalileiNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 436 30) (end 436 49)) (probe (position 436 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber"))) (kind featureTyping) (ordinal 0) (authored-target "GoertlerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GoertlerNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 438 32) (end 438 46)) (probe (position 438 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerParameter"))) (kind aliasBinding) (ordinal 0) (authored-target "goertlerNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::goertlerNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1009 28) (end 1009 45)) (probe (position 1009 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumber"))) (kind featureTyping) (ordinal 0) (authored-target "GraetzNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1179 43) (end 1179 75)) (probe (position 1179 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::graetzNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "GraetzNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GraetzNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1903 37) (end 1903 63)) (probe (position 1903 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofMagneticNumber"))) (kind featureTyping) (ordinal 0) (authored-target "GrashofMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofMagneticNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 85 29) (end 85 47)) (probe (position 85 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumber"))) (kind featureTyping) (ordinal 0) (authored-target "GrashofNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1128 44) (end 1128 77)) (probe (position 1128 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::grashofNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "GrashofNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::GrashofNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 455 27) (end 455 43)) (probe (position 455 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hagenNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HagenNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HagenNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1852 26) (end 1852 41)) (probe (position 1852 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hallNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HallNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HallNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1680 30) (end 1680 49)) (probe (position 1680 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hartmannNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HartmannNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HartmannNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 869 33) (end 869 40)) (probe (position 869 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferFactor"))) (kind aliasBinding) (ordinal 0) (authored-target "jFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1026 34) (end 1026 57)) (probe (position 1026 34))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::heatTransferNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HeatTransferNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HeatTransferNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 313 33) (end 313 55)) (probe (position 313 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hedströmNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HedströmNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HedströmNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1523 27) (end 1523 43)) (probe (position 1523 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::hookeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "HookeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::HookeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 867 23) (end 867 35)) (probe (position 867 23))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jFactor"))) (kind featureTyping) (ordinal 0) (authored-target "JFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JFactorValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1886 35) (end 1886 59)) (probe (position 1886 35))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::jouleMagneticNumber"))) (kind featureTyping) (ordinal 0) (authored-target "JouleMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::JouleMagneticNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 349 27) (end 349 39)) (probe (position 349 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kiebelNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "rossbyNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 136 29) (end 136 47)) (probe (position 136 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::knudsenNumber"))) (kind featureTyping) (ordinal 0) (authored-target "KnudsenNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::KnudsenNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1663 30) (end 1663 45)) (probe (position 1663 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::kármanNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "alfvénNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 277 30) (end 277 49)) (probe (position 277 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lagrangeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LagrangeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LagrangeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1988 36) (end 1988 61)) (probe (position 1988 36))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::landauGinzburgNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LandauGinzburgNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LandauGinzburgNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 644 29) (end 644 47)) (probe (position 644 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LaplaceNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LaplaceNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 472 27) (end 472 43)) (probe (position 472 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lavalNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LavalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LavalNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1470 27) (end 1470 43)) (probe (position 1470 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lewisNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LewisNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LewisNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 209 31) (end 209 51)) (probe (position 209 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::liftCoefficient"))) (kind featureTyping) (ordinal 0) (authored-target "LiftCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LiftCoefficientValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1334 43) (end 1334 75)) (probe (position 1334 43))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lockhartMartinelliParameter"))) (kind featureTyping) (ordinal 0) (authored-target "LockhartMartinelliParameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LockhartMartinelliParameterValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1574 29) (end 1574 47)) (probe (position 1574 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lorentzNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LorentzNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LorentzNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1869 31) (end 1869 51)) (probe (position 1869 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::lundquistNumber"))) (kind featureTyping) (ordinal 0) (authored-target "LundquistNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::LundquistNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1661 33) (end 1661 48)) (probe (position 1661 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machMagneticNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "alfvénNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::alfvénNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 119 26) (end 119 41)) (probe (position 119 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::machNumber"))) (kind featureTyping) (ordinal 0) (authored-target "MachNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MachNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1818 30) (end 1818 49)) (probe (position 1818 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticNumber"))) (kind featureTyping) (ordinal 0) (authored-target "MagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1733 38) (end 1733 65)) (probe (position 1733 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::magneticPressureNumber"))) (kind featureTyping) (ordinal 0) (authored-target "MagneticPressureNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MagneticPressureNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1317 31) (end 1317 51)) (probe (position 1317 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::marangoniNumber"))) (kind featureTyping) (ordinal 0) (authored-target "MarangoniNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MarangoniNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1196 34) (end 1196 57)) (probe (position 1196 34))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::massTransferFactor"))) (kind featureTyping) (ordinal 0) (authored-target "MassTransferFactorValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MassTransferFactorValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 402 34) (end 402 53)) (probe (position 402 34))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::moodyFrictionFactor"))) (kind aliasBinding) (ordinal 0) (authored-target "darcyFrictionFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::darcyFrictionFactor")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1247 28) (end 1247 45)) (probe (position 1247 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::mortonNumber"))) (kind featureTyping) (ordinal 0) (authored-target "MortonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::MortonNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1920 26) (end 1920 41)) (probe (position 1920 26))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nazeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "NazeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NazeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1642 37) (end 1642 63)) (probe (position 1642 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltElectricNumber"))) (kind featureTyping) (ordinal 0) (authored-target "NusseltElectricNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltElectricNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 816 29) (end 816 47)) (probe (position 816 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumber"))) (kind featureTyping) (ordinal 0) (authored-target "NusseltNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1145 44) (end 1145 77)) (probe (position 1145 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::nusseltNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "NusseltNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::NusseltNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1487 31) (end 1487 51)) (probe (position 1487 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ohnesorgeNumber"))) (kind featureTyping) (ordinal 0) (authored-target "OhnesorgeNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::OhnesorgeNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 296 31) (end 296 44)) (probe (position 296 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::plasticityNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "binghamNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::binghamNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 489 32) (end 489 53)) (probe (position 489 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::poiseuilleNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PoiseuilleNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PoiseuilleNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1043 33) (end 1043 55)) (probe (position 1043 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pomerantsevNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PomerantsevNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PomerantsevNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 593 31) (end 593 43)) (probe (position 593 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerCoefficient"))) (kind aliasBinding) (ordinal 0) (authored-target "stokesNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 506 27) (end 506 43)) (probe (position 506 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::powerNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PowerNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PowerNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1767 37) (end 1767 63)) (probe (position 1767 37))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlMagneticNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PrandtlMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlMagneticNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1436 29) (end 1436 47)) (probe (position 1436 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::prandtlNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PrandtlNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PrandtlNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 765 31) (end 765 51)) (probe (position 765 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumber"))) (kind featureTyping) (ordinal 0) (authored-target "PécletNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1111 46) (end 1111 81)) (probe (position 1111 46))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::pécletNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "PécletNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::PécletNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 782 30) (end 782 49)) (probe (position 782 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rayleighNumber"))) (kind featureTyping) (ordinal 0) (authored-target "RayleighNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RayleighNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 540 27) (end 540 43)) (probe (position 540 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reechNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ReechNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReechNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1937 38) (end 1937 65)) (probe (position 1937 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsElectricNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ReynoldsElectricNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsElectricNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1608 38) (end 1608 65)) (probe (position 1608 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsMagneticNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ReynoldsMagneticNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsMagneticNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 34 30) (end 34 49)) (probe (position 34 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::reynoldsNumber"))) (kind featureTyping) (ordinal 0) (authored-target "ReynoldsNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ReynoldsNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 523 32) (end 523 53)) (probe (position 523 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::richardsonNumber"))) (kind featureTyping) (ordinal 0) (authored-target "RichardsonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RichardsonNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1784 29) (end 1784 47)) (probe (position 1784 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::robertsNumber"))) (kind featureTyping) (ordinal 0) (authored-target "RobertsNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RobertsNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 347 28) (end 347 45)) (probe (position 347 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::rossbyNumber"))) (kind featureTyping) (ordinal 0) (authored-target "RossbyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::RossbyNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1453 29) (end 1453 47)) (probe (position 1453 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::schmidtNumber"))) (kind featureTyping) (ordinal 0) (authored-target "SchmidtNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SchmidtNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 680 32) (end 680 53)) (probe (position 680 32))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::sommerfeldNumber"))) (kind featureTyping) (ordinal 0) (authored-target "SommerfeldNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::SommerfeldNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 850 29) (end 850 47)) (probe (position 850 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StantonNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1162 44) (end 1162 77)) (probe (position 1162 44))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stantonNumberForMassTransfer"))) (kind featureTyping) (ordinal 0) (authored-target "StantonNumberForMassTransferValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StantonNumberForMassTransferValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1077 27) (end 1077 43)) (probe (position 1077 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::starkNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StarkNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StarkNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 922 28) (end 922 45)) (probe (position 922 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stefanNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StefanNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StefanNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 557 28) (end 557 45)) (probe (position 557 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StokesNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 627 35) (end 627 59)) (probe (position 627 35))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForDrag"))) (kind featureTyping) (ordinal 0) (authored-target "StokesNumberForDragValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForDragValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 610 38) (end 610 65)) (probe (position 610 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForGravity"))) (kind featureTyping) (ordinal 0) (authored-target "StokesNumberForGravityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForGravityValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 591 40) (end 591 69)) (probe (position 591 40))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForRotameter"))) (kind featureTyping) (ordinal 0) (authored-target "StokesNumberForRotameterValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForRotameterValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 574 49) (end 574 87)) (probe (position 574 49))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stokesNumberForVibratingParticles"))) (kind featureTyping) (ordinal 0) (authored-target "StokesNumberForVibratingParticlesValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StokesNumberForVibratingParticlesValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 153 30) (end 153 49)) (probe (position 153 30))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StrouhalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StrouhalNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1716 38) (end 1716 65)) (probe (position 1716 38))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartElectricalNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StuartElectricalNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartElectricalNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1801 28) (end 1801 45)) (probe (position 1801 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::stuartNumber"))) (kind featureTyping) (ordinal 0) (authored-target "StuartNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::StuartNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 646 29) (end 646 42)) (probe (position 646 29))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::suratmanNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "laplaceNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::laplaceNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 697 28) (end 697 45)) (probe (position 697 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::taylorNumber"))) (kind featureTyping) (ordinal 0) (authored-target "TaylorNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::TaylorNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 155 28) (end 155 42)) (probe (position 155 28))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thomsonNumber"))) (kind aliasBinding) (ordinal 0) (authored-target "strouhalNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::strouhalNumber")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 226 33) (end 226 55)) (probe (position 226 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::thrustCoefficient"))) (kind featureTyping) (ordinal 0) (authored-target "ThrustCoefficientValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::ThrustCoefficientValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 102 27) (end 102 43)) (probe (position 102 27))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weberNumber"))) (kind featureTyping) (ordinal 0) (authored-target "WeberNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeberNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 1540 33) (end 1540 55)) (probe (position 1540 33))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::weissenbergNumber"))) (kind featureTyping) (ordinal 0) (authored-target "WeissenbergNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WeissenbergNumberValue")))))
  )
  (query (document "memory://snapshot/isq_characteristic_numbers.md") (range (start 731 31) (end 731 51)) (probe (position 731 31))
    (reference (id (source (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::womersleyNumber"))) (kind featureTyping) (ordinal 0) (authored-target "WomersleyNumberValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/isq_characteristic_numbers.md") (qualified-name "ISQCharacteristicNumbers::WomersleyNumberValue")))))
  )
)
~~~
