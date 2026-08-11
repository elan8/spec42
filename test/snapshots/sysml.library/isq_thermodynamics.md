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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ISQThermodynamics"))) (name "ISQThermodynamics") (declared-name "ISQThermodynamics")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQThermodynamics::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQThermodynamics::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQThermodynamics::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (name "CelsiusTemperatureUnit") (declared-name "CelsiusTemperatureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (name "CelsiusTemperatureValue") (declared-name "CelsiusTemperatureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (name "CoefficientOfHeatTransferUnit") (declared-name "CoefficientOfHeatTransferUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (name "CoefficientOfHeatTransferValue") (declared-name "CoefficientOfHeatTransferValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfThermalInsulanceUnit"))) (name "CoefficientOfThermalInsulanceUnit") (declared-name "CoefficientOfThermalInsulanceUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfThermalInsulanceValue"))) (name "CoefficientOfThermalInsulanceValue") (declared-name "CoefficientOfThermalInsulanceValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (name "CubicExpansionCoefficientUnit") (declared-name "CubicExpansionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (name "CubicExpansionCoefficientValue") (declared-name "CubicExpansionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (name "DensityOfHeatFlowRateUnit") (declared-name "DensityOfHeatFlowRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (name "DensityOfHeatFlowRateValue") (declared-name "DensityOfHeatFlowRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (name "EnergyUnit") (declared-name "EnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (name "EnergyValue") (declared-name "EnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (name "EntropyUnit") (declared-name "EntropyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (name "EntropyValue") (declared-name "EntropyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (name "HeatCapacityUnit") (declared-name "HeatCapacityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (name "HeatCapacityValue") (declared-name "HeatCapacityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (name "HeatFlowRateUnit") (declared-name "HeatFlowRateUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (name "HeatFlowRateValue") (declared-name "HeatFlowRateValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (name "IsentropicCompressibilityUnit") (declared-name "IsentropicCompressibilityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (name "IsentropicCompressibilityValue") (declared-name "IsentropicCompressibilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (name "IsentropicExponentValue") (declared-name "IsentropicExponentValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (name "IsothermalCompressibilityUnit") (declared-name "IsothermalCompressibilityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (name "IsothermalCompressibilityValue") (declared-name "IsothermalCompressibilityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (name "JouleThomsonCoefficientUnit") (declared-name "JouleThomsonCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (name "JouleThomsonCoefficientValue") (declared-name "JouleThomsonCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (name "LinearExpansionCoefficientUnit") (declared-name "LinearExpansionCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (name "LinearExpansionCoefficientValue") (declared-name "LinearExpansionCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (name "MassConcentrationOfWaterUnit") (declared-name "MassConcentrationOfWaterUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (name "MassConcentrationOfWaterValue") (declared-name "MassConcentrationOfWaterValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (declared-name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (declared-name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (name "MassFractionOfDryMatterValue") (declared-name "MassFractionOfDryMatterValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (name "MassFractionOfWaterValue") (declared-name "MassFractionOfWaterValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (name "MassRatioOfWaterToDryMatterValue") (declared-name "MassRatioOfWaterToDryMatterValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (name "MassRatioOfWaterVapourToDryGasValue") (declared-name "MassRatioOfWaterVapourToDryGasValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (name "MassieuFunctionUnit") (declared-name "MassieuFunctionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (name "MassieuFunctionValue") (declared-name "MassieuFunctionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (name "MaximumThermalEfficiencyValue") (declared-name "MaximumThermalEfficiencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (name "PlanckFunctionUnit") (declared-name "PlanckFunctionUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (name "PlanckFunctionValue") (declared-name "PlanckFunctionValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (name "PressureCoefficientUnit") (declared-name "PressureCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (name "PressureCoefficientValue") (declared-name "PressureCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (name "RatioOfSpecificHeatCapacitiesValue") (declared-name "RatioOfSpecificHeatCapacitiesValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (name "RelativeHumidityValue") (declared-name "RelativeHumidityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (name "RelativeMassConcentrationOfVapourValue") (declared-name "RelativeMassConcentrationOfVapourValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (name "RelativeMassRatioOfVapourValue") (declared-name "RelativeMassRatioOfVapourValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (name "RelativePressureCoefficientUnit") (declared-name "RelativePressureCoefficientUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (name "RelativePressureCoefficientValue") (declared-name "RelativePressureCoefficientValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (name "SpecificEnergyUnit") (declared-name "SpecificEnergyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (name "SpecificEnergyValue") (declared-name "SpecificEnergyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (name "SpecificEnthalpyUnit") (declared-name "SpecificEnthalpyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (name "SpecificEnthalpyValue") (declared-name "SpecificEnthalpyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (name "SpecificEntropyUnit") (declared-name "SpecificEntropyUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (name "SpecificEntropyValue") (declared-name "SpecificEntropyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (name "SpecificGasConstantUnit") (declared-name "SpecificGasConstantUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (name "SpecificGasConstantValue") (declared-name "SpecificGasConstantValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (name "SpecificHeatCapacityAtConstantPressureUnit") (declared-name "SpecificHeatCapacityAtConstantPressureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (name "SpecificHeatCapacityAtConstantPressureValue") (declared-name "SpecificHeatCapacityAtConstantPressureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (name "SpecificHeatCapacityAtConstantVolumeUnit") (declared-name "SpecificHeatCapacityAtConstantVolumeUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (name "SpecificHeatCapacityAtConstantVolumeValue") (declared-name "SpecificHeatCapacityAtConstantVolumeValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (declared-name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (declared-name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (name "SpecificHeatCapacityUnit") (declared-name "SpecificHeatCapacityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (name "SpecificHeatCapacityValue") (declared-name "SpecificHeatCapacityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (name "SurfaceCoefficientOfHeatTransferUnit") (declared-name "SurfaceCoefficientOfHeatTransferUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (name "SurfaceCoefficientOfHeatTransferValue") (declared-name "SurfaceCoefficientOfHeatTransferValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::TemperatureUnit"))) (name "TemperatureUnit") (declared-name "TemperatureUnit"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::TemperatureValue"))) (name "TemperatureValue") (declared-name "TemperatureValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (name "ThermalConductanceUnit") (declared-name "ThermalConductanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (name "ThermalConductanceValue") (declared-name "ThermalConductanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (name "ThermalConductivityUnit") (declared-name "ThermalConductivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (name "ThermalConductivityValue") (declared-name "ThermalConductivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (name "ThermalDiffusivityUnit") (declared-name "ThermalDiffusivityUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (name "ThermalDiffusivityValue") (declared-name "ThermalDiffusivityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (name "ThermalEfficiencyValue") (declared-name "ThermalEfficiencyValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (name "ThermalInsulanceUnit") (declared-name "ThermalInsulanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (name "ThermalInsulanceValue") (declared-name "ThermalInsulanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (name "ThermalResistanceUnit") (declared-name "ThermalResistanceUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::thermodynamicTemperaturePF"))) (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (name "ThermalResistanceValue") (declared-name "ThermalResistanceValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::_documentation"))) (name ""))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::amountOfHeat"))) (name "amountOfHeat") (declared-name "amountOfHeat"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (name "celsiusTemperature") (declared-name "celsiusTemperature") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (name "coefficientOfHeatTransfer") (declared-name "coefficientOfHeatTransfer") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfThermalInsulance"))) (name "coefficientOfThermalInsulance") (declared-name "coefficientOfThermalInsulance"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (name "cubicExpansionCoefficient") (declared-name "cubicExpansionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (name "densityOfHeatFlowRate") (declared-name "densityOfHeatFlowRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))) (name "dewPointTemperature") (declared-name "dewPointTemperature") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (name "energy") (declared-name "energy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (name "enthalpy") (declared-name "enthalpy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (name "entropy") (declared-name "entropy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (name "gibbsEnergy") (declared-name "gibbsEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsFunction"))) (name "gibbsFunction") (declared-name "gibbsFunction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (name "heat") (declared-name "heat") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::heat::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::heat")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (name "heatCapacity") (declared-name "heatCapacity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (name "heatFlowRate") (declared-name "heatFlowRate") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (name "helmholtzEnergy") (declared-name "helmholtzEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzFunction"))) (name "helmholtzFunction") (declared-name "helmholtzFunction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (name "internalEnergy") (declared-name "internalEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (name "isentropicCompressibility") (declared-name "isentropicCompressibility") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExpansionFactor"))) (name "isentropicExpansionFactor") (declared-name "isentropicExpansionFactor"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (name "isentropicExponent") (declared-name "isentropicExponent") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (name "isothermalCompressibility") (declared-name "isothermalCompressibility") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (name "jouleThomsonCoefficient") (declared-name "jouleThomsonCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (name "latentHeat") (declared-name "latentHeat") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (name "linearExpansionCoefficient") (declared-name "linearExpansionCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (name "massConcentrationOfWater") (declared-name "massConcentrationOfWater") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (name "massConcentrationOfWaterVapourAbsoluteHumidity") (declared-name "massConcentrationOfWaterVapourAbsoluteHumidity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (name "massFractionOfDryMatter") (declared-name "massFractionOfDryMatter") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (name "massFractionOfWater") (declared-name "massFractionOfWater") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (name "massRatioOfWaterToDryMatter") (declared-name "massRatioOfWaterToDryMatter") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (name "massRatioOfWaterVapourToDryGas") (declared-name "massRatioOfWaterVapourToDryGas") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (name "massieuFunction") (declared-name "massieuFunction") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (name "maximumThermalEfficiency") (declared-name "maximumThermalEfficiency") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (name "planckFunction") (declared-name "planckFunction") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (name "pressureCoefficient") (declared-name "pressureCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (name "ratioOfSpecificHeatCapacities") (declared-name "ratioOfSpecificHeatCapacities") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (name "relativeHumidity") (declared-name "relativeHumidity") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (name "relativeMassConcentrationOfVapour") (declared-name "relativeMassConcentrationOfVapour") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (name "relativeMassRatioOfVapour") (declared-name "relativeMassRatioOfVapour") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (name "relativePressureCoefficient") (declared-name "relativePressureCoefficient") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (name "specificEnergy") (declared-name "specificEnergy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (name "specificEnthalpy") (declared-name "specificEnthalpy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (name "specificEntropy") (declared-name "specificEntropy") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (name "specificGasConstant") (declared-name "specificGasConstant") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (name "specificGibbsEnergy") (declared-name "specificGibbsEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsFunction"))) (name "specificGibbsFunction") (declared-name "specificGibbsFunction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (name "specificHeatCapacity") (declared-name "specificHeatCapacity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (name "specificHeatCapacityAtConstantPressure") (declared-name "specificHeatCapacityAtConstantPressure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (name "specificHeatCapacityAtConstantVolume") (declared-name "specificHeatCapacityAtConstantVolume") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (name "specificHeatCapacityAtSaturatedVapourPressure") (declared-name "specificHeatCapacityAtSaturatedVapourPressure") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (name "specificHelmholtzEnergy") (declared-name "specificHelmholtzEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzFunction"))) (name "specificHelmholtzFunction") (declared-name "specificHelmholtzFunction"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (name "specificInternalEnergy") (declared-name "specificInternalEnergy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::specificThermodynamicEnergy"))) (name "specificThermodynamicEnergy") (declared-name "specificThermodynamicEnergy"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (name "surfaceCoefficientOfHeatTransfer") (declared-name "surfaceCoefficientOfHeatTransfer") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::temperature"))) (name "temperature") (declared-name "temperature"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (name "thermalConductance") (declared-name "thermalConductance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (name "thermalConductivity") (declared-name "thermalConductivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (name "thermalDiffusivity") (declared-name "thermalDiffusivity") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (name "thermalEfficiency") (declared-name "thermalEfficiency") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (name "thermalInsulance") (declared-name "thermalInsulance") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (name "thermalResistance") (declared-name "thermalResistance") (declared (properties (ordered false) (unique false))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "ISQThermodynamics::thermodynamicEnergy"))) (name "thermodynamicEnergy") (declared-name "thermodynamicEnergy"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::heat::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy::_documentation"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (to (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::durationPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::lengthPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::massPF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::thermodynamicTemperaturePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/isq_thermodynamics.md"
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
        (range (start 30 4) (end 30 1261))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 8) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 8) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 44 8) (end 44 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 4) (end 49 286))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 8) (end 50 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 51 8) (end 51 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 4) (end 55 756))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 68 8) (end 68 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 8) (end 68 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 69 8) (end 69 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 4) (end 74 295))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 8) (end 75 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 76 8) (end 76 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 806))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 93 8) (end 93 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 8) (end 93 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 94 8) (end 94 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 4) (end 99 294))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 8) (end 100 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 101 8) (end 101 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 833))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 118 8) (end 118 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 8) (end 118 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 119 8) (end 119 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 124 4) (end 124 296))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 8) (end 125 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 126 8) (end 126 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 4) (end 130 737))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 143 8) (end 143 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 8) (end 143 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 144 8) (end 144 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 4) (end 149 631))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 8) (end 150 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 8) (end 151 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 8) (end 152 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 8) (end 153 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 154 8) (end 154 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 4) (end 158 855))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 171 8) (end 171 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 171 8) (end 171 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 172 8) (end 172 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 177 4) (end 177 483))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 8) (end 178 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 179 8) (end 179 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 8) (end 180 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 181 8) (end 181 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 4) (end 185 834))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 198 8) (end 198 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 8) (end 198 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 199 8) (end 199 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 204 4) (end 204 483))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 205 8) (end 205 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 8) (end 206 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 8) (end 207 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 208 8) (end 208 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 246 4) (end 246 559))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 259 8) (end 259 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 259 8) (end 259 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 260 8) (end 260 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 265 4) (end 265 470))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 266 8) (end 266 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 267 8) (end 267 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 268 8) (end 268 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 269 8) (end 269 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 4) (end 273 671))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 286 8) (end 286 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 8) (end 286 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 287 8) (end 287 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 292 4) (end 292 366))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 293 8) (end 293 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 294 8) (end 294 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 295 8) (end 295 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 4) (end 299 674))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 312 8) (end 312 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 312 8) (end 312 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 313 8) (end 313 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 318 4) (end 318 630))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 319 8) (end 319 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 320 8) (end 320 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 8) (end 321 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 322 8) (end 322 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 323 8) (end 323 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 821))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 340 8) (end 340 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 340 8) (end 340 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 341 8) (end 341 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 346 4) (end 346 523))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 347 8) (end 347 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 348 8) (end 348 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 349 8) (end 349 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 350 8) (end 350 120))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 354 4) (end 354 941))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 367 8) (end 367 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 367 8) (end 367 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 368 8) (end 368 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 4) (end 373 530))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 374 8) (end 374 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 375 8) (end 375 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 376 8) (end 376 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 377 8) (end 377 120))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 381 4) (end 381 743))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 394 8) (end 394 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 394 8) (end 394 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 395 8) (end 395 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 400 4) (end 400 513))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 401 8) (end 401 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 402 8) (end 402 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 403 8) (end 403 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 404 8) (end 404 120))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 412 4) (end 412 630))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 425 8) (end 425 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 425 8) (end 425 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 426 8) (end 426 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 431 4) (end 431 628))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 432 8) (end 432 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 433 8) (end 433 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 434 8) (end 434 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 435 8) (end 435 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 436 8) (end 436 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 4) (end 440 713))
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
        (range (start 454 8) (end 454 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 459 4) (end 459 629))
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
        (range (start 461 8) (end 461 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 462 8) (end 462 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 463 8) (end 463 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 464 8) (end 464 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 4) (end 468 769))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 481 8) (end 481 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 481 8) (end 481 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 482 8) (end 482 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 487 4) (end 487 367))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 488 8) (end 488 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 489 8) (end 489 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 490 8) (end 490 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 494 4) (end 494 781))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 507 8) (end 507 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 507 8) (end 507 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 508 8) (end 508 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 513 4) (end 513 623))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 514 8) (end 514 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 515 8) (end 515 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 516 8) (end 516 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 517 8) (end 517 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 518 8) (end 518 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 522 4) (end 522 729))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 535 8) (end 535 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 535 8) (end 535 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 536 8) (end 536 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 541 4) (end 541 522))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 542 8) (end 542 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 543 8) (end 543 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 544 8) (end 544 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 545 8) (end 545 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 549 4) (end 549 722))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 562 8) (end 562 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 562 8) (end 562 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 563 8) (end 563 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 568 4) (end 568 540))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 569 8) (end 569 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 570 8) (end 570 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 571 8) (end 571 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 572 8) (end 572 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 576 4) (end 576 713))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 589 8) (end 589 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 589 8) (end 589 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 590 8) (end 590 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 595 4) (end 595 538))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 596 8) (end 596 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 597 8) (end 597 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 598 8) (end 598 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 599 8) (end 599 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 603 4) (end 603 724))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 616 8) (end 616 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 616 8) (end 616 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 617 8) (end 617 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 622 4) (end 622 547))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 623 8) (end 623 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 624 8) (end 624 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 625 8) (end 625 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 626 8) (end 626 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 630 4) (end 630 877))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 647 4) (end 647 751))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 666 4) (end 666 716))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 679 8) (end 679 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 679 8) (end 679 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 680 8) (end 680 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 685 4) (end 685 618))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 686 8) (end 686 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 687 8) (end 687 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 688 8) (end 688 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 689 8) (end 689 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 690 8) (end 690 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 694 4) (end 694 688))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 707 8) (end 707 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 707 8) (end 707 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 708 8) (end 708 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 713 4) (end 713 517))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 714 8) (end 714 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 715 8) (end 715 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 716 8) (end 716 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 717 8) (end 717 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 721 4) (end 721 700))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 734 8) (end 734 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 734 8) (end 734 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 735 8) (end 735 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 740 4) (end 740 464))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 741 8) (end 741 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 742 8) (end 742 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 743 8) (end 743 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 744 8) (end 744 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 818 4) (end 818 597))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 831 8) (end 831 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 831 8) (end 831 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 832 8) (end 832 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 837 4) (end 837 363))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 838 8) (end 838 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 839 8) (end 839 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 840 8) (end 840 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 862 4) (end 862 609))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 875 8) (end 875 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 875 8) (end 875 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 876 8) (end 876 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 881 4) (end 881 365))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 882 8) (end 882 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 883 8) (end 883 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 884 8) (end 884 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 924 4) (end 924 678))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 937 8) (end 937 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 937 8) (end 937 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 938 8) (end 938 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 943 4) (end 943 626))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 944 8) (end 944 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 945 8) (end 945 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 946 8) (end 946 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 947 8) (end 947 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 948 8) (end 948 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 952 4) (end 952 664))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 965 8) (end 965 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 965 8) (end 965 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 966 8) (end 966 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 971 4) (end 971 625))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 972 8) (end 972 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 973 8) (end 973 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 974 8) (end 974 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 975 8) (end 975 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 976 8) (end 976 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 980 4) (end 980 812))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 993 8) (end 993 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 993 8) (end 993 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 994 8) (end 994 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 999 4) (end 999 633))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1000 8) (end 1000 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1001 8) (end 1001 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1002 8) (end 1002 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1003 8) (end 1003 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1004 8) (end 1004 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1008 4) (end 1008 589))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1025 4) (end 1025 819))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1042 4) (end 1042 634))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1055 8) (end 1055 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1055 8) (end 1055 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1056 8) (end 1056 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1061 4) (end 1061 521))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1062 8) (end 1062 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1063 8) (end 1063 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1064 8) (end 1064 124))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1065 8) (end 1065 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1069 4) (end 1069 766))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1082 8) (end 1082 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1082 8) (end 1082 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1083 8) (end 1083 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1088 4) (end 1088 365))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1089 8) (end 1089 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1090 8) (end 1090 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1091 8) (end 1091 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1095 4) (end 1095 828))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1108 8) (end 1108 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1108 8) (end 1108 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1109 8) (end 1109 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1114 4) (end 1114 387))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1115 8) (end 1115 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1116 8) (end 1116 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 1117 8) (end 1117 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1121 4) (end 1121 664))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1138 4) (end 1138 760))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1155 4) (end 1155 548))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1172 4) (end 1172 552))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1189 4) (end 1189 755))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1206 4) (end 1206 914))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1223 4) (end 1223 817))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1240 4) (end 1240 726))
      )
    )
  )
)
~~~
