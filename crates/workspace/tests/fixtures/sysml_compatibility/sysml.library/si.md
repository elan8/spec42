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
    doc /*
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
    attribute <kg> kilogram : MassUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = kilo;
            :>> referenceUnit = g;
        }
    }
    attribute <s> second : DurationUnit;
    attribute <A> ampere : ElectricCurrentUnit;
    attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit {
        attribute temperatureOfWaterAtTriplePointInK : DefinitionalQuantityValue {
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
    attribute <'Å'> 'ångström' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 1.0e-10;
        }
    }
    attribute <b> barn : AreaUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = 'm²';
            :>> conversionFactor = 1.0e-28;
        }
    }
    attribute <d> day : DurationUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = hour;
            :>> conversionFactor = 24;
        }
    }
    attribute <Da> dalton : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 1.66053906660e-27;
            :>> isExact = false;
        }
    }
    attribute <eV> electronvolt : EnergyUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = J;
            :>> conversionFactor = 1.602176487e-19;
            :>> isExact = false;
        }
    }
    attribute <h> hour : DurationUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = min;
            :>> conversionFactor = 60;
        }
    }
    attribute <min> minute : DurationUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = s;
            :>> conversionFactor = 60;
        }
    }
    attribute <L> litre : VolumeUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = 'm³';
            :>> conversionFactor = 1.0e-3;
        }
    }
    attribute tonne : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = kg;
            :>> conversionFactor = 1.0e-3;
        }
    }
    alias 'metric ton' for tonne;
    attribute <u> 'atomic mass unit' : MassUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = Da;
            :>> conversionFactor = 1.0;
        }
    }
    attribute <ua> 'astronomical unit' : LengthUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = m;
            :>> conversionFactor = 149597870691e11;
            :>> isExact = false;
        }
    }
    attribute <var> 'volt ampere reactive' : PowerUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = V*A;
            :>> conversionFactor = 1.0;
        }
    }
    attribute <'°'> degree : AngularMeasureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = rad;
            :>> conversionFactor = 1.745329E-02;
            :>> isExact = false;
        }
    }
    // conversionFactor should become pi/180
    attribute <'′'> 'minute (angle)' : AngularMeasureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = rad;
            :>> conversionFactor = 2.908882E-04;
            :>> isExact = false;
        }
    }
    alias arcmin for '′';
    attribute <'″'> 'second (angle)' : AngularMeasureUnit {
        :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = rad;
            :>> conversionFactor = 4.848137E-06;
            :>> isExact = false;
        }
    }
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
    attribute <nm> nanometre : LengthUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = nano;
            :>> referenceUnit = m;
        }
    }
    attribute <mm> millimetre : LengthUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = milli;
            :>> referenceUnit = m;
        }
    }
    attribute <cm> centimetre : LengthUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = centi;
            :>> referenceUnit = m;
        }
    }
    attribute <km> kilometre : LengthUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = kilo;
            :>> referenceUnit = m;
        }
    }

    /* Volume */
    attribute <mL> millilitre : VolumeUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = milli;
            :>> referenceUnit = L;
        }
    }

    /* Force */
    attribute <mN> millinewton : ForceUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = milli;
            :>> referenceUnit = N;
        }
    }

    /* Energy */
    attribute <kJ> kilojoule : EnergyUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = kilo;
            :>> referenceUnit = J;
        }
    }
    attribute <MJ> megajoule : EnergyUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = mega;
            :>> referenceUnit = J;
        }
    }
    attribute <GJ> gigajoule : EnergyUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = giga;
            :>> referenceUnit = J;
        }
    }

    /* Power */
    attribute <kW> kilowatt : PowerUnit {
        :>> unitConversion : ConversionByPrefix {
            :>> prefix = kilo;
            :>> referenceUnit = W;
        }
    }

    /* Speed */
    attribute <'km/h'> 'kilometre per hour' : SpeedUnit = km/h;

    /* 
	 * Celsius units
	 */

    attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit {
        doc /*
	     * degree Celsius unit for temperature interval (i.e. temperature difference) quantities
	     */

        attribute :>> unitConversion : ConversionByConvention {
            :>> referenceUnit = K;
            :>> conversionFactor = 1;
        }
    }

    attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
        doc /*
	     * degree Celsius interval scale for absolute (thermodynamic) temperature quantities
	     *
	     * The interval scale is defined with an explicit transformation with respect to 
	     * the kelvin thermodynamic temperature scale that specifies the zero shift.
	     */

        attribute :>> unit = '°C';
        attribute temperatureWaterAtFreezingPointInC : DefinitionalQuantityValue {
            :>> num = 0;
            :>> definition = "temperature in degree Celsius of pure water at freezing point";
        }
        private attribute temperatureWaterAtTriplePointInC : DefinitionalQuantityValue {
            :>> num = 1/100;
            :>> definition = "temperature in degree Celsius of pure water at the triple point";
        }
        private attribute celsiusToKelvinScaleMapping : QuantityValueMapping {
            :>> mappedQuantityValue = temperatureWaterAtTriplePointInC;
            :>> referenceQuantityValue = K.temperatureOfWaterAtTriplePointInK;
        }
        attribute :>> definitionalQuantityValues = (temperatureWaterAtTriplePointInC, temperatureWaterAtFreezingPointInC);
        attribute :>> quantityValueMapping = celsiusToKelvinScaleMapping;

        /* CoordinateFramePlacement (zero shift) w.r.t. the kelvin thermodynamic temperature scale */
        private attribute zeroDegreeCelsiusInKelvin : ThermodynamicTemperatureValue = 273.15 [K];
        attribute zeroDegreeCelsiusToKelvinShift : CoordinateFramePlacement :>> transformation {
            :>> source = K;
            :>> origin = zeroDegreeCelsiusInKelvin;
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'SI'
      (documentation)
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import public -> 'ISQ'[unresolved])
      (namespace_import public -> 'SIPrefixes'[unresolved])
      (attribute_usage 'gram' : 'MassUnit'[unresolved])
      (attribute_usage 'metre' : 'LengthUnit'[unresolved])
      (attribute_usage 'kilogram' : 'MassUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'second' : 'DurationUnit'[unresolved])
      (attribute_usage 'ampere' : 'ElectricCurrentUnit'[unresolved])
      (attribute_usage 'kelvin' : 'ThermodynamicTemperatureUnit'[unresolved] : 'TemperatureDifferenceUnit'[unresolved]
        (attribute_usage composite 'temperatureOfWaterAtTriplePointInK' : 'DefinitionalQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'definition'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'definitionalQuantityValues'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'ThermodynamicTemperatureUnit::quantityDimension'[unresolved] :>> 'TemperatureDifferenceUnit::quantityDimension'[unresolved]
          (reference_usage reference :>> 'ThermodynamicTemperatureUnit::quantityDimension::quantityPowerFactors'[unresolved] :>> 'TemperatureDifferenceUnit::quantityDimension::quantityPowerFactors'[unresolved])))
      (attribute_usage 'mole' : 'AmountOfSubstanceUnit'[unresolved])
      (attribute_usage 'candela' : 'LuminousIntensityUnit'[unresolved])
      (attribute_usage 'ISO/IEC 80000 International System of Units' : 'SystemOfUnits'[unresolved]
        (reference_usage reference :>> 'systemOfQuantities'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'baseUnits'[unresolved]
          (feature_value (=))))
      (attribute_usage 'byte' : 'StorageCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'baud' : 'ModulationRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'bit' : 'StorageCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'becquerel' : 'NuclearActivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb' : 'ElectricChargeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'decibel' : 'SoundPressureLevelUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'decade' : 'LogarithmicFrequencyRangeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'erlang' : 'TrafficIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'farad' : 'CapacitanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'gray' : 'AbsorbedDoseUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'henry' : 'PermeanceUnit'[unresolved] : 'InductanceUnit'[unresolved]
        (feature_value (=))
        (attribute_usage composite :>> 'PermeanceUnit::quantityDimension'[unresolved] :>> 'InductanceUnit::quantityDimension'[unresolved]
          (reference_usage reference :>> 'PermeanceUnit::quantityDimension::quantityPowerFactors'[unresolved] :>> 'InductanceUnit::quantityDimension::quantityPowerFactors'[unresolved])))
      (attribute_usage 'hartley' : 'InformationContentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'hertz' : 'FrequencyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule' : 'EnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lumen' : 'LuminousFluxUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lux' : 'IlluminanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton' : 'ForceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'natural unit of information' : 'InformationContentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'octet' : 'StorageCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'octave' : 'LogarithmicFrequencyRangeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal' : 'PressureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian' : 'AngularMeasureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'siemens' : 'ConductanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'shannon' : 'InformationContentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'steradian' : 'SolidAngularMeasureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'sievert' : 'DoseEquivalentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'tesla' : 'MagneticFluxDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'volt' : 'ElectricPotentialUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt' : 'PowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'weber' : 'MagneticFluxUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ohm' : 'ResistanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ångström' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'barn' : 'AreaUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'day' : 'DurationUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'dalton' : 'MassUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'electronvolt' : 'EnergyUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'hour' : 'DurationUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'minute' : 'DurationUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'litre' : 'VolumeUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'tonne' : 'MassUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (alias_member 'metric ton' -> 'SI::tonne'[attribute_usage])
      (attribute_usage 'atomic mass unit' : 'MassUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'astronomical unit' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'volt ampere reactive' : 'PowerUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'degree' : 'AngularMeasureUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'minute (angle)' : 'AngularMeasureUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (alias_member 'arcmin' -> 'SI::minute (angle)'[attribute_usage])
      (attribute_usage 'second (angle)' : 'AngularMeasureUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'isExact'[unresolved]
            (feature_value (=)))))
      (alias_member 'arcsec' -> 'SI::second (angle)'[attribute_usage])
      (attribute_usage 'ampere metre to the power minus 2 kelvin to the power minus 2' : 'RichardsonConstantUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ampere metre squared' : 'MagneticMomentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ampere metre squared joule to the power minus 1 second to the power minus 1' : 'GyromagneticRatioUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ampere second per kilogram' : 'GyromagneticRatioUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ampere per metre' : 'LinearElectricCurrentDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ampere per square metre' : 'ElectricCurrentDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'byte per second' : 'TransferRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'bit per second' : 'BinaryDigitRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'becquerel per kilogram' : 'SpecificActivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'becquerel per square metre' : 'SurfaceActivityDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'becquerel per cubic metre' : 'ActivityDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb metre' : 'ElectricDipoleMomentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb per kilogram second' : 'ExposureRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb per kilogram' : 'ExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb per metre' : 'LinearDensityOfElectricChargeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb per square metre' : 'SurfaceDensityOfElectricChargeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'coulomb per cubic metre' : 'ElectricChargeDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela metre to the power minus 2' : 'LuminanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela steradian' : 'LuminousFluxUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3' : 'LuminousEfficacyOfRadiationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela steradian metre to the power minus 2' : 'IlluminanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela steradian metre to the power minus 2 second' : 'LuminousExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'candela steradian second' : 'LuminousEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'electronvolt joule kilogram metre squared second to the power minus 2' : 'HartreeEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'electronvolt metre to the power minus 2 per kilogram' : 'TotalMassStoppingPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'electronvolt per metre' : 'TotalLinearStoppingPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'electronvolt per square metre' : 'EnergyFluenceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'farad per metre' : 'ElectricConstantUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'g per l' : 'MassConcentrationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'g per mole' : 'MolarMassUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'gray per second' : 'AbsorbedDoseRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'henry per metre' : 'MagneticConstantUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'henry to the power minus 1' : 'ReluctanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'hartley per second' : 'AverageInformationRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule metre squared per kilogram' : 'TotalMassStoppingPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule second' : 'ActionQuantityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule second electronvolt second' : 'TotalAngularMomentumUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule second to the power minus 1' : 'PowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per kilogram kelvin' : 'SpecificHeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per square metre nm' : 'SpectralRadiantExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per cubic metre nm' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per mole kelvin' : 'MolarHeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per kelvin' : 'HeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per kilogram' : 'SpecificEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per metre' : 'TotalLinearStoppingPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per square metre' : 'SpectralRadiantEnergyDensityInTermsOfWavenumberUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per cubic metre' : 'ElectromagneticEnergyDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per mole' : 'MolarInternalEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per nm' : 'SpectralRadiantEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule per second' : 'HeatFlowRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3' : 'EnergyDensityOfStatesUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kelvin per pascal' : 'JouleThomsonCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kelvin per watt' : 'ThermalResistanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kelvin to the power minus 1' : 'LinearExpansionCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre second to the power minus 1' : 'MomentumUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre second to the power minus 2' : 'ForceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre second to the power minus 3' : 'SpectralRadiantFluxUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre second to the power minus 3 kelvin to the power minus 1' : 'ThermalConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre second to the power minus 3 steradian to the power minus 1' : 'SpectralRadiantIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1' : 'LinearMassDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1 second to the power minus 1' : 'DynamicViscosityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1 second to the power minus 2' : 'PressureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1' : 'PressureCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1 second to the power minus 3' : 'SpectralIrradianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1' : 'SpectralRadianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 2' : 'SurfaceMassDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 2 second to the power minus 1' : 'MassFlowUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 2 second to the power minus 2' : 'SpectralRadiantEnergyDensityInTermsOfWavelengthUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 3' : 'MassDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre to the power minus 4 second to the power minus 1' : 'AcousticImpedanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared' : 'MomentOfInertiaUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 1' : 'AngularMomentumUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 2' : 'MomentOfForceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1' : 'HeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1' : 'MolarHeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 2 mole to the power minus 1' : 'MolarInternalEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 3' : 'PowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 3 ampere to the power minus 1' : 'ElectricPotentialDifferenceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1' : 'SeebeckCoefficientForSubstancesAAndBUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 3 kelvin to the power minus 1' : 'ThermalConductanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre squared second to the power minus 3 steradian to the power minus 1' : 'RadiantIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram metre cubed second to the power minus 3 ampere to the power minus 2' : 'ResistivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram mole to the power minus 1' : 'MolarMassUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 1' : 'MassFlowRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 2' : 'SurfaceTensionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 2 ampere to the power minus 1' : 'MagneticFluxDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 3' : 'DensityOfHeatFlowRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 3 kelvin to the power minus 1' : 'CoefficientOfHeatTransferUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram second to the power minus 3 steradian to the power minus 1' : 'RadianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 ampere' : 'ExposureRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre second to the power 2' : 'CompressibilityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre second to the power 2 kelvin' : 'JouleThomsonCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin' : 'ThermalResistanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2' : 'ElectrolyticConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre to the power minus 5 second to the power 2' : 'EnergyDensityOfStatesUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre squared' : 'MassAttenuationCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 metre cubed' : 'SpecificVolumeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second ampere' : 'GyromagneticRatioUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second to the power minus 1' : 'SpecificActivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second to the power 2' : 'EnergyDistributionOfCrossSectionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second to the power 2 ampere' : 'MobilityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1' : 'MolarConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power minus 1 second to the power 3 kelvin' : 'ThermalInsulanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power 2 metre to the power minus 2 second to the power minus 3' : 'SoundExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2' : 'LorenzCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lumen second' : 'LuminousEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lumen per square metre' : 'LuminousExitanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lumen per watt' : 'LuminousEfficacyOfRadiationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'lux second' : 'LuminousExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre second to the power minus 1' : 'SpeedUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre second to the power minus 2' : 'AccelerationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre per second' : 'SpeedUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 1' : 'CurvatureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 2' : 'PhotonExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 2 second to the power minus 1' : 'PhotonIrradianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 2 second to the power minus 1 steradian to the power minus 1' : 'PhotonRadianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 3' : 'ParticleConcentrationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 3 second' : 'DensityOfVibrationalStatesUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power minus 3 second to the power minus 1' : 'ActivityDensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared' : 'AreaUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared ampere' : 'MagneticDipoleMomentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared kelvin per watt' : 'ThermalInsulanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared mole to the power minus 1' : 'MolarAbsorptionCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared second to the power minus 1' : 'KinematicViscosityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared second to the power minus 2' : 'SpecificEnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared second to the power minus 2 kelvin to the power minus 1' : 'SpecificHeatCapacityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared second to the power minus 3' : 'DoseEquivalentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared steradian to the power minus 1' : 'DirectionDistributionOfCrossSectionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared per joule steradian' : 'DirectionAndEnergyDistributionOfCrossSectionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared per volt second' : 'MobilityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre squared per joule' : 'EnergyDistributionOfCrossSectionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre cubed' : 'VolumeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre cubed mole to the power minus 1' : 'MolarVolumeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre cubed second to the power minus 1' : 'VolumeFlowRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1' : 'HallCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power 4' : 'SecondAxialMomentOfAreaUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'metre to the power 4 second to the power minus 2' : 'TotalMassStoppingPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ml per l' : 'VolumeFractionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'mole kilogram to the power minus 1' : 'IonicStrengthUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'mole metre to the power minus 3' : 'AmountOfSubstanceConcentrationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'mole per kilogram' : 'MolalityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'mole per l' : 'AmountOfSubstanceConcentrationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'mole per cubic metre' : 'EquilibriumConstantOnConcentrationBasisUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton metre' : 'MomentOfForceUnit'[unresolved] : 'TorqueUnit'[unresolved]
        (feature_value (=))
        (attribute_usage composite :>> 'MomentOfForceUnit::quantityDimension'[unresolved] :>> 'TorqueUnit::quantityDimension'[unresolved]
          (reference_usage reference :>> 'MomentOfForceUnit::quantityDimension::quantityPowerFactors'[unresolved] :>> 'TorqueUnit::quantityDimension::quantityPowerFactors'[unresolved])))
      (attribute_usage 'newton metre second' : 'AngularImpulseUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton metre second to the power minus 1' : 'PowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton metre to the power minus 1' : 'SurfaceTensionUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton metre to the power minus 2' : 'PressureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'newton second' : 'ImpulseUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'natural unit of information per second' : 'AverageInformationRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'octet per second' : 'TransferRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal second' : 'DynamicViscosityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal second per metre' : 'CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal second per cubic metre' : 'AcousticImpedanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal per kelvin' : 'PressureCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal to the power minus 1' : 'CompressibilityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'pascal to the power 2 second' : 'SoundExposureUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian metre squared per kilogram to the power 1' : 'SpecificOpticalRotatoryPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian metre squared per mole' : 'MolarOpticalRotatoryPowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian second to the power minus 1' : 'AngularVelocityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian second to the power minus 2' : 'AngularAccelerationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'radian per metre' : 'PhaseCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'second ampere' : 'ElectricChargeUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'siemens metre squared per mole' : 'MolarConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'siemens per metre' : 'ConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'second to the power minus 1' : 'AngularVelocityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'second to the power minus 1 steradian to the power minus 1' : 'PhotonIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'second to the power minus 2' : 'AngularAccelerationUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'shannon per second' : 'AverageInformationRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'sievert per second' : 'DoseEquivalentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'volt ampere' : 'PowerUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'volt per kelvin' : 'SeebeckCoefficientForSubstancesAAndBUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'volt per metre' : 'ElectricFieldStrengthUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'volt to the power 2 per kelvin to the power 2' : 'LorenzCoefficientUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt hour' : 'EnergyUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per metre kelvin' : 'ThermalConductivityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per square metre kelvin' : 'CoefficientOfHeatTransferUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per square metre nm' : 'SpectralIrradianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per steradian square metre' : 'RadianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per steradian square metre nm' : 'SpectralRadianceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per steradian nm' : 'SpectralRadiantIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per kelvin' : 'ThermalConductanceUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per kilogram' : 'DoseEquivalentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per square metre' : 'DensityOfHeatFlowRateUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per nm' : 'SpectralRadiantFluxUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'watt per steradian' : 'RadiantIntensityUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'weber metre' : 'MagneticDipoleMomentUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'weber per metre' : 'MagneticVectorPotentialUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'ohm metre' : 'ResistivityUnit'[unresolved]
        (feature_value (=)))
      (alias_member 'm/s²' -> 'SI::metre second to the power minus 2'[attribute_usage])
      (attribute_usage 'nanometre' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'millimetre' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'centimetre' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'kilometre' : 'LengthUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'millilitre' : 'VolumeUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'millinewton' : 'ForceUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'kilojoule' : 'EnergyUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'megajoule' : 'EnergyUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'gigajoule' : 'EnergyUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'kilowatt' : 'PowerUnit'[unresolved]
        (reference_usage reference :>> 'unitConversion'[unresolved] : 'ConversionByPrefix'[unresolved]
          (reference_usage reference :>> 'prefix'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'kilometre per hour' : 'SpeedUnit'[unresolved]
        (feature_value (=)))
      (attribute_usage 'degree celsius (temperature difference)' : 'TemperatureDifferenceUnit'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'unitConversion'[unresolved] : 'ConversionByConvention'[unresolved]
          (reference_usage reference :>> 'referenceUnit'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'conversionFactor'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'degree celsius (absolute temperature scale)' : 'IntervalScale'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'unit'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'temperatureWaterAtFreezingPointInC' : 'DefinitionalQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'definition'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'temperatureWaterAtTriplePointInC' : 'DefinitionalQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'definition'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'celsiusToKelvinScaleMapping' : 'QuantityValueMapping'[unresolved]
          (reference_usage reference :>> 'mappedQuantityValue'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'referenceQuantityValue'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'definitionalQuantityValues'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'quantityValueMapping'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'zeroDegreeCelsiusInKelvin' : 'ThermodynamicTemperatureValue'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'zeroDegreeCelsiusToKelvinShift' : 'CoordinateFramePlacement'[unresolved] :>> 'transformation'[unresolved]
          (reference_usage reference :>> 'source'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'origin'[unresolved]
            (feature_value (=))))))))
~~~
