# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/ISQThermodynamics
type=file
~~~
# SOURCE
~~~sysml
standard library package ISQThermodynamics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-5:2019 "Thermodynamics"
     * see also https://www.iso.org/standard/64976.html
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


    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    /* See package ISQBase for the declarations of ThermodynamicTemperatureValue and ThermodynamicTemperatureUnit */

    alias TemperatureUnit for ThermodynamicTemperatureUnit;
    alias TemperatureValue for ThermodynamicTemperatureValue;
    alias temperature for thermodynamicTemperature;

    /* ISO-80000-5 item 5-2 Celsius temperature */
    attribute def CelsiusTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-2 Celsius temperature
         * symbol(s): `t`, `θ`
         * application domain: generic
         * name: CelsiusTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): °C
         * tensor order: 0
         * definition: temperature difference from the thermodynamic temperature of the ice point is called the Celsius temperature t, which is defined by the quantity equation: `t = T - T_0` where `T` is thermodynamic temperature (item 5-1) and `T_0 = 273,15 K`
         * remarks: The unit degree Celsius is a special name for the kelvin for use in stating values of Celsius temperature. The unit degree Celsius is by definition equal in magnitude to the kelvin. A difference or interval of temperature may be expressed in kelvin or in degrees Celsius. The thermodynamic temperature `T_0` is 0,01 K below the thermodynamic temperature of the triple point of water. The symbol °C for the degree Celsius shall be preceded by a space (see ISO 80000-1). Prefixes are not allowed in combination with the unit °C.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CelsiusTemperatureUnit[1];
    }

    attribute celsiusTemperature: CelsiusTemperatureValue[*] nonunique :> scalarQuantities;

    attribute def CelsiusTemperatureUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.1 linear expansion coefficient */
    attribute def LinearExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.1 linear expansion coefficient
         * symbol(s): `α_l`
         * application domain: generic
         * name: LinearExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of length with temperature: `α_l = 1/l * (dl)/(dT)` where l is length (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearExpansionCoefficientUnit[1];
    }

    attribute linearExpansionCoefficient: LinearExpansionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.2 cubic expansion coefficient */
    attribute def CubicExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.2 cubic expansion coefficient
         * symbol(s): `α_V`, `γ`
         * application domain: generic
         * name: CubicExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of volume with temperature: `α_V = 1/V * (dV)/(dT)` where `V` is volume (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Also called volumetric expansion coefficient. The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CubicExpansionCoefficientUnit[1];
    }

    attribute cubicExpansionCoefficient: CubicExpansionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def CubicExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.3 relative pressure coefficient */
    attribute def RelativePressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.3 relative pressure coefficient
         * symbol(s): `α_p`
         * application domain: generic
         * name: RelativePressureCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of pressure with temperature at constant volume: `α_p = 1/p * ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RelativePressureCoefficientUnit[1];
    }

    attribute relativePressureCoefficient: RelativePressureCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def RelativePressureCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-4 pressure coefficient */
    attribute def PressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-4 pressure coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PressureCoefficient
         * quantity dimension: L^-1*M^1*T^-2*Θ^-1
         * measurement unit(s): Pa/K, kg*m^-1*s^-2*K^-1
         * tensor order: 0
         * definition: change of pressure with temperature at constant volume: `β = ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PressureCoefficientUnit[1];
    }

    attribute pressureCoefficient: PressureCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PressureCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-5.1 isothermal compressibility */
    attribute def IsothermalCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.1 isothermal compressibility
         * symbol(s): `ϰ_T`
         * application domain: generic
         * name: IsothermalCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant temperature: `ϰ_T = -1/V * ((partial V)/(partial p))_T` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IsothermalCompressibilityUnit[1];
    }

    attribute isothermalCompressibility: IsothermalCompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def IsothermalCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-5.2 isentropic compressibility */
    attribute def IsentropicCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.2 isentropic compressibility
         * symbol(s): `ϰ_S`
         * application domain: generic
         * name: IsentropicCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant entropy: `ϰ_S = -1/V * ((partial V)/(partial p))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IsentropicCompressibilityUnit[1];
    }

    attribute isentropicCompressibility: IsentropicCompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def IsentropicCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-6.1 heat, amount of heat */
    attribute heat: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-6.1 heat, amount of heat
         * symbol(s): `Q`
         * application domain: generic
         * name: Heat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between the increase in the internal energy (item 5-20.2) of a system and the work (ISO 80000-4) done on the system, provided that the amounts of substances within the system are not changed
         * remarks: The heat transferred in an isothermal phase transformation should be expressed as the change in the appropriate state functions, e.g. `T ΔS`, where `T` is thermodynamic temperature (item 5-1) and `S` is entropy (item 5-18), or `ΔH`, where `H` is enthalpy (item 5-20.3). NOTE A supply of heat can correspond to an increase in thermodynamic temperature or to other effects, such as phase change or chemical processes; see item 5-6.2.
         */
    }

    alias amountOfHeat for heat;

    /* ISO-80000-5 item 5-6.2 latent heat */
    attribute latentHeat: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-6.2 latent heat
         * symbol(s): `Q`
         * application domain: generic
         * name: LatentHeat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy released or absorbed by a system during a constant-temperature process
         * remarks: Examples of latent heat are latent heat of fusion (melting) and latent heat of vaporization (boiling).
         */
    }

    /* ISO-80000-5 item 5-7 heat flow rate */
    attribute def HeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-7 heat flow rate
         * symbol(s): `dot(Q)`
         * application domain: generic
         * name: HeatFlowRate
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J/s, kg*m^2*s^-3
         * tensor order: 0
         * definition: time rate at which heat (item 5-6.1) crosses a given surface
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HeatFlowRateUnit[1];
    }

    attribute heatFlowRate: HeatFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def HeatFlowRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-8 density of heat flow rate */
    attribute def DensityOfHeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-8 density of heat flow rate
         * symbol(s): `q`, `φ`
         * application domain: generic
         * name: DensityOfHeatFlowRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: quotient of heat flow rate and area: `q = dot Q / A` where `dot Q` is heat flow rate (item 5-7) and A is area (ISO 80000-3) of a given surface
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DensityOfHeatFlowRateUnit[1];
    }

    attribute densityOfHeatFlowRate: DensityOfHeatFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def DensityOfHeatFlowRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-9 thermal conductivity */
    attribute def ThermalConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-9 thermal conductivity
         * symbol(s): `λ_l`, `(ϰ)`
         * application domain: generic
         * name: ThermalConductivity
         * quantity dimension: L^1*M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m*K), kg*m*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature gradient that has the same direction as the heat flow
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalConductivityUnit[1];
    }

    attribute thermalConductivity: ThermalConductivityValue[*] nonunique :> scalarQuantities;

    attribute def ThermalConductivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-10.1 coefficient of heat transfer */
    attribute def CoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.1 coefficient of heat transfer
         * symbol(s): `K`, `(k)`
         * application domain: generic
         * name: CoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature (item 5-1) difference
         * remarks: In building technology, the coefficient of heat transfer is often called thermal transmittance, with the symbol U (no longer recommended). See remark to item 5-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CoefficientOfHeatTransferUnit[1];
    }

    attribute coefficientOfHeatTransfer: CoefficientOfHeatTransferValue[*] nonunique :> scalarQuantities;

    attribute def CoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-10.2 surface coefficient of heat transfer */
    attribute def SurfaceCoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.2 surface coefficient of heat transfer
         * symbol(s): `h`, `(α)`
         * application domain: generic
         * name: SurfaceCoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate and the difference of the temperature at the surface and a reference temperature: `h = q / (T_s - T_r)` where q is density of heat flow rate (item 5-8), `T_s` is the thermodynamic temperature (item 5-1) at the surface, and `T_r` is a reference thermodynamic temperature characterizing the adjacent surroundings
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceCoefficientOfHeatTransferUnit[1];
    }

    attribute surfaceCoefficientOfHeatTransfer: SurfaceCoefficientOfHeatTransferValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceCoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-11 thermal insulance, coefficient of thermal insulance */
    attribute def ThermalInsulanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-11 thermal insulance, coefficient of thermal insulance
         * symbol(s): `M`
         * application domain: generic
         * name: ThermalInsulance
         * quantity dimension: M^-1*T^3*Θ^1
         * measurement unit(s): m^2*K/W, kg^-1*s^3*K
         * tensor order: 0
         * definition: inverse of coefficient of heat transfer `K`: `M = 1/K` where `K` is coefficient of heat transfer (item 5-10.1)
         * remarks: In building technology, this quantity is often called thermal resistance, with the symbol R.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalInsulanceUnit[1];
    }

    attribute thermalInsulance: ThermalInsulanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalInsulanceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    alias CoefficientOfThermalInsulanceUnit for ThermalInsulanceUnit;
    alias CoefficientOfThermalInsulanceValue for ThermalInsulanceValue;
    alias coefficientOfThermalInsulance for thermalInsulance;

    /* ISO-80000-5 item 5-12 thermal resistance */
    attribute def ThermalResistanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-12 thermal resistance
         * symbol(s): `R`
         * application domain: generic
         * name: ThermalResistance
         * quantity dimension: L^-2*M^-1*T^3*Θ^1
         * measurement unit(s): K/W, kg^-1*m^-2*s^3*K
         * tensor order: 0
         * definition: quotient of thermodynamic temperature (item 5-1) difference and heat flow rate (item 5-7)
         * remarks: See remark to item 5-11.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalResistanceUnit[1];
    }

    attribute thermalResistance: ThermalResistanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalResistanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-13 thermal conductance */
    attribute def ThermalConductanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-13 thermal conductance
         * symbol(s): `G`, `(H)`
         * application domain: generic
         * name: ThermalConductance
         * quantity dimension: L^2*M^1*T^-3*Θ^-1
         * measurement unit(s): W/K, kg*m^2*s^-3*K^-1
         * tensor order: 0
         * definition: inverse of thermal resistance `R`: `G = 1/R` where `R` is thermal resistance (item 5-12)
         * remarks: See remark to item 5-11. This quantity is also called heat transfer coefficient. See item 5-10.1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalConductanceUnit[1];
    }

    attribute thermalConductance: ThermalConductanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalConductanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-14 thermal diffusivity */
    attribute def ThermalDiffusivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-14 thermal diffusivity
         * symbol(s): `a`
         * application domain: generic
         * name: ThermalDiffusivity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of thermal conductivity and the product of mass density and specific heat capacity: `a = λ / (ρ C_p)` where `λ` is thermal conductivity (item 5-9), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (item 5-16.2)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalDiffusivityUnit[1];
    }

    attribute thermalDiffusivity: ThermalDiffusivityValue[*] nonunique :> scalarQuantities;

    attribute def ThermalDiffusivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-15 heat capacity */
    attribute def HeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-15 heat capacity
         * symbol(s): `C`
         * application domain: generic
         * name: HeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: derivative of added heat with respect to thermodynamic temperature of a system: `C = (dQ)/(dT)` where `Q` is amount of heat (item 5-6.1) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Heat capacity is not completely defined unless specified as seen in items 5-16.2, 5-16.3 and 5-16.4.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HeatCapacityUnit[1];
    }

    attribute heatCapacity: HeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def HeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.1 specific heat capacity */
    attribute def SpecificHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.1 specific heat capacity
         * symbol(s): `c`
         * application domain: generic
         * name: SpecificHeatCapacity
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of heat capacity and mass: `c = C/m` where `C` is heat capacity (item 5-15) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantities related to the amount of substance, see ISO 80000-9.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityUnit[1];
    }

    attribute specificHeatCapacity: SpecificHeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.2 specific heat capacity at constant pressure */
    attribute def SpecificHeatCapacityAtConstantPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.2 specific heat capacity at constant pressure
         * symbol(s): `c_p`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant pressure (ISO 80000-4)
         * remarks: Also called specific isobaric heat capacity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtConstantPressureUnit[1];
    }

    attribute specificHeatCapacityAtConstantPressure: SpecificHeatCapacityAtConstantPressureValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtConstantPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.3 specific heat capacity at constant volume */
    attribute def SpecificHeatCapacityAtConstantVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.3 specific heat capacity at constant volume
         * symbol(s): `c_V`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantVolume
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant volume (ISO 80000-3)
         * remarks: Also called specific isochoric heat capacity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtConstantVolumeUnit[1];
    }

    attribute specificHeatCapacityAtConstantVolume: SpecificHeatCapacityAtConstantVolumeValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtConstantVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.4 specific heat capacity at saturated vapour pressure */
    attribute def SpecificHeatCapacityAtSaturatedVapourPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.4 specific heat capacity at saturated vapour pressure
         * symbol(s): `c_"sat"`
         * application domain: generic
         * name: SpecificHeatCapacityAtSaturatedVapourPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at saturated vapour pressure (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtSaturatedVapourPressureUnit[1];
    }

    attribute specificHeatCapacityAtSaturatedVapourPressure: SpecificHeatCapacityAtSaturatedVapourPressureValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtSaturatedVapourPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-17.1 ratio of specific heat capacities */
    attribute def RatioOfSpecificHeatCapacitiesValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.1 ratio of specific heat capacities
         * symbol(s): `γ`
         * application domain: generic
         * name: RatioOfSpecificHeatCapacities (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of specific heat capacity at constant pressure and specific heat capacity at constant volume: `γ = c_p/c_V` where `c_p` is specific heat capacity at constant pressure (item 5-16.2) and `c_V` is specific heat capacity at constant volume (item 5-16.3)
         * remarks: This quantity can also be expressed by `γ = C_p/C_V` where `C_p` is heat capacity at constant pressure and `C_V` is heat capacity at constant volume.
         */
    }
    attribute ratioOfSpecificHeatCapacities: RatioOfSpecificHeatCapacitiesValue :> scalarQuantities;

    /* ISO-80000-5 item 5-17.2 isentropic exponent, isentropic expansion factor */
    attribute def IsentropicExponentValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.2 isentropic exponent, isentropic expansion factor
         * symbol(s): `ϰ`
         * application domain: generic
         * name: IsentropicExponent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: the negative of relative pressure change, divided by relative volume change, at constant entropy: `ϰ = -V/p * ((partial p)/(partial V))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: For an ideal gas, `ϰ` is equal to `γ` (item 5-17.1).
         */
    }
    attribute isentropicExponent: IsentropicExponentValue :> scalarQuantities;

    alias isentropicExpansionFactor for isentropicExponent;

    /* ISO-80000-5 item 5-18 entropy */
    attribute def EntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-18 entropy
         * symbol(s): `S`
         * application domain: generic
         * name: Entropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: natural logarithm of number of equally probable microscopic configurations in a macroscopic system, multiplied by the Boltzmann constant: `S = k lnW` where `W` is number of configurations and `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EntropyUnit[1];
    }

    attribute entropy: EntropyValue[*] nonunique :> scalarQuantities;

    attribute def EntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-19 specific entropy */
    attribute def SpecificEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-19 specific entropy
         * symbol(s): `s`
         * application domain: generic
         * name: SpecificEntropy
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of entropy and mass: `s = S/m` where `S` is entropy (item 5-18) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantity related to amount of substance, see ISO 80000-9.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEntropyUnit[1];
    }

    attribute specificEntropy: SpecificEntropyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-20.1 energy */
    attribute def EnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-20.1 energy
         * symbol(s): `E`
         * application domain: thermodynamics
         * name: Energy
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: ability of a system to do work (ISO 80000-4)
         * remarks: Energy exists in different forms that are mutually transformable into each other, either totally or partially. In contrast to internal energy (item 5-20.2), energy is not a state function.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyUnit[1];
    }

    attribute energy: EnergyValue[*] nonunique :> scalarQuantities;

    attribute def EnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-20.2 internal energy, thermodynamic energy */
    attribute internalEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.2 internal energy, thermodynamic energy
         * symbol(s): `U`
         * application domain: generic
         * name: InternalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy of a system whose change is given by the amount of the heat (item 5-6.1) transferred to the system and the work (ISO 80000-4) done on the system, provided that the system is closed and no chemical reactions occur
         * remarks: In thermodynamic text books, usually the formula `ΔU = Q + W` is used. Note that the zero of the energy is undefined.
         */
    }

    alias thermodynamicEnergy for internalEnergy;

    /* ISO-80000-5 item 5-20.3 enthalpy */
    attribute enthalpy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.3 enthalpy
         * symbol(s): `H`
         * application domain: generic
         * name: Enthalpy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of internal energy of the system and the product of pressure and volume of the system: `H = U + p*V` where U is internal energy (item 5-20.2), `p` is pressure (ISO 80000-4), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
    }

    /* ISO-80000-5 item 5-20.4 Helmholtz energy, Helmholtz function */
    attribute helmholtzEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.4 Helmholtz energy, Helmholtz function
         * symbol(s): `A`, `F`
         * application domain: generic
         * name: HelmholtzEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of internal energy of the system and the product of thermodynamic temperature and entropy of the system: `A = U - TS` where `U` is internal energy (item 5-20.2), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias helmholtzFunction for helmholtzEnergy;

    /* ISO-80000-5 item 5-20.5 Gibbs energy, Gibbs function */
    attribute gibbsEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.5 Gibbs energy, Gibbs function
         * symbol(s): `G`
         * application domain: generic
         * name: GibbsEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of the enthalpy and the product of thermodynamic temperature and entropy of the system: `G = H - T*S` where H is enthalpy (item 5-20.3), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias gibbsFunction for gibbsEnergy;

    /* ISO-80000-5 item 5-21.1 specific energy */
    attribute def SpecificEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.1 specific energy
         * symbol(s): `e`
         * application domain: generic
         * name: SpecificEnergy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of energy and mass: `e = E/m` where `E` is energy (item 5-20.1) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEnergyUnit[1];
    }

    attribute specificEnergy: SpecificEnergyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-21.2 specific internal energy, specific thermodynamic energy */
    attribute specificInternalEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.2 specific internal energy, specific thermodynamic energy
         * symbol(s): `u`
         * application domain: generic
         * name: SpecificInternalEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of internal energy and mass: `u = U/m` where `U` is internal energy (item 5-20.2) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
    }

    alias specificThermodynamicEnergy for specificInternalEnergy;

    /* ISO-80000-5 item 5-21.3 specific enthalpy */
    attribute def SpecificEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.3 specific enthalpy
         * symbol(s): `h`
         * application domain: generic
         * name: SpecificEnthalpy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of enthalpy and mass: `h = H/m` where `H` is enthalpy (item 5-20.3) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEnthalpyUnit[1];
    }

    attribute specificEnthalpy: SpecificEnthalpyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-21.4 specific Helmholtz energy, specific Helmholtz function */
    attribute specificHelmholtzEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.4 specific Helmholtz energy, specific Helmholtz function
         * symbol(s): `a`, `f`
         * application domain: generic
         * name: SpecificHelmholtzEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Helmholtz energy and mass: `a = A/m` where A is Helmholtz energy (item 5-20.4) and m is mass (ISO 80000-4)
         * remarks: The name specific Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias specificHelmholtzFunction for specificHelmholtzEnergy;

    /* ISO-80000-5 item 5-21.5 specific Gibbs energy, specific Gibbs function */
    attribute specificGibbsEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.5 specific Gibbs energy, specific Gibbs function
         * symbol(s): `g`
         * application domain: generic
         * name: SpecificGibbsEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Gibbs energy and mass: `g = G/m` where `G` is Gibbs energy (item 5-20.5) and `m` is mass (ISO 80000-4)
         * remarks: The name specific Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias specificGibbsFunction for specificGibbsEnergy;

    /* ISO-80000-5 item 5-22 Massieu function */
    attribute def MassieuFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-22 Massieu function
         * symbol(s): `J`
         * application domain: generic
         * name: MassieuFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Helmholtz energy and temperature: `J = -A/T` where `A` is Helmholtz energy (item 5-20.4) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassieuFunctionUnit[1];
    }

    attribute massieuFunction: MassieuFunctionValue[*] nonunique :> scalarQuantities;

    attribute def MassieuFunctionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-23 Planck function */
    attribute def PlanckFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-23 Planck function
         * symbol(s): `Y`
         * application domain: generic
         * name: PlanckFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Gibbs energy and temperature: `Y = -G/T` where G is Gibbs energy (item 5-20.5) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PlanckFunctionUnit[1];
    }

    attribute planckFunction: PlanckFunctionValue[*] nonunique :> scalarQuantities;

    attribute def PlanckFunctionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-24 Joule-Thomson coefficient */
    attribute def JouleThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-24 Joule-Thomson coefficient
         * symbol(s): `μ_"JT"`
         * application domain: generic
         * name: JouleThomsonCoefficient
         * quantity dimension: L^1*M^-1*T^2*Θ^1
         * measurement unit(s): K/Pa, kg^-1*m*s^2*K
         * tensor order: 0
         * definition: change of thermodynamic temperature with respect to pressure in a Joule-Thomson process at constant enthalpy: `μ_(JT) = ((partial T)/(partial p))_H` where `T` is thermodynamic temperature (item 5-1), `p` is pressure (ISO 80000-4) and H is enthalpy (item 5-20.3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: JouleThomsonCoefficientUnit[1];
    }

    attribute jouleThomsonCoefficient: JouleThomsonCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def JouleThomsonCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-25.1 thermal efficiency */
    attribute def ThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.1 thermal efficiency
         * symbol(s): `η`
         * application domain: thermodynamics
         * name: ThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of work (ISO 80000-4) delivered by a heat engine and supplied heat: `η = W/Q` where `W` is work (ISO 80000-4) and `Q` is heat (item 5-6.1)
         * remarks: None.
         */
    }
    attribute thermalEfficiency: ThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-25.2 maximum thermal efficiency */
    attribute def MaximumThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.2 maximum thermal efficiency
         * symbol(s): `η_"max"`
         * application domain: generic
         * name: MaximumThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency determined by the quotient of the temperatures of the hot source and the cold sink: `η_max = 1 - T_c/T_h` where `T_c` is the thermodynamic temperature (item 5-1) of the cold sink and `T_h` is the thermodynamic temperature (item 5-1) of the hot source
         * remarks: An ideal heat engine operating according to the Carnot process is delivering the maximum efficiency.
         */
    }
    attribute maximumThermalEfficiency: MaximumThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-26 specific gas constant */
    attribute def SpecificGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-26 specific gas constant
         * symbol(s): `R_s`
         * application domain: generic
         * name: SpecificGasConstant
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the Boltzmann constant `k` (ISO 80000-1) and the mass `m` (ISO 80000-4) of the gas particle
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificGasConstantUnit[1];
    }

    attribute specificGasConstant: SpecificGasConstantValue[*] nonunique :> scalarQuantities;

    attribute def SpecificGasConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-27 mass concentration of water */
    attribute def MassConcentrationOfWaterValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-27 mass concentration of water
         * symbol(s): `w`
         * application domain: generic
         * name: MassConcentrationOfWater
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water and a specified volume: `w = m/V` where `m` is mass (ISO 80000-4) of water, irrespective of the form of aggregation state, and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water at saturation is denoted `w_"sat"`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationOfWaterUnit[1];
    }

    attribute massConcentrationOfWater: MassConcentrationOfWaterValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationOfWaterUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-5 item 5-28 mass concentration of water vapour absolute humidity */
    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-28 mass concentration of water vapour absolute humidity
         * symbol(s): `v`
         * application domain: generic
         * name: MassConcentrationOfWaterVapourAbsoluteHumidity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water vapour and a specified volume: `v = m/V` where m is mass (ISO 80000-4) of water vapour and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water vapour at saturation is denoted `v_"sat"`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationOfWaterVapourAbsoluteHumidityUnit[1];
    }

    attribute massConcentrationOfWaterVapourAbsoluteHumidity: MassConcentrationOfWaterVapourAbsoluteHumidityValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-5 item 5-29 mass ratio of water to dry matter */
    attribute def MassRatioOfWaterToDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-29 mass ratio of water to dry matter
         * symbol(s): `u`
         * application domain: generic
         * name: MassRatioOfWaterToDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water and mass of dry matter: `u = m/m_d` where `m` is mass (ISO 80000-4) of water and `m_d` is mass of dry matter
         * remarks: Mass ratio of water to dry matter at saturation is denoted `u_"sat"`.
         */
    }
    attribute massRatioOfWaterToDryMatter: MassRatioOfWaterToDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-30 mass ratio of water vapour to dry gas */
    attribute def MassRatioOfWaterVapourToDryGasValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-30 mass ratio of water vapour to dry gas
         * symbol(s): `r`, `(x)`
         * application domain: generic
         * name: MassRatioOfWaterVapourToDryGas (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water vapour and mass of dry gas: `r = m/m_d` where `m` is mass (ISO 80000-4) of water vapour and `m_d` is mass of dry gas
         * remarks: Mass ratio of water vapour to dry gas at saturation is denoted `r_"sat"`. Mass ratio of water vapour to dry gas is also called mixing ratio.
         */
    }
    attribute massRatioOfWaterVapourToDryGas: MassRatioOfWaterVapourToDryGasValue :> scalarQuantities;

    /* ISO-80000-5 item 5-31 mass fraction of water */
    attribute def MassFractionOfWaterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-31 mass fraction of water
         * symbol(s): `w_(H_(2)O)`
         * application domain: generic
         * name: MassFractionOfWater (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_(H_(2)O) = u/(1+u)` where `u` is mass ratio of water to dry matter (item 5-29)
         * remarks: None.
         */
    }
    attribute massFractionOfWater: MassFractionOfWaterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-32 mass fraction of dry matter */
    attribute def MassFractionOfDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-32 mass fraction of dry matter
         * symbol(s): `w_d`
         * application domain: generic
         * name: MassFractionOfDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_d = 1 - w_(H_(2)O)` where `w_(H_(2)O)` is mass fraction of water (item 5-31)
         * remarks: None.
         */
    }
    attribute massFractionOfDryMatter: MassFractionOfDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-33 relative humidity */
    attribute def RelativeHumidityValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-33 relative humidity
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeHumidity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of partial pressure of water vapour and partial pressure at its saturation: `φ = p/p_"sat"` where `p` is partial pressure (ISO 80000-4) of vapour and `p_"sat"` is its partial pressure at saturation at the same temperature
         * remarks: Relative humidity is often referred to as RH and expressed in percent. See also remark in item 5-35.
         */
    }
    attribute relativeHumidity: RelativeHumidityValue :> scalarQuantities;

    /* ISO-80000-5 item 5-34 relative mass concentration of vapour */
    attribute def RelativeMassConcentrationOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-34 relative mass concentration of vapour
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeMassConcentrationOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass concentration of water vapour and mass concentration at its saturation: `φ = v/v_"sat"` where `v` is mass concentration of water vapour (item 5-28) and `v_"sat"` is its mass concentration of water vapour at saturation of the same temperature
         * remarks: For water vapour concentrations up to 1 kg/m^3, the relative humidity (item 5-33) is assumed to be equal to relative mass concentration of vapour. For details see Reference [8].
         */
    }
    attribute relativeMassConcentrationOfVapour: RelativeMassConcentrationOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-35 relative mass ratio of vapour */
    attribute def RelativeMassRatioOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-35 relative mass ratio of vapour
         * symbol(s): `ψ`
         * application domain: generic
         * name: RelativeMassRatioOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass ratio of water vapour to dry gas and mass ratio of water vapour to dry gas at saturation: `ψ = r/r_"sat"` where `r` is mass ratio of water vapour to dry gas (item 5-30) and `r_"sat"` is its mass ratio of water vapour to dry gas at saturation of the same temperature
         * remarks: This quantity is also used as an approximation of relative humidity (item 5-33).
         */
    }
    attribute relativeMassRatioOfVapour: RelativeMassRatioOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-36 dew-point temperature */
    attribute dewPointTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 5-36 dew-point temperature
         * symbol(s): `T_d`
         * application domain: generic
         * name: DewPointTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature at which water vapour in the air reaches saturation under isobaric conditions
         * remarks: The corresponding Celsius temperature, denoted `t_d`, is still called dew-point temperature. The unit for the corresponding Celsius temperature is degree Celsius, symbol °C.
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
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
semantic.unresolved_name 'scalarQuantities'
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
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
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
semantic.unresolved_name 'ThermodynamicTemperatureValue'
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
RegularComment,
RegularComment,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
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
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
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
KwAttribute,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ISQThermodynamics'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQBase::*')
    (comment)
    (comment)
    (comment)
    (alias_member 'TemperatureUnit' for 'ThermodynamicTemperatureUnit')
    (alias_member 'TemperatureValue' for 'ThermodynamicTemperatureValue')
    (alias_member 'temperature' for 'thermodynamicTemperature')
    (comment)
    (attribute_def 'CelsiusTemperatureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CelsiusTemperatureUnit' multiplicity))
    (attribute_usage 'celsiusTemperature' : 'CelsiusTemperatureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CelsiusTemperatureUnit' :> 'DerivedUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'LinearExpansionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'LinearExpansionCoefficientUnit' multiplicity))
    (attribute_usage 'linearExpansionCoefficient' : 'LinearExpansionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'LinearExpansionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'CubicExpansionCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CubicExpansionCoefficientUnit' multiplicity))
    (attribute_usage 'cubicExpansionCoefficient' : 'CubicExpansionCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CubicExpansionCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RelativePressureCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'RelativePressureCoefficientUnit' multiplicity))
    (attribute_usage 'relativePressureCoefficient' : 'RelativePressureCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'RelativePressureCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PressureCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PressureCoefficientUnit' multiplicity))
    (attribute_usage 'pressureCoefficient' : 'PressureCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PressureCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'IsothermalCompressibilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IsothermalCompressibilityUnit' multiplicity))
    (attribute_usage 'isothermalCompressibility' : 'IsothermalCompressibilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IsothermalCompressibilityUnit' :> 'DerivedUnit'
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
    (attribute_def 'IsentropicCompressibilityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'IsentropicCompressibilityUnit' multiplicity))
    (attribute_usage 'isentropicCompressibility' : 'IsentropicCompressibilityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'IsentropicCompressibilityUnit' :> 'DerivedUnit'
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
    (attribute_usage 'heat' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'amountOfHeat' for 'heat')
    (comment)
    (attribute_usage 'latentHeat' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_def 'HeatFlowRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'HeatFlowRateUnit' multiplicity))
    (attribute_usage 'heatFlowRate' : 'HeatFlowRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'HeatFlowRateUnit' :> 'DerivedUnit'
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
    (attribute_def 'DensityOfHeatFlowRateValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DensityOfHeatFlowRateUnit' multiplicity))
    (attribute_usage 'densityOfHeatFlowRate' : 'DensityOfHeatFlowRateValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'DensityOfHeatFlowRateUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalConductivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalConductivityUnit' multiplicity))
    (attribute_usage 'thermalConductivity' : 'ThermalConductivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalConductivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'CoefficientOfHeatTransferValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'CoefficientOfHeatTransferUnit' multiplicity))
    (attribute_usage 'coefficientOfHeatTransfer' : 'CoefficientOfHeatTransferValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'CoefficientOfHeatTransferUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SurfaceCoefficientOfHeatTransferValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SurfaceCoefficientOfHeatTransferUnit' multiplicity))
    (attribute_usage 'surfaceCoefficientOfHeatTransfer' : 'SurfaceCoefficientOfHeatTransferValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SurfaceCoefficientOfHeatTransferUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalInsulanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalInsulanceUnit' multiplicity))
    (attribute_usage 'thermalInsulance' : 'ThermalInsulanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalInsulanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (alias_member 'CoefficientOfThermalInsulanceUnit' for 'ThermalInsulanceUnit')
    (alias_member 'CoefficientOfThermalInsulanceValue' for 'ThermalInsulanceValue')
    (alias_member 'coefficientOfThermalInsulance' for 'thermalInsulance')
    (comment)
    (attribute_def 'ThermalResistanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalResistanceUnit' multiplicity))
    (attribute_usage 'thermalResistance' : 'ThermalResistanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalResistanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalConductanceValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalConductanceUnit' multiplicity))
    (attribute_usage 'thermalConductance' : 'ThermalConductanceValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalConductanceUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalDiffusivityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'ThermalDiffusivityUnit' multiplicity))
    (attribute_usage 'thermalDiffusivity' : 'ThermalDiffusivityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'ThermalDiffusivityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'HeatCapacityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'HeatCapacityUnit' multiplicity))
    (attribute_usage 'heatCapacity' : 'HeatCapacityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'HeatCapacityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificHeatCapacityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificHeatCapacityUnit' multiplicity))
    (attribute_usage 'specificHeatCapacity' : 'SpecificHeatCapacityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificHeatCapacityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificHeatCapacityAtConstantPressureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificHeatCapacityAtConstantPressureUnit' multiplicity))
    (attribute_usage 'specificHeatCapacityAtConstantPressure' : 'SpecificHeatCapacityAtConstantPressureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificHeatCapacityAtConstantPressureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificHeatCapacityAtConstantVolumeValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificHeatCapacityAtConstantVolumeUnit' multiplicity))
    (attribute_usage 'specificHeatCapacityAtConstantVolume' : 'SpecificHeatCapacityAtConstantVolumeValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificHeatCapacityAtConstantVolumeUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificHeatCapacityAtSaturatedVapourPressureValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificHeatCapacityAtSaturatedVapourPressureUnit' multiplicity))
    (attribute_usage 'specificHeatCapacityAtSaturatedVapourPressure' : 'SpecificHeatCapacityAtSaturatedVapourPressureValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificHeatCapacityAtSaturatedVapourPressureUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'RatioOfSpecificHeatCapacitiesValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'ratioOfSpecificHeatCapacities' : 'RatioOfSpecificHeatCapacitiesValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'IsentropicExponentValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'isentropicExponent' : 'IsentropicExponentValue' :> 'scalarQuantities')
    (alias_member 'isentropicExpansionFactor' for 'isentropicExponent')
    (comment)
    (attribute_def 'EntropyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EntropyUnit' multiplicity))
    (attribute_usage 'entropy' : 'EntropyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EntropyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'SpecificEntropyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificEntropyUnit' multiplicity))
    (attribute_usage 'specificEntropy' : 'SpecificEntropyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificEntropyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'EnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'EnergyUnit' multiplicity))
    (attribute_usage 'energy' : 'EnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'EnergyUnit' :> 'DerivedUnit'
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
    (attribute_usage 'internalEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'thermodynamicEnergy' for 'internalEnergy')
    (comment)
    (attribute_usage 'enthalpy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (comment)
    (attribute_usage 'helmholtzEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'helmholtzFunction' for 'helmholtzEnergy')
    (comment)
    (attribute_usage 'gibbsEnergy' : 'EnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'gibbsFunction' for 'gibbsEnergy')
    (comment)
    (attribute_def 'SpecificEnergyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificEnergyUnit' multiplicity))
    (attribute_usage 'specificEnergy' : 'SpecificEnergyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificEnergyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'specificInternalEnergy' : 'SpecificEnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'specificThermodynamicEnergy' for 'specificInternalEnergy')
    (comment)
    (attribute_def 'SpecificEnthalpyValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificEnthalpyUnit' multiplicity))
    (attribute_usage 'specificEnthalpy' : 'SpecificEnthalpyValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificEnthalpyUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_usage 'specificHelmholtzEnergy' : 'SpecificEnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'specificHelmholtzFunction' for 'specificHelmholtzEnergy')
    (comment)
    (attribute_usage 'specificGibbsEnergy' : 'SpecificEnergyValue' :> 'scalarQuantities'
      (documentation))
    (alias_member 'specificGibbsFunction' for 'specificGibbsEnergy')
    (comment)
    (attribute_def 'MassieuFunctionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassieuFunctionUnit' multiplicity))
    (attribute_usage 'massieuFunction' : 'MassieuFunctionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassieuFunctionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'PlanckFunctionValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'PlanckFunctionUnit' multiplicity))
    (attribute_usage 'planckFunction' : 'PlanckFunctionValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'PlanckFunctionUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'JouleThomsonCoefficientValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'JouleThomsonCoefficientUnit' multiplicity))
    (attribute_usage 'jouleThomsonCoefficient' : 'JouleThomsonCoefficientValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'JouleThomsonCoefficientUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'ThermalEfficiencyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'thermalEfficiency' : 'ThermalEfficiencyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MaximumThermalEfficiencyValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'maximumThermalEfficiency' : 'MaximumThermalEfficiencyValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'SpecificGasConstantValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'SpecificGasConstantUnit' multiplicity))
    (attribute_usage 'specificGasConstant' : 'SpecificGasConstantValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'SpecificGasConstantUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'thermodynamicTemperaturePF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassConcentrationOfWaterValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassConcentrationOfWaterUnit' multiplicity))
    (attribute_usage 'massConcentrationOfWater' : 'MassConcentrationOfWaterValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassConcentrationOfWaterUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassConcentrationOfWaterVapourAbsoluteHumidityValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'MassConcentrationOfWaterVapourAbsoluteHumidityUnit' multiplicity))
    (attribute_usage 'massConcentrationOfWaterVapourAbsoluteHumidity' : 'MassConcentrationOfWaterVapourAbsoluteHumidityValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_def 'MassConcentrationOfWaterVapourAbsoluteHumidityUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (comment)
    (attribute_def 'MassRatioOfWaterToDryMatterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massRatioOfWaterToDryMatter' : 'MassRatioOfWaterToDryMatterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MassRatioOfWaterVapourToDryGasValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massRatioOfWaterVapourToDryGas' : 'MassRatioOfWaterVapourToDryGasValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MassFractionOfWaterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massFractionOfWater' : 'MassFractionOfWaterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'MassFractionOfDryMatterValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'massFractionOfDryMatter' : 'MassFractionOfDryMatterValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RelativeHumidityValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeHumidity' : 'RelativeHumidityValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RelativeMassConcentrationOfVapourValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeMassConcentrationOfVapour' : 'RelativeMassConcentrationOfVapourValue' :> 'scalarQuantities')
    (comment)
    (attribute_def 'RelativeMassRatioOfVapourValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'relativeMassRatioOfVapour' : 'RelativeMassRatioOfVapourValue' :> 'scalarQuantities')
    (comment)
    (attribute_usage 'dewPointTemperature' : 'ThermodynamicTemperatureValue' :> 'scalarQuantities'
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package ISQThermodynamics {
    doc /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-5:2019 "Thermodynamics"
     * see also https://www.iso.org/standard/64976.html
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

    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    /* See package ISQBase for the declarations of ThermodynamicTemperatureValue and ThermodynamicTemperatureUnit */

    alias TemperatureUnit for ThermodynamicTemperatureUnit;
    alias TemperatureValue for ThermodynamicTemperatureValue;
    alias temperature for thermodynamicTemperature;

    /* ISO-80000-5 item 5-2 Celsius temperature */
    attribute def CelsiusTemperatureValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-2 Celsius temperature
         * symbol(s): `t`, `θ`
         * application domain: generic
         * name: CelsiusTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): °C
         * tensor order: 0
         * definition: temperature difference from the thermodynamic temperature of the ice point is called the Celsius temperature t, which is defined by the quantity equation: `t = T - T_0` where `T` is thermodynamic temperature (item 5-1) and `T_0 = 273,15 K`
         * remarks: The unit degree Celsius is a special name for the kelvin for use in stating values of Celsius temperature. The unit degree Celsius is by definition equal in magnitude to the kelvin. A difference or interval of temperature may be expressed in kelvin or in degrees Celsius. The thermodynamic temperature `T_0` is 0,01 K below the thermodynamic temperature of the triple point of water. The symbol °C for the degree Celsius shall be preceded by a space (see ISO 80000-1). Prefixes are not allowed in combination with the unit °C.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CelsiusTemperatureUnit [1];
    }

    attribute celsiusTemperature : CelsiusTemperatureValue :> scalarQuantities [*] nonunique;

    attribute def CelsiusTemperatureUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }

    /* ISO-80000-5 item 5-3.1 linear expansion coefficient */
    attribute def LinearExpansionCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-3.1 linear expansion coefficient
         * symbol(s): `α_l`
         * application domain: generic
         * name: LinearExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of length with temperature: `α_l = 1/l * (dl)/(dT)` where l is length (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearExpansionCoefficientUnit [1];
    }

    attribute linearExpansionCoefficient : LinearExpansionCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def LinearExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }

    /* ISO-80000-5 item 5-3.2 cubic expansion coefficient */
    attribute def CubicExpansionCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-3.2 cubic expansion coefficient
         * symbol(s): `α_V`, `γ`
         * application domain: generic
         * name: CubicExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of volume with temperature: `α_V = 1/V * (dV)/(dT)` where `V` is volume (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Also called volumetric expansion coefficient. The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CubicExpansionCoefficientUnit [1];
    }

    attribute cubicExpansionCoefficient : CubicExpansionCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def CubicExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }

    /* ISO-80000-5 item 5-3.3 relative pressure coefficient */
    attribute def RelativePressureCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-3.3 relative pressure coefficient
         * symbol(s): `α_p`
         * application domain: generic
         * name: RelativePressureCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of pressure with temperature at constant volume: `α_p = 1/p * ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RelativePressureCoefficientUnit [1];
    }

    attribute relativePressureCoefficient : RelativePressureCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def RelativePressureCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }

    /* ISO-80000-5 item 5-4 pressure coefficient */
    attribute def PressureCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-4 pressure coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PressureCoefficient
         * quantity dimension: L^-1*M^1*T^-2*Θ^-1
         * measurement unit(s): Pa/K, kg*m^-1*s^-2*K^-1
         * tensor order: 0
         * definition: change of pressure with temperature at constant volume: `β = ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PressureCoefficientUnit [1];
    }

    attribute pressureCoefficient : PressureCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def PressureCoefficientUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-5.1 isothermal compressibility */
    attribute def IsothermalCompressibilityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-5.1 isothermal compressibility
         * symbol(s): `ϰ_T`
         * application domain: generic
         * name: IsothermalCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant temperature: `ϰ_T = -1/V * ((partial V)/(partial p))_T` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IsothermalCompressibilityUnit [1];
    }

    attribute isothermalCompressibility : IsothermalCompressibilityValue :> scalarQuantities [*] nonunique;

    attribute def IsothermalCompressibilityUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-5.2 isentropic compressibility */
    attribute def IsentropicCompressibilityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-5.2 isentropic compressibility
         * symbol(s): `ϰ_S`
         * application domain: generic
         * name: IsentropicCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant entropy: `ϰ_S = -1/V * ((partial V)/(partial p))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IsentropicCompressibilityUnit [1];
    }

    attribute isentropicCompressibility : IsentropicCompressibilityValue :> scalarQuantities [*] nonunique;

    attribute def IsentropicCompressibilityUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-6.1 heat, amount of heat */
    attribute heat : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-6.1 heat, amount of heat
         * symbol(s): `Q`
         * application domain: generic
         * name: Heat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between the increase in the internal energy (item 5-20.2) of a system and the work (ISO 80000-4) done on the system, provided that the amounts of substances within the system are not changed
         * remarks: The heat transferred in an isothermal phase transformation should be expressed as the change in the appropriate state functions, e.g. `T ΔS`, where `T` is thermodynamic temperature (item 5-1) and `S` is entropy (item 5-18), or `ΔH`, where `H` is enthalpy (item 5-20.3). NOTE A supply of heat can correspond to an increase in thermodynamic temperature or to other effects, such as phase change or chemical processes; see item 5-6.2.
         */
    }

    alias amountOfHeat for heat;

    /* ISO-80000-5 item 5-6.2 latent heat */
    attribute latentHeat : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-6.2 latent heat
         * symbol(s): `Q`
         * application domain: generic
         * name: LatentHeat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy released or absorbed by a system during a constant-temperature process
         * remarks: Examples of latent heat are latent heat of fusion (melting) and latent heat of vaporization (boiling).
         */
    }

    /* ISO-80000-5 item 5-7 heat flow rate */
    attribute def HeatFlowRateValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-7 heat flow rate
         * symbol(s): `dot(Q)`
         * application domain: generic
         * name: HeatFlowRate
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J/s, kg*m^2*s^-3
         * tensor order: 0
         * definition: time rate at which heat (item 5-6.1) crosses a given surface
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HeatFlowRateUnit [1];
    }

    attribute heatFlowRate : HeatFlowRateValue :> scalarQuantities [*] nonunique;

    attribute def HeatFlowRateUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-8 density of heat flow rate */
    attribute def DensityOfHeatFlowRateValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-8 density of heat flow rate
         * symbol(s): `q`, `φ`
         * application domain: generic
         * name: DensityOfHeatFlowRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: quotient of heat flow rate and area: `q = dot Q / A` where `dot Q` is heat flow rate (item 5-7) and A is area (ISO 80000-3) of a given surface
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DensityOfHeatFlowRateUnit [1];
    }

    attribute densityOfHeatFlowRate : DensityOfHeatFlowRateValue :> scalarQuantities [*] nonunique;

    attribute def DensityOfHeatFlowRateUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-9 thermal conductivity */
    attribute def ThermalConductivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-9 thermal conductivity
         * symbol(s): `λ_l`, `(ϰ)`
         * application domain: generic
         * name: ThermalConductivity
         * quantity dimension: L^1*M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m*K), kg*m*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature gradient that has the same direction as the heat flow
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalConductivityUnit [1];
    }

    attribute thermalConductivity : ThermalConductivityValue :> scalarQuantities [*] nonunique;

    attribute def ThermalConductivityUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-10.1 coefficient of heat transfer */
    attribute def CoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-10.1 coefficient of heat transfer
         * symbol(s): `K`, `(k)`
         * application domain: generic
         * name: CoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature (item 5-1) difference
         * remarks: In building technology, the coefficient of heat transfer is often called thermal transmittance, with the symbol U (no longer recommended). See remark to item 5-13.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CoefficientOfHeatTransferUnit [1];
    }

    attribute coefficientOfHeatTransfer : CoefficientOfHeatTransferValue :> scalarQuantities [*] nonunique;

    attribute def CoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-10.2 surface coefficient of heat transfer */
    attribute def SurfaceCoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-10.2 surface coefficient of heat transfer
         * symbol(s): `h`, `(α)`
         * application domain: generic
         * name: SurfaceCoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate and the difference of the temperature at the surface and a reference temperature: `h = q / (T_s - T_r)` where q is density of heat flow rate (item 5-8), `T_s` is the thermodynamic temperature (item 5-1) at the surface, and `T_r` is a reference thermodynamic temperature characterizing the adjacent surroundings
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceCoefficientOfHeatTransferUnit [1];
    }

    attribute surfaceCoefficientOfHeatTransfer : SurfaceCoefficientOfHeatTransferValue :> scalarQuantities [*] nonunique;

    attribute def SurfaceCoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-11 thermal insulance, coefficient of thermal insulance */
    attribute def ThermalInsulanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-11 thermal insulance, coefficient of thermal insulance
         * symbol(s): `M`
         * application domain: generic
         * name: ThermalInsulance
         * quantity dimension: M^-1*T^3*Θ^1
         * measurement unit(s): m^2*K/W, kg^-1*s^3*K
         * tensor order: 0
         * definition: inverse of coefficient of heat transfer `K`: `M = 1/K` where `K` is coefficient of heat transfer (item 5-10.1)
         * remarks: In building technology, this quantity is often called thermal resistance, with the symbol R.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalInsulanceUnit [1];
    }

    attribute thermalInsulance : ThermalInsulanceValue :> scalarQuantities [*] nonunique;

    attribute def ThermalInsulanceUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = 3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    alias CoefficientOfThermalInsulanceUnit for ThermalInsulanceUnit;
    alias CoefficientOfThermalInsulanceValue for ThermalInsulanceValue;
    alias coefficientOfThermalInsulance for thermalInsulance;

    /* ISO-80000-5 item 5-12 thermal resistance */
    attribute def ThermalResistanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-12 thermal resistance
         * symbol(s): `R`
         * application domain: generic
         * name: ThermalResistance
         * quantity dimension: L^-2*M^-1*T^3*Θ^1
         * measurement unit(s): K/W, kg^-1*m^-2*s^3*K
         * tensor order: 0
         * definition: quotient of thermodynamic temperature (item 5-1) difference and heat flow rate (item 5-7)
         * remarks: See remark to item 5-11.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalResistanceUnit [1];
    }

    attribute thermalResistance : ThermalResistanceValue :> scalarQuantities [*] nonunique;

    attribute def ThermalResistanceUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-13 thermal conductance */
    attribute def ThermalConductanceValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-13 thermal conductance
         * symbol(s): `G`, `(H)`
         * application domain: generic
         * name: ThermalConductance
         * quantity dimension: L^2*M^1*T^-3*Θ^-1
         * measurement unit(s): W/K, kg*m^2*s^-3*K^-1
         * tensor order: 0
         * definition: inverse of thermal resistance `R`: `G = 1/R` where `R` is thermal resistance (item 5-12)
         * remarks: See remark to item 5-11. This quantity is also called heat transfer coefficient. See item 5-10.1.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalConductanceUnit [1];
    }

    attribute thermalConductance : ThermalConductanceValue :> scalarQuantities [*] nonunique;

    attribute def ThermalConductanceUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-14 thermal diffusivity */
    attribute def ThermalDiffusivityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-14 thermal diffusivity
         * symbol(s): `a`
         * application domain: generic
         * name: ThermalDiffusivity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of thermal conductivity and the product of mass density and specific heat capacity: `a = λ / (ρ C_p)` where `λ` is thermal conductivity (item 5-9), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (item 5-16.2)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalDiffusivityUnit [1];
    }

    attribute thermalDiffusivity : ThermalDiffusivityValue :> scalarQuantities [*] nonunique;

    attribute def ThermalDiffusivityUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-15 heat capacity */
    attribute def HeatCapacityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-15 heat capacity
         * symbol(s): `C`
         * application domain: generic
         * name: HeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: derivative of added heat with respect to thermodynamic temperature of a system: `C = (dQ)/(dT)` where `Q` is amount of heat (item 5-6.1) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Heat capacity is not completely defined unless specified as seen in items 5-16.2, 5-16.3 and 5-16.4.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HeatCapacityUnit [1];
    }

    attribute heatCapacity : HeatCapacityValue :> scalarQuantities [*] nonunique;

    attribute def HeatCapacityUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-16.1 specific heat capacity */
    attribute def SpecificHeatCapacityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-16.1 specific heat capacity
         * symbol(s): `c`
         * application domain: generic
         * name: SpecificHeatCapacity
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of heat capacity and mass: `c = C/m` where `C` is heat capacity (item 5-15) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantities related to the amount of substance, see ISO 80000-9.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityUnit [1];
    }

    attribute specificHeatCapacity : SpecificHeatCapacityValue :> scalarQuantities [*] nonunique;

    attribute def SpecificHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-16.2 specific heat capacity at constant pressure */
    attribute def SpecificHeatCapacityAtConstantPressureValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-16.2 specific heat capacity at constant pressure
         * symbol(s): `c_p`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant pressure (ISO 80000-4)
         * remarks: Also called specific isobaric heat capacity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtConstantPressureUnit [1];
    }

    attribute specificHeatCapacityAtConstantPressure : SpecificHeatCapacityAtConstantPressureValue :> scalarQuantities [*] nonunique;

    attribute def SpecificHeatCapacityAtConstantPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-16.3 specific heat capacity at constant volume */
    attribute def SpecificHeatCapacityAtConstantVolumeValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-16.3 specific heat capacity at constant volume
         * symbol(s): `c_V`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantVolume
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant volume (ISO 80000-3)
         * remarks: Also called specific isochoric heat capacity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtConstantVolumeUnit [1];
    }

    attribute specificHeatCapacityAtConstantVolume : SpecificHeatCapacityAtConstantVolumeValue :> scalarQuantities [*] nonunique;

    attribute def SpecificHeatCapacityAtConstantVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-16.4 specific heat capacity at saturated vapour pressure */
    attribute def SpecificHeatCapacityAtSaturatedVapourPressureValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-16.4 specific heat capacity at saturated vapour pressure
         * symbol(s): `c_"sat"`
         * application domain: generic
         * name: SpecificHeatCapacityAtSaturatedVapourPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at saturated vapour pressure (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtSaturatedVapourPressureUnit [1];
    }

    attribute specificHeatCapacityAtSaturatedVapourPressure : SpecificHeatCapacityAtSaturatedVapourPressureValue :> scalarQuantities [*] nonunique;

    attribute def SpecificHeatCapacityAtSaturatedVapourPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-17.1 ratio of specific heat capacities */
    attribute def RatioOfSpecificHeatCapacitiesValue :> DimensionOneValue {
        doc /*
         * source: item 5-17.1 ratio of specific heat capacities
         * symbol(s): `γ`
         * application domain: generic
         * name: RatioOfSpecificHeatCapacities (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of specific heat capacity at constant pressure and specific heat capacity at constant volume: `γ = c_p/c_V` where `c_p` is specific heat capacity at constant pressure (item 5-16.2) and `c_V` is specific heat capacity at constant volume (item 5-16.3)
         * remarks: This quantity can also be expressed by `γ = C_p/C_V` where `C_p` is heat capacity at constant pressure and `C_V` is heat capacity at constant volume.
         */
    }
    attribute ratioOfSpecificHeatCapacities : RatioOfSpecificHeatCapacitiesValue :> scalarQuantities;

    /* ISO-80000-5 item 5-17.2 isentropic exponent, isentropic expansion factor */
    attribute def IsentropicExponentValue :> DimensionOneValue {
        doc /*
         * source: item 5-17.2 isentropic exponent, isentropic expansion factor
         * symbol(s): `ϰ`
         * application domain: generic
         * name: IsentropicExponent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: the negative of relative pressure change, divided by relative volume change, at constant entropy: `ϰ = -V/p * ((partial p)/(partial V))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: For an ideal gas, `ϰ` is equal to `γ` (item 5-17.1).
         */
    }
    attribute isentropicExponent : IsentropicExponentValue :> scalarQuantities;

    alias isentropicExpansionFactor for isentropicExponent;

    /* ISO-80000-5 item 5-18 entropy */
    attribute def EntropyValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-18 entropy
         * symbol(s): `S`
         * application domain: generic
         * name: Entropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: natural logarithm of number of equally probable microscopic configurations in a macroscopic system, multiplied by the Boltzmann constant: `S = k lnW` where `W` is number of configurations and `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EntropyUnit [1];
    }

    attribute entropy : EntropyValue :> scalarQuantities [*] nonunique;

    attribute def EntropyUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-19 specific entropy */
    attribute def SpecificEntropyValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-19 specific entropy
         * symbol(s): `s`
         * application domain: generic
         * name: SpecificEntropy
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of entropy and mass: `s = S/m` where `S` is entropy (item 5-18) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantity related to amount of substance, see ISO 80000-9.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEntropyUnit [1];
    }

    attribute specificEntropy : SpecificEntropyValue :> scalarQuantities [*] nonunique;

    attribute def SpecificEntropyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-20.1 energy */
    attribute def EnergyValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-20.1 energy
         * symbol(s): `E`
         * application domain: thermodynamics
         * name: Energy
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: ability of a system to do work (ISO 80000-4)
         * remarks: Energy exists in different forms that are mutually transformable into each other, either totally or partially. In contrast to internal energy (item 5-20.2), energy is not a state function.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyUnit [1];
    }

    attribute energy : EnergyValue :> scalarQuantities [*] nonunique;

    attribute def EnergyUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-20.2 internal energy, thermodynamic energy */
    attribute internalEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-20.2 internal energy, thermodynamic energy
         * symbol(s): `U`
         * application domain: generic
         * name: InternalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy of a system whose change is given by the amount of the heat (item 5-6.1) transferred to the system and the work (ISO 80000-4) done on the system, provided that the system is closed and no chemical reactions occur
         * remarks: In thermodynamic text books, usually the formula `ΔU = Q + W` is used. Note that the zero of the energy is undefined.
         */
    }

    alias thermodynamicEnergy for internalEnergy;

    /* ISO-80000-5 item 5-20.3 enthalpy */
    attribute enthalpy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-20.3 enthalpy
         * symbol(s): `H`
         * application domain: generic
         * name: Enthalpy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of internal energy of the system and the product of pressure and volume of the system: `H = U + p*V` where U is internal energy (item 5-20.2), `p` is pressure (ISO 80000-4), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
    }

    /* ISO-80000-5 item 5-20.4 Helmholtz energy, Helmholtz function */
    attribute helmholtzEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-20.4 Helmholtz energy, Helmholtz function
         * symbol(s): `A`, `F`
         * application domain: generic
         * name: HelmholtzEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of internal energy of the system and the product of thermodynamic temperature and entropy of the system: `A = U - TS` where `U` is internal energy (item 5-20.2), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias helmholtzFunction for helmholtzEnergy;

    /* ISO-80000-5 item 5-20.5 Gibbs energy, Gibbs function */
    attribute gibbsEnergy : EnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-20.5 Gibbs energy, Gibbs function
         * symbol(s): `G`
         * application domain: generic
         * name: GibbsEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of the enthalpy and the product of thermodynamic temperature and entropy of the system: `G = H - T*S` where H is enthalpy (item 5-20.3), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias gibbsFunction for gibbsEnergy;

    /* ISO-80000-5 item 5-21.1 specific energy */
    attribute def SpecificEnergyValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-21.1 specific energy
         * symbol(s): `e`
         * application domain: generic
         * name: SpecificEnergy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of energy and mass: `e = E/m` where `E` is energy (item 5-20.1) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEnergyUnit [1];
    }

    attribute specificEnergy : SpecificEnergyValue :> scalarQuantities [*] nonunique;

    attribute def SpecificEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-5 item 5-21.2 specific internal energy, specific thermodynamic energy */
    attribute specificInternalEnergy : SpecificEnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-21.2 specific internal energy, specific thermodynamic energy
         * symbol(s): `u`
         * application domain: generic
         * name: SpecificInternalEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of internal energy and mass: `u = U/m` where `U` is internal energy (item 5-20.2) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
    }

    alias specificThermodynamicEnergy for specificInternalEnergy;

    /* ISO-80000-5 item 5-21.3 specific enthalpy */
    attribute def SpecificEnthalpyValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-21.3 specific enthalpy
         * symbol(s): `h`
         * application domain: generic
         * name: SpecificEnthalpy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of enthalpy and mass: `h = H/m` where `H` is enthalpy (item 5-20.3) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEnthalpyUnit [1];
    }

    attribute specificEnthalpy : SpecificEnthalpyValue :> scalarQuantities [*] nonunique;

    attribute def SpecificEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }

    /* ISO-80000-5 item 5-21.4 specific Helmholtz energy, specific Helmholtz function */
    attribute specificHelmholtzEnergy : SpecificEnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-21.4 specific Helmholtz energy, specific Helmholtz function
         * symbol(s): `a`, `f`
         * application domain: generic
         * name: SpecificHelmholtzEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Helmholtz energy and mass: `a = A/m` where A is Helmholtz energy (item 5-20.4) and m is mass (ISO 80000-4)
         * remarks: The name specific Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias specificHelmholtzFunction for specificHelmholtzEnergy;

    /* ISO-80000-5 item 5-21.5 specific Gibbs energy, specific Gibbs function */
    attribute specificGibbsEnergy : SpecificEnergyValue :> scalarQuantities {
        doc /*
         * source: item 5-21.5 specific Gibbs energy, specific Gibbs function
         * symbol(s): `g`
         * application domain: generic
         * name: SpecificGibbsEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Gibbs energy and mass: `g = G/m` where `G` is Gibbs energy (item 5-20.5) and `m` is mass (ISO 80000-4)
         * remarks: The name specific Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias specificGibbsFunction for specificGibbsEnergy;

    /* ISO-80000-5 item 5-22 Massieu function */
    attribute def MassieuFunctionValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-22 Massieu function
         * symbol(s): `J`
         * application domain: generic
         * name: MassieuFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Helmholtz energy and temperature: `J = -A/T` where `A` is Helmholtz energy (item 5-20.4) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassieuFunctionUnit [1];
    }

    attribute massieuFunction : MassieuFunctionValue :> scalarQuantities [*] nonunique;

    attribute def MassieuFunctionUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-23 Planck function */
    attribute def PlanckFunctionValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-23 Planck function
         * symbol(s): `Y`
         * application domain: generic
         * name: PlanckFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Gibbs energy and temperature: `Y = -G/T` where G is Gibbs energy (item 5-20.5) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PlanckFunctionUnit [1];
    }

    attribute planckFunction : PlanckFunctionValue :> scalarQuantities [*] nonunique;

    attribute def PlanckFunctionUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-24 Joule-Thomson coefficient */
    attribute def JouleThomsonCoefficientValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-24 Joule-Thomson coefficient
         * symbol(s): `μ_"JT"`
         * application domain: generic
         * name: JouleThomsonCoefficient
         * quantity dimension: L^1*M^-1*T^2*Θ^1
         * measurement unit(s): K/Pa, kg^-1*m*s^2*K
         * tensor order: 0
         * definition: change of thermodynamic temperature with respect to pressure in a Joule-Thomson process at constant enthalpy: `μ_(JT) = ((partial T)/(partial p))_H` where `T` is thermodynamic temperature (item 5-1), `p` is pressure (ISO 80000-4) and H is enthalpy (item 5-20.3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : JouleThomsonCoefficientUnit [1];
    }

    attribute jouleThomsonCoefficient : JouleThomsonCoefficientValue :> scalarQuantities [*] nonunique;

    attribute def JouleThomsonCoefficientUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-25.1 thermal efficiency */
    attribute def ThermalEfficiencyValue :> DimensionOneValue {
        doc /*
         * source: item 5-25.1 thermal efficiency
         * symbol(s): `η`
         * application domain: thermodynamics
         * name: ThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of work (ISO 80000-4) delivered by a heat engine and supplied heat: `η = W/Q` where `W` is work (ISO 80000-4) and `Q` is heat (item 5-6.1)
         * remarks: None.
         */
    }
    attribute thermalEfficiency : ThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-25.2 maximum thermal efficiency */
    attribute def MaximumThermalEfficiencyValue :> DimensionOneValue {
        doc /*
         * source: item 5-25.2 maximum thermal efficiency
         * symbol(s): `η_"max"`
         * application domain: generic
         * name: MaximumThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency determined by the quotient of the temperatures of the hot source and the cold sink: `η_max = 1 - T_c/T_h` where `T_c` is the thermodynamic temperature (item 5-1) of the cold sink and `T_h` is the thermodynamic temperature (item 5-1) of the hot source
         * remarks: An ideal heat engine operating according to the Carnot process is delivering the maximum efficiency.
         */
    }
    attribute maximumThermalEfficiency : MaximumThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-26 specific gas constant */
    attribute def SpecificGasConstantValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-26 specific gas constant
         * symbol(s): `R_s`
         * application domain: generic
         * name: SpecificGasConstant
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the Boltzmann constant `k` (ISO 80000-1) and the mass `m` (ISO 80000-4) of the gas particle
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificGasConstantUnit [1];
    }

    attribute specificGasConstant : SpecificGasConstantValue :> scalarQuantities [*] nonunique;

    attribute def SpecificGasConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor [1] {
            :>> quantity = isq.'Θ';
            :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }

    /* ISO-80000-5 item 5-27 mass concentration of water */
    attribute def MassConcentrationOfWaterValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-27 mass concentration of water
         * symbol(s): `w`
         * application domain: generic
         * name: MassConcentrationOfWater
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water and a specified volume: `w = m/V` where `m` is mass (ISO 80000-4) of water, irrespective of the form of aggregation state, and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water at saturation is denoted `w_"sat"`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationOfWaterUnit [1];
    }

    attribute massConcentrationOfWater : MassConcentrationOfWaterValue :> scalarQuantities [*] nonunique;

    attribute def MassConcentrationOfWaterUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-28 mass concentration of water vapour absolute humidity */
    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityValue :> ScalarQuantityValue {
        doc /*
         * source: item 5-28 mass concentration of water vapour absolute humidity
         * symbol(s): `v`
         * application domain: generic
         * name: MassConcentrationOfWaterVapourAbsoluteHumidity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water vapour and a specified volume: `v = m/V` where m is mass (ISO 80000-4) of water vapour and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water vapour at saturation is denoted `v_"sat"`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationOfWaterVapourAbsoluteHumidityUnit [1];
    }

    attribute massConcentrationOfWaterVapourAbsoluteHumidity : MassConcentrationOfWaterVapourAbsoluteHumidityValue :> scalarQuantities [*] nonunique;

    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityUnit :> DerivedUnit {
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

    /* ISO-80000-5 item 5-29 mass ratio of water to dry matter */
    attribute def MassRatioOfWaterToDryMatterValue :> DimensionOneValue {
        doc /*
         * source: item 5-29 mass ratio of water to dry matter
         * symbol(s): `u`
         * application domain: generic
         * name: MassRatioOfWaterToDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water and mass of dry matter: `u = m/m_d` where `m` is mass (ISO 80000-4) of water and `m_d` is mass of dry matter
         * remarks: Mass ratio of water to dry matter at saturation is denoted `u_"sat"`.
         */
    }
    attribute massRatioOfWaterToDryMatter : MassRatioOfWaterToDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-30 mass ratio of water vapour to dry gas */
    attribute def MassRatioOfWaterVapourToDryGasValue :> DimensionOneValue {
        doc /*
         * source: item 5-30 mass ratio of water vapour to dry gas
         * symbol(s): `r`, `(x)`
         * application domain: generic
         * name: MassRatioOfWaterVapourToDryGas (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water vapour and mass of dry gas: `r = m/m_d` where `m` is mass (ISO 80000-4) of water vapour and `m_d` is mass of dry gas
         * remarks: Mass ratio of water vapour to dry gas at saturation is denoted `r_"sat"`. Mass ratio of water vapour to dry gas is also called mixing ratio.
         */
    }
    attribute massRatioOfWaterVapourToDryGas : MassRatioOfWaterVapourToDryGasValue :> scalarQuantities;

    /* ISO-80000-5 item 5-31 mass fraction of water */
    attribute def MassFractionOfWaterValue :> DimensionOneValue {
        doc /*
         * source: item 5-31 mass fraction of water
         * symbol(s): `w_(H_(2)O)`
         * application domain: generic
         * name: MassFractionOfWater (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_(H_(2)O) = u/(1+u)` where `u` is mass ratio of water to dry matter (item 5-29)
         * remarks: None.
         */
    }
    attribute massFractionOfWater : MassFractionOfWaterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-32 mass fraction of dry matter */
    attribute def MassFractionOfDryMatterValue :> DimensionOneValue {
        doc /*
         * source: item 5-32 mass fraction of dry matter
         * symbol(s): `w_d`
         * application domain: generic
         * name: MassFractionOfDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_d = 1 - w_(H_(2)O)` where `w_(H_(2)O)` is mass fraction of water (item 5-31)
         * remarks: None.
         */
    }
    attribute massFractionOfDryMatter : MassFractionOfDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-33 relative humidity */
    attribute def RelativeHumidityValue :> DimensionOneValue {
        doc /*
         * source: item 5-33 relative humidity
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeHumidity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of partial pressure of water vapour and partial pressure at its saturation: `φ = p/p_"sat"` where `p` is partial pressure (ISO 80000-4) of vapour and `p_"sat"` is its partial pressure at saturation at the same temperature
         * remarks: Relative humidity is often referred to as RH and expressed in percent. See also remark in item 5-35.
         */
    }
    attribute relativeHumidity : RelativeHumidityValue :> scalarQuantities;

    /* ISO-80000-5 item 5-34 relative mass concentration of vapour */
    attribute def RelativeMassConcentrationOfVapourValue :> DimensionOneValue {
        doc /*
         * source: item 5-34 relative mass concentration of vapour
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeMassConcentrationOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass concentration of water vapour and mass concentration at its saturation: `φ = v/v_"sat"` where `v` is mass concentration of water vapour (item 5-28) and `v_"sat"` is its mass concentration of water vapour at saturation of the same temperature
         * remarks: For water vapour concentrations up to 1 kg/m^3, the relative humidity (item 5-33) is assumed to be equal to relative mass concentration of vapour. For details see Reference [8].
         */
    }
    attribute relativeMassConcentrationOfVapour : RelativeMassConcentrationOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-35 relative mass ratio of vapour */
    attribute def RelativeMassRatioOfVapourValue :> DimensionOneValue {
        doc /*
         * source: item 5-35 relative mass ratio of vapour
         * symbol(s): `ψ`
         * application domain: generic
         * name: RelativeMassRatioOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass ratio of water vapour to dry gas and mass ratio of water vapour to dry gas at saturation: `ψ = r/r_"sat"` where `r` is mass ratio of water vapour to dry gas (item 5-30) and `r_"sat"` is its mass ratio of water vapour to dry gas at saturation of the same temperature
         * remarks: This quantity is also used as an approximation of relative humidity (item 5-33).
         */
    }
    attribute relativeMassRatioOfVapour : RelativeMassRatioOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-36 dew-point temperature */
    attribute dewPointTemperature : ThermodynamicTemperatureValue :> scalarQuantities {
        doc /*
         * source: item 5-36 dew-point temperature
         * symbol(s): `T_d`
         * application domain: generic
         * name: DewPointTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature at which water vapour in the air reaches saturation under isobaric conditions
         * remarks: The corresponding Celsius temperature, denoted `t_d`, is still called dew-point temperature. The unit for the corresponding Celsius temperature is degree Celsius, symbol °C.
         */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ISQThermodynamics'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQBase'[unresolved])
      (alias_member 'TemperatureUnit' -> 'ThermodynamicTemperatureUnit'[unresolved])
      (alias_member 'TemperatureValue' -> 'ThermodynamicTemperatureValue'[unresolved])
      (alias_member 'temperature' -> 'thermodynamicTemperature'[unresolved])
      (attribute_def 'CelsiusTemperatureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::CelsiusTemperatureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'celsiusTemperature' : 'ISQThermodynamics::CelsiusTemperatureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'CelsiusTemperatureUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'LinearExpansionCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::LinearExpansionCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'linearExpansionCoefficient' : 'ISQThermodynamics::LinearExpansionCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'LinearExpansionCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CubicExpansionCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::CubicExpansionCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'cubicExpansionCoefficient' : 'ISQThermodynamics::CubicExpansionCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'CubicExpansionCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'RelativePressureCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::RelativePressureCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'relativePressureCoefficient' : 'ISQThermodynamics::RelativePressureCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'RelativePressureCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'PressureCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::PressureCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'pressureCoefficient' : 'ISQThermodynamics::PressureCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'PressureCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'IsothermalCompressibilityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::IsothermalCompressibilityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'isothermalCompressibility' : 'ISQThermodynamics::IsothermalCompressibilityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'IsothermalCompressibilityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
      (attribute_def 'IsentropicCompressibilityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::IsentropicCompressibilityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'isentropicCompressibility' : 'ISQThermodynamics::IsentropicCompressibilityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'IsentropicCompressibilityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
      (attribute_usage 'heat' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'amountOfHeat' -> 'ISQThermodynamics::heat'[attribute_usage])
      (attribute_usage 'latentHeat' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_def 'HeatFlowRateValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::HeatFlowRateUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'heatFlowRate' : 'ISQThermodynamics::HeatFlowRateValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'HeatFlowRateUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
      (attribute_def 'DensityOfHeatFlowRateValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::DensityOfHeatFlowRateUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'densityOfHeatFlowRate' : 'ISQThermodynamics::DensityOfHeatFlowRateValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'DensityOfHeatFlowRateUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
      (attribute_def 'ThermalConductivityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::ThermalConductivityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thermalConductivity' : 'ISQThermodynamics::ThermalConductivityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThermalConductivityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'CoefficientOfHeatTransferValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::CoefficientOfHeatTransferUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'coefficientOfHeatTransfer' : 'ISQThermodynamics::CoefficientOfHeatTransferValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'CoefficientOfHeatTransferUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SurfaceCoefficientOfHeatTransferValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'surfaceCoefficientOfHeatTransfer' : 'ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SurfaceCoefficientOfHeatTransferUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'ThermalInsulanceValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::ThermalInsulanceUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thermalInsulance' : 'ISQThermodynamics::ThermalInsulanceValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThermalInsulanceUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (alias_member 'CoefficientOfThermalInsulanceUnit' -> 'ISQThermodynamics::ThermalInsulanceUnit'[attribute_def])
      (alias_member 'CoefficientOfThermalInsulanceValue' -> 'ISQThermodynamics::ThermalInsulanceValue'[attribute_def])
      (alias_member 'coefficientOfThermalInsulance' -> 'ISQThermodynamics::thermalInsulance'[attribute_usage])
      (attribute_def 'ThermalResistanceValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::ThermalResistanceUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thermalResistance' : 'ISQThermodynamics::ThermalResistanceValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThermalResistanceUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'ThermalConductanceValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::ThermalConductanceUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thermalConductance' : 'ISQThermodynamics::ThermalConductanceValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThermalConductanceUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'ThermalDiffusivityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::ThermalDiffusivityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'thermalDiffusivity' : 'ISQThermodynamics::ThermalDiffusivityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'ThermalDiffusivityUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_def 'HeatCapacityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::HeatCapacityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'heatCapacity' : 'ISQThermodynamics::HeatCapacityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'HeatCapacityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SpecificHeatCapacityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificHeatCapacityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificHeatCapacity' : 'ISQThermodynamics::SpecificHeatCapacityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificHeatCapacityUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SpecificHeatCapacityAtConstantPressureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificHeatCapacityAtConstantPressure' : 'ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificHeatCapacityAtConstantPressureUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SpecificHeatCapacityAtConstantVolumeValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificHeatCapacityAtConstantVolume' : 'ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificHeatCapacityAtConstantVolumeUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SpecificHeatCapacityAtSaturatedVapourPressureValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificHeatCapacityAtSaturatedVapourPressure' : 'ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificHeatCapacityAtSaturatedVapourPressureUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'RatioOfSpecificHeatCapacitiesValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'ratioOfSpecificHeatCapacities' : 'ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'IsentropicExponentValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'isentropicExponent' : 'ISQThermodynamics::IsentropicExponentValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (alias_member 'isentropicExpansionFactor' -> 'ISQThermodynamics::isentropicExponent'[attribute_usage])
      (attribute_def 'EntropyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::EntropyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'entropy' : 'ISQThermodynamics::EntropyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'EntropyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'SpecificEntropyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificEntropyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificEntropy' : 'ISQThermodynamics::SpecificEntropyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificEntropyUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'EnergyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::EnergyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'energy' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'EnergyUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
      (attribute_usage 'internalEnergy' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'thermodynamicEnergy' -> 'ISQThermodynamics::internalEnergy'[attribute_usage])
      (attribute_usage 'enthalpy' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (attribute_usage 'helmholtzEnergy' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'helmholtzFunction' -> 'ISQThermodynamics::helmholtzEnergy'[attribute_usage])
      (attribute_usage 'gibbsEnergy' : 'ISQThermodynamics::EnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'gibbsFunction' -> 'ISQThermodynamics::gibbsEnergy'[attribute_usage])
      (attribute_def 'SpecificEnergyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificEnergyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificEnergy' : 'ISQThermodynamics::SpecificEnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificEnergyUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_usage 'specificInternalEnergy' : 'ISQThermodynamics::SpecificEnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'specificThermodynamicEnergy' -> 'ISQThermodynamics::specificInternalEnergy'[attribute_usage])
      (attribute_def 'SpecificEnthalpyValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificEnthalpyUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificEnthalpy' : 'ISQThermodynamics::SpecificEnthalpyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificEnthalpyUnit' :> 'DerivedUnit'[unresolved]
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
      (attribute_usage 'specificHelmholtzEnergy' : 'ISQThermodynamics::SpecificEnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'specificHelmholtzFunction' -> 'ISQThermodynamics::specificHelmholtzEnergy'[attribute_usage])
      (attribute_usage 'specificGibbsEnergy' : 'ISQThermodynamics::SpecificEnergyValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (documentation))
      (alias_member 'specificGibbsFunction' -> 'ISQThermodynamics::specificGibbsEnergy'[attribute_usage])
      (attribute_def 'MassieuFunctionValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::MassieuFunctionUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'massieuFunction' : 'ISQThermodynamics::MassieuFunctionValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'MassieuFunctionUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'PlanckFunctionValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::PlanckFunctionUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'planckFunction' : 'ISQThermodynamics::PlanckFunctionValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'PlanckFunctionUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'JouleThomsonCoefficientValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::JouleThomsonCoefficientUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'jouleThomsonCoefficient' : 'ISQThermodynamics::JouleThomsonCoefficientValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'JouleThomsonCoefficientUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'ThermalEfficiencyValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'thermalEfficiency' : 'ISQThermodynamics::ThermalEfficiencyValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'MaximumThermalEfficiencyValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'maximumThermalEfficiency' : 'ISQThermodynamics::MaximumThermalEfficiencyValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'SpecificGasConstantValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::SpecificGasConstantUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'specificGasConstant' : 'ISQThermodynamics::SpecificGasConstantValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'SpecificGasConstantUnit' :> 'DerivedUnit'[unresolved]
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
        (attribute_usage composite 'thermodynamicTemperaturePF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'MassConcentrationOfWaterValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::MassConcentrationOfWaterUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'massConcentrationOfWater' : 'ISQThermodynamics::MassConcentrationOfWaterValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'MassConcentrationOfWaterUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'MassConcentrationOfWaterVapourAbsoluteHumidityValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit'[attribute_def]
          (multiplicity_range [1])))
      (attribute_usage 'massConcentrationOfWaterVapourAbsoluteHumidity' : 'ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_def 'MassConcentrationOfWaterVapourAbsoluteHumidityUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'MassRatioOfWaterToDryMatterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'massRatioOfWaterToDryMatter' : 'ISQThermodynamics::MassRatioOfWaterToDryMatterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'MassRatioOfWaterVapourToDryGasValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'massRatioOfWaterVapourToDryGas' : 'ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'MassFractionOfWaterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'massFractionOfWater' : 'ISQThermodynamics::MassFractionOfWaterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'MassFractionOfDryMatterValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'massFractionOfDryMatter' : 'ISQThermodynamics::MassFractionOfDryMatterValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'RelativeHumidityValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'relativeHumidity' : 'ISQThermodynamics::RelativeHumidityValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'RelativeMassConcentrationOfVapourValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'relativeMassConcentrationOfVapour' : 'ISQThermodynamics::RelativeMassConcentrationOfVapourValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_def 'RelativeMassRatioOfVapourValue' :> 'DimensionOneValue'[unresolved]
        (documentation))
      (attribute_usage 'relativeMassRatioOfVapour' : 'ISQThermodynamics::RelativeMassRatioOfVapourValue'[attribute_def] :> 'scalarQuantities'[unresolved])
      (attribute_usage 'dewPointTemperature' : 'ThermodynamicTemperatureValue'[unresolved] :> 'scalarQuantities'[unresolved]
        (documentation)))))
~~~
