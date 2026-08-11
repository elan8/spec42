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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_thermodynamics.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 4) (end 55 756))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 806))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 4) (end 105 833))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 4) (end 130 737))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 4) (end 158 855))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 4) (end 185 834))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 246 4) (end 246 559))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 4) (end 273 671))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 4) (end 299 674))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 821))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 354 4) (end 354 941))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 381 4) (end 381 743))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 412 4) (end 412 630))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 4) (end 440 713))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 4) (end 468 769))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 494 4) (end 494 781))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 522 4) (end 522 729))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 549 4) (end 549 722))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 576 4) (end 576 713))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 603 4) (end 603 724))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 694 4) (end 694 688))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 721 4) (end 721 700))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 818 4) (end 818 597))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 862 4) (end 862 609))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 924 4) (end 924 678))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 952 4) (end 952 664))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 980 4) (end 980 812))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1069 4) (end 1069 766))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1095 4) (end 1095 828))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "22e3b33bf2462bfe63352f3fa582cb81d9b7c95547e1f91c5b881b1b7ad0ab23") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics"))) (kind "package") (name "ISQThermodynamics") (declared-name "ISQThermodynamics") (range (start (line 0) (character 0)) (end (line 0) (character 64627))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 33))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 4)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 19)) (end (line 16) (character 40))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 17) (character 4)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQBase::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 17) (character 19)) (end (line 17) (character 26))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (kind "attribute def") (name "CelsiusTemperatureUnit") (declared-name "CelsiusTemperatureUnit") (range (start (line 49) (character 4)) (end (line 49) (character 286))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 51) (character 8)) (end (line 51) (character 98))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 51) (character 22)) (end (line 51) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 50) (character 8)) (end (line 50) (character 123))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (kind "attribute def") (name "CelsiusTemperatureValue") (declared-name "CelsiusTemperatureValue") (range (start (line 30) (character 4)) (end (line 30) (character 1261))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 30) (character 4)) (end (line 30) (character 1261))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 44) (character 8)) (end (line 44) (character 54))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CelsiusTemperatureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 43) (character 8)) (end (line 43) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 43) (character 22)) (end (line 43) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (kind "attribute def") (name "CoefficientOfHeatTransferUnit") (declared-name "CoefficientOfHeatTransferUnit") (range (start (line 346) (character 4)) (end (line 346) (character 523))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 348) (character 8)) (end (line 348) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 347) (character 8)) (end (line 347) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 350) (character 8)) (end (line 350) (character 120))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 350) (character 22)) (end (line 350) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 349) (character 8)) (end (line 349) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (kind "attribute def") (name "CoefficientOfHeatTransferValue") (declared-name "CoefficientOfHeatTransferValue") (range (start (line 327) (character 4)) (end (line 327) (character 821))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 327) (character 4)) (end (line 327) (character 821))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 341) (character 8)) (end (line 341) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)) (redefinition (reference "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 340) (character 8)) (end (line 340) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 340) (character 22)) (end (line 340) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfThermalInsulanceUnit"))) (kind "alias") (name "CoefficientOfThermalInsulanceUnit") (declared-name "CoefficientOfThermalInsulanceUnit") (range (start (line 407) (character 4)) (end (line 407) (character 69))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfThermalInsulanceValue"))) (kind "alias") (name "CoefficientOfThermalInsulanceValue") (declared-name "CoefficientOfThermalInsulanceValue") (range (start (line 408) (character 4)) (end (line 408) (character 71))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (kind "attribute def") (name "CubicExpansionCoefficientUnit") (declared-name "CubicExpansionCoefficientUnit") (range (start (line 99) (character 4)) (end (line 99) (character 294))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 101) (character 8)) (end (line 101) (character 98))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 101) (character 22)) (end (line 101) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 100) (character 8)) (end (line 100) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (kind "attribute def") (name "CubicExpansionCoefficientValue") (declared-name "CubicExpansionCoefficientValue") (range (start (line 80) (character 4)) (end (line 80) (character 806))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 80) (character 4)) (end (line 80) (character 806))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 94) (character 8)) (end (line 94) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "CubicExpansionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 94) (character 22)) (end (line 94) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 93) (character 8)) (end (line 93) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 93) (character 22)) (end (line 93) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (kind "attribute def") (name "DensityOfHeatFlowRateUnit") (declared-name "DensityOfHeatFlowRateUnit") (range (start (line 292) (character 4)) (end (line 292) (character 366))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 294) (character 8)) (end (line 294) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 293) (character 8)) (end (line 293) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 295) (character 8)) (end (line 295) (character 92))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 295) (character 22)) (end (line 295) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (kind "attribute def") (name "DensityOfHeatFlowRateValue") (declared-name "DensityOfHeatFlowRateValue") (range (start (line 273) (character 4)) (end (line 273) (character 671))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::_documentation"))) (kind "documentation") (name "") (range (start (line 273) (character 4)) (end (line 273) (character 671))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 287) (character 8)) (end (line 287) (character 57))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)) (redefinition (reference "mRef") (range (start (line 287) (character 22)) (end (line 287) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 286) (character 8)) (end (line 286) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 286) (character 22)) (end (line 286) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (kind "attribute def") (name "EnergyUnit") (declared-name "EnergyUnit") (range (start (line 740) (character 4)) (end (line 740) (character 464))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 743) (character 8)) (end (line 743) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 741) (character 8)) (end (line 741) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 742) (character 8)) (end (line 742) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 744) (character 8)) (end (line 744) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 744) (character 22)) (end (line 744) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (kind "attribute def") (name "EnergyValue") (declared-name "EnergyValue") (range (start (line 721) (character 4)) (end (line 721) (character 700))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 721) (character 4)) (end (line 721) (character 700))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 735) (character 8)) (end (line 735) (character 42))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 735) (character 22)) (end (line 735) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 734) (character 8)) (end (line 734) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 734) (character 22)) (end (line 734) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (kind "attribute def") (name "EntropyUnit") (declared-name "EntropyUnit") (range (start (line 685) (character 4)) (end (line 685) (character 618))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 688) (character 8)) (end (line 688) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 686) (character 8)) (end (line 686) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 687) (character 8)) (end (line 687) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 690) (character 8)) (end (line 690) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 690) (character 22)) (end (line 690) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 689) (character 8)) (end (line 689) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (kind "attribute def") (name "EntropyValue") (declared-name "EntropyValue") (range (start (line 666) (character 4)) (end (line 666) (character 716))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 666) (character 4)) (end (line 666) (character 716))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 680) (character 8)) (end (line 680) (character 43))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "EntropyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 680) (character 22)) (end (line 680) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 679) (character 8)) (end (line 679) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 679) (character 22)) (end (line 679) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (kind "attribute def") (name "HeatCapacityUnit") (declared-name "HeatCapacityUnit") (range (start (line 513) (character 4)) (end (line 513) (character 623))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 516) (character 8)) (end (line 516) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 514) (character 8)) (end (line 514) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 515) (character 8)) (end (line 515) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 518) (character 8)) (end (line 518) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 518) (character 22)) (end (line 518) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 517) (character 8)) (end (line 517) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (kind "attribute def") (name "HeatCapacityValue") (declared-name "HeatCapacityValue") (range (start (line 494) (character 4)) (end (line 494) (character 781))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 494) (character 4)) (end (line 494) (character 781))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 508) (character 8)) (end (line 508) (character 48))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "HeatCapacityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 508) (character 22)) (end (line 508) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 507) (character 8)) (end (line 507) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 507) (character 22)) (end (line 507) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (kind "attribute def") (name "HeatFlowRateUnit") (declared-name "HeatFlowRateUnit") (range (start (line 265) (character 4)) (end (line 265) (character 470))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 268) (character 8)) (end (line 268) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 266) (character 8)) (end (line 266) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 267) (character 8)) (end (line 267) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 269) (character 8)) (end (line 269) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 269) (character 22)) (end (line 269) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (kind "attribute def") (name "HeatFlowRateValue") (declared-name "HeatFlowRateValue") (range (start (line 246) (character 4)) (end (line 246) (character 559))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::_documentation"))) (kind "documentation") (name "") (range (start (line 246) (character 4)) (end (line 246) (character 559))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 260) (character 8)) (end (line 260) (character 48))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "HeatFlowRateUnit") (range none)) (redefinition (reference "mRef") (range (start (line 260) (character 22)) (end (line 260) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 259) (character 8)) (end (line 259) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 259) (character 22)) (end (line 259) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (kind "attribute def") (name "IsentropicCompressibilityUnit") (declared-name "IsentropicCompressibilityUnit") (range (start (line 204) (character 4)) (end (line 204) (character 483))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 207) (character 8)) (end (line 207) (character 104))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 205) (character 8)) (end (line 205) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 206) (character 8)) (end (line 206) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 208) (character 8)) (end (line 208) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 208) (character 22)) (end (line 208) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (kind "attribute def") (name "IsentropicCompressibilityValue") (declared-name "IsentropicCompressibilityValue") (range (start (line 185) (character 4)) (end (line 185) (character 834))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 185) (character 4)) (end (line 185) (character 834))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 199) (character 8)) (end (line 199) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IsentropicCompressibilityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 199) (character 22)) (end (line 199) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 198) (character 8)) (end (line 198) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 198) (character 22)) (end (line 198) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (kind "attribute def") (name "IsentropicExponentValue") (declared-name "IsentropicExponentValue") (range (start (line 647) (character 4)) (end (line 647) (character 751))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue::_documentation"))) (kind "documentation") (name "") (range (start (line 647) (character 4)) (end (line 647) (character 751))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (kind "attribute def") (name "IsothermalCompressibilityUnit") (declared-name "IsothermalCompressibilityUnit") (range (start (line 177) (character 4)) (end (line 177) (character 483))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 180) (character 8)) (end (line 180) (character 104))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 178) (character 8)) (end (line 178) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 179) (character 8)) (end (line 179) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 181) (character 8)) (end (line 181) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 181) (character 22)) (end (line 181) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (kind "attribute def") (name "IsothermalCompressibilityValue") (declared-name "IsothermalCompressibilityValue") (range (start (line 158) (character 4)) (end (line 158) (character 855))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 158) (character 4)) (end (line 158) (character 855))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 172) (character 8)) (end (line 172) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "IsothermalCompressibilityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 172) (character 22)) (end (line 172) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 171) (character 8)) (end (line 171) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 171) (character 22)) (end (line 171) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (kind "attribute def") (name "JouleThomsonCoefficientUnit") (declared-name "JouleThomsonCoefficientUnit") (range (start (line 999) (character 4)) (end (line 999) (character 633))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1002) (character 8)) (end (line 1002) (character 104))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1000) (character 8)) (end (line 1000) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1001) (character 8)) (end (line 1001) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1004) (character 8)) (end (line 1004) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1004) (character 22)) (end (line 1004) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 1003) (character 8)) (end (line 1003) (character 123))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (kind "attribute def") (name "JouleThomsonCoefficientValue") (declared-name "JouleThomsonCoefficientValue") (range (start (line 980) (character 4)) (end (line 980) (character 812))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 980) (character 4)) (end (line 980) (character 812))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 994) (character 8)) (end (line 994) (character 59))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "JouleThomsonCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 994) (character 22)) (end (line 994) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 993) (character 8)) (end (line 993) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 993) (character 22)) (end (line 993) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (kind "attribute def") (name "LinearExpansionCoefficientUnit") (declared-name "LinearExpansionCoefficientUnit") (range (start (line 74) (character 4)) (end (line 74) (character 295))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 76) (character 8)) (end (line 76) (character 98))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 76) (character 22)) (end (line 76) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 75) (character 8)) (end (line 75) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (kind "attribute def") (name "LinearExpansionCoefficientValue") (declared-name "LinearExpansionCoefficientValue") (range (start (line 55) (character 4)) (end (line 55) (character 756))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 55) (character 4)) (end (line 55) (character 756))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 69) (character 8)) (end (line 69) (character 62))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "LinearExpansionCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 69) (character 22)) (end (line 69) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 68) (character 8)) (end (line 68) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 68) (character 22)) (end (line 68) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (kind "attribute def") (name "MassConcentrationOfWaterUnit") (declared-name "MassConcentrationOfWaterUnit") (range (start (line 1088) (character 4)) (end (line 1088) (character 365))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1089) (character 8)) (end (line 1089) (character 103))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1090) (character 8)) (end (line 1090) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1091) (character 8)) (end (line 1091) (character 90))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1091) (character 22)) (end (line 1091) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (kind "attribute def") (name "MassConcentrationOfWaterValue") (declared-name "MassConcentrationOfWaterValue") (range (start (line 1069) (character 4)) (end (line 1069) (character 766))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1069) (character 4)) (end (line 1069) (character 766))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1083) (character 8)) (end (line 1083) (character 60))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConcentrationOfWaterUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1083) (character 22)) (end (line 1083) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1082) (character 8)) (end (line 1082) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1082) (character 22)) (end (line 1082) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (kind "attribute def") (name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (declared-name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (range (start (line 1114) (character 4)) (end (line 1114) (character 387))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1115) (character 8)) (end (line 1115) (character 103))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 1116) (character 8)) (end (line 1116) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1117) (character 8)) (end (line 1117) (character 90))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1117) (character 22)) (end (line 1117) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (kind "attribute def") (name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (declared-name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (range (start (line 1095) (character 4)) (end (line 1095) (character 828))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1095) (character 4)) (end (line 1095) (character 828))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1109) (character 8)) (end (line 1109) (character 82))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1109) (character 22)) (end (line 1109) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1108) (character 8)) (end (line 1108) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1108) (character 22)) (end (line 1108) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (kind "attribute def") (name "MassFractionOfDryMatterValue") (declared-name "MassFractionOfDryMatterValue") (range (start (line 1172) (character 4)) (end (line 1172) (character 552))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1172) (character 4)) (end (line 1172) (character 552))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (kind "attribute def") (name "MassFractionOfWaterValue") (declared-name "MassFractionOfWaterValue") (range (start (line 1155) (character 4)) (end (line 1155) (character 548))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1155) (character 4)) (end (line 1155) (character 548))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (kind "attribute def") (name "MassRatioOfWaterToDryMatterValue") (declared-name "MassRatioOfWaterToDryMatterValue") (range (start (line 1121) (character 4)) (end (line 1121) (character 664))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1121) (character 4)) (end (line 1121) (character 664))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (kind "attribute def") (name "MassRatioOfWaterVapourToDryGasValue") (declared-name "MassRatioOfWaterVapourToDryGasValue") (range (start (line 1138) (character 4)) (end (line 1138) (character 760))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1138) (character 4)) (end (line 1138) (character 760))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (kind "attribute def") (name "MassieuFunctionUnit") (declared-name "MassieuFunctionUnit") (range (start (line 943) (character 4)) (end (line 943) (character 626))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 946) (character 8)) (end (line 946) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 944) (character 8)) (end (line 944) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 945) (character 8)) (end (line 945) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 948) (character 8)) (end (line 948) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 948) (character 22)) (end (line 948) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 947) (character 8)) (end (line 947) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (kind "attribute def") (name "MassieuFunctionValue") (declared-name "MassieuFunctionValue") (range (start (line 924) (character 4)) (end (line 924) (character 678))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 924) (character 4)) (end (line 924) (character 678))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 938) (character 8)) (end (line 938) (character 51))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassieuFunctionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 938) (character 22)) (end (line 938) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 937) (character 8)) (end (line 937) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 937) (character 22)) (end (line 937) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (kind "attribute def") (name "MaximumThermalEfficiencyValue") (declared-name "MaximumThermalEfficiencyValue") (range (start (line 1025) (character 4)) (end (line 1025) (character 819))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1025) (character 4)) (end (line 1025) (character 819))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (kind "attribute def") (name "PlanckFunctionUnit") (declared-name "PlanckFunctionUnit") (range (start (line 971) (character 4)) (end (line 971) (character 625))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 974) (character 8)) (end (line 974) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 972) (character 8)) (end (line 972) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 973) (character 8)) (end (line 973) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 976) (character 8)) (end (line 976) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 976) (character 22)) (end (line 976) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 975) (character 8)) (end (line 975) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (kind "attribute def") (name "PlanckFunctionValue") (declared-name "PlanckFunctionValue") (range (start (line 952) (character 4)) (end (line 952) (character 664))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::_documentation"))) (kind "documentation") (name "") (range (start (line 952) (character 4)) (end (line 952) (character 664))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 966) (character 8)) (end (line 966) (character 50))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PlanckFunctionUnit") (range none)) (redefinition (reference "mRef") (range (start (line 966) (character 22)) (end (line 966) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 965) (character 8)) (end (line 965) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 965) (character 22)) (end (line 965) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (kind "attribute def") (name "PressureCoefficientUnit") (declared-name "PressureCoefficientUnit") (range (start (line 149) (character 4)) (end (line 149) (character 631))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 152) (character 8)) (end (line 152) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 150) (character 8)) (end (line 150) (character 103))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 151) (character 8)) (end (line 151) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 154) (character 8)) (end (line 154) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 154) (character 22)) (end (line 154) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 153) (character 8)) (end (line 153) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (kind "attribute def") (name "PressureCoefficientValue") (declared-name "PressureCoefficientValue") (range (start (line 130) (character 4)) (end (line 130) (character 737))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 130) (character 4)) (end (line 130) (character 737))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 144) (character 8)) (end (line 144) (character 55))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "PressureCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 144) (character 22)) (end (line 144) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 143) (character 8)) (end (line 143) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 143) (character 22)) (end (line 143) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (kind "attribute def") (name "RatioOfSpecificHeatCapacitiesValue") (declared-name "RatioOfSpecificHeatCapacitiesValue") (range (start (line 630) (character 4)) (end (line 630) (character 877))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue::_documentation"))) (kind "documentation") (name "") (range (start (line 630) (character 4)) (end (line 630) (character 877))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 14) (character 4)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 19)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (kind "attribute def") (name "RelativeHumidityValue") (declared-name "RelativeHumidityValue") (range (start (line 1189) (character 4)) (end (line 1189) (character 755))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1189) (character 4)) (end (line 1189) (character 755))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (kind "attribute def") (name "RelativeMassConcentrationOfVapourValue") (declared-name "RelativeMassConcentrationOfVapourValue") (range (start (line 1206) (character 4)) (end (line 1206) (character 914))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1206) (character 4)) (end (line 1206) (character 914))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (kind "attribute def") (name "RelativeMassRatioOfVapourValue") (declared-name "RelativeMassRatioOfVapourValue") (range (start (line 1223) (character 4)) (end (line 1223) (character 817))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1223) (character 4)) (end (line 1223) (character 817))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (kind "attribute def") (name "RelativePressureCoefficientUnit") (declared-name "RelativePressureCoefficientUnit") (range (start (line 124) (character 4)) (end (line 124) (character 296))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 126) (character 8)) (end (line 126) (character 98))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 126) (character 22)) (end (line 126) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 125) (character 8)) (end (line 125) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (kind "attribute def") (name "RelativePressureCoefficientValue") (declared-name "RelativePressureCoefficientValue") (range (start (line 105) (character 4)) (end (line 105) (character 833))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::_documentation"))) (kind "documentation") (name "") (range (start (line 105) (character 4)) (end (line 105) (character 833))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 119) (character 8)) (end (line 119) (character 63))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "RelativePressureCoefficientUnit") (range none)) (redefinition (reference "mRef") (range (start (line 119) (character 22)) (end (line 119) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 118) (character 8)) (end (line 118) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 118) (character 22)) (end (line 118) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (kind "attribute def") (name "SpecificEnergyUnit") (declared-name "SpecificEnergyUnit") (range (start (line 837) (character 4)) (end (line 837) (character 363))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 839) (character 8)) (end (line 839) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 838) (character 8)) (end (line 838) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 840) (character 8)) (end (line 840) (character 94))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 840) (character 22)) (end (line 840) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (kind "attribute def") (name "SpecificEnergyValue") (declared-name "SpecificEnergyValue") (range (start (line 818) (character 4)) (end (line 818) (character 597))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 818) (character 4)) (end (line 818) (character 597))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 832) (character 8)) (end (line 832) (character 50))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificEnergyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 832) (character 22)) (end (line 832) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 831) (character 8)) (end (line 831) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 831) (character 22)) (end (line 831) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (kind "attribute def") (name "SpecificEnthalpyUnit") (declared-name "SpecificEnthalpyUnit") (range (start (line 881) (character 4)) (end (line 881) (character 365))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 883) (character 8)) (end (line 883) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 882) (character 8)) (end (line 882) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 884) (character 8)) (end (line 884) (character 94))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 884) (character 22)) (end (line 884) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (kind "attribute def") (name "SpecificEnthalpyValue") (declared-name "SpecificEnthalpyValue") (range (start (line 862) (character 4)) (end (line 862) (character 609))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 862) (character 4)) (end (line 862) (character 609))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 876) (character 8)) (end (line 876) (character 52))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificEnthalpyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 876) (character 22)) (end (line 876) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 875) (character 8)) (end (line 875) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 875) (character 22)) (end (line 875) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (kind "attribute def") (name "SpecificEntropyUnit") (declared-name "SpecificEntropyUnit") (range (start (line 713) (character 4)) (end (line 713) (character 517))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 715) (character 8)) (end (line 715) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 714) (character 8)) (end (line 714) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 717) (character 8)) (end (line 717) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 717) (character 22)) (end (line 717) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 716) (character 8)) (end (line 716) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (kind "attribute def") (name "SpecificEntropyValue") (declared-name "SpecificEntropyValue") (range (start (line 694) (character 4)) (end (line 694) (character 688))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 694) (character 4)) (end (line 694) (character 688))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 708) (character 8)) (end (line 708) (character 51))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificEntropyUnit") (range none)) (redefinition (reference "mRef") (range (start (line 708) (character 22)) (end (line 708) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 707) (character 8)) (end (line 707) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 707) (character 22)) (end (line 707) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (kind "attribute def") (name "SpecificGasConstantUnit") (declared-name "SpecificGasConstantUnit") (range (start (line 1061) (character 4)) (end (line 1061) (character 521))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 1063) (character 8)) (end (line 1063) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 1062) (character 8)) (end (line 1062) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 1065) (character 8)) (end (line 1065) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 1065) (character 22)) (end (line 1065) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 1064) (character 8)) (end (line 1064) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (kind "attribute def") (name "SpecificGasConstantValue") (declared-name "SpecificGasConstantValue") (range (start (line 1042) (character 4)) (end (line 1042) (character 634))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1042) (character 4)) (end (line 1042) (character 634))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 1056) (character 8)) (end (line 1056) (character 55))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificGasConstantUnit") (range none)) (redefinition (reference "mRef") (range (start (line 1056) (character 22)) (end (line 1056) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 1055) (character 8)) (end (line 1055) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 1055) (character 22)) (end (line 1055) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (kind "attribute def") (name "SpecificHeatCapacityAtConstantPressureUnit") (declared-name "SpecificHeatCapacityAtConstantPressureUnit") (range (start (line 568) (character 4)) (end (line 568) (character 540))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 570) (character 8)) (end (line 570) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 569) (character 8)) (end (line 569) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 572) (character 8)) (end (line 572) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 572) (character 22)) (end (line 572) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 571) (character 8)) (end (line 571) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (kind "attribute def") (name "SpecificHeatCapacityAtConstantPressureValue") (declared-name "SpecificHeatCapacityAtConstantPressureValue") (range (start (line 549) (character 4)) (end (line 549) (character 722))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 549) (character 4)) (end (line 549) (character 722))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 563) (character 8)) (end (line 563) (character 74))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificHeatCapacityAtConstantPressureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 563) (character 22)) (end (line 563) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 562) (character 8)) (end (line 562) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 562) (character 22)) (end (line 562) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (kind "attribute def") (name "SpecificHeatCapacityAtConstantVolumeUnit") (declared-name "SpecificHeatCapacityAtConstantVolumeUnit") (range (start (line 595) (character 4)) (end (line 595) (character 538))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 597) (character 8)) (end (line 597) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 596) (character 8)) (end (line 596) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 599) (character 8)) (end (line 599) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 599) (character 22)) (end (line 599) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 598) (character 8)) (end (line 598) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (kind "attribute def") (name "SpecificHeatCapacityAtConstantVolumeValue") (declared-name "SpecificHeatCapacityAtConstantVolumeValue") (range (start (line 576) (character 4)) (end (line 576) (character 713))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::_documentation"))) (kind "documentation") (name "") (range (start (line 576) (character 4)) (end (line 576) (character 713))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 590) (character 8)) (end (line 590) (character 72))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificHeatCapacityAtConstantVolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 590) (character 22)) (end (line 590) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 589) (character 8)) (end (line 589) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 589) (character 22)) (end (line 589) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (kind "attribute def") (name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (declared-name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (range (start (line 622) (character 4)) (end (line 622) (character 547))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 624) (character 8)) (end (line 624) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 623) (character 8)) (end (line 623) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 626) (character 8)) (end (line 626) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 626) (character 22)) (end (line 626) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 625) (character 8)) (end (line 625) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (kind "attribute def") (name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (declared-name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (range (start (line 603) (character 4)) (end (line 603) (character 724))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::_documentation"))) (kind "documentation") (name "") (range (start (line 603) (character 4)) (end (line 603) (character 724))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 617) (character 8)) (end (line 617) (character 81))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (range none)) (redefinition (reference "mRef") (range (start (line 617) (character 22)) (end (line 617) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 616) (character 8)) (end (line 616) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 616) (character 22)) (end (line 616) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (kind "attribute def") (name "SpecificHeatCapacityUnit") (declared-name "SpecificHeatCapacityUnit") (range (start (line 541) (character 4)) (end (line 541) (character 522))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 543) (character 8)) (end (line 543) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 542) (character 8)) (end (line 542) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 545) (character 8)) (end (line 545) (character 122))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 545) (character 22)) (end (line 545) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 544) (character 8)) (end (line 544) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (kind "attribute def") (name "SpecificHeatCapacityValue") (declared-name "SpecificHeatCapacityValue") (range (start (line 522) (character 4)) (end (line 522) (character 729))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 522) (character 4)) (end (line 522) (character 729))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 536) (character 8)) (end (line 536) (character 56))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 536) (character 22)) (end (line 536) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 535) (character 8)) (end (line 535) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 535) (character 22)) (end (line 535) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (kind "attribute def") (name "SurfaceCoefficientOfHeatTransferUnit") (declared-name "SurfaceCoefficientOfHeatTransferUnit") (range (start (line 373) (character 4)) (end (line 373) (character 530))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 375) (character 8)) (end (line 375) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 374) (character 8)) (end (line 374) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 377) (character 8)) (end (line 377) (character 120))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 377) (character 22)) (end (line 377) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 376) (character 8)) (end (line 376) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (kind "attribute def") (name "SurfaceCoefficientOfHeatTransferValue") (declared-name "SurfaceCoefficientOfHeatTransferValue") (range (start (line 354) (character 4)) (end (line 354) (character 941))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::_documentation"))) (kind "documentation") (name "") (range (start (line 354) (character 4)) (end (line 354) (character 941))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 368) (character 8)) (end (line 368) (character 68))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "SurfaceCoefficientOfHeatTransferUnit") (range none)) (redefinition (reference "mRef") (range (start (line 368) (character 22)) (end (line 368) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 367) (character 8)) (end (line 367) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 367) (character 22)) (end (line 367) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::TemperatureUnit"))) (kind "alias") (name "TemperatureUnit") (declared-name "TemperatureUnit") (range (start (line 25) (character 4)) (end (line 25) (character 59))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::TemperatureValue"))) (kind "alias") (name "TemperatureValue") (declared-name "TemperatureValue") (range (start (line 26) (character 4)) (end (line 26) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (kind "attribute def") (name "ThermalConductanceUnit") (declared-name "ThermalConductanceUnit") (range (start (line 459) (character 4)) (end (line 459) (character 629))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 462) (character 8)) (end (line 462) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 460) (character 8)) (end (line 460) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 461) (character 8)) (end (line 461) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 464) (character 8)) (end (line 464) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 464) (character 22)) (end (line 464) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 463) (character 8)) (end (line 463) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (kind "attribute def") (name "ThermalConductanceValue") (declared-name "ThermalConductanceValue") (range (start (line 440) (character 4)) (end (line 440) (character 713))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 440) (character 4)) (end (line 440) (character 713))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 454) (character 8)) (end (line 454) (character 54))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalConductanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 454) (character 22)) (end (line 454) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 453) (character 8)) (end (line 453) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 453) (character 22)) (end (line 453) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (kind "attribute def") (name "ThermalConductivityUnit") (declared-name "ThermalConductivityUnit") (range (start (line 318) (character 4)) (end (line 318) (character 630))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 321) (character 8)) (end (line 321) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 319) (character 8)) (end (line 319) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 320) (character 8)) (end (line 320) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 323) (character 8)) (end (line 323) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 323) (character 22)) (end (line 323) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 322) (character 8)) (end (line 322) (character 124))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (kind "attribute def") (name "ThermalConductivityValue") (declared-name "ThermalConductivityValue") (range (start (line 299) (character 4)) (end (line 299) (character 674))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 299) (character 4)) (end (line 299) (character 674))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 313) (character 8)) (end (line 313) (character 55))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalConductivityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 313) (character 22)) (end (line 313) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 312) (character 8)) (end (line 312) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 312) (character 22)) (end (line 312) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (kind "attribute def") (name "ThermalDiffusivityUnit") (declared-name "ThermalDiffusivityUnit") (range (start (line 487) (character 4)) (end (line 487) (character 367))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 489) (character 8)) (end (line 489) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 488) (character 8)) (end (line 488) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 490) (character 8)) (end (line 490) (character 94))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 490) (character 22)) (end (line 490) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (kind "attribute def") (name "ThermalDiffusivityValue") (declared-name "ThermalDiffusivityValue") (range (start (line 468) (character 4)) (end (line 468) (character 769))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 468) (character 4)) (end (line 468) (character 769))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 482) (character 8)) (end (line 482) (character 54))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalDiffusivityUnit") (range none)) (redefinition (reference "mRef") (range (start (line 482) (character 22)) (end (line 482) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 481) (character 8)) (end (line 481) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 481) (character 22)) (end (line 481) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (kind "attribute def") (name "ThermalEfficiencyValue") (declared-name "ThermalEfficiencyValue") (range (start (line 1008) (character 4)) (end (line 1008) (character 589))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue::_documentation"))) (kind "documentation") (name "") (range (start (line 1008) (character 4)) (end (line 1008) (character 589))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (kind "attribute def") (name "ThermalInsulanceUnit") (declared-name "ThermalInsulanceUnit") (range (start (line 400) (character 4)) (end (line 400) (character 513))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 402) (character 8)) (end (line 402) (character 104))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 401) (character 8)) (end (line 401) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 404) (character 8)) (end (line 404) (character 120))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 404) (character 22)) (end (line 404) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 403) (character 8)) (end (line 403) (character 123))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (kind "attribute def") (name "ThermalInsulanceValue") (declared-name "ThermalInsulanceValue") (range (start (line 381) (character 4)) (end (line 381) (character 743))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 381) (character 4)) (end (line 381) (character 743))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 395) (character 8)) (end (line 395) (character 52))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 395) (character 22)) (end (line 395) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 394) (character 8)) (end (line 394) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 394) (character 22)) (end (line 394) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (kind "attribute def") (name "ThermalResistanceUnit") (declared-name "ThermalResistanceUnit") (range (start (line 431) (character 4)) (end (line 431) (character 628))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (range (start (line 434) (character 8)) (end (line 434) (character 104))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (range (start (line 432) (character 8)) (end (line 432) (character 103))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (range (start (line 433) (character 8)) (end (line 433) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 436) (character 8)) (end (line 436) (character 130))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 436) (character 22)) (end (line 436) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::thermodynamicTemperaturePF"))) (kind "attribute") (name "thermodynamicTemperaturePF") (declared-name "thermodynamicTemperaturePF") (range (start (line 435) (character 8)) (end (line 435) (character 123))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (kind "attribute def") (name "ThermalResistanceValue") (declared-name "ThermalResistanceValue") (range (start (line 412) (character 4)) (end (line 412) (character 630))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::_documentation"))) (kind "documentation") (name "") (range (start (line 412) (character 4)) (end (line 412) (character 630))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 426) (character 8)) (end (line 426) (character 53))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThermalResistanceUnit") (range none)) (redefinition (reference "mRef") (range (start (line 426) (character 22)) (end (line 426) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 425) (character 8)) (end (line 425) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 425) (character 22)) (end (line 425) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 64627))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::amountOfHeat"))) (kind "alias") (name "amountOfHeat") (declared-name "amountOfHeat") (range (start (line 227) (character 4)) (end (line 227) (character 32))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (kind "attribute def") (name "celsiusTemperature") (declared-name "celsiusTemperature") (range (start (line 47) (character 4)) (end (line 47) (character 91))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CelsiusTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (kind "attribute def") (name "coefficientOfHeatTransfer") (declared-name "coefficientOfHeatTransfer") (range (start (line 344) (character 4)) (end (line 344) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfThermalInsulance"))) (kind "alias") (name "coefficientOfThermalInsulance") (declared-name "coefficientOfThermalInsulance") (range (start (line 409) (character 4)) (end (line 409) (character 61))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (kind "attribute def") (name "cubicExpansionCoefficient") (declared-name "cubicExpansionCoefficient") (range (start (line 97) (character 4)) (end (line 97) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "CubicExpansionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (kind "attribute def") (name "densityOfHeatFlowRate") (declared-name "densityOfHeatFlowRate") (range (start (line 290) (character 4)) (end (line 290) (character 97))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))) (kind "attribute def") (name "dewPointTemperature") (declared-name "dewPointTemperature") (range (start (line 1240) (character 4)) (end (line 1240) (character 726))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature::_documentation"))) (kind "documentation") (name "") (range (start (line 1240) (character 4)) (end (line 1240) (character 726))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (kind "attribute def") (name "energy") (declared-name "energy") (range (start (line 738) (character 4)) (end (line 738) (character 67))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (kind "attribute def") (name "enthalpy") (declared-name "enthalpy") (range (start (line 766) (character 4)) (end (line 766) (character 627))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy::_documentation"))) (kind "documentation") (name "") (range (start (line 766) (character 4)) (end (line 766) (character 627))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (kind "attribute def") (name "entropy") (declared-name "entropy") (range (start (line 683) (character 4)) (end (line 683) (character 69))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EntropyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (kind "attribute def") (name "gibbsEnergy") (declared-name "gibbsEnergy") (range (start (line 800) (character 4)) (end (line 800) (character 741))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 800) (character 4)) (end (line 800) (character 741))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::gibbsFunction"))) (kind "alias") (name "gibbsFunction") (declared-name "gibbsFunction") (range (start (line 815) (character 4)) (end (line 815) (character 40))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (kind "attribute def") (name "heat") (declared-name "heat") (range (start (line 212) (character 4)) (end (line 212) (character 1050))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::heat::_documentation"))) (kind "documentation") (name "") (range (start (line 212) (character 4)) (end (line 212) (character 1050))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::heat"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (kind "attribute def") (name "heatCapacity") (declared-name "heatCapacity") (range (start (line 511) (character 4)) (end (line 511) (character 79))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (kind "attribute def") (name "heatFlowRate") (declared-name "heatFlowRate") (range (start (line 263) (character 4)) (end (line 263) (character 79))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatFlowRateValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (kind "attribute def") (name "helmholtzEnergy") (declared-name "helmholtzEnergy") (range (start (line 782) (character 4)) (end (line 782) (character 791))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 782) (character 4)) (end (line 782) (character 791))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzFunction"))) (kind "alias") (name "helmholtzFunction") (declared-name "helmholtzFunction") (range (start (line 797) (character 4)) (end (line 797) (character 48))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (kind "attribute def") (name "internalEnergy") (declared-name "internalEnergy") (range (start (line 748) (character 4)) (end (line 748) (character 791))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 748) (character 4)) (end (line 748) (character 791))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (kind "attribute def") (name "isentropicCompressibility") (declared-name "isentropicCompressibility") (range (start (line 202) (character 4)) (end (line 202) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "IsentropicCompressibilityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExpansionFactor"))) (kind "alias") (name "isentropicExpansionFactor") (declared-name "isentropicExpansionFactor") (range (start (line 663) (character 4)) (end (line 663) (character 59))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (kind "attribute def") (name "isentropicExponent") (declared-name "isentropicExponent") (range (start (line 661) (character 4)) (end (line 661) (character 78))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "IsentropicExponentValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (kind "attribute def") (name "isothermalCompressibility") (declared-name "isothermalCompressibility") (range (start (line 175) (character 4)) (end (line 175) (character 105))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "IsothermalCompressibilityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (kind "attribute def") (name "jouleThomsonCoefficient") (declared-name "jouleThomsonCoefficient") (range (start (line 997) (character 4)) (end (line 997) (character 101))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleThomsonCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (kind "attribute def") (name "latentHeat") (declared-name "latentHeat") (range (start (line 230) (character 4)) (end (line 230) (character 598))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat::_documentation"))) (kind "documentation") (name "") (range (start (line 230) (character 4)) (end (line 230) (character 598))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (kind "attribute def") (name "linearExpansionCoefficient") (declared-name "linearExpansionCoefficient") (range (start (line 72) (character 4)) (end (line 72) (character 107))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearExpansionCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (kind "attribute def") (name "massConcentrationOfWater") (declared-name "massConcentrationOfWater") (range (start (line 1086) (character 4)) (end (line 1086) (character 103))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationOfWaterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (kind "attribute def") (name "massConcentrationOfWaterVapourAbsoluteHumidity") (declared-name "massConcentrationOfWaterVapourAbsoluteHumidity") (range (start (line 1112) (character 4)) (end (line 1112) (character 147))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (kind "attribute def") (name "massFractionOfDryMatter") (declared-name "massFractionOfDryMatter") (range (start (line 1186) (character 4)) (end (line 1186) (character 88))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFractionOfDryMatterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (kind "attribute def") (name "massFractionOfWater") (declared-name "massFractionOfWater") (range (start (line 1169) (character 4)) (end (line 1169) (character 80))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFractionOfWaterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (kind "attribute def") (name "massRatioOfWaterToDryMatter") (declared-name "massRatioOfWaterToDryMatter") (range (start (line 1135) (character 4)) (end (line 1135) (character 96))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassRatioOfWaterToDryMatterValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (kind "attribute def") (name "massRatioOfWaterVapourToDryGas") (declared-name "massRatioOfWaterVapourToDryGas") (range (start (line 1152) (character 4)) (end (line 1152) (character 102))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassRatioOfWaterVapourToDryGasValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (kind "attribute def") (name "massieuFunction") (declared-name "massieuFunction") (range (start (line 941) (character 4)) (end (line 941) (character 85))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassieuFunctionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (kind "attribute def") (name "maximumThermalEfficiency") (declared-name "maximumThermalEfficiency") (range (start (line 1039) (character 4)) (end (line 1039) (character 90))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "MaximumThermalEfficiencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (kind "attribute def") (name "planckFunction") (declared-name "planckFunction") (range (start (line 969) (character 4)) (end (line 969) (character 83))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PlanckFunctionValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (kind "attribute def") (name "pressureCoefficient") (declared-name "pressureCoefficient") (range (start (line 147) (character 4)) (end (line 147) (character 93))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (kind "attribute def") (name "ratioOfSpecificHeatCapacities") (declared-name "ratioOfSpecificHeatCapacities") (range (start (line 644) (character 4)) (end (line 644) (character 100))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RatioOfSpecificHeatCapacitiesValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (kind "attribute def") (name "relativeHumidity") (declared-name "relativeHumidity") (range (start (line 1203) (character 4)) (end (line 1203) (character 74))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeHumidityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (kind "attribute def") (name "relativeMassConcentrationOfVapour") (declared-name "relativeMassConcentrationOfVapour") (range (start (line 1220) (character 4)) (end (line 1220) (character 108))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeMassConcentrationOfVapourValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (kind "attribute def") (name "relativeMassRatioOfVapour") (declared-name "relativeMassRatioOfVapour") (range (start (line 1237) (character 4)) (end (line 1237) (character 92))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativeMassRatioOfVapourValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (kind "attribute def") (name "relativePressureCoefficient") (declared-name "relativePressureCoefficient") (range (start (line 122) (character 4)) (end (line 122) (character 109))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "RelativePressureCoefficientValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (kind "attribute def") (name "specificEnergy") (declared-name "specificEnergy") (range (start (line 835) (character 4)) (end (line 835) (character 83))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (kind "attribute def") (name "specificEnthalpy") (declared-name "specificEnthalpy") (range (start (line 879) (character 4)) (end (line 879) (character 87))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnthalpyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (kind "attribute def") (name "specificEntropy") (declared-name "specificEntropy") (range (start (line 711) (character 4)) (end (line 711) (character 85))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEntropyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (kind "attribute def") (name "specificGasConstant") (declared-name "specificGasConstant") (range (start (line 1059) (character 4)) (end (line 1059) (character 93))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificGasConstantValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (kind "attribute def") (name "specificGibbsEnergy") (declared-name "specificGibbsEnergy") (range (start (line 906) (character 4)) (end (line 906) (character 687))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 906) (character 4)) (end (line 906) (character 687))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsFunction"))) (kind "alias") (name "specificGibbsFunction") (declared-name "specificGibbsFunction") (range (start (line 921) (character 4)) (end (line 921) (character 56))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (kind "attribute def") (name "specificHeatCapacity") (declared-name "specificHeatCapacity") (range (start (line 539) (character 4)) (end (line 539) (character 95))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (kind "attribute def") (name "specificHeatCapacityAtConstantPressure") (declared-name "specificHeatCapacityAtConstantPressure") (range (start (line 566) (character 4)) (end (line 566) (character 131))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityAtConstantPressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (kind "attribute def") (name "specificHeatCapacityAtConstantVolume") (declared-name "specificHeatCapacityAtConstantVolume") (range (start (line 593) (character 4)) (end (line 593) (character 127))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityAtConstantVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (kind "attribute def") (name "specificHeatCapacityAtSaturatedVapourPressure") (declared-name "specificHeatCapacityAtSaturatedVapourPressure") (range (start (line 620) (character 4)) (end (line 620) (character 145))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityAtSaturatedVapourPressureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (kind "attribute def") (name "specificHelmholtzEnergy") (declared-name "specificHelmholtzEnergy") (range (start (line 888) (character 4)) (end (line 888) (character 716))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 888) (character 4)) (end (line 888) (character 716))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzFunction"))) (kind "alias") (name "specificHelmholtzFunction") (declared-name "specificHelmholtzFunction") (range (start (line 903) (character 4)) (end (line 903) (character 64))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (kind "attribute def") (name "specificInternalEnergy") (declared-name "specificInternalEnergy") (range (start (line 844) (character 4)) (end (line 844) (character 625))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy::_documentation"))) (kind "documentation") (name "") (range (start (line 844) (character 4)) (end (line 844) (character 625))) (parent (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::specificThermodynamicEnergy"))) (kind "alias") (name "specificThermodynamicEnergy") (declared-name "specificThermodynamicEnergy") (range (start (line 859) (character 4)) (end (line 859) (character 65))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (kind "attribute def") (name "surfaceCoefficientOfHeatTransfer") (declared-name "surfaceCoefficientOfHeatTransfer") (range (start (line 371) (character 4)) (end (line 371) (character 119))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceCoefficientOfHeatTransferValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::temperature"))) (kind "alias") (name "temperature") (declared-name "temperature") (range (start (line 27) (character 4)) (end (line 27) (character 51))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (kind "attribute def") (name "thermalConductance") (declared-name "thermalConductance") (range (start (line 457) (character 4)) (end (line 457) (character 91))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (kind "attribute def") (name "thermalConductivity") (declared-name "thermalConductivity") (range (start (line 316) (character 4)) (end (line 316) (character 93))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (kind "attribute def") (name "thermalDiffusivity") (declared-name "thermalDiffusivity") (range (start (line 485) (character 4)) (end (line 485) (character 91))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalDiffusivityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (kind "attribute def") (name "thermalEfficiency") (declared-name "thermalEfficiency") (range (start (line 1022) (character 4)) (end (line 1022) (character 76))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalEfficiencyValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (kind "attribute def") (name "thermalInsulance") (declared-name "thermalInsulance") (range (start (line 398) (character 4)) (end (line 398) (character 87))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (kind "attribute def") (name "thermalResistance") (declared-name "thermalResistance") (range (start (line 429) (character 4)) (end (line 429) (character 89))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "ISQThermodynamics::thermodynamicEnergy"))) (kind "alias") (name "thermodynamicEnergy") (declared-name "thermodynamicEnergy") (range (start (line 763) (character 4)) (end (line 763) (character 49))) (parent (node (document "d0") (qualified-name "ISQThermodynamics"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 15) (character 19)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 16) (character 19)) (end (line 16) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQBase::*") (range (start (line 17) (character 19)) (end (line 17) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 51) (character 22)) (end (line 51) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CelsiusTemperatureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 44) (character 22)) (end (line 44) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 43) (character 22)) (end (line 43) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 350) (character 22)) (end (line 350) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 341) (character 22)) (end (line 341) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 340) (character 22)) (end (line 340) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 101) (character 22)) (end (line 101) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "CubicExpansionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 94) (character 22)) (end (line 94) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 93) (character 22)) (end (line 93) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 295) (character 22)) (end (line 295) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 287) (character 22)) (end (line 287) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 286) (character 22)) (end (line 286) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 744) (character 22)) (end (line 744) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 735) (character 22)) (end (line 735) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 734) (character 22)) (end (line 734) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 690) (character 22)) (end (line 690) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "EntropyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 680) (character 22)) (end (line 680) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 679) (character 22)) (end (line 679) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 518) (character 22)) (end (line 518) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 508) (character 22)) (end (line 508) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 507) (character 22)) (end (line 507) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 269) (character 22)) (end (line 269) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatFlowRateUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 260) (character 22)) (end (line 260) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 259) (character 22)) (end (line 259) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 208) (character 22)) (end (line 208) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IsentropicCompressibilityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 199) (character 22)) (end (line 199) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 198) (character 22)) (end (line 198) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 181) (character 22)) (end (line 181) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "IsothermalCompressibilityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 172) (character 22)) (end (line 172) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 171) (character 22)) (end (line 171) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1004) (character 22)) (end (line 1004) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 994) (character 22)) (end (line 994) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 993) (character 22)) (end (line 993) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 76) (character 22)) (end (line 76) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearExpansionCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 69) (character 22)) (end (line 69) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 68) (character 22)) (end (line 68) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1091) (character 22)) (end (line 1091) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationOfWaterUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1083) (character 22)) (end (line 1083) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1082) (character 22)) (end (line 1082) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1117) (character 22)) (end (line 1117) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1109) (character 22)) (end (line 1109) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1108) (character 22)) (end (line 1108) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 948) (character 22)) (end (line 948) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "MassieuFunctionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 938) (character 22)) (end (line 938) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 937) (character 22)) (end (line 937) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 976) (character 22)) (end (line 976) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanckFunctionUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 966) (character 22)) (end (line 966) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 965) (character 22)) (end (line 965) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 154) (character 22)) (end (line 154) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 144) (character 22)) (end (line 144) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 143) (character 22)) (end (line 143) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 14) (character 19)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 126) (character 22)) (end (line 126) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativePressureCoefficientUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 119) (character 22)) (end (line 119) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 118) (character 22)) (end (line 118) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 840) (character 22)) (end (line 840) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 832) (character 22)) (end (line 832) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 831) (character 22)) (end (line 831) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 884) (character 22)) (end (line 884) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnthalpyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 876) (character 22)) (end (line 876) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 875) (character 22)) (end (line 875) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 717) (character 22)) (end (line 717) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEntropyUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 708) (character 22)) (end (line 708) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 707) (character 22)) (end (line 707) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 1065) (character 22)) (end (line 1065) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificGasConstantUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 1056) (character 22)) (end (line 1056) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 1055) (character 22)) (end (line 1055) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 572) (character 22)) (end (line 572) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtConstantPressureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 563) (character 22)) (end (line 563) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 562) (character 22)) (end (line 562) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 599) (character 22)) (end (line 599) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtConstantVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 590) (character 22)) (end (line 590) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 589) (character 22)) (end (line 589) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 626) (character 22)) (end (line 626) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 617) (character 22)) (end (line 617) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 616) (character 22)) (end (line 616) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 545) (character 22)) (end (line 545) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 536) (character 22)) (end (line 536) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 535) (character 22)) (end (line 535) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 377) (character 22)) (end (line 377) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceCoefficientOfHeatTransferUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 368) (character 22)) (end (line 368) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 367) (character 22)) (end (line 367) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 464) (character 22)) (end (line 464) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 454) (character 22)) (end (line 454) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 453) (character 22)) (end (line 453) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 323) (character 22)) (end (line 323) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 313) (character 22)) (end (line 313) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 312) (character 22)) (end (line 312) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 490) (character 22)) (end (line 490) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusivityUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 482) (character 22)) (end (line 482) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 481) (character 22)) (end (line 481) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 404) (character 22)) (end (line 404) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 395) (character 22)) (end (line 395) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 394) (character 22)) (end (line 394) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 436) (character 22)) (end (line 436) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::thermodynamicTemperaturePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 426) (character 22)) (end (line 426) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 425) (character 22)) (end (line 425) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "CelsiusTemperatureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "CubicExpansionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::dewPointTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (kind featureTyping) (ordinal 0)) (authored-target "EntropyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatFlowRateValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (kind featureTyping) (ordinal 0)) (authored-target "IsentropicCompressibilityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (kind featureTyping) (ordinal 0)) (authored-target "IsentropicExponentValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (kind featureTyping) (ordinal 0)) (authored-target "IsothermalCompressibilityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearExpansionCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationOfWaterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionOfDryMatterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionOfWaterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRatioOfWaterToDryMatterValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRatioOfWaterVapourToDryGasValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassieuFunctionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximumThermalEfficiencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (kind featureTyping) (ordinal 0)) (authored-target "PlanckFunctionValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (kind featureTyping) (ordinal 0)) (authored-target "RatioOfSpecificHeatCapacitiesValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeHumidityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeMassConcentrationOfVapourValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativeMassRatioOfVapourValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (kind featureTyping) (ordinal 0)) (authored-target "RelativePressureCoefficientValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnthalpyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEntropyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificGasConstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtConstantPressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtConstantVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityAtSaturatedVapourPressureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceCoefficientOfHeatTransferValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalDiffusivityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalEfficiencyValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue")))))
    (reference (id (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CelsiusTemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::celsiusTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CoefficientOfHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::coefficientOfHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::CubicExpansionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::cubicExpansionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::DensityOfHeatFlowRateValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::densityOfHeatFlowRate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::energy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::enthalpy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EntropyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::entropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::gibbsEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::heat"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatCapacityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::heatCapacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::HeatFlowRateValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::heatFlowRate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::helmholtzEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::internalEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicCompressibilityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicCompressibility"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsentropicExponentValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::isentropicExponent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::IsothermalCompressibilityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::isothermalCompressibility"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::JouleThomsonCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::jouleThomsonCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::EnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::latentHeat"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::LinearExpansionCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::linearExpansionCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWater"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassConcentrationOfWaterVapourAbsoluteHumidityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massConcentrationOfWaterVapourAbsoluteHumidity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfDryMatterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfDryMatter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassFractionOfWaterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massFractionOfWater"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterToDryMatterValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterToDryMatter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassRatioOfWaterVapourToDryGasValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massRatioOfWaterVapourToDryGas"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MassieuFunctionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::massieuFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::MaximumThermalEfficiencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::maximumThermalEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PlanckFunctionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::planckFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::PressureCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::pressureCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RatioOfSpecificHeatCapacitiesValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::ratioOfSpecificHeatCapacities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeHumidityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeHumidity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassConcentrationOfVapourValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassConcentrationOfVapour"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativeMassRatioOfVapourValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::relativeMassRatioOfVapour"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::RelativePressureCoefficientValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::relativePressureCoefficient"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnthalpyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEnthalpy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEntropyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificEntropy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificGasConstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGasConstant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificGibbsEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantPressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtConstantVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtConstantVolume"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificHeatCapacityAtSaturatedVapourPressureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHeatCapacityAtSaturatedVapourPressure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificHelmholtzEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SpecificEnergyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::specificInternalEnergy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::SurfaceCoefficientOfHeatTransferValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::surfaceCoefficientOfHeatTransfer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalConductivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalConductivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalDiffusivityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalDiffusivity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalEfficiencyValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalInsulanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalInsulance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (target (node (document "d0") (qualified-name "ISQThermodynamics::ThermalResistanceValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ISQThermodynamics::thermalResistance"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
