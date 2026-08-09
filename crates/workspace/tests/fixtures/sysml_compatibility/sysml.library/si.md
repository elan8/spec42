# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/SI
type=file
~~~
# SOURCE
~~~sysml
standard library package SI {
	doc
	/*
	 * International System of (Measurement) Units -- Système International d'Unités (SI), as defined in ISO/IEC 80000
	 *
	 * Note 1: In accordance with ISO/IEC 80000 en-GB spelling is used for the names and definitions of the units.
	 * Note 2: This is a representative but not yet complete list of measurement units.
	 */

    private import MeasurementReferences::*;
    public import ISQ::*;
    public import SIPrefixes::*;

    /*
     * SI simple unit needed in support of creation of the base units
     */
    attribute <g> gram : MassUnit;

    /*
     * SI base units
     */
    attribute <m> metre : LengthUnit;
    attribute <kg> kilogram : MassUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = g; } }
    attribute <s> second : DurationUnit;
    attribute <A> ampere : ElectricCurrentUnit;
    attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit {
        attribute temperatureOfWaterAtTriplePointInK: DefinitionalQuantityValue {
            :>> num = 27316/100;
            :>> definition = "temperature in kelvin of pure water at the triple point";
        }
        attribute :>> definitionalQuantityValues = temperatureOfWaterAtTriplePointInK;
        attribute :>> ThermodynamicTemperatureUnit::quantityDimension, TemperatureDifferenceUnit::quantityDimension {
            :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <mol> mole : AmountOfSubstanceUnit;
    attribute <cd> candela : LuminousIntensityUnit;

    /*
     * Declare the SI system of units with its explicit base units
     * and its associated system of quantities, the ISQ.
     */
	attribute <si> 'ISO/IEC 80000 International System of Units' : SystemOfUnits {
		:>> systemOfQuantities = isq;
		:>> baseUnits = (m, kg, s, A, K, mol, cd);
	}

    /*
     * Units with special names
     */
    attribute <B> byte : StorageCapacityUnit = one;
    attribute <Bd> baud : ModulationRateUnit = s^-1;
    attribute <bit> bit : StorageCapacityUnit = one;
    attribute <Bq> becquerel : NuclearActivityUnit = s^-1;
    attribute <C> coulomb : ElectricChargeUnit = A*s;
    attribute <dB> decibel : SoundPressureLevelUnit = one;
    attribute <dec> decade : LogarithmicFrequencyRangeUnit = one;
    attribute <E> erlang : TrafficIntensityUnit = one;
    attribute <F> farad : CapacitanceUnit = C/V;
    attribute <Gy> gray : AbsorbedDoseUnit = J/kg;
    attribute <H> henry : PermeanceUnit, InductanceUnit = Wb/A {
        attribute :>> PermeanceUnit::quantityDimension, InductanceUnit::quantityDimension {
            :>> PermeanceUnit::quantityDimension::quantityPowerFactors, InductanceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <Hart> hartley : InformationContentUnit = one;
    attribute <Hz> hertz : FrequencyUnit = s^-1;
    attribute <J> joule : EnergyUnit = N*m;
    //attribute <kat> katal : CatalyticActivityUnit = mol/s;
    attribute <lm> lumen : LuminousFluxUnit = cd*sr;
    attribute <lx> lux : IlluminanceUnit = lm/m^2;
    attribute <N> newton : ForceUnit = kg*m/s^2;
    attribute <nat> 'natural unit of information' : InformationContentUnit = one;
    attribute <o> octet : StorageCapacityUnit = one;
    attribute <oct> octave : LogarithmicFrequencyRangeUnit = one;
    attribute <Pa> pascal : PressureUnit = N/m^2;
    attribute <rad> radian : AngularMeasureUnit = m/m;
    attribute <S> siemens : ConductanceUnit = 'Ω'^-1;
    attribute <Sh> shannon : InformationContentUnit = one;
    attribute <sr> steradian : SolidAngularMeasureUnit = m^2/m^2;
    attribute <Sv> sievert : DoseEquivalentUnit = J/kg;
    attribute <T> tesla : MagneticFluxDensityUnit = Wb/m^2;
    attribute <V> volt : ElectricPotentialUnit = W/A;
    attribute <W> watt : PowerUnit = J/s;
    attribute <Wb> weber : MagneticFluxUnit = V*s;
    attribute <'Ω'> ohm : ResistanceUnit = V/A;

    /*
     * Units recognized in SI as specified in ISO 80000-1:2009
     */
    attribute <'Å'> 'ångström' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.0e-10; } }
    attribute <b> barn : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = 'm²'; :>> conversionFactor = 1.0e-28; } }
    attribute <d> day: DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = hour; :>> conversionFactor = 24; } }
    attribute <Da> dalton : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.66053906660e-27; :>> isExact = false; } }
    attribute <eV> electronvolt : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.602176487e-19; :>> isExact = false; } }
    attribute <h> hour: DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = min; :>> conversionFactor = 60; } }
    attribute <min> minute : DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = s; :>> conversionFactor = 60; } }
    attribute <L> litre : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = 'm³'; :>> conversionFactor = 1.0e-3; } }
    attribute tonne : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.0e-3; } }
    alias 'metric ton' for tonne;
    attribute <u> 'atomic mass unit' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Da; :>> conversionFactor = 1.0; } }
    attribute <ua> 'astronomical unit' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 149597870691e11; :>> isExact = false; } }
    attribute <var> 'volt ampere reactive' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = V*A; :>> conversionFactor = 1.0; } }
    attribute <'°'> degree : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 1.745329E-02; :>> isExact = false; } } // conversionFactor should become pi/180
    attribute <'′'> 'minute (angle)' : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 2.908882E-04; :>> isExact = false; } }
    alias arcmin for '′';
    attribute <'″'> 'second (angle)' : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 4.848137E-06; :>> isExact = false; } }
    alias arcsec for '″';

    /*
     * Derived units used in parts 3 to 12 of ISO/IEC 80000
     */
    attribute <'A⋅m⁻²⋅K⁻²'> 'ampere metre to the power minus 2 kelvin to the power minus 2' : RichardsonConstantUnit = A*m^-2*K^-2;
    attribute <'A⋅m²'> 'ampere metre squared' : MagneticMomentUnit = A*m^2;
    attribute <'A⋅m²⋅J⁻¹⋅s⁻¹'> 'ampere metre squared joule to the power minus 1 second to the power minus 1' : GyromagneticRatioUnit = A*m^2*J^-1*s^-1;
    attribute <'A⋅s/kg'> 'ampere second per kilogram' : GyromagneticRatioUnit = A*s/kg;
    attribute <'A/m'> 'ampere per metre' : LinearElectricCurrentDensityUnit = A/m;
    attribute <'A/m²'> 'ampere per square metre' : ElectricCurrentDensityUnit = A/m^2;
    attribute <'B/s'> 'byte per second' : TransferRateUnit = B/s;
    attribute <'bit/s'> 'bit per second' : BinaryDigitRateUnit = bit/s;
    attribute <'Bq/kg'> 'becquerel per kilogram' : SpecificActivityUnit = Bq/kg;
    attribute <'Bq/m²'> 'becquerel per square metre' : SurfaceActivityDensityUnit = Bq/m^2;
    attribute <'Bq/m³'> 'becquerel per cubic metre' : ActivityDensityUnit = Bq/m^3;
    attribute <'C⋅m'> 'coulomb metre' : ElectricDipoleMomentUnit = C*m;
    attribute <'C/(kg⋅s)'> 'coulomb per kilogram second' : ExposureRateUnit = C/(kg*s);
    attribute <'C/kg'> 'coulomb per kilogram' : ExposureUnit = C/kg;
    attribute <'C/m'> 'coulomb per metre' : LinearDensityOfElectricChargeUnit = C/m;
    attribute <'C/m²'> 'coulomb per square metre' : SurfaceDensityOfElectricChargeUnit = C/m^2;
    attribute <'C/m³'> 'coulomb per cubic metre' : ElectricChargeDensityUnit = C/m^3;
    attribute <'cd⋅m⁻²'> 'candela metre to the power minus 2' : LuminanceUnit = cd*m^-2;
    attribute <'cd⋅sr'> 'candela steradian' : LuminousFluxUnit = cd*sr;
    attribute <'cd⋅sr⋅kg⁻¹⋅m⁻²⋅s³'> 'candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3' : LuminousEfficacyOfRadiationUnit = cd*sr*kg^-1*m^-2*s^3;
    attribute <'cd⋅sr⋅m⁻²'> 'candela steradian metre to the power minus 2' : IlluminanceUnit = cd*sr*m^-2;
    attribute <'cd⋅sr⋅m⁻²⋅s'> 'candela steradian metre to the power minus 2 second' : LuminousExposureUnit = cd*sr*m^-2*s;
    attribute <'cd⋅sr⋅s'> 'candela steradian second' : LuminousEnergyUnit = cd*sr*s;
    attribute <'eV⋅J⋅kg⋅m²⋅s⁻²'> 'electronvolt joule kilogram metre squared second to the power minus 2' : HartreeEnergyUnit = eV*J*kg*m^2*s^-2;
    attribute <'eV⋅m⁻²/kg'> 'electronvolt metre to the power minus 2 per kilogram' : TotalMassStoppingPowerUnit = eV*m^-2/kg;
    attribute <'eV/m'> 'electronvolt per metre' : TotalLinearStoppingPowerUnit = eV/m;
    attribute <'eV/m²'> 'electronvolt per square metre' : EnergyFluenceUnit = eV/m^2;
    attribute <'F/m'> 'farad per metre' : ElectricConstantUnit = F/m;
    attribute <'g/L'> 'g per l' : MassConcentrationUnit = g/L;
    attribute <'g/mol'> 'g per mole' : MolarMassUnit = g/mol;
    attribute <'Gy/s'> 'gray per second' : AbsorbedDoseRateUnit = Gy/s;
    attribute <'H/m'> 'henry per metre' : MagneticConstantUnit = H/m;
    attribute <'H⁻¹'> 'henry to the power minus 1' : ReluctanceUnit = H^-1;
    attribute <'Hart/s'> 'hartley per second' : AverageInformationRateUnit = Hart/s;
    attribute <'J⋅m²/kg'> 'joule metre squared per kilogram' : TotalMassStoppingPowerUnit = J*m^2/kg;
    attribute <'J⋅s'> 'joule second' : ActionQuantityUnit = J*s;
    attribute <'J⋅s⋅eV⋅s'> 'joule second electronvolt second' : TotalAngularMomentumUnit = J*s*eV*s;
    attribute <'J⋅s⁻¹'> 'joule second to the power minus 1' : PowerUnit = J*s^-1;
    attribute <'J/(kg⋅K)'> 'joule per kilogram kelvin' : SpecificHeatCapacityUnit = J/(kg*K);
    attribute <'J/(m²⋅nm)'> 'joule per square metre nm' : SpectralRadiantExposureUnit = J/(m^2*nm);
    attribute <'J/(m³⋅nm)'> 'joule per cubic metre nm' : SpectralRadiantEnergyDensityInTermsOfWavelengthUnit = J/(m^3*nm);
    attribute <'J/(mol⋅K)'> 'joule per mole kelvin' : MolarHeatCapacityUnit = J/(mol*K);
    attribute <'J/K'> 'joule per kelvin' : HeatCapacityUnit = J/K;
    attribute <'J/kg'> 'joule per kilogram' : SpecificEnergyUnit = J/kg;
    attribute <'J/m'> 'joule per metre' : TotalLinearStoppingPowerUnit = J/m;
    attribute <'J/m²'> 'joule per square metre' : SpectralRadiantEnergyDensityInTermsOfWavenumberUnit = J/m^2;
    attribute <'J/m³'> 'joule per cubic metre' : ElectromagneticEnergyDensityUnit = J/m^3;
    attribute <'J/mol'> 'joule per mole' : MolarInternalEnergyUnit = J/mol;
    attribute <'J/nm'> 'joule per nm' : SpectralRadiantEnergyUnit = J/nm;
    attribute <'J/s'> 'joule per second' : HeatFlowRateUnit = J/s;
    attribute <'J⁻¹⋅m⁻³⋅eV⁻¹⋅m⁻³'> 'joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3' : EnergyDensityOfStatesUnit = J^-1*m^-3*eV^-1*m^-3;
    attribute <'K/Pa'> 'kelvin per pascal' : JouleThomsonCoefficientUnit = K/Pa;
    attribute <'K/W'> 'kelvin per watt' : ThermalResistanceUnit = K/W;
    attribute <'K⁻¹'> 'kelvin to the power minus 1' : LinearExpansionCoefficientUnit = K^-1;
    attribute <'kg⋅m⋅s⁻¹'> 'kilogram metre second to the power minus 1' : MomentumUnit = kg*m*s^-1;
    attribute <'kg⋅m⋅s⁻²'> 'kilogram metre second to the power minus 2' : ForceUnit = kg*m*s^-2;
    attribute <'kg⋅m⋅s⁻³'> 'kilogram metre second to the power minus 3' : SpectralRadiantFluxUnit = kg*m*s^-3;
    attribute <'kg⋅m⋅s⁻³⋅K⁻¹'> 'kilogram metre second to the power minus 3 kelvin to the power minus 1' : ThermalConductivityUnit = kg*m*s^-3*K^-1;
    attribute <'kg⋅m⋅s⁻³⋅sr⁻¹'> 'kilogram metre second to the power minus 3 steradian to the power minus 1' : SpectralRadiantIntensityUnit = kg*m*s^-3*sr^-1;
    attribute <'kg⋅m⁻¹'> 'kilogram metre to the power minus 1' : LinearMassDensityUnit = kg*m^-1;
    attribute <'kg⋅m⁻¹⋅s⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 1' : DynamicViscosityUnit = kg*m^-1*s^-1;
    attribute <'kg⋅m⁻¹⋅s⁻²'> 'kilogram metre to the power minus 1 second to the power minus 2' : PressureUnit = kg*m^-1*s^-2;
    attribute <'kg⋅m⁻¹⋅s⁻²⋅K⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1' : PressureCoefficientUnit = kg*m^-1*s^-2*K^-1;
    attribute <'kg⋅m⁻¹⋅s⁻³'> 'kilogram metre to the power minus 1 second to the power minus 3' : SpectralIrradianceUnit = kg*m^-1*s^-3;
    attribute <'kg⋅m⁻¹⋅s⁻³⋅sr⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1' : SpectralRadianceUnit = kg*m^-1*s^-3*sr^-1;
    attribute <'kg⋅m⁻²'> 'kilogram metre to the power minus 2' : SurfaceMassDensityUnit = kg*m^-2;
    attribute <'kg⋅m⁻²⋅s⁻¹'> 'kilogram metre to the power minus 2 second to the power minus 1' : MassFlowUnit = kg*m^-2*s^-1;
    attribute <'kg⋅m⁻²⋅s⁻²'> 'kilogram metre to the power minus 2 second to the power minus 2' : SpectralRadiantEnergyDensityInTermsOfWavelengthUnit = kg*m^-2*s^-2;
    attribute <'kg⋅m⁻³'> 'kilogram metre to the power minus 3' : MassDensityUnit = kg*m^-3;
    attribute <'kg⋅m⁻⁴⋅s⁻¹'> 'kilogram metre to the power minus 4 second to the power minus 1' : AcousticImpedanceUnit = kg*m^-4*s^-1;
    attribute <'kg⋅m²'> 'kilogram metre squared' : MomentOfInertiaUnit = kg*m^2;
    attribute <'kg⋅m²⋅s⁻¹'> 'kilogram metre squared second to the power minus 1' : AngularMomentumUnit = kg*m^2*s^-1;
    attribute <'kg⋅m²⋅s⁻²'> 'kilogram metre squared second to the power minus 2' : MomentOfForceUnit = kg*m^2*s^-2;
    attribute <'kg⋅m²⋅s⁻²⋅K⁻¹'> 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1' : HeatCapacityUnit = kg*m^2*s^-2*K^-1;
    attribute <'kg⋅m²⋅s⁻²⋅K⁻¹⋅mol⁻¹'> 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1' : MolarHeatCapacityUnit = kg*m^2*s^-2*K^-1*mol^-1;
    attribute <'kg⋅m²⋅s⁻²⋅mol⁻¹'> 'kilogram metre squared second to the power minus 2 mole to the power minus 1' : MolarInternalEnergyUnit = kg*m^2*s^-2*mol^-1;
    attribute <'kg⋅m²⋅s⁻³'> 'kilogram metre squared second to the power minus 3' : PowerUnit = kg*m^2*s^-3;
    attribute <'kg⋅m²⋅s⁻³⋅A⁻¹'> 'kilogram metre squared second to the power minus 3 ampere to the power minus 1' : ElectricPotentialDifferenceUnit = kg*m^2*s^-3*A^-1;
    attribute <'kg⋅m²⋅s⁻³⋅A⁻¹⋅K⁻¹'> 'kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1' : SeebeckCoefficientForSubstancesAAndBUnit = kg*m^2*s^-3*A^-1*K^-1;
    attribute <'kg⋅m²⋅s⁻³⋅K⁻¹'> 'kilogram metre squared second to the power minus 3 kelvin to the power minus 1' : ThermalConductanceUnit = kg*m^2*s^-3*K^-1;
    attribute <'kg⋅m²⋅s⁻³⋅sr⁻¹'> 'kilogram metre squared second to the power minus 3 steradian to the power minus 1' : RadiantIntensityUnit = kg*m^2*s^-3*sr^-1;
    attribute <'kg⋅m³⋅s⁻³⋅A⁻²'> 'kilogram metre cubed second to the power minus 3 ampere to the power minus 2' : ResistivityUnit = kg*m^3*s^-3*A^-2;
    attribute <'kg⋅mol⁻¹'> 'kilogram mole to the power minus 1' : MolarMassUnit = kg*mol^-1;
    attribute <'kg⋅s⁻¹'> 'kilogram second to the power minus 1' : MassFlowRateUnit = kg*s^-1;
    attribute <'kg⋅s⁻²'> 'kilogram second to the power minus 2' : SurfaceTensionUnit = kg*s^-2;
    attribute <'kg⋅s⁻²⋅A⁻¹'> 'kilogram second to the power minus 2 ampere to the power minus 1' : MagneticFluxDensityUnit = kg*s^-2*A^-1;
    attribute <'kg⋅s⁻³'> 'kilogram second to the power minus 3' : DensityOfHeatFlowRateUnit = kg*s^-3;
    attribute <'kg⋅s⁻³⋅K⁻¹'> 'kilogram second to the power minus 3 kelvin to the power minus 1' : CoefficientOfHeatTransferUnit = kg*s^-3*K^-1;
    attribute <'kg⋅s⁻³⋅sr⁻¹'> 'kilogram second to the power minus 3 steradian to the power minus 1' : RadianceUnit = kg*s^-3*sr^-1;
    attribute <'kg⁻¹⋅A'> 'kilogram to the power minus 1 ampere' : ExposureRateUnit = kg^-1*A;
    attribute <'kg⁻¹⋅m⋅s²'> 'kilogram to the power minus 1 metre second to the power 2' : CompressibilityUnit = kg^-1*m*s^2;
    attribute <'kg⁻¹⋅m⋅s²⋅K'> 'kilogram to the power minus 1 metre second to the power 2 kelvin' : JouleThomsonCoefficientUnit = kg^-1*m*s^2*K;
    attribute <'kg⁻¹⋅m⁻²⋅s³⋅K'> 'kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin' : ThermalResistanceUnit = kg^-1*m^-2*s^3*K;
    attribute <'kg⁻¹⋅m⁻³⋅s³⋅A²'> 'kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2' : ElectrolyticConductivityUnit = kg^-1*m^-3*s^3*A^2;
    attribute <'kg⁻¹⋅m⁻⁵⋅s²'> 'kilogram to the power minus 1 metre to the power minus 5 second to the power 2' : EnergyDensityOfStatesUnit = kg^-1*m^-5*s^2;
    attribute <'kg⁻¹⋅m²'> 'kilogram to the power minus 1 metre squared' : MassAttenuationCoefficientUnit = kg^-1*m^2;
    attribute <'kg⁻¹⋅m³'> 'kilogram to the power minus 1 metre cubed' : SpecificVolumeUnit = kg^-1*m^3;
    attribute <'kg⁻¹⋅s⋅A'> 'kilogram to the power minus 1 second ampere' : GyromagneticRatioUnit = kg^-1*s*A;
    attribute <'kg⁻¹⋅s⁻¹'> 'kilogram to the power minus 1 second to the power minus 1' : SpecificActivityUnit = kg^-1*s^-1;
    attribute <'kg⁻¹⋅s²'> 'kilogram to the power minus 1 second to the power 2' : EnergyDistributionOfCrossSectionUnit = kg^-1*s^2;
    attribute <'kg⁻¹⋅s²⋅A'> 'kilogram to the power minus 1 second to the power 2 ampere' : MobilityUnit = kg^-1*s^2*A;
    attribute <'kg⁻¹⋅s³⋅A²⋅mol⁻¹'> 'kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1' : MolarConductivityUnit = kg^-1*s^3*A^2*mol^-1;
    attribute <'kg⁻¹⋅s³⋅K'> 'kilogram to the power minus 1 second to the power 3 kelvin' : ThermalInsulanceUnit = kg^-1*s^3*K;
    attribute <'kg²⋅m⁻²⋅s⁻³'> 'kilogram to the power 2 metre to the power minus 2 second to the power minus 3' : SoundExposureUnit = kg^2*m^-2*s^-3;
    attribute <'kg²⋅m⁴⋅s⁻⁶⋅A⁻²⋅K⁻²'> 'kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2' : LorenzCoefficientUnit = kg^2*m^4*s^-6*A^-2*K^-2;
    attribute <'lm⋅s'> 'lumen second' : LuminousEnergyUnit = lm*s;
    attribute <'lm/m²'> 'lumen per square metre' : LuminousExitanceUnit = lm/m^2;
    attribute <'lm/W'> 'lumen per watt' : LuminousEfficacyOfRadiationUnit = lm/W;
    attribute <'lx⋅s'> 'lux second' : LuminousExposureUnit = lx*s;
    attribute <'m⋅s⁻¹'> 'metre second to the power minus 1' : SpeedUnit = m*s^-1;
    attribute <'m⋅s⁻²'> 'metre second to the power minus 2' : AccelerationUnit = m*s^-2;
    attribute <'m/s'> 'metre per second' : SpeedUnit = m/s;
    attribute <'m⁻¹'> 'metre to the power minus 1' : CurvatureUnit = m^-1;
    attribute <'m⁻²'> 'metre to the power minus 2' : PhotonExposureUnit = m^-2;
    attribute <'m⁻²⋅s⁻¹'> 'metre to the power minus 2 second to the power minus 1' : PhotonIrradianceUnit = m^-2*s^-1;
    attribute <'m⁻²⋅s⁻¹⋅sr⁻¹'> 'metre to the power minus 2 second to the power minus 1 steradian to the power minus 1' : PhotonRadianceUnit = m^-2*s^-1*sr^-1;
    attribute <'m⁻³'> 'metre to the power minus 3' : ParticleConcentrationUnit = m^-3;
    attribute <'m⁻³⋅s'> 'metre to the power minus 3 second' : DensityOfVibrationalStatesUnit = m^-3*s;
    attribute <'m⁻³⋅s⁻¹'> 'metre to the power minus 3 second to the power minus 1' : ActivityDensityUnit = m^-3*s^-1;
    attribute <'m²'> 'metre squared' : AreaUnit = m^2;
    attribute <'m²⋅A'> 'metre squared ampere' : MagneticDipoleMomentUnit = m^2*A;
    attribute <'m²⋅K/W'> 'metre squared kelvin per watt' : ThermalInsulanceUnit = m^2*K/W;
    attribute <'m²⋅mol⁻¹'> 'metre squared mole to the power minus 1' : MolarAbsorptionCoefficientUnit = m^2*mol^-1;
    attribute <'m²⋅s⁻¹'> 'metre squared second to the power minus 1' : KinematicViscosityUnit = m^2*s^-1;
    attribute <'m²⋅s⁻²'> 'metre squared second to the power minus 2' : SpecificEnergyUnit = m^2*s^-2;
    attribute <'m²⋅s⁻²⋅K⁻¹'> 'metre squared second to the power minus 2 kelvin to the power minus 1' : SpecificHeatCapacityUnit = m^2*s^-2*K^-1;
    attribute <'m²⋅s⁻³'> 'metre squared second to the power minus 3' : DoseEquivalentUnit = m^2*s^-3;
    attribute <'m²⋅sr⁻¹'> 'metre squared steradian to the power minus 1' : DirectionDistributionOfCrossSectionUnit = m^2*sr^-1;
    attribute <'m²/(J⋅sr)'> 'metre squared per joule steradian' : DirectionAndEnergyDistributionOfCrossSectionUnit = m^2/(J*sr);
    attribute <'m²/(V⋅s)'> 'metre squared per volt second' : MobilityUnit = m^2/(V*s);
    attribute <'m²/J'> 'metre squared per joule' : EnergyDistributionOfCrossSectionUnit = m^2/J;
    attribute <'m³'> 'metre cubed' : VolumeUnit = m^3;
    attribute <'m³⋅mol⁻¹'> 'metre cubed mole to the power minus 1' : MolarVolumeUnit = m^3*mol^-1;
    attribute <'m³⋅s⁻¹'> 'metre cubed second to the power minus 1' : VolumeFlowRateUnit = m^3*s^-1;
    attribute <'m³/C⋅m³⋅s⁻¹⋅A⁻¹'> 'metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1' : HallCoefficientUnit = m^3/C*m^3*s^-1*A^-1;
    attribute <'m⁴'> 'metre to the power 4' : SecondAxialMomentOfAreaUnit = m^4;
    attribute <'m⁴⋅s⁻²'> 'metre to the power 4 second to the power minus 2' : TotalMassStoppingPowerUnit = m^4*s^-2;
    attribute <'mL/L '> 'ml per l' : VolumeFractionUnit = mL/L;
    attribute <'mol⋅kg⁻¹'> 'mole kilogram to the power minus 1' : IonicStrengthUnit = mol*kg^-1;
    attribute <'mol⋅m⁻³'> 'mole metre to the power minus 3' : AmountOfSubstanceConcentrationUnit = mol*m^-3;
    attribute <'mol/kg'> 'mole per kilogram' : MolalityUnit = mol/kg;
    attribute <'mol/L'> 'mole per l' : AmountOfSubstanceConcentrationUnit = mol/L;
    attribute <'mol/m³'> 'mole per cubic metre' : EquilibriumConstantOnConcentrationBasisUnit = mol/m^3;
    attribute <'N⋅m'> 'newton metre' : MomentOfForceUnit, TorqueUnit = N*m {
        attribute :>> MomentOfForceUnit::quantityDimension, TorqueUnit::quantityDimension {
            :>> MomentOfForceUnit::quantityDimension::quantityPowerFactors, TorqueUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <'N⋅m⋅s'> 'newton metre second' : AngularImpulseUnit = N*m*s;
    attribute <'N⋅m⋅s⁻¹'> 'newton metre second to the power minus 1' : PowerUnit = N*m*s^-1;
    attribute <'N⋅m⁻¹'> 'newton metre to the power minus 1' : SurfaceTensionUnit = N*m^-1;
    attribute <'N⋅m⁻²'> 'newton metre to the power minus 2' : PressureUnit = N*m^-2;
    attribute <'N⋅s'> 'newton second' : ImpulseUnit = N*s;
    attribute <'nat/s'> 'natural unit of information per second' : AverageInformationRateUnit = nat/s;
    attribute <'o/s'> 'octet per second' : TransferRateUnit = o/s;
    attribute <'Pa⋅s'> 'pascal second' : DynamicViscosityUnit = Pa*s;
    attribute <'Pa⋅s/m'> 'pascal second per metre' : CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit = Pa*s/m;
    attribute <'Pa⋅s/m³'> 'pascal second per cubic metre' : AcousticImpedanceUnit = Pa*s/m^3;
    attribute <'Pa/K'> 'pascal per kelvin' : PressureCoefficientUnit = Pa/K;
    attribute <'Pa⁻¹'> 'pascal to the power minus 1' : CompressibilityUnit = Pa^-1;
    attribute <'Pa²⋅s'> 'pascal to the power 2 second' : SoundExposureUnit = Pa^2*s;
    attribute <'rad⋅m²/kg¹'> 'radian metre squared per kilogram to the power 1' : SpecificOpticalRotatoryPowerUnit = rad*m^2/kg^1;
    attribute <'rad⋅m²/mol'> 'radian metre squared per mole' : MolarOpticalRotatoryPowerUnit = rad*m^2/mol;
    attribute <'rad⋅s⁻¹'> 'radian second to the power minus 1' : AngularVelocityUnit = rad*s^-1;
    attribute <'rad⋅s⁻²'> 'radian second to the power minus 2' : AngularAccelerationUnit = rad*s^-2;
    attribute <'rad/m'> 'radian per metre' : PhaseCoefficientUnit = rad/m;
    attribute <'s⋅A'> 'second ampere' : ElectricChargeUnit = s*A;
    attribute <'S⋅m²/mol'> 'siemens metre squared per mole' : MolarConductivityUnit = S*m^2/mol;
    attribute <'S/m'> 'siemens per metre' : ConductivityUnit = S/m;
    attribute <'s⁻¹'> 'second to the power minus 1' : AngularVelocityUnit = s^-1;
    attribute <'s⁻¹⋅sr⁻¹'> 'second to the power minus 1 steradian to the power minus 1' : PhotonIntensityUnit = s^-1*sr^-1;
    attribute <'s⁻²'> 'second to the power minus 2' : AngularAccelerationUnit = s^-2;
    attribute <'Sh/s'> 'shannon per second' : AverageInformationRateUnit = Sh/s;
    attribute <'Sv/s'> 'sievert per second' : DoseEquivalentUnit = Sv/s;
    attribute <'V⋅A'> 'volt ampere' : PowerUnit = V*A;
    attribute <'V/K'> 'volt per kelvin' : SeebeckCoefficientForSubstancesAAndBUnit = V/K;
    attribute <'V/m'> 'volt per metre' : ElectricFieldStrengthUnit = V/m;
    attribute <'V²/K²'> 'volt to the power 2 per kelvin to the power 2' : LorenzCoefficientUnit = V^2/K^2;
    attribute <'W⋅h'> 'watt hour' : EnergyUnit = W*h;
    attribute <'W/(m⋅K)'> 'watt per metre kelvin' : ThermalConductivityUnit = W/(m*K);
    attribute <'W/(m²⋅K)'> 'watt per square metre kelvin' : CoefficientOfHeatTransferUnit = W/(m^2*K);
    attribute <'W/(m²⋅nm)'> 'watt per square metre nm' : SpectralIrradianceUnit = W/(m^2*nm);
    attribute <'W/(sr⋅m²)'> 'watt per steradian square metre' : RadianceUnit = W/(sr*m^2);
    attribute <'W/(sr⋅m²⋅nm)'> 'watt per steradian square metre nm' : SpectralRadianceUnit = W/(sr*m^2*nm);
    attribute <'W/(sr⋅nm)'> 'watt per steradian nm' : SpectralRadiantIntensityUnit = W/(sr*nm);
    attribute <'W/K'> 'watt per kelvin' : ThermalConductanceUnit = W/K;
    attribute <'W/kg'> 'watt per kilogram' : DoseEquivalentUnit = W/kg;
    attribute <'W/m²'> 'watt per square metre' : DensityOfHeatFlowRateUnit = W/m^2;
    attribute <'W/nm'> 'watt per nm' : SpectralRadiantFluxUnit = W/nm;
    attribute <'W/sr'> 'watt per steradian' : RadiantIntensityUnit = W/sr;
    attribute <'Wb⋅m'> 'weber metre' : MagneticDipoleMomentUnit = Wb*m;
    attribute <'Wb/m'> 'weber per metre' : MagneticVectorPotentialUnit = Wb/m;
    attribute <'Ω⋅m'> 'ohm metre' : ResistivityUnit = 'Ω'*m;

    alias 'm/s²' for 'm⋅s⁻²';

    /*
     * Prefixed units
     */

    /* Length */
    attribute <nm> nanometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = nano; :>> referenceUnit = m; } }
    attribute <mm> millimetre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = m; } }
    attribute <cm> centimetre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = centi; :>> referenceUnit = m; } }
    attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }

    /* Volume */
    attribute <mL> millilitre : VolumeUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = L; } }

    /* Force */
    attribute <mN> millinewton : ForceUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = N; } }

    /* Energy */
    attribute <kJ> kilojoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = J; } }
    attribute <MJ> megajoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = mega; :>> referenceUnit = J; } }
    attribute <GJ> gigajoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = giga; :>> referenceUnit = J; } }

    /* Power */
    attribute <kW> kilowatt : PowerUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = W; } }

    /* Speed */
    attribute <'km/h'> 'kilometre per hour': SpeedUnit = km/h;

	/* 
	 * Celsius units
	 */
	 
    attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit {
    	doc
	    /*
	     * degree Celsius unit for temperature interval (i.e. temperature difference) quantities
	     */
	     
        attribute :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; }
   	}

    attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
    	doc
	     /*
	     * degree Celsius interval scale for absolute (thermodynamic) temperature quantities
	     *
	     * The interval scale is defined with an explicit transformation with respect to 
	     * the kelvin thermodynamic temperature scale that specifies the zero shift.
	     */
 
        attribute :>> unit = '°C';
        attribute temperatureWaterAtFreezingPointInC: DefinitionalQuantityValue {
            :>> num = 0; :>> definition = "temperature in degree Celsius of pure water at freezing point";
        }
        private attribute temperatureWaterAtTriplePointInC: DefinitionalQuantityValue {
            :>> num = 1/100; :>> definition = "temperature in degree Celsius of pure water at the triple point";
        }
        private attribute celsiusToKelvinScaleMapping: QuantityValueMapping {
            :>> mappedQuantityValue = temperatureWaterAtTriplePointInC; 
            :>> referenceQuantityValue = K.temperatureOfWaterAtTriplePointInK;
        }
        attribute :>> definitionalQuantityValues = (temperatureWaterAtTriplePointInC, temperatureWaterAtFreezingPointInC);
        attribute :>> quantityValueMapping = celsiusToKelvinScaleMapping;

        /* CoordinateFramePlacement (zero shift) w.r.t. the kelvin thermodynamic temperature scale */
        private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
        attribute zeroDegreeCelsiusToKelvinShift : CoordinateFramePlacement :>> transformation { 
        	:>> source = K; :>> origin = zeroDegreeCelsiusInKelvin;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'ElectricCurrentUnit'
semantic.unresolved_name 'ThermodynamicTemperatureUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'AmountOfSubstanceUnit'
semantic.unresolved_name 'LuminousIntensityUnit'
semantic.unresolved_name 'SystemOfUnits'
semantic.unresolved_name 'systemOfQuantities'
semantic.unresolved_name 'baseUnits'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'ModulationRateUnit'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'NuclearActivityUnit'
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'SoundPressureLevelUnit'
semantic.unresolved_name 'LogarithmicFrequencyRangeUnit'
semantic.unresolved_name 'TrafficIntensityUnit'
semantic.unresolved_name 'CapacitanceUnit'
semantic.unresolved_name 'AbsorbedDoseUnit'
semantic.unresolved_name 'PermeanceUnit'
semantic.unresolved_name 'InductanceUnit'
semantic.unresolved_name 'PermeanceUnit::quantityDimension'
semantic.unresolved_name 'InductanceUnit::quantityDimension'
semantic.unresolved_name 'PermeanceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'InductanceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'FrequencyUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'LuminousFluxUnit'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'LogarithmicFrequencyRangeUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'ConductanceUnit'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'SolidAngularMeasureUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'MagneticFluxDensityUnit'
semantic.unresolved_name 'ElectricPotentialUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'MagneticFluxUnit'
semantic.unresolved_name 'ResistanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'RichardsonConstantUnit'
semantic.unresolved_name 'MagneticMomentUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'LinearElectricCurrentDensityUnit'
semantic.unresolved_name 'ElectricCurrentDensityUnit'
semantic.unresolved_name 'TransferRateUnit'
semantic.unresolved_name 'BinaryDigitRateUnit'
semantic.unresolved_name 'SpecificActivityUnit'
semantic.unresolved_name 'SurfaceActivityDensityUnit'
semantic.unresolved_name 'ActivityDensityUnit'
semantic.unresolved_name 'ElectricDipoleMomentUnit'
semantic.unresolved_name 'ExposureRateUnit'
semantic.unresolved_name 'ExposureUnit'
semantic.unresolved_name 'LinearDensityOfElectricChargeUnit'
semantic.unresolved_name 'SurfaceDensityOfElectricChargeUnit'
semantic.unresolved_name 'ElectricChargeDensityUnit'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'LuminousFluxUnit'
semantic.unresolved_name 'LuminousEfficacyOfRadiationUnit'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'LuminousExposureUnit'
semantic.unresolved_name 'LuminousEnergyUnit'
semantic.unresolved_name 'HartreeEnergyUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'TotalLinearStoppingPowerUnit'
semantic.unresolved_name 'EnergyFluenceUnit'
semantic.unresolved_name 'ElectricConstantUnit'
semantic.unresolved_name 'MassConcentrationUnit'
semantic.unresolved_name 'MolarMassUnit'
semantic.unresolved_name 'AbsorbedDoseRateUnit'
semantic.unresolved_name 'MagneticConstantUnit'
semantic.unresolved_name 'ReluctanceUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'ActionQuantityUnit'
semantic.unresolved_name 'TotalAngularMomentumUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpectralRadiantExposureUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'
semantic.unresolved_name 'MolarHeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'TotalLinearStoppingPowerUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit'
semantic.unresolved_name 'ElectromagneticEnergyDensityUnit'
semantic.unresolved_name 'MolarInternalEnergyUnit'
semantic.unresolved_name 'SpectralRadiantEnergyUnit'
semantic.unresolved_name 'HeatFlowRateUnit'
semantic.unresolved_name 'EnergyDensityOfStatesUnit'
semantic.unresolved_name 'JouleThomsonCoefficientUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'LinearExpansionCoefficientUnit'
semantic.unresolved_name 'MomentumUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'SpectralRadiantFluxUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'SpectralRadiantIntensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'PressureCoefficientUnit'
semantic.unresolved_name 'SpectralIrradianceUnit'
semantic.unresolved_name 'SpectralRadianceUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'MassFlowUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'AcousticImpedanceUnit'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'AngularMomentumUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'MolarHeatCapacityUnit'
semantic.unresolved_name 'MolarInternalEnergyUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'ElectricPotentialDifferenceUnit'
semantic.unresolved_name 'SeebeckCoefficientForSubstancesAAndBUnit'
semantic.unresolved_name 'ThermalConductanceUnit'
semantic.unresolved_name 'RadiantIntensityUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'MolarMassUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'MagneticFluxDensityUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'RadianceUnit'
semantic.unresolved_name 'ExposureRateUnit'
semantic.unresolved_name 'CompressibilityUnit'
semantic.unresolved_name 'JouleThomsonCoefficientUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ElectrolyticConductivityUnit'
semantic.unresolved_name 'EnergyDensityOfStatesUnit'
semantic.unresolved_name 'MassAttenuationCoefficientUnit'
semantic.unresolved_name 'SpecificVolumeUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'SpecificActivityUnit'
semantic.unresolved_name 'EnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'MobilityUnit'
semantic.unresolved_name 'MolarConductivityUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'SoundExposureUnit'
semantic.unresolved_name 'LorenzCoefficientUnit'
semantic.unresolved_name 'LuminousEnergyUnit'
semantic.unresolved_name 'LuminousExitanceUnit'
semantic.unresolved_name 'LuminousEfficacyOfRadiationUnit'
semantic.unresolved_name 'LuminousExposureUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'CurvatureUnit'
semantic.unresolved_name 'PhotonExposureUnit'
semantic.unresolved_name 'PhotonIrradianceUnit'
semantic.unresolved_name 'PhotonRadianceUnit'
semantic.unresolved_name 'ParticleConcentrationUnit'
semantic.unresolved_name 'DensityOfVibrationalStatesUnit'
semantic.unresolved_name 'ActivityDensityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'MagneticDipoleMomentUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'MolarAbsorptionCoefficientUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'DirectionDistributionOfCrossSectionUnit'
semantic.unresolved_name 'DirectionAndEnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'MobilityUnit'
semantic.unresolved_name 'EnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'MolarVolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'HallCoefficientUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'VolumeFractionUnit'
semantic.unresolved_name 'IonicStrengthUnit'
semantic.unresolved_name 'AmountOfSubstanceConcentrationUnit'
semantic.unresolved_name 'MolalityUnit'
semantic.unresolved_name 'AmountOfSubstanceConcentrationUnit'
semantic.unresolved_name 'EquilibriumConstantOnConcentrationBasisUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'TorqueUnit'
semantic.unresolved_name 'MomentOfForceUnit::quantityDimension'
semantic.unresolved_name 'TorqueUnit::quantityDimension'
semantic.unresolved_name 'MomentOfForceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TorqueUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'AngularImpulseUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'ImpulseUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'TransferRateUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit'
semantic.unresolved_name 'AcousticImpedanceUnit'
semantic.unresolved_name 'PressureCoefficientUnit'
semantic.unresolved_name 'CompressibilityUnit'
semantic.unresolved_name 'SoundExposureUnit'
semantic.unresolved_name 'SpecificOpticalRotatoryPowerUnit'
semantic.unresolved_name 'MolarOpticalRotatoryPowerUnit'
semantic.unresolved_name 'AngularVelocityUnit'
semantic.unresolved_name 'AngularAccelerationUnit'
semantic.unresolved_name 'PhaseCoefficientUnit'
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'MolarConductivityUnit'
semantic.unresolved_name 'ConductivityUnit'
semantic.unresolved_name 'AngularVelocityUnit'
semantic.unresolved_name 'PhotonIntensityUnit'
semantic.unresolved_name 'AngularAccelerationUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SeebeckCoefficientForSubstancesAAndBUnit'
semantic.unresolved_name 'ElectricFieldStrengthUnit'
semantic.unresolved_name 'LorenzCoefficientUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'SpectralIrradianceUnit'
semantic.unresolved_name 'RadianceUnit'
semantic.unresolved_name 'SpectralRadianceUnit'
semantic.unresolved_name 'SpectralRadiantIntensityUnit'
semantic.unresolved_name 'ThermalConductanceUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'SpectralRadiantFluxUnit'
semantic.unresolved_name 'RadiantIntensityUnit'
semantic.unresolved_name 'MagneticDipoleMomentUnit'
semantic.unresolved_name 'MagneticVectorPotentialUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'QuantityValueMapping'
semantic.unresolved_name 'mappedQuantityValue'
semantic.unresolved_name 'referenceQuantityValue'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'quantityValueMapping'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'CoordinateFramePlacement'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'source'
semantic.unresolved_name 'origin'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'ElectricCurrentUnit'
semantic.unresolved_name 'ThermodynamicTemperatureUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension'
semantic.unresolved_name 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'AmountOfSubstanceUnit'
semantic.unresolved_name 'LuminousIntensityUnit'
semantic.unresolved_name 'SystemOfUnits'
semantic.unresolved_name 'systemOfQuantities'
semantic.unresolved_name 'baseUnits'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'ModulationRateUnit'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'NuclearActivityUnit'
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'SoundPressureLevelUnit'
semantic.unresolved_name 'LogarithmicFrequencyRangeUnit'
semantic.unresolved_name 'TrafficIntensityUnit'
semantic.unresolved_name 'CapacitanceUnit'
semantic.unresolved_name 'AbsorbedDoseUnit'
semantic.unresolved_name 'PermeanceUnit'
semantic.unresolved_name 'InductanceUnit'
semantic.unresolved_name 'PermeanceUnit::quantityDimension'
semantic.unresolved_name 'InductanceUnit::quantityDimension'
semantic.unresolved_name 'PermeanceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'InductanceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'FrequencyUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'LuminousFluxUnit'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'StorageCapacityUnit'
semantic.unresolved_name 'LogarithmicFrequencyRangeUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'ConductanceUnit'
semantic.unresolved_name 'InformationContentUnit'
semantic.unresolved_name 'SolidAngularMeasureUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'MagneticFluxDensityUnit'
semantic.unresolved_name 'ElectricPotentialUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'MagneticFluxUnit'
semantic.unresolved_name 'ResistanceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'DurationUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'MassUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'AngularMeasureUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'isExact'
semantic.unresolved_name 'RichardsonConstantUnit'
semantic.unresolved_name 'MagneticMomentUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'LinearElectricCurrentDensityUnit'
semantic.unresolved_name 'ElectricCurrentDensityUnit'
semantic.unresolved_name 'TransferRateUnit'
semantic.unresolved_name 'BinaryDigitRateUnit'
semantic.unresolved_name 'SpecificActivityUnit'
semantic.unresolved_name 'SurfaceActivityDensityUnit'
semantic.unresolved_name 'ActivityDensityUnit'
semantic.unresolved_name 'ElectricDipoleMomentUnit'
semantic.unresolved_name 'ExposureRateUnit'
semantic.unresolved_name 'ExposureUnit'
semantic.unresolved_name 'LinearDensityOfElectricChargeUnit'
semantic.unresolved_name 'SurfaceDensityOfElectricChargeUnit'
semantic.unresolved_name 'ElectricChargeDensityUnit'
semantic.unresolved_name 'LuminanceUnit'
semantic.unresolved_name 'LuminousFluxUnit'
semantic.unresolved_name 'LuminousEfficacyOfRadiationUnit'
semantic.unresolved_name 'IlluminanceUnit'
semantic.unresolved_name 'LuminousExposureUnit'
semantic.unresolved_name 'LuminousEnergyUnit'
semantic.unresolved_name 'HartreeEnergyUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'TotalLinearStoppingPowerUnit'
semantic.unresolved_name 'EnergyFluenceUnit'
semantic.unresolved_name 'ElectricConstantUnit'
semantic.unresolved_name 'MassConcentrationUnit'
semantic.unresolved_name 'MolarMassUnit'
semantic.unresolved_name 'AbsorbedDoseRateUnit'
semantic.unresolved_name 'MagneticConstantUnit'
semantic.unresolved_name 'ReluctanceUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'ActionQuantityUnit'
semantic.unresolved_name 'TotalAngularMomentumUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'SpectralRadiantExposureUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'
semantic.unresolved_name 'MolarHeatCapacityUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'TotalLinearStoppingPowerUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit'
semantic.unresolved_name 'ElectromagneticEnergyDensityUnit'
semantic.unresolved_name 'MolarInternalEnergyUnit'
semantic.unresolved_name 'SpectralRadiantEnergyUnit'
semantic.unresolved_name 'HeatFlowRateUnit'
semantic.unresolved_name 'EnergyDensityOfStatesUnit'
semantic.unresolved_name 'JouleThomsonCoefficientUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'LinearExpansionCoefficientUnit'
semantic.unresolved_name 'MomentumUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'SpectralRadiantFluxUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'SpectralRadiantIntensityUnit'
semantic.unresolved_name 'LinearMassDensityUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'PressureCoefficientUnit'
semantic.unresolved_name 'SpectralIrradianceUnit'
semantic.unresolved_name 'SpectralRadianceUnit'
semantic.unresolved_name 'SurfaceMassDensityUnit'
semantic.unresolved_name 'MassFlowUnit'
semantic.unresolved_name 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'
semantic.unresolved_name 'MassDensityUnit'
semantic.unresolved_name 'AcousticImpedanceUnit'
semantic.unresolved_name 'MomentOfInertiaUnit'
semantic.unresolved_name 'AngularMomentumUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'HeatCapacityUnit'
semantic.unresolved_name 'MolarHeatCapacityUnit'
semantic.unresolved_name 'MolarInternalEnergyUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'ElectricPotentialDifferenceUnit'
semantic.unresolved_name 'SeebeckCoefficientForSubstancesAAndBUnit'
semantic.unresolved_name 'ThermalConductanceUnit'
semantic.unresolved_name 'RadiantIntensityUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'MolarMassUnit'
semantic.unresolved_name 'MassFlowRateUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'MagneticFluxDensityUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'RadianceUnit'
semantic.unresolved_name 'ExposureRateUnit'
semantic.unresolved_name 'CompressibilityUnit'
semantic.unresolved_name 'JouleThomsonCoefficientUnit'
semantic.unresolved_name 'ThermalResistanceUnit'
semantic.unresolved_name 'ElectrolyticConductivityUnit'
semantic.unresolved_name 'EnergyDensityOfStatesUnit'
semantic.unresolved_name 'MassAttenuationCoefficientUnit'
semantic.unresolved_name 'SpecificVolumeUnit'
semantic.unresolved_name 'GyromagneticRatioUnit'
semantic.unresolved_name 'SpecificActivityUnit'
semantic.unresolved_name 'EnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'MobilityUnit'
semantic.unresolved_name 'MolarConductivityUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'SoundExposureUnit'
semantic.unresolved_name 'LorenzCoefficientUnit'
semantic.unresolved_name 'LuminousEnergyUnit'
semantic.unresolved_name 'LuminousExitanceUnit'
semantic.unresolved_name 'LuminousEfficacyOfRadiationUnit'
semantic.unresolved_name 'LuminousExposureUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'AccelerationUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'CurvatureUnit'
semantic.unresolved_name 'PhotonExposureUnit'
semantic.unresolved_name 'PhotonIrradianceUnit'
semantic.unresolved_name 'PhotonRadianceUnit'
semantic.unresolved_name 'ParticleConcentrationUnit'
semantic.unresolved_name 'DensityOfVibrationalStatesUnit'
semantic.unresolved_name 'ActivityDensityUnit'
semantic.unresolved_name 'AreaUnit'
semantic.unresolved_name 'MagneticDipoleMomentUnit'
semantic.unresolved_name 'ThermalInsulanceUnit'
semantic.unresolved_name 'MolarAbsorptionCoefficientUnit'
semantic.unresolved_name 'KinematicViscosityUnit'
semantic.unresolved_name 'SpecificEnergyUnit'
semantic.unresolved_name 'SpecificHeatCapacityUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'DirectionDistributionOfCrossSectionUnit'
semantic.unresolved_name 'DirectionAndEnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'MobilityUnit'
semantic.unresolved_name 'EnergyDistributionOfCrossSectionUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'MolarVolumeUnit'
semantic.unresolved_name 'VolumeFlowRateUnit'
semantic.unresolved_name 'HallCoefficientUnit'
semantic.unresolved_name 'SecondAxialMomentOfAreaUnit'
semantic.unresolved_name 'TotalMassStoppingPowerUnit'
semantic.unresolved_name 'VolumeFractionUnit'
semantic.unresolved_name 'IonicStrengthUnit'
semantic.unresolved_name 'AmountOfSubstanceConcentrationUnit'
semantic.unresolved_name 'MolalityUnit'
semantic.unresolved_name 'AmountOfSubstanceConcentrationUnit'
semantic.unresolved_name 'EquilibriumConstantOnConcentrationBasisUnit'
semantic.unresolved_name 'MomentOfForceUnit'
semantic.unresolved_name 'TorqueUnit'
semantic.unresolved_name 'MomentOfForceUnit::quantityDimension'
semantic.unresolved_name 'TorqueUnit::quantityDimension'
semantic.unresolved_name 'MomentOfForceUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'TorqueUnit::quantityDimension::quantityPowerFactors'
semantic.unresolved_name 'AngularImpulseUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SurfaceTensionUnit'
semantic.unresolved_name 'PressureUnit'
semantic.unresolved_name 'ImpulseUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'TransferRateUnit'
semantic.unresolved_name 'DynamicViscosityUnit'
semantic.unresolved_name 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit'
semantic.unresolved_name 'AcousticImpedanceUnit'
semantic.unresolved_name 'PressureCoefficientUnit'
semantic.unresolved_name 'CompressibilityUnit'
semantic.unresolved_name 'SoundExposureUnit'
semantic.unresolved_name 'SpecificOpticalRotatoryPowerUnit'
semantic.unresolved_name 'MolarOpticalRotatoryPowerUnit'
semantic.unresolved_name 'AngularVelocityUnit'
semantic.unresolved_name 'AngularAccelerationUnit'
semantic.unresolved_name 'PhaseCoefficientUnit'
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'MolarConductivityUnit'
semantic.unresolved_name 'ConductivityUnit'
semantic.unresolved_name 'AngularVelocityUnit'
semantic.unresolved_name 'PhotonIntensityUnit'
semantic.unresolved_name 'AngularAccelerationUnit'
semantic.unresolved_name 'AverageInformationRateUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'SeebeckCoefficientForSubstancesAAndBUnit'
semantic.unresolved_name 'ElectricFieldStrengthUnit'
semantic.unresolved_name 'LorenzCoefficientUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'ThermalConductivityUnit'
semantic.unresolved_name 'CoefficientOfHeatTransferUnit'
semantic.unresolved_name 'SpectralIrradianceUnit'
semantic.unresolved_name 'RadianceUnit'
semantic.unresolved_name 'SpectralRadianceUnit'
semantic.unresolved_name 'SpectralRadiantIntensityUnit'
semantic.unresolved_name 'ThermalConductanceUnit'
semantic.unresolved_name 'DoseEquivalentUnit'
semantic.unresolved_name 'DensityOfHeatFlowRateUnit'
semantic.unresolved_name 'SpectralRadiantFluxUnit'
semantic.unresolved_name 'RadiantIntensityUnit'
semantic.unresolved_name 'MagneticDipoleMomentUnit'
semantic.unresolved_name 'MagneticVectorPotentialUnit'
semantic.unresolved_name 'ResistivityUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'LengthUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'ForceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'EnergyUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByPrefix'
semantic.unresolved_name 'prefix'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'SpeedUnit'
semantic.unresolved_name 'TemperatureDifferenceUnit'
semantic.unresolved_name 'unitConversion'
semantic.unresolved_name 'ConversionByConvention'
semantic.unresolved_name 'referenceUnit'
semantic.unresolved_name 'conversionFactor'
semantic.unresolved_name 'IntervalScale'
semantic.unresolved_name 'unit'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'DefinitionalQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'definition'
semantic.unresolved_name 'QuantityValueMapping'
semantic.unresolved_name 'mappedQuantityValue'
semantic.unresolved_name 'referenceQuantityValue'
semantic.unresolved_name 'definitionalQuantityValues'
semantic.unresolved_name 'quantityValueMapping'
semantic.unresolved_name 'ThermodynamicTemperatureValue'
semantic.unresolved_name 'CoordinateFramePlacement'
semantic.unresolved_name 'transformation'
semantic.unresolved_name 'source'
semantic.unresolved_name 'origin'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Comma,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Slash,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Semicolon,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Comma,Ident,Eq,Ident,Slash,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
LineComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,UnrestrictedName,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
RegularComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,UnrestrictedName,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,CloseCurly,CloseCurly,
KwAlias,UnrestrictedName,KwFor,Ident,Semicolon,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,KwVar,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Star,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,LineComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,ExponentialValue,Semicolon,ColonGtGt,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
RegularComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Comma,Ident,Eq,Ident,Star,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,
ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Caret,DecimalValue,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Star,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,Minus,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Caret,DecimalValue,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,Caret,DecimalValue,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,UnrestrictedName,Star,Ident,Semicolon,
KwAlias,UnrestrictedName,KwFor,UnrestrictedName,Semicolon,
RegularComment,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
RegularComment,
KwAttribute,OpenAngle,Ident,CloseAngle,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,CloseCurly,CloseCurly,
RegularComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
RegularComment,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,UnrestrictedName,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Slash,DecimalValue,Semicolon,ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
RegularComment,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SI'
    (documentation)
    (import_decl private 'MeasurementReferences::*')
    (import_decl public 'ISQ::*')
    (import_decl public 'SIPrefixes::*')
    (comment)
    (attribute_usage 'gram' : 'MassUnit')
    (comment)
    (attribute_usage 'metre' : 'LengthUnit')
    (attribute_usage 'kilogram' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'second' : 'DurationUnit')
    (attribute_usage 'ampere' : 'ElectricCurrentUnit')
    (attribute_usage 'kelvin' : 'ThermodynamicTemperatureUnit', 'TemperatureDifferenceUnit'
      (attribute_usage 'temperatureOfWaterAtTriplePointInK' : 'DefinitionalQuantityValue'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value))
      (attribute_usage :>> 'definitionalQuantityValues' value)
      (attribute_usage :>> 'ThermodynamicTemperatureUnit::quantityDimension', 'TemperatureDifferenceUnit::quantityDimension'
        (default_ref_usage :>> 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors', 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors')))
    (attribute_usage 'mole' : 'AmountOfSubstanceUnit')
    (attribute_usage 'candela' : 'LuminousIntensityUnit')
    (comment)
    (attribute_usage ''ISO/IEC 80000 International System of Units'' : 'SystemOfUnits'
      (default_ref_usage :>> 'systemOfQuantities' value)
      (default_ref_usage :>> 'baseUnits' value))
    (comment)
    (attribute_usage 'byte' : 'StorageCapacityUnit' value)
    (attribute_usage 'baud' : 'ModulationRateUnit' value)
    (attribute_usage 'bit' : 'StorageCapacityUnit' value)
    (attribute_usage 'becquerel' : 'NuclearActivityUnit' value)
    (attribute_usage 'coulomb' : 'ElectricChargeUnit' value)
    (attribute_usage 'decibel' : 'SoundPressureLevelUnit' value)
    (attribute_usage 'decade' : 'LogarithmicFrequencyRangeUnit' value)
    (attribute_usage 'erlang' : 'TrafficIntensityUnit' value)
    (attribute_usage 'farad' : 'CapacitanceUnit' value)
    (attribute_usage 'gray' : 'AbsorbedDoseUnit' value)
    (attribute_usage 'henry' : 'PermeanceUnit', 'InductanceUnit' value
      (attribute_usage :>> 'PermeanceUnit::quantityDimension', 'InductanceUnit::quantityDimension'
        (default_ref_usage :>> 'PermeanceUnit::quantityDimension::quantityPowerFactors', 'InductanceUnit::quantityDimension::quantityPowerFactors')))
    (attribute_usage 'hartley' : 'InformationContentUnit' value)
    (attribute_usage 'hertz' : 'FrequencyUnit' value)
    (attribute_usage 'joule' : 'EnergyUnit' value)
    (line_comment)
    (attribute_usage 'lumen' : 'LuminousFluxUnit' value)
    (attribute_usage 'lux' : 'IlluminanceUnit' value)
    (attribute_usage 'newton' : 'ForceUnit' value)
    (attribute_usage ''natural unit of information'' : 'InformationContentUnit' value)
    (attribute_usage 'octet' : 'StorageCapacityUnit' value)
    (attribute_usage 'octave' : 'LogarithmicFrequencyRangeUnit' value)
    (attribute_usage 'pascal' : 'PressureUnit' value)
    (attribute_usage 'radian' : 'AngularMeasureUnit' value)
    (attribute_usage 'siemens' : 'ConductanceUnit' value)
    (attribute_usage 'shannon' : 'InformationContentUnit' value)
    (attribute_usage 'steradian' : 'SolidAngularMeasureUnit' value)
    (attribute_usage 'sievert' : 'DoseEquivalentUnit' value)
    (attribute_usage 'tesla' : 'MagneticFluxDensityUnit' value)
    (attribute_usage 'volt' : 'ElectricPotentialUnit' value)
    (attribute_usage 'watt' : 'PowerUnit' value)
    (attribute_usage 'weber' : 'MagneticFluxUnit' value)
    (attribute_usage 'ohm' : 'ResistanceUnit' value)
    (comment)
    (attribute_usage ''ångström'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'barn' : 'AreaUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'day' : 'DurationUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'dalton' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage 'electronvolt' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage 'hour' : 'DurationUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'minute' : 'DurationUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'litre' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'tonne' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (alias_member ''metric ton'' for 'tonne')
    (attribute_usage ''atomic mass unit'' : 'MassUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''astronomical unit'' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (attribute_usage ''volt ampere reactive'' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage 'degree' : 'AngularMeasureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (line_comment)
    (attribute_usage ''minute (angle)'' : 'AngularMeasureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (alias_member 'arcmin' for ''′'')
    (attribute_usage ''second (angle)'' : 'AngularMeasureUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)
        (default_ref_usage :>> 'isExact' value)))
    (alias_member 'arcsec' for ''″'')
    (comment)
    (attribute_usage ''ampere metre to the power minus 2 kelvin to the power minus 2'' : 'RichardsonConstantUnit' value)
    (attribute_usage ''ampere metre squared'' : 'MagneticMomentUnit' value)
    (attribute_usage ''ampere metre squared joule to the power minus 1 second to the power minus 1'' : 'GyromagneticRatioUnit' value)
    (attribute_usage ''ampere second per kilogram'' : 'GyromagneticRatioUnit' value)
    (attribute_usage ''ampere per metre'' : 'LinearElectricCurrentDensityUnit' value)
    (attribute_usage ''ampere per square metre'' : 'ElectricCurrentDensityUnit' value)
    (attribute_usage ''byte per second'' : 'TransferRateUnit' value)
    (attribute_usage ''bit per second'' : 'BinaryDigitRateUnit' value)
    (attribute_usage ''becquerel per kilogram'' : 'SpecificActivityUnit' value)
    (attribute_usage ''becquerel per square metre'' : 'SurfaceActivityDensityUnit' value)
    (attribute_usage ''becquerel per cubic metre'' : 'ActivityDensityUnit' value)
    (attribute_usage ''coulomb metre'' : 'ElectricDipoleMomentUnit' value)
    (attribute_usage ''coulomb per kilogram second'' : 'ExposureRateUnit' value)
    (attribute_usage ''coulomb per kilogram'' : 'ExposureUnit' value)
    (attribute_usage ''coulomb per metre'' : 'LinearDensityOfElectricChargeUnit' value)
    (attribute_usage ''coulomb per square metre'' : 'SurfaceDensityOfElectricChargeUnit' value)
    (attribute_usage ''coulomb per cubic metre'' : 'ElectricChargeDensityUnit' value)
    (attribute_usage ''candela metre to the power minus 2'' : 'LuminanceUnit' value)
    (attribute_usage ''candela steradian'' : 'LuminousFluxUnit' value)
    (attribute_usage ''candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3'' : 'LuminousEfficacyOfRadiationUnit' value)
    (attribute_usage ''candela steradian metre to the power minus 2'' : 'IlluminanceUnit' value)
    (attribute_usage ''candela steradian metre to the power minus 2 second'' : 'LuminousExposureUnit' value)
    (attribute_usage ''candela steradian second'' : 'LuminousEnergyUnit' value)
    (attribute_usage ''electronvolt joule kilogram metre squared second to the power minus 2'' : 'HartreeEnergyUnit' value)
    (attribute_usage ''electronvolt metre to the power minus 2 per kilogram'' : 'TotalMassStoppingPowerUnit' value)
    (attribute_usage ''electronvolt per metre'' : 'TotalLinearStoppingPowerUnit' value)
    (attribute_usage ''electronvolt per square metre'' : 'EnergyFluenceUnit' value)
    (attribute_usage ''farad per metre'' : 'ElectricConstantUnit' value)
    (attribute_usage ''g per l'' : 'MassConcentrationUnit' value)
    (attribute_usage ''g per mole'' : 'MolarMassUnit' value)
    (attribute_usage ''gray per second'' : 'AbsorbedDoseRateUnit' value)
    (attribute_usage ''henry per metre'' : 'MagneticConstantUnit' value)
    (attribute_usage ''henry to the power minus 1'' : 'ReluctanceUnit' value)
    (attribute_usage ''hartley per second'' : 'AverageInformationRateUnit' value)
    (attribute_usage ''joule metre squared per kilogram'' : 'TotalMassStoppingPowerUnit' value)
    (attribute_usage ''joule second'' : 'ActionQuantityUnit' value)
    (attribute_usage ''joule second electronvolt second'' : 'TotalAngularMomentumUnit' value)
    (attribute_usage ''joule second to the power minus 1'' : 'PowerUnit' value)
    (attribute_usage ''joule per kilogram kelvin'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''joule per square metre nm'' : 'SpectralRadiantExposureUnit' value)
    (attribute_usage ''joule per cubic metre nm'' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit' value)
    (attribute_usage ''joule per mole kelvin'' : 'MolarHeatCapacityUnit' value)
    (attribute_usage ''joule per kelvin'' : 'HeatCapacityUnit' value)
    (attribute_usage ''joule per kilogram'' : 'SpecificEnergyUnit' value)
    (attribute_usage ''joule per metre'' : 'TotalLinearStoppingPowerUnit' value)
    (attribute_usage ''joule per square metre'' : 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit' value)
    (attribute_usage ''joule per cubic metre'' : 'ElectromagneticEnergyDensityUnit' value)
    (attribute_usage ''joule per mole'' : 'MolarInternalEnergyUnit' value)
    (attribute_usage ''joule per nm'' : 'SpectralRadiantEnergyUnit' value)
    (attribute_usage ''joule per second'' : 'HeatFlowRateUnit' value)
    (attribute_usage ''joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3'' : 'EnergyDensityOfStatesUnit' value)
    (attribute_usage ''kelvin per pascal'' : 'JouleThomsonCoefficientUnit' value)
    (attribute_usage ''kelvin per watt'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''kelvin to the power minus 1'' : 'LinearExpansionCoefficientUnit' value)
    (attribute_usage ''kilogram metre second to the power minus 1'' : 'MomentumUnit' value)
    (attribute_usage ''kilogram metre second to the power minus 2'' : 'ForceUnit' value)
    (attribute_usage ''kilogram metre second to the power minus 3'' : 'SpectralRadiantFluxUnit' value)
    (attribute_usage ''kilogram metre second to the power minus 3 kelvin to the power minus 1'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''kilogram metre second to the power minus 3 steradian to the power minus 1'' : 'SpectralRadiantIntensityUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1'' : 'LinearMassDensityUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1 second to the power minus 1'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1 second to the power minus 2'' : 'PressureUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1'' : 'PressureCoefficientUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1 second to the power minus 3'' : 'SpectralIrradianceUnit' value)
    (attribute_usage ''kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1'' : 'SpectralRadianceUnit' value)
    (attribute_usage ''kilogram metre to the power minus 2'' : 'SurfaceMassDensityUnit' value)
    (attribute_usage ''kilogram metre to the power minus 2 second to the power minus 1'' : 'MassFlowUnit' value)
    (attribute_usage ''kilogram metre to the power minus 2 second to the power minus 2'' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit' value)
    (attribute_usage ''kilogram metre to the power minus 3'' : 'MassDensityUnit' value)
    (attribute_usage ''kilogram metre to the power minus 4 second to the power minus 1'' : 'AcousticImpedanceUnit' value)
    (attribute_usage ''kilogram metre squared'' : 'MomentOfInertiaUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 1'' : 'AngularMomentumUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 2'' : 'MomentOfForceUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 2 kelvin to the power minus 1'' : 'HeatCapacityUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1'' : 'MolarHeatCapacityUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 2 mole to the power minus 1'' : 'MolarInternalEnergyUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 3'' : 'PowerUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 3 ampere to the power minus 1'' : 'ElectricPotentialDifferenceUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1'' : 'SeebeckCoefficientForSubstancesAAndBUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 3 kelvin to the power minus 1'' : 'ThermalConductanceUnit' value)
    (attribute_usage ''kilogram metre squared second to the power minus 3 steradian to the power minus 1'' : 'RadiantIntensityUnit' value)
    (attribute_usage ''kilogram metre cubed second to the power minus 3 ampere to the power minus 2'' : 'ResistivityUnit' value)
    (attribute_usage ''kilogram mole to the power minus 1'' : 'MolarMassUnit' value)
    (attribute_usage ''kilogram second to the power minus 1'' : 'MassFlowRateUnit' value)
    (attribute_usage ''kilogram second to the power minus 2'' : 'SurfaceTensionUnit' value)
    (attribute_usage ''kilogram second to the power minus 2 ampere to the power minus 1'' : 'MagneticFluxDensityUnit' value)
    (attribute_usage ''kilogram second to the power minus 3'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''kilogram second to the power minus 3 kelvin to the power minus 1'' : 'CoefficientOfHeatTransferUnit' value)
    (attribute_usage ''kilogram second to the power minus 3 steradian to the power minus 1'' : 'RadianceUnit' value)
    (attribute_usage ''kilogram to the power minus 1 ampere'' : 'ExposureRateUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre second to the power 2'' : 'CompressibilityUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre second to the power 2 kelvin'' : 'JouleThomsonCoefficientUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin'' : 'ThermalResistanceUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2'' : 'ElectrolyticConductivityUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre to the power minus 5 second to the power 2'' : 'EnergyDensityOfStatesUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre squared'' : 'MassAttenuationCoefficientUnit' value)
    (attribute_usage ''kilogram to the power minus 1 metre cubed'' : 'SpecificVolumeUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second ampere'' : 'GyromagneticRatioUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second to the power minus 1'' : 'SpecificActivityUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second to the power 2'' : 'EnergyDistributionOfCrossSectionUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second to the power 2 ampere'' : 'MobilityUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1'' : 'MolarConductivityUnit' value)
    (attribute_usage ''kilogram to the power minus 1 second to the power 3 kelvin'' : 'ThermalInsulanceUnit' value)
    (attribute_usage ''kilogram to the power 2 metre to the power minus 2 second to the power minus 3'' : 'SoundExposureUnit' value)
    (attribute_usage ''kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2'' : 'LorenzCoefficientUnit' value)
    (attribute_usage ''lumen second'' : 'LuminousEnergyUnit' value)
    (attribute_usage ''lumen per square metre'' : 'LuminousExitanceUnit' value)
    (attribute_usage ''lumen per watt'' : 'LuminousEfficacyOfRadiationUnit' value)
    (attribute_usage ''lux second'' : 'LuminousExposureUnit' value)
    (attribute_usage ''metre second to the power minus 1'' : 'SpeedUnit' value)
    (attribute_usage ''metre second to the power minus 2'' : 'AccelerationUnit' value)
    (attribute_usage ''metre per second'' : 'SpeedUnit' value)
    (attribute_usage ''metre to the power minus 1'' : 'CurvatureUnit' value)
    (attribute_usage ''metre to the power minus 2'' : 'PhotonExposureUnit' value)
    (attribute_usage ''metre to the power minus 2 second to the power minus 1'' : 'PhotonIrradianceUnit' value)
    (attribute_usage ''metre to the power minus 2 second to the power minus 1 steradian to the power minus 1'' : 'PhotonRadianceUnit' value)
    (attribute_usage ''metre to the power minus 3'' : 'ParticleConcentrationUnit' value)
    (attribute_usage ''metre to the power minus 3 second'' : 'DensityOfVibrationalStatesUnit' value)
    (attribute_usage ''metre to the power minus 3 second to the power minus 1'' : 'ActivityDensityUnit' value)
    (attribute_usage ''metre squared'' : 'AreaUnit' value)
    (attribute_usage ''metre squared ampere'' : 'MagneticDipoleMomentUnit' value)
    (attribute_usage ''metre squared kelvin per watt'' : 'ThermalInsulanceUnit' value)
    (attribute_usage ''metre squared mole to the power minus 1'' : 'MolarAbsorptionCoefficientUnit' value)
    (attribute_usage ''metre squared second to the power minus 1'' : 'KinematicViscosityUnit' value)
    (attribute_usage ''metre squared second to the power minus 2'' : 'SpecificEnergyUnit' value)
    (attribute_usage ''metre squared second to the power minus 2 kelvin to the power minus 1'' : 'SpecificHeatCapacityUnit' value)
    (attribute_usage ''metre squared second to the power minus 3'' : 'DoseEquivalentUnit' value)
    (attribute_usage ''metre squared steradian to the power minus 1'' : 'DirectionDistributionOfCrossSectionUnit' value)
    (attribute_usage ''metre squared per joule steradian'' : 'DirectionAndEnergyDistributionOfCrossSectionUnit' value)
    (attribute_usage ''metre squared per volt second'' : 'MobilityUnit' value)
    (attribute_usage ''metre squared per joule'' : 'EnergyDistributionOfCrossSectionUnit' value)
    (attribute_usage ''metre cubed'' : 'VolumeUnit' value)
    (attribute_usage ''metre cubed mole to the power minus 1'' : 'MolarVolumeUnit' value)
    (attribute_usage ''metre cubed second to the power minus 1'' : 'VolumeFlowRateUnit' value)
    (attribute_usage ''metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1'' : 'HallCoefficientUnit' value)
    (attribute_usage ''metre to the power 4'' : 'SecondAxialMomentOfAreaUnit' value)
    (attribute_usage ''metre to the power 4 second to the power minus 2'' : 'TotalMassStoppingPowerUnit' value)
    (attribute_usage ''ml per l'' : 'VolumeFractionUnit' value)
    (attribute_usage ''mole kilogram to the power minus 1'' : 'IonicStrengthUnit' value)
    (attribute_usage ''mole metre to the power minus 3'' : 'AmountOfSubstanceConcentrationUnit' value)
    (attribute_usage ''mole per kilogram'' : 'MolalityUnit' value)
    (attribute_usage ''mole per l'' : 'AmountOfSubstanceConcentrationUnit' value)
    (attribute_usage ''mole per cubic metre'' : 'EquilibriumConstantOnConcentrationBasisUnit' value)
    (attribute_usage ''newton metre'' : 'MomentOfForceUnit', 'TorqueUnit' value
      (attribute_usage :>> 'MomentOfForceUnit::quantityDimension', 'TorqueUnit::quantityDimension'
        (default_ref_usage :>> 'MomentOfForceUnit::quantityDimension::quantityPowerFactors', 'TorqueUnit::quantityDimension::quantityPowerFactors')))
    (attribute_usage ''newton metre second'' : 'AngularImpulseUnit' value)
    (attribute_usage ''newton metre second to the power minus 1'' : 'PowerUnit' value)
    (attribute_usage ''newton metre to the power minus 1'' : 'SurfaceTensionUnit' value)
    (attribute_usage ''newton metre to the power minus 2'' : 'PressureUnit' value)
    (attribute_usage ''newton second'' : 'ImpulseUnit' value)
    (attribute_usage ''natural unit of information per second'' : 'AverageInformationRateUnit' value)
    (attribute_usage ''octet per second'' : 'TransferRateUnit' value)
    (attribute_usage ''pascal second'' : 'DynamicViscosityUnit' value)
    (attribute_usage ''pascal second per metre'' : 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit' value)
    (attribute_usage ''pascal second per cubic metre'' : 'AcousticImpedanceUnit' value)
    (attribute_usage ''pascal per kelvin'' : 'PressureCoefficientUnit' value)
    (attribute_usage ''pascal to the power minus 1'' : 'CompressibilityUnit' value)
    (attribute_usage ''pascal to the power 2 second'' : 'SoundExposureUnit' value)
    (attribute_usage ''radian metre squared per kilogram to the power 1'' : 'SpecificOpticalRotatoryPowerUnit' value)
    (attribute_usage ''radian metre squared per mole'' : 'MolarOpticalRotatoryPowerUnit' value)
    (attribute_usage ''radian second to the power minus 1'' : 'AngularVelocityUnit' value)
    (attribute_usage ''radian second to the power minus 2'' : 'AngularAccelerationUnit' value)
    (attribute_usage ''radian per metre'' : 'PhaseCoefficientUnit' value)
    (attribute_usage ''second ampere'' : 'ElectricChargeUnit' value)
    (attribute_usage ''siemens metre squared per mole'' : 'MolarConductivityUnit' value)
    (attribute_usage ''siemens per metre'' : 'ConductivityUnit' value)
    (attribute_usage ''second to the power minus 1'' : 'AngularVelocityUnit' value)
    (attribute_usage ''second to the power minus 1 steradian to the power minus 1'' : 'PhotonIntensityUnit' value)
    (attribute_usage ''second to the power minus 2'' : 'AngularAccelerationUnit' value)
    (attribute_usage ''shannon per second'' : 'AverageInformationRateUnit' value)
    (attribute_usage ''sievert per second'' : 'DoseEquivalentUnit' value)
    (attribute_usage ''volt ampere'' : 'PowerUnit' value)
    (attribute_usage ''volt per kelvin'' : 'SeebeckCoefficientForSubstancesAAndBUnit' value)
    (attribute_usage ''volt per metre'' : 'ElectricFieldStrengthUnit' value)
    (attribute_usage ''volt to the power 2 per kelvin to the power 2'' : 'LorenzCoefficientUnit' value)
    (attribute_usage ''watt hour'' : 'EnergyUnit' value)
    (attribute_usage ''watt per metre kelvin'' : 'ThermalConductivityUnit' value)
    (attribute_usage ''watt per square metre kelvin'' : 'CoefficientOfHeatTransferUnit' value)
    (attribute_usage ''watt per square metre nm'' : 'SpectralIrradianceUnit' value)
    (attribute_usage ''watt per steradian square metre'' : 'RadianceUnit' value)
    (attribute_usage ''watt per steradian square metre nm'' : 'SpectralRadianceUnit' value)
    (attribute_usage ''watt per steradian nm'' : 'SpectralRadiantIntensityUnit' value)
    (attribute_usage ''watt per kelvin'' : 'ThermalConductanceUnit' value)
    (attribute_usage ''watt per kilogram'' : 'DoseEquivalentUnit' value)
    (attribute_usage ''watt per square metre'' : 'DensityOfHeatFlowRateUnit' value)
    (attribute_usage ''watt per nm'' : 'SpectralRadiantFluxUnit' value)
    (attribute_usage ''watt per steradian'' : 'RadiantIntensityUnit' value)
    (attribute_usage ''weber metre'' : 'MagneticDipoleMomentUnit' value)
    (attribute_usage ''weber per metre'' : 'MagneticVectorPotentialUnit' value)
    (attribute_usage ''ohm metre'' : 'ResistivityUnit' value)
    (alias_member ''m/s²'' for ''m⋅s⁻²'')
    (comment)
    (comment)
    (attribute_usage 'nanometre' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'millimetre' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'centimetre' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'kilometre' : 'LengthUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (comment)
    (attribute_usage 'millilitre' : 'VolumeUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (comment)
    (attribute_usage 'millinewton' : 'ForceUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (comment)
    (attribute_usage 'kilojoule' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'megajoule' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (attribute_usage 'gigajoule' : 'EnergyUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (comment)
    (attribute_usage 'kilowatt' : 'PowerUnit'
      (default_ref_usage :>> 'unitConversion' : 'ConversionByPrefix'
        (default_ref_usage :>> 'prefix' value)
        (default_ref_usage :>> 'referenceUnit' value)))
    (comment)
    (attribute_usage ''kilometre per hour'' : 'SpeedUnit' value)
    (comment)
    (attribute_usage ''degree celsius (temperature difference)'' : 'TemperatureDifferenceUnit'
      (documentation)
      (attribute_usage :>> 'unitConversion' : 'ConversionByConvention'
        (default_ref_usage :>> 'referenceUnit' value)
        (default_ref_usage :>> 'conversionFactor' value)))
    (attribute_usage ''degree celsius (absolute temperature scale)'' : 'IntervalScale'
      (documentation)
      (attribute_usage :>> 'unit' value)
      (attribute_usage 'temperatureWaterAtFreezingPointInC' : 'DefinitionalQuantityValue'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value))
      (attribute_usage private 'temperatureWaterAtTriplePointInC' : 'DefinitionalQuantityValue'
        (default_ref_usage :>> 'num' value)
        (default_ref_usage :>> 'definition' value))
      (attribute_usage private 'celsiusToKelvinScaleMapping' : 'QuantityValueMapping'
        (default_ref_usage :>> 'mappedQuantityValue' value)
        (default_ref_usage :>> 'referenceQuantityValue' value))
      (attribute_usage :>> 'definitionalQuantityValues' value)
      (attribute_usage :>> 'quantityValueMapping' value)
      (comment)
      (attribute_usage private 'zeroDegreeCelsiusInKelvin' : 'ThermodynamicTemperatureValue' value)
      (attribute_usage 'zeroDegreeCelsiusToKelvinShift' : 'CoordinateFramePlacement' :>> 'transformation'
        (default_ref_usage :>> 'source' value)
        (default_ref_usage :>> 'origin' value)))))
~~~
# FORMAT
~~~sysml
standard library package SI {
    doc
    /*
	 * International System of (Measurement) Units -- Système International d'Unités (SI), as defined in ISO/IEC 80000
	 *
	 * Note 1: In accordance with ISO/IEC 80000 en-GB spelling is used for the names and definitions of the units.
	 * Note 2: This is a representative but not yet complete list of measurement units.
	 */

    private import MeasurementReferences::*;
    public import ISQ::*;
    public import SIPrefixes::*;

    /*
     * SI simple unit needed in support of creation of the base units
     */
    attribute <g> gram : MassUnit;

    /*
     * SI base units
     */
    attribute <m> metre : LengthUnit;
    attribute <kg> kilogram : MassUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = g; } }
    attribute <s> second : DurationUnit;
    attribute <A> ampere : ElectricCurrentUnit;
    attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit {
        attribute temperatureOfWaterAtTriplePointInK: DefinitionalQuantityValue {
            :>> num = 27316/100;
            :>> definition = "temperature in kelvin of pure water at the triple point";
        }
        attribute :>> definitionalQuantityValues = temperatureOfWaterAtTriplePointInK;
        attribute :>> ThermodynamicTemperatureUnit::quantityDimension, TemperatureDifferenceUnit::quantityDimension {
            :>> ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors, TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <mol> mole : AmountOfSubstanceUnit;
    attribute <cd> candela : LuminousIntensityUnit;

    /*
     * Declare the SI system of units with its explicit base units
     * and its associated system of quantities, the ISQ.
     */
    attribute <si> 'ISO/IEC 80000 International System of Units' : SystemOfUnits {
        :>> systemOfQuantities = isq;
        :>> baseUnits = (m, kg, s, A, K, mol, cd);
    }

    /*
     * Units with special names
     */
    attribute <B> byte : StorageCapacityUnit = one;
    attribute <Bd> baud : ModulationRateUnit = s^-1;
    attribute <bit> bit : StorageCapacityUnit = one;
    attribute <Bq> becquerel : NuclearActivityUnit = s^-1;
    attribute <C> coulomb : ElectricChargeUnit = A*s;
    attribute <dB> decibel : SoundPressureLevelUnit = one;
    attribute <dec> decade : LogarithmicFrequencyRangeUnit = one;
    attribute <E> erlang : TrafficIntensityUnit = one;
    attribute <F> farad : CapacitanceUnit = C/V;
    attribute <Gy> gray : AbsorbedDoseUnit = J/kg;
    attribute <H> henry : PermeanceUnit, InductanceUnit = Wb/A {
        attribute :>> PermeanceUnit::quantityDimension, InductanceUnit::quantityDimension {
            :>> PermeanceUnit::quantityDimension::quantityPowerFactors, InductanceUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <Hart> hartley : InformationContentUnit = one;
    attribute <Hz> hertz : FrequencyUnit = s^-1;
    attribute <J> joule : EnergyUnit = N*m;
    //attribute <kat> katal : CatalyticActivityUnit = mol/s;
    attribute <lm> lumen : LuminousFluxUnit = cd*sr;
    attribute <lx> lux : IlluminanceUnit = lm/m^2;
    attribute <N> newton : ForceUnit = kg*m/s^2;
    attribute <nat> 'natural unit of information' : InformationContentUnit = one;
    attribute <o> octet : StorageCapacityUnit = one;
    attribute <oct> octave : LogarithmicFrequencyRangeUnit = one;
    attribute <Pa> pascal : PressureUnit = N/m^2;
    attribute <rad> radian : AngularMeasureUnit = m/m;
    attribute <S> siemens : ConductanceUnit = 'Ω'^-1;
    attribute <Sh> shannon : InformationContentUnit = one;
    attribute <sr> steradian : SolidAngularMeasureUnit = m^2/m^2;
    attribute <Sv> sievert : DoseEquivalentUnit = J/kg;
    attribute <T> tesla : MagneticFluxDensityUnit = Wb/m^2;
    attribute <V> volt : ElectricPotentialUnit = W/A;
    attribute <W> watt : PowerUnit = J/s;
    attribute <Wb> weber : MagneticFluxUnit = V*s;
    attribute <'Ω'> ohm : ResistanceUnit = V/A;

    /*
     * Units recognized in SI as specified in ISO 80000-1:2009
     */
    attribute <'Å'> 'ångström' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1.0e-10; } }
    attribute <b> barn : AreaUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = 'm²'; :>> conversionFactor = 1.0e-28; } }
    attribute <d> day: DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = hour; :>> conversionFactor = 24; } }
    attribute <Da> dalton : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.66053906660e-27; :>> isExact = false; } }
    attribute <eV> electronvolt : EnergyUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = J; :>> conversionFactor = 1.602176487e-19; :>> isExact = false; } }
    attribute <h> hour: DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = min; :>> conversionFactor = 60; } }
    attribute <min> minute : DurationUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = s; :>> conversionFactor = 60; } }
    attribute <L> litre : VolumeUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = 'm³'; :>> conversionFactor = 1.0e-3; } }
    attribute tonne : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = kg; :>> conversionFactor = 1.0e-3; } }
    alias 'metric ton' for tonne;
    attribute <u> 'atomic mass unit' : MassUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = Da; :>> conversionFactor = 1.0; } }
    attribute <ua> 'astronomical unit' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 149597870691e11; :>> isExact = false; } }
    attribute <var> 'volt ampere reactive' : PowerUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = V*A; :>> conversionFactor = 1.0; } }
    attribute <'°'> degree : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 1.745329E-02; :>> isExact = false; } } // conversionFactor should become pi/180
    attribute <'′'> 'minute (angle)' : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 2.908882E-04; :>> isExact = false; } }
    alias arcmin for '′';
    attribute <'″'> 'second (angle)' : AngularMeasureUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = rad; :>> conversionFactor = 4.848137E-06; :>> isExact = false; } }
    alias arcsec for '″';

    /*
     * Derived units used in parts 3 to 12 of ISO/IEC 80000
     */
    attribute <'A⋅m⁻²⋅K⁻²'> 'ampere metre to the power minus 2 kelvin to the power minus 2' : RichardsonConstantUnit = A*m^-2*K^-2;
    attribute <'A⋅m²'> 'ampere metre squared' : MagneticMomentUnit = A*m^2;
    attribute <'A⋅m²⋅J⁻¹⋅s⁻¹'> 'ampere metre squared joule to the power minus 1 second to the power minus 1' : GyromagneticRatioUnit = A*m^2*J^-1*s^-1;
    attribute <'A⋅s/kg'> 'ampere second per kilogram' : GyromagneticRatioUnit = A*s/kg;
    attribute <'A/m'> 'ampere per metre' : LinearElectricCurrentDensityUnit = A/m;
    attribute <'A/m²'> 'ampere per square metre' : ElectricCurrentDensityUnit = A/m^2;
    attribute <'B/s'> 'byte per second' : TransferRateUnit = B/s;
    attribute <'bit/s'> 'bit per second' : BinaryDigitRateUnit = bit/s;
    attribute <'Bq/kg'> 'becquerel per kilogram' : SpecificActivityUnit = Bq/kg;
    attribute <'Bq/m²'> 'becquerel per square metre' : SurfaceActivityDensityUnit = Bq/m^2;
    attribute <'Bq/m³'> 'becquerel per cubic metre' : ActivityDensityUnit = Bq/m^3;
    attribute <'C⋅m'> 'coulomb metre' : ElectricDipoleMomentUnit = C*m;
    attribute <'C/(kg⋅s)'> 'coulomb per kilogram second' : ExposureRateUnit = C/(kg*s);
    attribute <'C/kg'> 'coulomb per kilogram' : ExposureUnit = C/kg;
    attribute <'C/m'> 'coulomb per metre' : LinearDensityOfElectricChargeUnit = C/m;
    attribute <'C/m²'> 'coulomb per square metre' : SurfaceDensityOfElectricChargeUnit = C/m^2;
    attribute <'C/m³'> 'coulomb per cubic metre' : ElectricChargeDensityUnit = C/m^3;
    attribute <'cd⋅m⁻²'> 'candela metre to the power minus 2' : LuminanceUnit = cd*m^-2;
    attribute <'cd⋅sr'> 'candela steradian' : LuminousFluxUnit = cd*sr;
    attribute <'cd⋅sr⋅kg⁻¹⋅m⁻²⋅s³'> 'candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3' : LuminousEfficacyOfRadiationUnit = cd*sr*kg^-1*m^-2*s^3;
    attribute <'cd⋅sr⋅m⁻²'> 'candela steradian metre to the power minus 2' : IlluminanceUnit = cd*sr*m^-2;
    attribute <'cd⋅sr⋅m⁻²⋅s'> 'candela steradian metre to the power minus 2 second' : LuminousExposureUnit = cd*sr*m^-2*s;
    attribute <'cd⋅sr⋅s'> 'candela steradian second' : LuminousEnergyUnit = cd*sr*s;
    attribute <'eV⋅J⋅kg⋅m²⋅s⁻²'> 'electronvolt joule kilogram metre squared second to the power minus 2' : HartreeEnergyUnit = eV*J*kg*m^2*s^-2;
    attribute <'eV⋅m⁻²/kg'> 'electronvolt metre to the power minus 2 per kilogram' : TotalMassStoppingPowerUnit = eV*m^-2/kg;
    attribute <'eV/m'> 'electronvolt per metre' : TotalLinearStoppingPowerUnit = eV/m;
    attribute <'eV/m²'> 'electronvolt per square metre' : EnergyFluenceUnit = eV/m^2;
    attribute <'F/m'> 'farad per metre' : ElectricConstantUnit = F/m;
    attribute <'g/L'> 'g per l' : MassConcentrationUnit = g/L;
    attribute <'g/mol'> 'g per mole' : MolarMassUnit = g/mol;
    attribute <'Gy/s'> 'gray per second' : AbsorbedDoseRateUnit = Gy/s;
    attribute <'H/m'> 'henry per metre' : MagneticConstantUnit = H/m;
    attribute <'H⁻¹'> 'henry to the power minus 1' : ReluctanceUnit = H^-1;
    attribute <'Hart/s'> 'hartley per second' : AverageInformationRateUnit = Hart/s;
    attribute <'J⋅m²/kg'> 'joule metre squared per kilogram' : TotalMassStoppingPowerUnit = J*m^2/kg;
    attribute <'J⋅s'> 'joule second' : ActionQuantityUnit = J*s;
    attribute <'J⋅s⋅eV⋅s'> 'joule second electronvolt second' : TotalAngularMomentumUnit = J*s*eV*s;
    attribute <'J⋅s⁻¹'> 'joule second to the power minus 1' : PowerUnit = J*s^-1;
    attribute <'J/(kg⋅K)'> 'joule per kilogram kelvin' : SpecificHeatCapacityUnit = J/(kg*K);
    attribute <'J/(m²⋅nm)'> 'joule per square metre nm' : SpectralRadiantExposureUnit = J/(m^2*nm);
    attribute <'J/(m³⋅nm)'> 'joule per cubic metre nm' : SpectralRadiantEnergyDensityInTermsOfWavelengthUnit = J/(m^3*nm);
    attribute <'J/(mol⋅K)'> 'joule per mole kelvin' : MolarHeatCapacityUnit = J/(mol*K);
    attribute <'J/K'> 'joule per kelvin' : HeatCapacityUnit = J/K;
    attribute <'J/kg'> 'joule per kilogram' : SpecificEnergyUnit = J/kg;
    attribute <'J/m'> 'joule per metre' : TotalLinearStoppingPowerUnit = J/m;
    attribute <'J/m²'> 'joule per square metre' : SpectralRadiantEnergyDensityInTermsOfWavenumberUnit = J/m^2;
    attribute <'J/m³'> 'joule per cubic metre' : ElectromagneticEnergyDensityUnit = J/m^3;
    attribute <'J/mol'> 'joule per mole' : MolarInternalEnergyUnit = J/mol;
    attribute <'J/nm'> 'joule per nm' : SpectralRadiantEnergyUnit = J/nm;
    attribute <'J/s'> 'joule per second' : HeatFlowRateUnit = J/s;
    attribute <'J⁻¹⋅m⁻³⋅eV⁻¹⋅m⁻³'> 'joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3' : EnergyDensityOfStatesUnit = J^-1*m^-3*eV^-1*m^-3;
    attribute <'K/Pa'> 'kelvin per pascal' : JouleThomsonCoefficientUnit = K/Pa;
    attribute <'K/W'> 'kelvin per watt' : ThermalResistanceUnit = K/W;
    attribute <'K⁻¹'> 'kelvin to the power minus 1' : LinearExpansionCoefficientUnit = K^-1;
    attribute <'kg⋅m⋅s⁻¹'> 'kilogram metre second to the power minus 1' : MomentumUnit = kg*m*s^-1;
    attribute <'kg⋅m⋅s⁻²'> 'kilogram metre second to the power minus 2' : ForceUnit = kg*m*s^-2;
    attribute <'kg⋅m⋅s⁻³'> 'kilogram metre second to the power minus 3' : SpectralRadiantFluxUnit = kg*m*s^-3;
    attribute <'kg⋅m⋅s⁻³⋅K⁻¹'> 'kilogram metre second to the power minus 3 kelvin to the power minus 1' : ThermalConductivityUnit = kg*m*s^-3*K^-1;
    attribute <'kg⋅m⋅s⁻³⋅sr⁻¹'> 'kilogram metre second to the power minus 3 steradian to the power minus 1' : SpectralRadiantIntensityUnit = kg*m*s^-3*sr^-1;
    attribute <'kg⋅m⁻¹'> 'kilogram metre to the power minus 1' : LinearMassDensityUnit = kg*m^-1;
    attribute <'kg⋅m⁻¹⋅s⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 1' : DynamicViscosityUnit = kg*m^-1*s^-1;
    attribute <'kg⋅m⁻¹⋅s⁻²'> 'kilogram metre to the power minus 1 second to the power minus 2' : PressureUnit = kg*m^-1*s^-2;
    attribute <'kg⋅m⁻¹⋅s⁻²⋅K⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1' : PressureCoefficientUnit = kg*m^-1*s^-2*K^-1;
    attribute <'kg⋅m⁻¹⋅s⁻³'> 'kilogram metre to the power minus 1 second to the power minus 3' : SpectralIrradianceUnit = kg*m^-1*s^-3;
    attribute <'kg⋅m⁻¹⋅s⁻³⋅sr⁻¹'> 'kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1' : SpectralRadianceUnit = kg*m^-1*s^-3*sr^-1;
    attribute <'kg⋅m⁻²'> 'kilogram metre to the power minus 2' : SurfaceMassDensityUnit = kg*m^-2;
    attribute <'kg⋅m⁻²⋅s⁻¹'> 'kilogram metre to the power minus 2 second to the power minus 1' : MassFlowUnit = kg*m^-2*s^-1;
    attribute <'kg⋅m⁻²⋅s⁻²'> 'kilogram metre to the power minus 2 second to the power minus 2' : SpectralRadiantEnergyDensityInTermsOfWavelengthUnit = kg*m^-2*s^-2;
    attribute <'kg⋅m⁻³'> 'kilogram metre to the power minus 3' : MassDensityUnit = kg*m^-3;
    attribute <'kg⋅m⁻⁴⋅s⁻¹'> 'kilogram metre to the power minus 4 second to the power minus 1' : AcousticImpedanceUnit = kg*m^-4*s^-1;
    attribute <'kg⋅m²'> 'kilogram metre squared' : MomentOfInertiaUnit = kg*m^2;
    attribute <'kg⋅m²⋅s⁻¹'> 'kilogram metre squared second to the power minus 1' : AngularMomentumUnit = kg*m^2*s^-1;
    attribute <'kg⋅m²⋅s⁻²'> 'kilogram metre squared second to the power minus 2' : MomentOfForceUnit = kg*m^2*s^-2;
    attribute <'kg⋅m²⋅s⁻²⋅K⁻¹'> 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1' : HeatCapacityUnit = kg*m^2*s^-2*K^-1;
    attribute <'kg⋅m²⋅s⁻²⋅K⁻¹⋅mol⁻¹'> 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1' : MolarHeatCapacityUnit = kg*m^2*s^-2*K^-1*mol^-1;
    attribute <'kg⋅m²⋅s⁻²⋅mol⁻¹'> 'kilogram metre squared second to the power minus 2 mole to the power minus 1' : MolarInternalEnergyUnit = kg*m^2*s^-2*mol^-1;
    attribute <'kg⋅m²⋅s⁻³'> 'kilogram metre squared second to the power minus 3' : PowerUnit = kg*m^2*s^-3;
    attribute <'kg⋅m²⋅s⁻³⋅A⁻¹'> 'kilogram metre squared second to the power minus 3 ampere to the power minus 1' : ElectricPotentialDifferenceUnit = kg*m^2*s^-3*A^-1;
    attribute <'kg⋅m²⋅s⁻³⋅A⁻¹⋅K⁻¹'> 'kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1' : SeebeckCoefficientForSubstancesAAndBUnit = kg*m^2*s^-3*A^-1*K^-1;
    attribute <'kg⋅m²⋅s⁻³⋅K⁻¹'> 'kilogram metre squared second to the power minus 3 kelvin to the power minus 1' : ThermalConductanceUnit = kg*m^2*s^-3*K^-1;
    attribute <'kg⋅m²⋅s⁻³⋅sr⁻¹'> 'kilogram metre squared second to the power minus 3 steradian to the power minus 1' : RadiantIntensityUnit = kg*m^2*s^-3*sr^-1;
    attribute <'kg⋅m³⋅s⁻³⋅A⁻²'> 'kilogram metre cubed second to the power minus 3 ampere to the power minus 2' : ResistivityUnit = kg*m^3*s^-3*A^-2;
    attribute <'kg⋅mol⁻¹'> 'kilogram mole to the power minus 1' : MolarMassUnit = kg*mol^-1;
    attribute <'kg⋅s⁻¹'> 'kilogram second to the power minus 1' : MassFlowRateUnit = kg*s^-1;
    attribute <'kg⋅s⁻²'> 'kilogram second to the power minus 2' : SurfaceTensionUnit = kg*s^-2;
    attribute <'kg⋅s⁻²⋅A⁻¹'> 'kilogram second to the power minus 2 ampere to the power minus 1' : MagneticFluxDensityUnit = kg*s^-2*A^-1;
    attribute <'kg⋅s⁻³'> 'kilogram second to the power minus 3' : DensityOfHeatFlowRateUnit = kg*s^-3;
    attribute <'kg⋅s⁻³⋅K⁻¹'> 'kilogram second to the power minus 3 kelvin to the power minus 1' : CoefficientOfHeatTransferUnit = kg*s^-3*K^-1;
    attribute <'kg⋅s⁻³⋅sr⁻¹'> 'kilogram second to the power minus 3 steradian to the power minus 1' : RadianceUnit = kg*s^-3*sr^-1;
    attribute <'kg⁻¹⋅A'> 'kilogram to the power minus 1 ampere' : ExposureRateUnit = kg^-1*A;
    attribute <'kg⁻¹⋅m⋅s²'> 'kilogram to the power minus 1 metre second to the power 2' : CompressibilityUnit = kg^-1*m*s^2;
    attribute <'kg⁻¹⋅m⋅s²⋅K'> 'kilogram to the power minus 1 metre second to the power 2 kelvin' : JouleThomsonCoefficientUnit = kg^-1*m*s^2*K;
    attribute <'kg⁻¹⋅m⁻²⋅s³⋅K'> 'kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin' : ThermalResistanceUnit = kg^-1*m^-2*s^3*K;
    attribute <'kg⁻¹⋅m⁻³⋅s³⋅A²'> 'kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2' : ElectrolyticConductivityUnit = kg^-1*m^-3*s^3*A^2;
    attribute <'kg⁻¹⋅m⁻⁵⋅s²'> 'kilogram to the power minus 1 metre to the power minus 5 second to the power 2' : EnergyDensityOfStatesUnit = kg^-1*m^-5*s^2;
    attribute <'kg⁻¹⋅m²'> 'kilogram to the power minus 1 metre squared' : MassAttenuationCoefficientUnit = kg^-1*m^2;
    attribute <'kg⁻¹⋅m³'> 'kilogram to the power minus 1 metre cubed' : SpecificVolumeUnit = kg^-1*m^3;
    attribute <'kg⁻¹⋅s⋅A'> 'kilogram to the power minus 1 second ampere' : GyromagneticRatioUnit = kg^-1*s*A;
    attribute <'kg⁻¹⋅s⁻¹'> 'kilogram to the power minus 1 second to the power minus 1' : SpecificActivityUnit = kg^-1*s^-1;
    attribute <'kg⁻¹⋅s²'> 'kilogram to the power minus 1 second to the power 2' : EnergyDistributionOfCrossSectionUnit = kg^-1*s^2;
    attribute <'kg⁻¹⋅s²⋅A'> 'kilogram to the power minus 1 second to the power 2 ampere' : MobilityUnit = kg^-1*s^2*A;
    attribute <'kg⁻¹⋅s³⋅A²⋅mol⁻¹'> 'kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1' : MolarConductivityUnit = kg^-1*s^3*A^2*mol^-1;
    attribute <'kg⁻¹⋅s³⋅K'> 'kilogram to the power minus 1 second to the power 3 kelvin' : ThermalInsulanceUnit = kg^-1*s^3*K;
    attribute <'kg²⋅m⁻²⋅s⁻³'> 'kilogram to the power 2 metre to the power minus 2 second to the power minus 3' : SoundExposureUnit = kg^2*m^-2*s^-3;
    attribute <'kg²⋅m⁴⋅s⁻⁶⋅A⁻²⋅K⁻²'> 'kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2' : LorenzCoefficientUnit = kg^2*m^4*s^-6*A^-2*K^-2;
    attribute <'lm⋅s'> 'lumen second' : LuminousEnergyUnit = lm*s;
    attribute <'lm/m²'> 'lumen per square metre' : LuminousExitanceUnit = lm/m^2;
    attribute <'lm/W'> 'lumen per watt' : LuminousEfficacyOfRadiationUnit = lm/W;
    attribute <'lx⋅s'> 'lux second' : LuminousExposureUnit = lx*s;
    attribute <'m⋅s⁻¹'> 'metre second to the power minus 1' : SpeedUnit = m*s^-1;
    attribute <'m⋅s⁻²'> 'metre second to the power minus 2' : AccelerationUnit = m*s^-2;
    attribute <'m/s'> 'metre per second' : SpeedUnit = m/s;
    attribute <'m⁻¹'> 'metre to the power minus 1' : CurvatureUnit = m^-1;
    attribute <'m⁻²'> 'metre to the power minus 2' : PhotonExposureUnit = m^-2;
    attribute <'m⁻²⋅s⁻¹'> 'metre to the power minus 2 second to the power minus 1' : PhotonIrradianceUnit = m^-2*s^-1;
    attribute <'m⁻²⋅s⁻¹⋅sr⁻¹'> 'metre to the power minus 2 second to the power minus 1 steradian to the power minus 1' : PhotonRadianceUnit = m^-2*s^-1*sr^-1;
    attribute <'m⁻³'> 'metre to the power minus 3' : ParticleConcentrationUnit = m^-3;
    attribute <'m⁻³⋅s'> 'metre to the power minus 3 second' : DensityOfVibrationalStatesUnit = m^-3*s;
    attribute <'m⁻³⋅s⁻¹'> 'metre to the power minus 3 second to the power minus 1' : ActivityDensityUnit = m^-3*s^-1;
    attribute <'m²'> 'metre squared' : AreaUnit = m^2;
    attribute <'m²⋅A'> 'metre squared ampere' : MagneticDipoleMomentUnit = m^2*A;
    attribute <'m²⋅K/W'> 'metre squared kelvin per watt' : ThermalInsulanceUnit = m^2*K/W;
    attribute <'m²⋅mol⁻¹'> 'metre squared mole to the power minus 1' : MolarAbsorptionCoefficientUnit = m^2*mol^-1;
    attribute <'m²⋅s⁻¹'> 'metre squared second to the power minus 1' : KinematicViscosityUnit = m^2*s^-1;
    attribute <'m²⋅s⁻²'> 'metre squared second to the power minus 2' : SpecificEnergyUnit = m^2*s^-2;
    attribute <'m²⋅s⁻²⋅K⁻¹'> 'metre squared second to the power minus 2 kelvin to the power minus 1' : SpecificHeatCapacityUnit = m^2*s^-2*K^-1;
    attribute <'m²⋅s⁻³'> 'metre squared second to the power minus 3' : DoseEquivalentUnit = m^2*s^-3;
    attribute <'m²⋅sr⁻¹'> 'metre squared steradian to the power minus 1' : DirectionDistributionOfCrossSectionUnit = m^2*sr^-1;
    attribute <'m²/(J⋅sr)'> 'metre squared per joule steradian' : DirectionAndEnergyDistributionOfCrossSectionUnit = m^2/(J*sr);
    attribute <'m²/(V⋅s)'> 'metre squared per volt second' : MobilityUnit = m^2/(V*s);
    attribute <'m²/J'> 'metre squared per joule' : EnergyDistributionOfCrossSectionUnit = m^2/J;
    attribute <'m³'> 'metre cubed' : VolumeUnit = m^3;
    attribute <'m³⋅mol⁻¹'> 'metre cubed mole to the power minus 1' : MolarVolumeUnit = m^3*mol^-1;
    attribute <'m³⋅s⁻¹'> 'metre cubed second to the power minus 1' : VolumeFlowRateUnit = m^3*s^-1;
    attribute <'m³/C⋅m³⋅s⁻¹⋅A⁻¹'> 'metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1' : HallCoefficientUnit = m^3/C*m^3*s^-1*A^-1;
    attribute <'m⁴'> 'metre to the power 4' : SecondAxialMomentOfAreaUnit = m^4;
    attribute <'m⁴⋅s⁻²'> 'metre to the power 4 second to the power minus 2' : TotalMassStoppingPowerUnit = m^4*s^-2;
    attribute <'mL/L '> 'ml per l' : VolumeFractionUnit = mL/L;
    attribute <'mol⋅kg⁻¹'> 'mole kilogram to the power minus 1' : IonicStrengthUnit = mol*kg^-1;
    attribute <'mol⋅m⁻³'> 'mole metre to the power minus 3' : AmountOfSubstanceConcentrationUnit = mol*m^-3;
    attribute <'mol/kg'> 'mole per kilogram' : MolalityUnit = mol/kg;
    attribute <'mol/L'> 'mole per l' : AmountOfSubstanceConcentrationUnit = mol/L;
    attribute <'mol/m³'> 'mole per cubic metre' : EquilibriumConstantOnConcentrationBasisUnit = mol/m^3;
    attribute <'N⋅m'> 'newton metre' : MomentOfForceUnit, TorqueUnit = N*m {
        attribute :>> MomentOfForceUnit::quantityDimension, TorqueUnit::quantityDimension {
            :>> MomentOfForceUnit::quantityDimension::quantityPowerFactors, TorqueUnit::quantityDimension::quantityPowerFactors;
        }
    }
    attribute <'N⋅m⋅s'> 'newton metre second' : AngularImpulseUnit = N*m*s;
    attribute <'N⋅m⋅s⁻¹'> 'newton metre second to the power minus 1' : PowerUnit = N*m*s^-1;
    attribute <'N⋅m⁻¹'> 'newton metre to the power minus 1' : SurfaceTensionUnit = N*m^-1;
    attribute <'N⋅m⁻²'> 'newton metre to the power minus 2' : PressureUnit = N*m^-2;
    attribute <'N⋅s'> 'newton second' : ImpulseUnit = N*s;
    attribute <'nat/s'> 'natural unit of information per second' : AverageInformationRateUnit = nat/s;
    attribute <'o/s'> 'octet per second' : TransferRateUnit = o/s;
    attribute <'Pa⋅s'> 'pascal second' : DynamicViscosityUnit = Pa*s;
    attribute <'Pa⋅s/m'> 'pascal second per metre' : CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit = Pa*s/m;
    attribute <'Pa⋅s/m³'> 'pascal second per cubic metre' : AcousticImpedanceUnit = Pa*s/m^3;
    attribute <'Pa/K'> 'pascal per kelvin' : PressureCoefficientUnit = Pa/K;
    attribute <'Pa⁻¹'> 'pascal to the power minus 1' : CompressibilityUnit = Pa^-1;
    attribute <'Pa²⋅s'> 'pascal to the power 2 second' : SoundExposureUnit = Pa^2*s;
    attribute <'rad⋅m²/kg¹'> 'radian metre squared per kilogram to the power 1' : SpecificOpticalRotatoryPowerUnit = rad*m^2/kg^1;
    attribute <'rad⋅m²/mol'> 'radian metre squared per mole' : MolarOpticalRotatoryPowerUnit = rad*m^2/mol;
    attribute <'rad⋅s⁻¹'> 'radian second to the power minus 1' : AngularVelocityUnit = rad*s^-1;
    attribute <'rad⋅s⁻²'> 'radian second to the power minus 2' : AngularAccelerationUnit = rad*s^-2;
    attribute <'rad/m'> 'radian per metre' : PhaseCoefficientUnit = rad/m;
    attribute <'s⋅A'> 'second ampere' : ElectricChargeUnit = s*A;
    attribute <'S⋅m²/mol'> 'siemens metre squared per mole' : MolarConductivityUnit = S*m^2/mol;
    attribute <'S/m'> 'siemens per metre' : ConductivityUnit = S/m;
    attribute <'s⁻¹'> 'second to the power minus 1' : AngularVelocityUnit = s^-1;
    attribute <'s⁻¹⋅sr⁻¹'> 'second to the power minus 1 steradian to the power minus 1' : PhotonIntensityUnit = s^-1*sr^-1;
    attribute <'s⁻²'> 'second to the power minus 2' : AngularAccelerationUnit = s^-2;
    attribute <'Sh/s'> 'shannon per second' : AverageInformationRateUnit = Sh/s;
    attribute <'Sv/s'> 'sievert per second' : DoseEquivalentUnit = Sv/s;
    attribute <'V⋅A'> 'volt ampere' : PowerUnit = V*A;
    attribute <'V/K'> 'volt per kelvin' : SeebeckCoefficientForSubstancesAAndBUnit = V/K;
    attribute <'V/m'> 'volt per metre' : ElectricFieldStrengthUnit = V/m;
    attribute <'V²/K²'> 'volt to the power 2 per kelvin to the power 2' : LorenzCoefficientUnit = V^2/K^2;
    attribute <'W⋅h'> 'watt hour' : EnergyUnit = W*h;
    attribute <'W/(m⋅K)'> 'watt per metre kelvin' : ThermalConductivityUnit = W/(m*K);
    attribute <'W/(m²⋅K)'> 'watt per square metre kelvin' : CoefficientOfHeatTransferUnit = W/(m^2*K);
    attribute <'W/(m²⋅nm)'> 'watt per square metre nm' : SpectralIrradianceUnit = W/(m^2*nm);
    attribute <'W/(sr⋅m²)'> 'watt per steradian square metre' : RadianceUnit = W/(sr*m^2);
    attribute <'W/(sr⋅m²⋅nm)'> 'watt per steradian square metre nm' : SpectralRadianceUnit = W/(sr*m^2*nm);
    attribute <'W/(sr⋅nm)'> 'watt per steradian nm' : SpectralRadiantIntensityUnit = W/(sr*nm);
    attribute <'W/K'> 'watt per kelvin' : ThermalConductanceUnit = W/K;
    attribute <'W/kg'> 'watt per kilogram' : DoseEquivalentUnit = W/kg;
    attribute <'W/m²'> 'watt per square metre' : DensityOfHeatFlowRateUnit = W/m^2;
    attribute <'W/nm'> 'watt per nm' : SpectralRadiantFluxUnit = W/nm;
    attribute <'W/sr'> 'watt per steradian' : RadiantIntensityUnit = W/sr;
    attribute <'Wb⋅m'> 'weber metre' : MagneticDipoleMomentUnit = Wb*m;
    attribute <'Wb/m'> 'weber per metre' : MagneticVectorPotentialUnit = Wb/m;
    attribute <'Ω⋅m'> 'ohm metre' : ResistivityUnit = 'Ω'*m;

    alias 'm/s²' for 'm⋅s⁻²';

    /*
     * Prefixed units
     */

    /* Length */
    attribute <nm> nanometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = nano; :>> referenceUnit = m; } }
    attribute <mm> millimetre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = m; } }
    attribute <cm> centimetre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = centi; :>> referenceUnit = m; } }
    attribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }

    /* Volume */
    attribute <mL> millilitre : VolumeUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = L; } }

    /* Force */
    attribute <mN> millinewton : ForceUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = milli; :>> referenceUnit = N; } }

    /* Energy */
    attribute <kJ> kilojoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = J; } }
    attribute <MJ> megajoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = mega; :>> referenceUnit = J; } }
    attribute <GJ> gigajoule : EnergyUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = giga; :>> referenceUnit = J; } }

    /* Power */
    attribute <kW> kilowatt : PowerUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = W; } }

    /* Speed */
    attribute <'km/h'> 'kilometre per hour': SpeedUnit = km/h;

    /* 
	 * Celsius units
	 */

    attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit {
        doc
        /*
	     * degree Celsius unit for temperature interval (i.e. temperature difference) quantities
	     */

        attribute :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; }
    }

    attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
        doc
        /*
	     * degree Celsius interval scale for absolute (thermodynamic) temperature quantities
	     *
	     * The interval scale is defined with an explicit transformation with respect to 
	     * the kelvin thermodynamic temperature scale that specifies the zero shift.
	     */

        attribute :>> unit = '°C';
        attribute temperatureWaterAtFreezingPointInC: DefinitionalQuantityValue {
            :>> num = 0; :>> definition = "temperature in degree Celsius of pure water at freezing point";
        }
        private attribute temperatureWaterAtTriplePointInC: DefinitionalQuantityValue {
            :>> num = 1/100; :>> definition = "temperature in degree Celsius of pure water at the triple point";
        }
        private attribute celsiusToKelvinScaleMapping: QuantityValueMapping {
            :>> mappedQuantityValue = temperatureWaterAtTriplePointInC;
            :>> referenceQuantityValue = K.temperatureOfWaterAtTriplePointInK;
        }
        attribute :>> definitionalQuantityValues = (temperatureWaterAtTriplePointInC, temperatureWaterAtFreezingPointInC);
        attribute :>> quantityValueMapping = celsiusToKelvinScaleMapping;

        /* CoordinateFramePlacement (zero shift) w.r.t. the kelvin thermodynamic temperature scale */
        private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
        attribute zeroDegreeCelsiusToKelvinShift : CoordinateFramePlacement :>> transformation {
            :>> source = K; :>> origin = zeroDegreeCelsiusInKelvin;
        }
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SI"))) (name "SI") (declared-name "SI")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SI::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SI::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SI::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (name "ISO/IEC 80000 International System of Units") (declared-name "ISO/IEC 80000 International System of Units") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (name "baseUnits") (declared-name "baseUnits") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (name "systemOfQuantities") (declared-name "systemOfQuantities") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "SI::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere"))) (name "ampere") (declared-name "ampere") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere metre squared"))) (name "ampere metre squared") (declared-name "ampere metre squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere metre squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (name "ampere metre squared joule to the power minus 1 second to the power minus 1") (declared-name "ampere metre squared joule to the power minus 1 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "J")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (name "ampere metre to the power minus 2 kelvin to the power minus 2") (declared-name "ampere metre to the power minus 2 kelvin to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere per metre"))) (name "ampere per metre") (declared-name "ampere per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere per square metre"))) (name "ampere per square metre") (declared-name "ampere per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (name "ampere second per kilogram") (declared-name "ampere second per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (role feature-value))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "SI::arcmin"))) (name "arcmin") (declared-name "arcmin"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "SI::arcsec"))) (name "arcsec") (declared-name "arcsec"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::astronomical unit"))) (name "astronomical unit") (declared-name "astronomical unit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::astronomical unit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::atomic mass unit"))) (name "atomic mass unit") (declared-name "atomic mass unit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::atomic mass unit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::barn"))) (name "barn") (declared-name "barn") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::barn")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::baud"))) (name "baud") (declared-name "baud") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::baud"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::becquerel"))) (name "becquerel") (declared-name "becquerel") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::becquerel"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (name "becquerel per cubic metre") (declared-name "becquerel per cubic metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Bq")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (name "becquerel per kilogram") (declared-name "becquerel per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Bq")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (name "becquerel per square metre") (declared-name "becquerel per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Bq")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::bit"))) (name "bit") (declared-name "bit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::bit"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::bit per second"))) (name "bit per second") (declared-name "bit per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "bit")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::bit per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::byte"))) (name "byte") (declared-name "byte") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::byte"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::byte per second"))) (name "byte per second") (declared-name "byte per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "B")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::byte per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela"))) (name "candela") (declared-name "candela") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (name "candela metre to the power minus 2") (declared-name "candela metre to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela steradian"))) (name "candela steradian") (declared-name "candela steradian") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela steradian"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (declared-name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))) (expression (kind "featureReference") (reference "kg")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (name "candela steradian metre to the power minus 2") (declared-name "candela steradian metre to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (name "candela steradian metre to the power minus 2 second") (declared-name "candela steradian metre to the power minus 2 second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::candela steradian second"))) (name "candela steradian second") (declared-name "candela steradian second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::candela steradian second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::centimetre"))) (name "centimetre") (declared-name "centimetre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::centimetre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb"))) (name "coulomb") (declared-name "coulomb") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb metre"))) (name "coulomb metre") (declared-name "coulomb metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (name "coulomb per cubic metre") (declared-name "coulomb per cubic metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (name "coulomb per kilogram") (declared-name "coulomb per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (name "coulomb per kilogram second") (declared-name "coulomb per kilogram second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb per metre"))) (name "coulomb per metre") (declared-name "coulomb per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (name "coulomb per square metre") (declared-name "coulomb per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::dalton"))) (name "dalton") (declared-name "dalton") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::dalton")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::day"))) (name "day") (declared-name "day") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::day::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::day")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::decade"))) (name "decade") (declared-name "decade") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::decade"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::decibel"))) (name "decibel") (declared-name "decibel") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::decibel"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::degree"))) (name "degree") (declared-name "degree") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (name "degree celsius (absolute temperature scale)") (declared-name "degree celsius (absolute temperature scale)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::celsiusToKelvinScaleMapping"))) (name "celsiusToKelvinScaleMapping") (declared-name "celsiusToKelvinScaleMapping") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (name "quantityValueMapping") (declared-name "quantityValueMapping") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtFreezingPointInC"))) (name "temperatureWaterAtFreezingPointInC") (declared-name "temperatureWaterAtFreezingPointInC") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtTriplePointInC"))) (name "temperatureWaterAtTriplePointInC") (declared-name "temperatureWaterAtTriplePointInC") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusInKelvin"))) (name "zeroDegreeCelsiusInKelvin") (declared-name "zeroDegreeCelsiusInKelvin") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (name "zeroDegreeCelsiusToKelvinShift") (declared-name "zeroDegreeCelsiusToKelvinShift") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (name "degree celsius (temperature difference)") (declared-name "degree celsius (temperature difference)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::electronvolt"))) (name "electronvolt") (declared-name "electronvolt") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::electronvolt")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (name "electronvolt joule kilogram metre squared second to the power minus 2") (declared-name "electronvolt joule kilogram metre squared second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "eV")) (expression (kind "featureReference") (reference "J")))) (expression (kind "featureReference") (reference "kg")))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (name "electronvolt metre to the power minus 2 per kilogram") (declared-name "electronvolt metre to the power minus 2 per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "eV")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (name "electronvolt per metre") (declared-name "electronvolt per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "eV")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (name "electronvolt per square metre") (declared-name "electronvolt per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "eV")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::erlang"))) (name "erlang") (declared-name "erlang") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::erlang"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::farad"))) (name "farad") (declared-name "farad") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "C")) (expression (kind "featureReference") (reference "V")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::farad"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::farad per metre"))) (name "farad per metre") (declared-name "farad per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "F")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::farad per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::g per l"))) (name "g per l") (declared-name "g per l") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "g")) (expression (kind "featureReference") (reference "L")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::g per l"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::g per mole"))) (name "g per mole") (declared-name "g per mole") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "g")) (expression (kind "featureReference") (reference "mol")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::g per mole"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::gigajoule"))) (name "gigajoule") (declared-name "gigajoule") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::gigajoule")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::gram"))) (name "gram") (declared-name "gram") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::gray"))) (name "gray") (declared-name "gray") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::gray"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::gray per second"))) (name "gray per second") (declared-name "gray per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Gy")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::gray per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::hartley"))) (name "hartley") (declared-name "hartley") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::hartley"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::hartley per second"))) (name "hartley per second") (declared-name "hartley per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Hart")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::hartley per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::henry"))) (name "henry") (declared-name "henry") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Wb")) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::henry"))) (role feature-value)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::henry")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::henry per metre"))) (name "henry per metre") (declared-name "henry per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "H")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::henry per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (name "henry to the power minus 1") (declared-name "henry to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "H")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::hertz"))) (name "hertz") (declared-name "hertz") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::hertz"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::hour"))) (name "hour") (declared-name "hour") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::hour")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule"))) (name "joule") (declared-name "joule") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (name "joule metre squared per kilogram") (declared-name "joule metre squared per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (name "joule per cubic metre") (declared-name "joule per cubic metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (name "joule per cubic metre nm") (declared-name "joule per cubic metre nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "nm")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per kelvin"))) (name "joule per kelvin") (declared-name "joule per kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per kilogram"))) (name "joule per kilogram") (declared-name "joule per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (name "joule per kilogram kelvin") (declared-name "joule per kilogram kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "K")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per metre"))) (name "joule per metre") (declared-name "joule per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per mole"))) (name "joule per mole") (declared-name "joule per mole") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "mol")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per mole"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (name "joule per mole kelvin") (declared-name "joule per mole kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "K")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per nm"))) (name "joule per nm") (declared-name "joule per nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "nm")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per second"))) (name "joule per second") (declared-name "joule per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per square metre"))) (name "joule per square metre") (declared-name "joule per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (name "joule per square metre nm") (declared-name "joule per square metre nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "nm")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule second"))) (name "joule second") (declared-name "joule second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (name "joule second electronvolt second") (declared-name "joule second electronvolt second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "eV")))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (name "joule second to the power minus 1") (declared-name "joule second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (declared-name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "J")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "eV")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kelvin"))) (name "kelvin") (declared-name "kelvin") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kelvin")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kelvin")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kelvin::temperatureOfWaterAtTriplePointInK"))) (name "temperatureOfWaterAtTriplePointInK") (declared-name "temperatureOfWaterAtTriplePointInK") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kelvin")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (name "kelvin per pascal") (declared-name "kelvin per pascal") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "K")) (expression (kind "featureReference") (reference "Pa")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kelvin per watt"))) (name "kelvin per watt") (declared-name "kelvin per watt") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "K")) (expression (kind "featureReference") (reference "W")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kelvin per watt"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (name "kelvin to the power minus 1") (declared-name "kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "K")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram"))) (name "kilogram") (declared-name "kilogram") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kilogram")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (declared-name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (name "kilogram metre second to the power minus 1") (declared-name "kilogram metre second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (name "kilogram metre second to the power minus 2") (declared-name "kilogram metre second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (name "kilogram metre second to the power minus 3") (declared-name "kilogram metre second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (name "kilogram metre second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre second to the power minus 3 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (name "kilogram metre squared") (declared-name "kilogram metre squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (name "kilogram metre squared second to the power minus 1") (declared-name "kilogram metre squared second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (name "kilogram metre squared second to the power minus 2") (declared-name "kilogram metre squared second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (name "kilogram metre squared second to the power minus 3") (declared-name "kilogram metre squared second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (name "kilogram metre to the power minus 1") (declared-name "kilogram metre to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (name "kilogram metre to the power minus 1 second to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (name "kilogram metre to the power minus 1 second to the power minus 2") (declared-name "kilogram metre to the power minus 1 second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (name "kilogram metre to the power minus 1 second to the power minus 3") (declared-name "kilogram metre to the power minus 1 second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (name "kilogram metre to the power minus 2") (declared-name "kilogram metre to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (name "kilogram metre to the power minus 2 second to the power minus 1") (declared-name "kilogram metre to the power minus 2 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (name "kilogram metre to the power minus 2 second to the power minus 2") (declared-name "kilogram metre to the power minus 2 second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (name "kilogram metre to the power minus 3") (declared-name "kilogram metre to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (name "kilogram metre to the power minus 4 second to the power minus 1") (declared-name "kilogram metre to the power minus 4 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 4)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (name "kilogram mole to the power minus 1") (declared-name "kilogram mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (name "kilogram second to the power minus 1") (declared-name "kilogram second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (name "kilogram second to the power minus 2") (declared-name "kilogram second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (name "kilogram second to the power minus 2 ampere to the power minus 1") (declared-name "kilogram second to the power minus 2 ampere to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (name "kilogram second to the power minus 3") (declared-name "kilogram second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (name "kilogram second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram second to the power minus 3 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (name "kilogram second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram second to the power minus 3 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (declared-name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 4)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 6)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (declared-name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (name "kilogram to the power minus 1 ampere") (declared-name "kilogram to the power minus 1 ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (name "kilogram to the power minus 1 metre cubed") (declared-name "kilogram to the power minus 1 metre cubed") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (name "kilogram to the power minus 1 metre second to the power 2") (declared-name "kilogram to the power minus 1 metre second to the power 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (name "kilogram to the power minus 1 metre second to the power 2 kelvin") (declared-name "kilogram to the power minus 1 metre second to the power 2 kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (name "kilogram to the power minus 1 metre squared") (declared-name "kilogram to the power minus 1 metre squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "A")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 5)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (name "kilogram to the power minus 1 second ampere") (declared-name "kilogram to the power minus 1 second ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (name "kilogram to the power minus 1 second to the power 2") (declared-name "kilogram to the power minus 1 second to the power 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (name "kilogram to the power minus 1 second to the power 2 ampere") (declared-name "kilogram to the power minus 1 second to the power 2 ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "A")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (name "kilogram to the power minus 1 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 second to the power 3 kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (name "kilogram to the power minus 1 second to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilojoule"))) (name "kilojoule") (declared-name "kilojoule") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kilojoule")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilometre"))) (name "kilometre") (declared-name "kilometre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kilometre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilometre per hour"))) (name "kilometre per hour") (declared-name "kilometre per hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "km")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::kilometre per hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::kilowatt"))) (name "kilowatt") (declared-name "kilowatt") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::kilowatt")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::litre"))) (name "litre") (declared-name "litre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::litre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lumen"))) (name "lumen") (declared-name "lumen") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "cd")) (expression (kind "featureReference") (reference "sr")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lumen"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lumen per square metre"))) (name "lumen per square metre") (declared-name "lumen per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lm")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lumen per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lumen per watt"))) (name "lumen per watt") (declared-name "lumen per watt") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lm")) (expression (kind "featureReference") (reference "W")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lumen per watt"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lumen second"))) (name "lumen second") (declared-name "lumen second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lm")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lumen second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lux"))) (name "lux") (declared-name "lux") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "lm")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lux"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::lux second"))) (name "lux second") (declared-name "lux second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "lx")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::lux second"))) (role feature-value))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "SI::m/s²"))) (name "m/s²") (declared-name "m/s²"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::megajoule"))) (name "megajoule") (declared-name "megajoule") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::megajoule")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre"))) (name "metre") (declared-name "metre") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre cubed"))) (name "metre cubed") (declared-name "metre cubed") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre cubed"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (name "metre cubed mole to the power minus 1") (declared-name "metre cubed mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (declared-name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "C")))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "A")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (name "metre cubed second to the power minus 1") (declared-name "metre cubed second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 3)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre per second"))) (name "metre per second") (declared-name "metre per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "m")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (name "metre second to the power minus 1") (declared-name "metre second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "m")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (name "metre second to the power minus 2") (declared-name "metre second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "m")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared"))) (name "metre squared") (declared-name "metre squared") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared ampere"))) (name "metre squared ampere") (declared-name "metre squared ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (name "metre squared kelvin per watt") (declared-name "metre squared kelvin per watt") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "K")))) (expression (kind "featureReference") (reference "W")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (name "metre squared mole to the power minus 1") (declared-name "metre squared mole to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "mol")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared per joule"))) (name "metre squared per joule") (declared-name "metre squared per joule") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "J")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared per joule"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (name "metre squared per joule steradian") (declared-name "metre squared per joule steradian") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "sr")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (name "metre squared per volt second") (declared-name "metre squared per volt second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "s")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (name "metre squared second to the power minus 1") (declared-name "metre squared second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (name "metre squared second to the power minus 2") (declared-name "metre squared second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (name "metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "metre squared second to the power minus 2 kelvin to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "K")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (name "metre squared second to the power minus 3") (declared-name "metre squared second to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (name "metre squared steradian to the power minus 1") (declared-name "metre squared steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power 4"))) (name "metre to the power 4") (declared-name "metre to the power 4") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 4)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power 4"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (name "metre to the power 4 second to the power minus 2") (declared-name "metre to the power 4 second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 4)))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (name "metre to the power minus 1") (declared-name "metre to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (name "metre to the power minus 2") (declared-name "metre to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (name "metre to the power minus 2 second to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (name "metre to the power minus 3") (declared-name "metre to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (name "metre to the power minus 3 second") (declared-name "metre to the power minus 3 second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (name "metre to the power minus 3 second to the power minus 1") (declared-name "metre to the power minus 3 second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (role feature-value))))
        (element (kind "alias") (id (node (document "d0") (qualified-name "SI::metric ton"))) (name "metric ton") (declared-name "metric ton"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::millilitre"))) (name "millilitre") (declared-name "millilitre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::millilitre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::millimetre"))) (name "millimetre") (declared-name "millimetre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::millimetre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::millinewton"))) (name "millinewton") (declared-name "millinewton") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::millinewton")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::minute"))) (name "minute") (declared-name "minute") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::minute")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::minute (angle)"))) (name "minute (angle)") (declared-name "minute (angle)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::minute (angle)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ml per l"))) (name "ml per l") (declared-name "ml per l") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mL")) (expression (kind "featureReference") (reference "L")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ml per l"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole"))) (name "mole") (declared-name "mole") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (name "mole kilogram to the power minus 1") (declared-name "mole kilogram to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "kg")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (name "mole metre to the power minus 3") (declared-name "mole metre to the power minus 3") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 3)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (name "mole per cubic metre") (declared-name "mole per cubic metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole per kilogram"))) (name "mole per kilogram") (declared-name "mole per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::mole per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::mole per l"))) (name "mole per l") (declared-name "mole per l") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mol")) (expression (kind "featureReference") (reference "L")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::mole per l"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::nanometre"))) (name "nanometre") (declared-name "nanometre") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::nanometre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::natural unit of information"))) (name "natural unit of information") (declared-name "natural unit of information") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::natural unit of information"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (name "natural unit of information per second") (declared-name "natural unit of information per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "nat")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton"))) (name "newton") (declared-name "newton") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "kg")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton metre"))) (name "newton metre") (declared-name "newton metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton metre"))) (role feature-value)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::newton metre")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton metre second"))) (name "newton metre second") (declared-name "newton metre second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton metre second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (name "newton metre second to the power minus 1") (declared-name "newton metre second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (name "newton metre to the power minus 1") (declared-name "newton metre to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (name "newton metre to the power minus 2") (declared-name "newton metre to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::newton second"))) (name "newton second") (declared-name "newton second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::newton second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::octave"))) (name "octave") (declared-name "octave") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::octave"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::octet"))) (name "octet") (declared-name "octet") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::octet"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::octet per second"))) (name "octet per second") (declared-name "octet per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "o")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::octet per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ohm"))) (name "ohm") (declared-name "ohm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ohm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ohm metre"))) (name "ohm metre") (declared-name "ohm metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Ω")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::ohm metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal"))) (name "pascal") (declared-name "pascal") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (name "pascal per kelvin") (declared-name "pascal per kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal second"))) (name "pascal second") (declared-name "pascal second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (name "pascal second per cubic metre") (declared-name "pascal second per cubic metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal second per metre"))) (name "pascal second per metre") (declared-name "pascal second per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "featureReference") (reference "s")))) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal second per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (name "pascal to the power 2 second") (declared-name "pascal to the power 2 second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (name "pascal to the power minus 1") (declared-name "pascal to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "Pa")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian"))) (name "radian") (declared-name "radian") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "m")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (name "radian metre squared per kilogram to the power 1") (declared-name "radian metre squared per kilogram to the power 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "rad")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "kg")))) (expression (kind "integerLiteral") (literal 1)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (name "radian metre squared per mole") (declared-name "radian metre squared per mole") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "rad")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "mol")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian per metre"))) (name "radian per metre") (declared-name "radian per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "rad")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (name "radian second to the power minus 1") (declared-name "radian second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "rad")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (name "radian second to the power minus 2") (declared-name "radian second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "rad")) (expression (kind "featureReference") (reference "s")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second"))) (name "second") (declared-name "second") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second (angle)"))) (name "second (angle)") (declared-name "second (angle)") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::second (angle)")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second ampere"))) (name "second ampere") (declared-name "second ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "s")) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::second ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (name "second to the power minus 1") (declared-name "second to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (name "second to the power minus 1 steradian to the power minus 1") (declared-name "second to the power minus 1 steradian to the power minus 1") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))) (expression (kind "featureReference") (reference "sr")))) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (name "second to the power minus 2") (declared-name "second to the power minus 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "s")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 2)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::shannon"))) (name "shannon") (declared-name "shannon") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "one")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::shannon"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::shannon per second"))) (name "shannon per second") (declared-name "shannon per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Sh")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::shannon per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::siemens"))) (name "siemens") (declared-name "siemens") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "Ω")) (expression (kind "unary") (operator "-") (children (expression (kind "integerLiteral") (literal 1)))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::siemens"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (name "siemens metre squared per mole") (declared-name "siemens metre squared per mole") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "S")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "mol")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::siemens per metre"))) (name "siemens per metre") (declared-name "siemens per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "S")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::siemens per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::sievert"))) (name "sievert") (declared-name "sievert") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::sievert"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::sievert per second"))) (name "sievert per second") (declared-name "sievert per second") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Sv")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::sievert per second"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::steradian"))) (name "steradian") (declared-name "steradian") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::steradian"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::tesla"))) (name "tesla") (declared-name "tesla") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Wb")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::tesla"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::tonne"))) (name "tonne") (declared-name "tonne") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::tonne")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt"))) (name "volt") (declared-name "volt") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::volt"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt ampere"))) (name "volt ampere") (declared-name "volt ampere") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "A")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::volt ampere"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (name "volt ampere reactive") (declared-name "volt ampere reactive") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::volt ampere reactive")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt per kelvin"))) (name "volt per kelvin") (declared-name "volt per kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::volt per kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt per metre"))) (name "volt per metre") (declared-name "volt per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::volt per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (name "volt to the power 2 per kelvin to the power 2") (declared-name "volt to the power 2 per kelvin to the power 2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "V")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "K")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt"))) (name "watt") (declared-name "watt") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "J")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt hour"))) (name "watt hour") (declared-name "watt hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt hour"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per kelvin"))) (name "watt per kelvin") (declared-name "watt per kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "K")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per kilogram"))) (name "watt per kilogram") (declared-name "watt per kilogram") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "kg")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per kilogram"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (name "watt per metre kelvin") (declared-name "watt per metre kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "m")) (expression (kind "featureReference") (reference "K")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per nm"))) (name "watt per nm") (declared-name "watt per nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "nm")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per square metre"))) (name "watt per square metre") (declared-name "watt per square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (name "watt per square metre kelvin") (declared-name "watt per square metre kelvin") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "K")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (name "watt per square metre nm") (declared-name "watt per square metre nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "featureReference") (reference "m")) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "nm")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per steradian"))) (name "watt per steradian") (declared-name "watt per steradian") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "featureReference") (reference "sr")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per steradian"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (name "watt per steradian nm") (declared-name "watt per steradian nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "sr")) (expression (kind "featureReference") (reference "nm")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (name "watt per steradian square metre") (declared-name "watt per steradian square metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "sr")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (name "watt per steradian square metre nm") (declared-name "watt per steradian square metre nm") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "W")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "*") (children (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "sr")) (expression (kind "featureReference") (reference "m")))) (expression (kind "integerLiteral") (literal 2)))) (expression (kind "featureReference") (reference "nm")))))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::weber"))) (name "weber") (declared-name "weber") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "V")) (expression (kind "featureReference") (reference "s")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::weber"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::weber metre"))) (name "weber metre") (declared-name "weber metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "Wb")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::weber metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::weber per metre"))) (name "weber per metre") (declared-name "weber per metre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "Wb")) (expression (kind "featureReference") (reference "m")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "SI::weber per metre"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "SI::ångström"))) (name "ångström") (declared-name "ångström") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SI::ångström")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SI::_documentation"))) (to (node (document "d0") (qualified-name "SI"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::_documentation"))) (to (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::_documentation"))) (to (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/si.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 4) (end 9 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 4) (end 10 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 4) (end 11 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 4) (end 16 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 4) (end 21 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 4) (end 22 127))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 22 41) (end 22 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 41) (end 22 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 4) (end 23 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 4) (end 24 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 4) (end 25 673))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 8) (end 26 212))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 30 8) (end 30 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 8) (end 31 282))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 4) (end 35 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 4) (end 36 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 1) (end 42 159))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 2) (end 43 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 44 2) (end 44 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 4) (end 50 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 4) (end 51 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 4) (end 52 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 4) (end 53 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 4) (end 54 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 4) (end 55 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 4) (end 57 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 4) (end 58 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 4) (end 59 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 4) (end 60 301))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 61 8) (end 61 230))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 4) (end 66 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 4) (end 67 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 4) (end 69 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 4) (end 70 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 4) (end 72 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 4) (end 73 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 4) (end 74 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 4) (end 75 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 4) (end 76 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 4) (end 77 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 4) (end 78 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 4) (end 79 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 4) (end 81 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 4) (end 82 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 4) (end 83 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 4) (end 84 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 4) (end 85 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 4) (end 90 152))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 90 49) (end 90 150))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 90 49) (end 90 150))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 4) (end 91 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 91 36) (end 91 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 91 36) (end 91 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 4) (end 92 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 92 38) (end 92 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 38) (end 92 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 4) (end 93 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 93 39) (end 93 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 39) (end 93 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 4) (end 94 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 94 47) (end 94 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 47) (end 94 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 4) (end 95 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 95 39) (end 95 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 39) (end 95 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 96 4) (end 96 142))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 96 44) (end 96 140))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 96 44) (end 96 140))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 4) (end 97 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 97 39) (end 97 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 39) (end 97 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 4) (end 98 136))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 98 33) (end 98 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 33) (end 98 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 4) (end 100 150))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 100 50) (end 100 148))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 50) (end 100 148))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 4) (end 101 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 101 54) (end 101 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 54) (end 101 184))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 4) (end 102 158))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 102 57) (end 102 156))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 102 57) (end 102 156))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 4) (end 103 182))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 103 51) (end 103 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 51) (end 103 180))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 4) (end 104 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 104 62) (end 104 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 62) (end 104 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 4) (end 106 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 106 62) (end 106 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 62) (end 106 191))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 4) (end 112 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 4) (end 113 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 114 4) (end 114 164))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 4) (end 115 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 4) (end 116 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 4) (end 117 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 4) (end 118 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 4) (end 119 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 120 4) (end 120 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 4) (end 121 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 4) (end 122 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 123 4) (end 123 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 124 4) (end 124 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 4) (end 125 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 126 4) (end 126 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 4) (end 127 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 4) (end 128 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 129 4) (end 129 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 4) (end 130 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 4) (end 131 207))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 4) (end 132 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 133 4) (end 133 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 4) (end 134 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 4) (end 135 156))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 4) (end 136 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 4) (end 137 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 4) (end 138 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 4) (end 139 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 4) (end 140 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 141 4) (end 141 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 142 4) (end 142 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 4) (end 143 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 144 4) (end 144 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 4) (end 145 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 146 4) (end 146 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 4) (end 147 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 148 4) (end 148 106))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 4) (end 149 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 4) (end 150 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 151 4) (end 151 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 4) (end 152 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 4) (end 153 90))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 154 4) (end 154 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 155 4) (end 155 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 156 4) (end 156 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 157 4) (end 157 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 4) (end 158 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 4) (end 159 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 4) (end 160 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 4) (end 161 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 162 4) (end 162 221))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 4) (end 163 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 4) (end 164 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 4) (end 165 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 166 4) (end 166 106))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 4) (end 167 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 168 4) (end 168 117))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 4) (end 169 159))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 170 4) (end 170 169))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 171 4) (end 171 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 172 4) (end 172 143))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 173 4) (end 173 135))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 4) (end 174 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 4) (end 175 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 4) (end 176 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 177 4) (end 177 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 178 4) (end 178 135))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 179 4) (end 179 174))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 4) (end 180 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 181 4) (end 181 145))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 182 4) (end 182 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 4) (end 183 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 4) (end 184 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 185 4) (end 185 164))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 186 4) (end 186 213))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 4) (end 187 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 4) (end 188 115))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 4) (end 189 179))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 4) (end 190 230))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 191 4) (end 191 170))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 192 4) (end 192 173))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 193 4) (end 193 161))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 4) (end 194 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 195 4) (end 195 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 196 4) (end 196 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 4) (end 197 147))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 4) (end 198 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 4) (end 199 153))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 4) (end 200 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 201 4) (end 201 98))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 202 4) (end 202 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 203 4) (end 203 153))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 204 4) (end 204 176))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 205 4) (end 205 202))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 4) (end 206 168))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 4) (end 207 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 208 4) (end 208 109))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 209 4) (end 209 116))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 210 4) (end 210 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 211 4) (end 211 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 212 4) (end 212 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 4) (end 213 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 214 4) (end 214 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 215 4) (end 215 159))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 216 4) (end 216 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 217 4) (end 217 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 218 4) (end 218 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 219 4) (end 219 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 220 4) (end 220 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 221 4) (end 221 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 4) (end 222 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 4) (end 223 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 224 4) (end 224 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 4) (end 225 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 226 4) (end 226 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 227 4) (end 227 171))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 4) (end 228 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 229 4) (end 229 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 230 4) (end 230 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 4) (end 231 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 4) (end 232 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 233 4) (end 233 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 234 4) (end 234 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 235 4) (end 235 111))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 236 4) (end 236 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 237 4) (end 237 155))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 238 4) (end 238 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 239 4) (end 239 133))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 240 4) (end 240 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 241 4) (end 241 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 242 4) (end 242 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 243 4) (end 243 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 244 4) (end 244 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 245 4) (end 245 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 246 4) (end 246 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 247 4) (end 247 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 248 4) (end 248 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 249 4) (end 249 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 250 4) (end 250 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 251 4) (end 251 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 252 4) (end 252 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 253 4) (end 253 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 254 4) (end 254 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 255 4) (end 255 315))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 256 8) (end 256 230))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 260 4) (end 260 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 261 4) (end 261 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 262 4) (end 262 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 263 4) (end 263 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 264 4) (end 264 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 265 4) (end 265 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 266 4) (end 266 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 267 4) (end 267 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 268 4) (end 268 121))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 269 4) (end 269 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 270 4) (end 270 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 271 4) (end 271 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 272 4) (end 272 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 273 4) (end 273 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 274 4) (end 274 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 275 4) (end 275 101))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 276 4) (end 276 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 277 4) (end 277 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 278 4) (end 278 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 279 4) (end 279 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 280 4) (end 280 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 281 4) (end 281 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 282 4) (end 282 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 283 4) (end 283 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 284 4) (end 284 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 285 4) (end 285 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 286 4) (end 286 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 287 4) (end 287 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 288 4) (end 288 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 289 4) (end 289 108))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 290 4) (end 290 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 291 4) (end 291 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 292 4) (end 292 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 293 4) (end 293 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 294 4) (end 294 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 295 4) (end 295 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 296 4) (end 296 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 297 4) (end 297 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 298 4) (end 298 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 299 4) (end 299 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 300 4) (end 300 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 301 4) (end 301 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 302 4) (end 302 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 303 4) (end 303 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 304 4) (end 304 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 313 4) (end 313 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 313 44) (end 313 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 313 44) (end 313 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 314 4) (end 314 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 314 45) (end 314 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 314 45) (end 314 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 315 4) (end 315 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 315 45) (end 315 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 315 45) (end 315 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 316 4) (end 316 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 316 44) (end 316 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 316 44) (end 316 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 319 4) (end 319 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 319 45) (end 319 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 319 45) (end 319 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 322 4) (end 322 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 322 45) (end 322 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 322 45) (end 322 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 325 4) (end 325 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 325 44) (end 325 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 325 44) (end 325 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 326 4) (end 326 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 326 44) (end 326 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 326 44) (end 326 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 4) (end 327 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 327 44) (end 327 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 44) (end 327 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 330 4) (end 330 128))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 330 42) (end 330 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 330 42) (end 330 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 4) (end 333 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 339 4) (end 339 340))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 345 8) (end 345 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 345 8) (end 345 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 348 4) (end 348 1647))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 357 8) (end 357 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 358 8) (end 358 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 361 8) (end 361 210))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 364 8) (end 364 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 368 8) (end 368 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 369 8) (end 369 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 372 8) (end 372 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 373 8) (end 373 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 8) (end 373 172))
      )
    )
  )
)
~~~
