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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "si.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 18) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 18) (end 11 28))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 22) (end 31 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 71) (end 31 115))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 4) (end 60 301))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 22) (end 61 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 56) (end 61 89))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 255 4) (end 255 315))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 256 22) (end 256 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 256 60) (end 256 89))
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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 372 8) (end 372 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 372 8) (end 372 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 373 8) (end 373 172))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 373 80) (end 373 94))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "952d591133b74150c878a2f0b1627331ea8ec95a2f708d0978fc2304abcef40f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SI"))) (kind "package") (name "SI") (declared-name "SI"))
    (element (id (node (document "d0") (qualified-name "SI::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SI::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SI::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "public") (import (reference "SIPrefixes::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (kind "attribute def") (name "ISO/IEC 80000 International System of Units") (declared-name "ISO/IEC 80000 International System of Units") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SystemOfUnits")))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (kind "attribute") (name "baseUnits") (declared-name "baseUnits") (parent (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseUnits")))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (kind "attribute") (name "systemOfQuantities") (declared-name "systemOfQuantities") (parent (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "systemOfQuantities")))))
    (element (id (node (document "d0") (qualified-name "SI::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::ampere"))) (kind "attribute def") (name "ampere") (declared-name "ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricCurrentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre squared"))) (kind "attribute def") (name "ampere metre squared") (declared-name "ampere metre squared") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticMomentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "ampere metre squared joule to the power minus 1 second to the power minus 1") (declared-name "ampere metre squared joule to the power minus 1 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (kind "attribute def") (name "ampere metre to the power minus 2 kelvin to the power minus 2") (declared-name "ampere metre to the power minus 2 kelvin to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RichardsonConstantUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere per metre"))) (kind "attribute def") (name "ampere per metre") (declared-name "ampere per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearElectricCurrentDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere per square metre"))) (kind "attribute def") (name "ampere per square metre") (declared-name "ampere per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricCurrentDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (kind "attribute def") (name "ampere second per kilogram") (declared-name "ampere second per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::arcmin"))) (kind "alias") (name "arcmin") (declared-name "arcmin") (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::arcsec"))) (kind "alias") (name "arcsec") (declared-name "arcsec") (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::astronomical unit"))) (kind "attribute def") (name "astronomical unit") (declared-name "astronomical unit") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::astronomical unit"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::atomic mass unit"))) (kind "attribute def") (name "atomic mass unit") (declared-name "atomic mass unit") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::atomic mass unit"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::barn"))) (kind "attribute def") (name "barn") (declared-name "barn") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::barn"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::baud"))) (kind "attribute def") (name "baud") (declared-name "baud") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ModulationRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel"))) (kind "attribute def") (name "becquerel") (declared-name "becquerel") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "NuclearActivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (kind "attribute def") (name "becquerel per cubic metre") (declared-name "becquerel per cubic metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (kind "attribute def") (name "becquerel per kilogram") (declared-name "becquerel per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificActivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (kind "attribute def") (name "becquerel per square metre") (declared-name "becquerel per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceActivityDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::bit"))) (kind "attribute def") (name "bit") (declared-name "bit") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::bit per second"))) (kind "attribute def") (name "bit per second") (declared-name "bit per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "BinaryDigitRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::byte"))) (kind "attribute def") (name "byte") (declared-name "byte") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::byte per second"))) (kind "attribute def") (name "byte per second") (declared-name "byte per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransferRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela"))) (kind "attribute def") (name "candela") (declared-name "candela") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (kind "attribute def") (name "candela metre to the power minus 2") (declared-name "candela metre to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian"))) (kind "attribute def") (name "candela steradian") (declared-name "candela steradian") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousFluxUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (kind "attribute def") (name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (declared-name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfRadiationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (kind "attribute def") (name "candela steradian metre to the power minus 2") (declared-name "candela steradian metre to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (kind "attribute def") (name "candela steradian metre to the power minus 2 second") (declared-name "candela steradian metre to the power minus 2 second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian second"))) (kind "attribute def") (name "candela steradian second") (declared-name "candela steradian second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::centimetre"))) (kind "attribute def") (name "centimetre") (declared-name "centimetre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::centimetre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb"))) (kind "attribute def") (name "coulomb") (declared-name "coulomb") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb metre"))) (kind "attribute def") (name "coulomb metre") (declared-name "coulomb metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricDipoleMomentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (kind "attribute def") (name "coulomb per cubic metre") (declared-name "coulomb per cubic metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (kind "attribute def") (name "coulomb per kilogram") (declared-name "coulomb per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (kind "attribute def") (name "coulomb per kilogram second") (declared-name "coulomb per kilogram second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per metre"))) (kind "attribute def") (name "coulomb per metre") (declared-name "coulomb per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearDensityOfElectricChargeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (kind "attribute def") (name "coulomb per square metre") (declared-name "coulomb per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceDensityOfElectricChargeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::dalton"))) (kind "attribute def") (name "dalton") (declared-name "dalton") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::dalton"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::day"))) (kind "attribute def") (name "day") (declared-name "day") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::day"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::decade"))) (kind "attribute def") (name "decade") (declared-name "decade") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicFrequencyRangeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::decibel"))) (kind "attribute def") (name "decibel") (declared-name "decibel") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundPressureLevelUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::degree"))) (kind "attribute def") (name "degree") (declared-name "degree") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (kind "attribute def") (name "degree celsius (absolute temperature scale)") (declared-name "degree celsius (absolute temperature scale)") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IntervalScale")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::celsiusToKelvinScaleMapping"))) (kind "attribute") (name "celsiusToKelvinScaleMapping") (declared-name "celsiusToKelvinScaleMapping") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityValueMapping")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (kind "attribute") (name "quantityValueMapping") (declared-name "quantityValueMapping") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityValueMapping")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtFreezingPointInC"))) (kind "attribute") (name "temperatureWaterAtFreezingPointInC") (declared-name "temperatureWaterAtFreezingPointInC") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtTriplePointInC"))) (kind "attribute") (name "temperatureWaterAtTriplePointInC") (declared-name "temperatureWaterAtTriplePointInC") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusInKelvin"))) (kind "attribute") (name "zeroDegreeCelsiusInKelvin") (declared-name "zeroDegreeCelsiusInKelvin") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "ThermodynamicTemperatureValue")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind "attribute") (name "zeroDegreeCelsiusToKelvinShift") (declared-name "zeroDegreeCelsiusToKelvinShift") (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFramePlacement")) (redefinition (reference "transformation")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (kind "attribute def") (name "degree celsius (temperature difference)") (declared-name "degree celsius (temperature difference)") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TemperatureDifferenceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::degree"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt"))) (kind "attribute def") (name "electronvolt") (declared-name "electronvolt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (kind "attribute def") (name "electronvolt joule kilogram metre squared second to the power minus 2") (declared-name "electronvolt joule kilogram metre squared second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HartreeEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (kind "attribute def") (name "electronvolt metre to the power minus 2 per kilogram") (declared-name "electronvolt metre to the power minus 2 per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (kind "attribute def") (name "electronvolt per metre") (declared-name "electronvolt per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalLinearStoppingPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (kind "attribute def") (name "electronvolt per square metre") (declared-name "electronvolt per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyFluenceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::electronvolt"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::erlang"))) (kind "attribute def") (name "erlang") (declared-name "erlang") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TrafficIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::farad"))) (kind "attribute def") (name "farad") (declared-name "farad") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CapacitanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::farad per metre"))) (kind "attribute def") (name "farad per metre") (declared-name "farad per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricConstantUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::g per l"))) (kind "attribute def") (name "g per l") (declared-name "g per l") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::g per mole"))) (kind "attribute def") (name "g per mole") (declared-name "g per mole") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::gigajoule"))) (kind "attribute def") (name "gigajoule") (declared-name "gigajoule") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::gigajoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::gram"))) (kind "attribute def") (name "gram") (declared-name "gram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::gray"))) (kind "attribute def") (name "gray") (declared-name "gray") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorbedDoseUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::gray per second"))) (kind "attribute def") (name "gray per second") (declared-name "gray per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorbedDoseRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::hartley"))) (kind "attribute def") (name "hartley") (declared-name "hartley") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::hartley per second"))) (kind "attribute def") (name "hartley per second") (declared-name "hartley per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::henry"))) (kind "attribute def") (name "henry") (declared-name "henry") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PermeanceUnit")) (typing (reference "InductanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::henry per metre"))) (kind "attribute def") (name "henry per metre") (declared-name "henry per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticConstantUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (kind "attribute def") (name "henry to the power minus 1") (declared-name "henry to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReluctanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "SI::henry"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "PermeanceUnit::quantityDimension")) (redefinition (reference "InductanceUnit::quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "SI::hertz"))) (kind "attribute def") (name "hertz") (declared-name "hertz") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::hour"))) (kind "attribute def") (name "hour") (declared-name "hour") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::hour"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::joule"))) (kind "attribute def") (name "joule") (declared-name "joule") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (kind "attribute def") (name "joule metre squared per kilogram") (declared-name "joule metre squared per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (kind "attribute def") (name "joule per cubic metre") (declared-name "joule per cubic metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectromagneticEnergyDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (kind "attribute def") (name "joule per cubic metre nm") (declared-name "joule per cubic metre nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kelvin"))) (kind "attribute def") (name "joule per kelvin") (declared-name "joule per kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kilogram"))) (kind "attribute def") (name "joule per kilogram") (declared-name "joule per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (kind "attribute def") (name "joule per kilogram kelvin") (declared-name "joule per kilogram kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per metre"))) (kind "attribute def") (name "joule per metre") (declared-name "joule per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalLinearStoppingPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per mole"))) (kind "attribute def") (name "joule per mole") (declared-name "joule per mole") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (kind "attribute def") (name "joule per mole kelvin") (declared-name "joule per mole kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per nm"))) (kind "attribute def") (name "joule per nm") (declared-name "joule per nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per second"))) (kind "attribute def") (name "joule per second") (declared-name "joule per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatFlowRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per square metre"))) (kind "attribute def") (name "joule per square metre") (declared-name "joule per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (kind "attribute def") (name "joule per square metre nm") (declared-name "joule per square metre nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule second"))) (kind "attribute def") (name "joule second") (declared-name "joule second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActionQuantityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (kind "attribute def") (name "joule second electronvolt second") (declared-name "joule second electronvolt second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalAngularMomentumUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (kind "attribute def") (name "joule second to the power minus 1") (declared-name "joule second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (kind "attribute def") (name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (declared-name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDensityOfStatesUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin"))) (kind "attribute def") (name "kelvin") (declared-name "kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureUnit")) (typing (reference "TemperatureDifferenceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (kind "attribute def") (name "kelvin per pascal") (declared-name "kelvin per pascal") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleThomsonCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin per watt"))) (kind "attribute def") (name "kelvin per watt") (declared-name "kelvin per watt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (kind "attribute def") (name "kelvin to the power minus 1") (declared-name "kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearExpansionCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ThermodynamicTemperatureUnit::quantityDimension")) (redefinition (reference "TemperatureDifferenceUnit::quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::temperatureOfWaterAtTriplePointInK"))) (kind "attribute") (name "temperatureOfWaterAtTriplePointInK") (declared-name "temperatureOfWaterAtTriplePointInK") (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram"))) (kind "attribute def") (name "kilogram") (declared-name "kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (kind "attribute def") (name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (declared-name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 1") (declared-name "kilogram metre second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentumUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (kind "attribute def") (name "kilogram metre second to the power minus 2") (declared-name "kilogram metre second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (kind "attribute def") (name "kilogram metre second to the power minus 3") (declared-name "kilogram metre second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantFluxUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre second to the power minus 3 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (kind "attribute def") (name "kilogram metre squared") (declared-name "kilogram metre squared") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfInertiaUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 1") (declared-name "kilogram metre squared second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMomentumUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2") (declared-name "kilogram metre squared second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3") (declared-name "kilogram metre squared second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialDifferenceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1") (declared-name "kilogram metre to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 2") (declared-name "kilogram metre to the power minus 1 second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 3") (declared-name "kilogram metre to the power minus 1 second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralIrradianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 2") (declared-name "kilogram metre to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 2 second to the power minus 1") (declared-name "kilogram metre to the power minus 2 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 2 second to the power minus 2") (declared-name "kilogram metre to the power minus 2 second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (kind "attribute def") (name "kilogram metre to the power minus 3") (declared-name "kilogram metre to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 4 second to the power minus 1") (declared-name "kilogram metre to the power minus 4 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcousticImpedanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (kind "attribute def") (name "kilogram mole to the power minus 1") (declared-name "kilogram mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 1") (declared-name "kilogram second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (kind "attribute def") (name "kilogram second to the power minus 2") (declared-name "kilogram second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 2 ampere to the power minus 1") (declared-name "kilogram second to the power minus 2 ampere to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (kind "attribute def") (name "kilogram second to the power minus 3") (declared-name "kilogram second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram second to the power minus 3 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram second to the power minus 3 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (kind "attribute def") (name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (declared-name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorenzCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (kind "attribute def") (name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (declared-name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 ampere") (declared-name "kilogram to the power minus 1 ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (kind "attribute def") (name "kilogram to the power minus 1 metre cubed") (declared-name "kilogram to the power minus 1 metre cubed") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificVolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre second to the power 2") (declared-name "kilogram to the power minus 1 metre second to the power 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 metre second to the power 2 kelvin") (declared-name "kilogram to the power minus 1 metre second to the power 2 kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleThomsonCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (kind "attribute def") (name "kilogram to the power minus 1 metre squared") (declared-name "kilogram to the power minus 1 metre squared") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassAttenuationCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectrolyticConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDensityOfStatesUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 second ampere") (declared-name "kilogram to the power minus 1 second ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 2") (declared-name "kilogram to the power minus 1 second to the power 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDistributionOfCrossSectionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 2 ampere") (declared-name "kilogram to the power minus 1 second to the power 2 ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MobilityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 second to the power 3 kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificActivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::kilogram"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::kilojoule"))) (kind "attribute def") (name "kilojoule") (declared-name "kilojoule") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::kilojoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre"))) (kind "attribute def") (name "kilometre") (declared-name "kilometre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre per hour"))) (kind "attribute def") (name "kilometre per hour") (declared-name "kilometre per hour") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::kilometre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::kilowatt"))) (kind "attribute def") (name "kilowatt") (declared-name "kilowatt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::kilowatt"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::litre"))) (kind "attribute def") (name "litre") (declared-name "litre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::litre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::lumen"))) (kind "attribute def") (name "lumen") (declared-name "lumen") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousFluxUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::lumen per square metre"))) (kind "attribute def") (name "lumen per square metre") (declared-name "lumen per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExitanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::lumen per watt"))) (kind "attribute def") (name "lumen per watt") (declared-name "lumen per watt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfRadiationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::lumen second"))) (kind "attribute def") (name "lumen second") (declared-name "lumen second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::lux"))) (kind "attribute def") (name "lux") (declared-name "lux") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::lux second"))) (kind "attribute def") (name "lux second") (declared-name "lux second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::m/s²"))) (kind "alias") (name "m/s²") (declared-name "m/s²") (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::megajoule"))) (kind "attribute def") (name "megajoule") (declared-name "megajoule") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::megajoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::metre"))) (kind "attribute def") (name "metre") (declared-name "metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed"))) (kind "attribute def") (name "metre cubed") (declared-name "metre cubed") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (kind "attribute def") (name "metre cubed mole to the power minus 1") (declared-name "metre cubed mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarVolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (kind "attribute def") (name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (declared-name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HallCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (kind "attribute def") (name "metre cubed second to the power minus 1") (declared-name "metre cubed second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre per second"))) (kind "attribute def") (name "metre per second") (declared-name "metre per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (kind "attribute def") (name "metre second to the power minus 1") (declared-name "metre second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (kind "attribute def") (name "metre second to the power minus 2") (declared-name "metre second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared"))) (kind "attribute def") (name "metre squared") (declared-name "metre squared") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared ampere"))) (kind "attribute def") (name "metre squared ampere") (declared-name "metre squared ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticDipoleMomentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (kind "attribute def") (name "metre squared kelvin per watt") (declared-name "metre squared kelvin per watt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (kind "attribute def") (name "metre squared mole to the power minus 1") (declared-name "metre squared mole to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarAbsorptionCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per joule"))) (kind "attribute def") (name "metre squared per joule") (declared-name "metre squared per joule") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDistributionOfCrossSectionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (kind "attribute def") (name "metre squared per joule steradian") (declared-name "metre squared per joule steradian") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DirectionAndEnergyDistributionOfCrossSectionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (kind "attribute def") (name "metre squared per volt second") (declared-name "metre squared per volt second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MobilityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (kind "attribute def") (name "metre squared second to the power minus 1") (declared-name "metre squared second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "KinematicViscosityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (kind "attribute def") (name "metre squared second to the power minus 2") (declared-name "metre squared second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "metre squared second to the power minus 2 kelvin to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (kind "attribute def") (name "metre squared second to the power minus 3") (declared-name "metre squared second to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (kind "attribute def") (name "metre squared steradian to the power minus 1") (declared-name "metre squared steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DirectionDistributionOfCrossSectionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power 4"))) (kind "attribute def") (name "metre to the power 4") (declared-name "metre to the power 4") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondAxialMomentOfAreaUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (kind "attribute def") (name "metre to the power 4 second to the power minus 2") (declared-name "metre to the power 4 second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 1") (declared-name "metre to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CurvatureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (kind "attribute def") (name "metre to the power minus 2") (declared-name "metre to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 2 second to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIrradianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonRadianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (kind "attribute def") (name "metre to the power minus 3") (declared-name "metre to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (kind "attribute def") (name "metre to the power minus 3 second") (declared-name "metre to the power minus 3 second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfVibrationalStatesUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 3 second to the power minus 1") (declared-name "metre to the power minus 3 second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::metric ton"))) (kind "alias") (name "metric ton") (declared-name "metric ton") (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::millilitre"))) (kind "attribute def") (name "millilitre") (declared-name "millilitre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::millilitre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::millimetre"))) (kind "attribute def") (name "millimetre") (declared-name "millimetre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::millimetre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::millinewton"))) (kind "attribute def") (name "millinewton") (declared-name "millinewton") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::millinewton"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::minute"))) (kind "attribute def") (name "minute") (declared-name "minute") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::minute (angle)"))) (kind "attribute def") (name "minute (angle)") (declared-name "minute (angle)") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::minute (angle)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::minute"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::ml per l"))) (kind "attribute def") (name "ml per l") (declared-name "ml per l") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFractionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole"))) (kind "attribute def") (name "mole") (declared-name "mole") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (kind "attribute def") (name "mole kilogram to the power minus 1") (declared-name "mole kilogram to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IonicStrengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (kind "attribute def") (name "mole metre to the power minus 3") (declared-name "mole metre to the power minus 3") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (kind "attribute def") (name "mole per cubic metre") (declared-name "mole per cubic metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole per kilogram"))) (kind "attribute def") (name "mole per kilogram") (declared-name "mole per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolalityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::mole per l"))) (kind "attribute def") (name "mole per l") (declared-name "mole per l") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::nanometre"))) (kind "attribute def") (name "nanometre") (declared-name "nanometre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::nanometre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::natural unit of information"))) (kind "attribute def") (name "natural unit of information") (declared-name "natural unit of information") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (kind "attribute def") (name "natural unit of information per second") (declared-name "natural unit of information per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton"))) (kind "attribute def") (name "newton") (declared-name "newton") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre"))) (kind "attribute def") (name "newton metre") (declared-name "newton metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit")) (typing (reference "TorqueUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre second"))) (kind "attribute def") (name "newton metre second") (declared-name "newton metre second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularImpulseUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (kind "attribute def") (name "newton metre second to the power minus 1") (declared-name "newton metre second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (kind "attribute def") (name "newton metre to the power minus 1") (declared-name "newton metre to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (kind "attribute def") (name "newton metre to the power minus 2") (declared-name "newton metre to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "SI::newton metre"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "MomentOfForceUnit::quantityDimension")) (redefinition (reference "TorqueUnit::quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "SI::newton second"))) (kind "attribute def") (name "newton second") (declared-name "newton second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ImpulseUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::octave"))) (kind "attribute def") (name "octave") (declared-name "octave") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicFrequencyRangeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::octet"))) (kind "attribute def") (name "octet") (declared-name "octet") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::octet per second"))) (kind "attribute def") (name "octet per second") (declared-name "octet per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransferRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ohm"))) (kind "attribute def") (name "ohm") (declared-name "ohm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ohm metre"))) (kind "attribute def") (name "ohm metre") (declared-name "ohm metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal"))) (kind "attribute def") (name "pascal") (declared-name "pascal") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (kind "attribute def") (name "pascal per kelvin") (declared-name "pascal per kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second"))) (kind "attribute def") (name "pascal second") (declared-name "pascal second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (kind "attribute def") (name "pascal second per cubic metre") (declared-name "pascal second per cubic metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcousticImpedanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second per metre"))) (kind "attribute def") (name "pascal second per metre") (declared-name "pascal second per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (kind "attribute def") (name "pascal to the power 2 second") (declared-name "pascal to the power 2 second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (kind "attribute def") (name "pascal to the power minus 1") (declared-name "pascal to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian"))) (kind "attribute def") (name "radian") (declared-name "radian") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (kind "attribute def") (name "radian metre squared per kilogram to the power 1") (declared-name "radian metre squared per kilogram to the power 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificOpticalRotatoryPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (kind "attribute def") (name "radian metre squared per mole") (declared-name "radian metre squared per mole") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarOpticalRotatoryPowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian per metre"))) (kind "attribute def") (name "radian per metre") (declared-name "radian per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (kind "attribute def") (name "radian second to the power minus 1") (declared-name "radian second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (kind "attribute def") (name "radian second to the power minus 2") (declared-name "radian second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second"))) (kind "attribute def") (name "second") (declared-name "second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second (angle)"))) (kind "attribute def") (name "second (angle)") (declared-name "second (angle)") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::second (angle)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::second ampere"))) (kind "attribute def") (name "second ampere") (declared-name "second ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (kind "attribute def") (name "second to the power minus 1") (declared-name "second to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (kind "attribute def") (name "second to the power minus 1 steradian to the power minus 1") (declared-name "second to the power minus 1 steradian to the power minus 1") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (kind "attribute def") (name "second to the power minus 2") (declared-name "second to the power minus 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::shannon"))) (kind "attribute def") (name "shannon") (declared-name "shannon") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::shannon per second"))) (kind "attribute def") (name "shannon per second") (declared-name "shannon per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::siemens"))) (kind "attribute def") (name "siemens") (declared-name "siemens") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ConductanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (kind "attribute def") (name "siemens metre squared per mole") (declared-name "siemens metre squared per mole") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::siemens per metre"))) (kind "attribute def") (name "siemens per metre") (declared-name "siemens per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::sievert"))) (kind "attribute def") (name "sievert") (declared-name "sievert") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::sievert per second"))) (kind "attribute def") (name "sievert per second") (declared-name "sievert per second") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::steradian"))) (kind "attribute def") (name "steradian") (declared-name "steradian") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SolidAngularMeasureUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::tesla"))) (kind "attribute def") (name "tesla") (declared-name "tesla") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::tonne"))) (kind "attribute def") (name "tonne") (declared-name "tonne") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::tonne"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::volt"))) (kind "attribute def") (name "volt") (declared-name "volt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere"))) (kind "attribute def") (name "volt ampere") (declared-name "volt ampere") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (kind "attribute def") (name "volt ampere reactive") (declared-name "volt ampere reactive") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
    (element (id (node (document "d0") (qualified-name "SI::volt per kelvin"))) (kind "attribute def") (name "volt per kelvin") (declared-name "volt per kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::volt per metre"))) (kind "attribute def") (name "volt per metre") (declared-name "volt per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricFieldStrengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (kind "attribute def") (name "volt to the power 2 per kelvin to the power 2") (declared-name "volt to the power 2 per kelvin to the power 2") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorenzCoefficientUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt"))) (kind "attribute def") (name "watt") (declared-name "watt") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt hour"))) (kind "attribute def") (name "watt hour") (declared-name "watt hour") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per kelvin"))) (kind "attribute def") (name "watt per kelvin") (declared-name "watt per kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductanceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per kilogram"))) (kind "attribute def") (name "watt per kilogram") (declared-name "watt per kilogram") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (kind "attribute def") (name "watt per metre kelvin") (declared-name "watt per metre kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per nm"))) (kind "attribute def") (name "watt per nm") (declared-name "watt per nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantFluxUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre"))) (kind "attribute def") (name "watt per square metre") (declared-name "watt per square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (kind "attribute def") (name "watt per square metre kelvin") (declared-name "watt per square metre kelvin") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (kind "attribute def") (name "watt per square metre nm") (declared-name "watt per square metre nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralIrradianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian"))) (kind "attribute def") (name "watt per steradian") (declared-name "watt per steradian") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (kind "attribute def") (name "watt per steradian nm") (declared-name "watt per steradian nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantIntensityUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (kind "attribute def") (name "watt per steradian square metre") (declared-name "watt per steradian square metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (kind "attribute def") (name "watt per steradian square metre nm") (declared-name "watt per steradian square metre nm") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadianceUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::weber"))) (kind "attribute def") (name "weber") (declared-name "weber") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::weber metre"))) (kind "attribute def") (name "weber metre") (declared-name "weber metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticDipoleMomentUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::weber per metre"))) (kind "attribute def") (name "weber per metre") (declared-name "weber per metre") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticVectorPotentialUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ångström"))) (kind "attribute def") (name "ångström") (declared-name "ångström") (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit")))))
    (element (id (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "SI::ångström"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention")) (redefinition (reference "unitConversion")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SI::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SIPrefixes::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (kind featureTyping) (ordinal 0)) (authored-target "SystemOfUnits") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (kind redefinition) (ordinal 0)) (authored-target "baseUnits") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (kind redefinition) (ordinal 0)) (authored-target "systemOfQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticMomentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "RichardsonConstantUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearElectricCurrentDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::barn::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::baud"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulationRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel"))) (kind featureTyping) (ordinal 0)) (authored-target "NuclearActivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificActivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceActivityDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::bit"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::bit per second"))) (kind featureTyping) (ordinal 0)) (authored-target "BinaryDigitRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::byte"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::byte per second"))) (kind featureTyping) (ordinal 0)) (authored-target "TransferRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::centimetre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricDipoleMomentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearDensityOfElectricChargeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceDensityOfElectricChargeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::dalton::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::day"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::day::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::decade"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::decibel"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPressureLevelUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (kind featureTyping) (ordinal 0)) (authored-target "IntervalScale") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::celsiusToKelvinScaleMapping"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityValueMapping") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (kind redefinition) (ordinal 0)) (authored-target "quantityValueMapping") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtFreezingPointInC"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtTriplePointInC"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusInKelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFramePlacement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "HartreeEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalLinearStoppingPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyFluenceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::electronvolt::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::erlang"))) (kind featureTyping) (ordinal 0)) (authored-target "TrafficIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::farad"))) (kind featureTyping) (ordinal 0)) (authored-target "CapacitanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::farad per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricConstantUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::g per l"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::g per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::gigajoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::gram"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gray"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorbedDoseUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gray per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorbedDoseRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hartley"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hartley per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry"))) (kind featureTyping) (ordinal 0)) (authored-target "PermeanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry"))) (kind featureTyping) (ordinal 1)) (authored-target "InductanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticConstantUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ReluctanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "PermeanceUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "InductanceUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hertz"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::hour::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectromagneticEnergyDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalLinearStoppingPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per second"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatFlowRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second"))) (kind featureTyping) (ordinal 0)) (authored-target "ActionQuantityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalAngularMomentumUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureDifferenceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearExpansionCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "TemperatureDifferenceUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::temperatureOfWaterAtTriplePointInK"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentumUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMomentumUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialDifferenceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificVolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAttenuationCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDistributionOfCrossSectionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "MobilityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificActivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilogram::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilojoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilometre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilowatt::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::litre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExitanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lux"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lux second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::megajoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "HallCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre per second"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticDipoleMomentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarAbsorptionCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per joule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDistributionOfCrossSectionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "DirectionAndEnergyDistributionOfCrossSectionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (kind featureTyping) (ordinal 0)) (authored-target "MobilityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "DirectionDistributionOfCrossSectionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power 4"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIrradianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonRadianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfVibrationalStatesUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millilitre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millimetre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millinewton::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::minute::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ml per l"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per l"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::nanometre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::natural unit of information"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre"))) (kind featureTyping) (ordinal 1)) (authored-target "TorqueUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre second"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularImpulseUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "MomentOfForceUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "TorqueUnit::quantityDimension") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton second"))) (kind featureTyping) (ordinal 0)) (authored-target "ImpulseUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octave"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octet"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octet per second"))) (kind featureTyping) (ordinal 0)) (authored-target "TransferRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ohm"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ohm metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::second (angle)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::second ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::shannon"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::shannon per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens"))) (kind featureTyping) (ordinal 0)) (authored-target "ConductanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::sievert"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::sievert per second"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tesla"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::tonne::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricFieldStrengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt hour"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticDipoleMomentUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticVectorPotentialUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ångström::unitConversion")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (target (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (target (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (target (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (target (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (target (node (document "d0") (qualified-name "SI::day::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (target (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (target (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (target (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (target (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (target (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (target (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (target (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (target (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (target (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (target (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (target (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (target (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (target (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (target (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (target (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (target (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (target (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (target (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "SI::ampere metre squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ampere per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ampere per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ampere second per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::baud")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::becquerel")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::becquerel per cubic metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::becquerel per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::becquerel per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::bit")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::bit per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::byte")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::byte per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::candela metre to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::candela steradian")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::candela steradian second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb per cubic metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb per kilogram second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::coulomb per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::decade")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::decibel")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::electronvolt per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::electronvolt per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::erlang")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::farad")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::farad per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::g per l")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::g per mole")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::gray")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::gray per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::hartley")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::hartley per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::henry")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::henry per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::henry to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::hertz")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule metre squared per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per cubic metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per cubic metre nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per kilogram kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per mole")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per mole kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule per square metre nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule second electronvolt second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kelvin per pascal")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kelvin per watt")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::kilometre per hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lumen")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lumen per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lumen per watt")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lumen second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lux")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::lux second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre cubed")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared kelvin per watt")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared per joule")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared per joule steradian")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared per volt second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power 4")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 3 second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ml per l")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::mole metre to the power minus 3")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::mole per cubic metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::mole per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::mole per l")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::natural unit of information")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::natural unit of information per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::newton")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton metre second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton metre to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton metre to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::newton second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::octave")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::octet")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::octet per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::ohm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::ohm metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal per kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal second per cubic metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal second per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal to the power 2 second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::pascal to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian metre squared per mole")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::radian second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::second ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::second to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::second to the power minus 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::shannon")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::shannon per second")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "SI::siemens")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::siemens metre squared per mole")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::siemens per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::sievert")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::sievert per second")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::steradian")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::tesla")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::volt")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::volt ampere")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::volt per kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::volt per metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt hour")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per kilogram")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per metre kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per square metre kelvin")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per square metre nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per steradian")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per steradian nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per steradian square metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::watt per steradian square metre nm")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::weber")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::weber metre")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "SI::weber per metre")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 10 18) (end 10 21)) (probe (position 10 18))
      (reference
        (source (document "d0") (qualified-name "SI::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 10 18) (end 10 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 357 22) (end 357 26)) (probe (position 357 22))
      (reference
        (source (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))
        (kind redefinition) (ordinal 0) (authored-target "unit")
        (range (start 357 22) (end 357 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit") (range (start 357 8) (end 357 35)))
        )
      )
    )
    (query (range (start 11 18) (end 11 28)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "SI::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SIPrefixes::*")
        (range (start 11 18) (end 11 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 2) (end 44 15)) (probe (position 44 2))
      (reference
        (source (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))
        (kind redefinition) (ordinal 0) (authored-target "baseUnits")
        (range (start 44 2) (end 44 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits") (range (start 44 2) (end 44 44)))
        )
      )
    )
    (query (range (start 345 22) (end 345 36)) (probe (position 345 22))
      (reference
        (source (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 345 22) (end 345 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion") (range (start 345 8) (end 345 113)))
        )
      )
    )
    (query (range (start 373 80) (end 373 94)) (probe (position 373 80))
      (reference
        (source (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))
        (kind redefinition) (ordinal 0) (authored-target "transformation")
        (range (start 373 80) (end 373 94))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 41) (end 22 59)) (probe (position 22 41))
      (reference
        (source (document "d0") (qualified-name "SI::kilogram::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 22 41) (end 22 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::kilogram::unitConversion") (range (start 22 41) (end 22 125)))
        )
      )
    )
    (query (range (start 90 49) (end 90 67)) (probe (position 90 49))
      (reference
        (source (document "d0") (qualified-name "SI::ångström::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 90 49) (end 90 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::ångström::unitConversion") (range (start 90 49) (end 90 150)))
        )
      )
    )
    (query (range (start 91 36) (end 91 54)) (probe (position 91 36))
      (reference
        (source (document "d0") (qualified-name "SI::barn::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 91 36) (end 91 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::barn::unitConversion") (range (start 91 36) (end 91 141)))
        )
      )
    )
    (query (range (start 92 38) (end 92 56)) (probe (position 92 38))
      (reference
        (source (document "d0") (qualified-name "SI::day::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 92 38) (end 92 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::day::unitConversion") (range (start 92 38) (end 92 137)))
        )
      )
    )
    (query (range (start 93 39) (end 93 57)) (probe (position 93 39))
      (reference
        (source (document "d0") (qualified-name "SI::dalton::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 93 39) (end 93 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::dalton::unitConversion") (range (start 93 39) (end 93 172)))
        )
      )
    )
    (query (range (start 94 47) (end 94 65)) (probe (position 94 47))
      (reference
        (source (document "d0") (qualified-name "SI::electronvolt::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 94 47) (end 94 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::electronvolt::unitConversion") (range (start 94 47) (end 94 177)))
        )
      )
    )
    (query (range (start 95 39) (end 95 57)) (probe (position 95 39))
      (reference
        (source (document "d0") (qualified-name "SI::hour::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 95 39) (end 95 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::hour::unitConversion") (range (start 95 39) (end 95 137)))
        )
      )
    )
    (query (range (start 96 44) (end 96 62)) (probe (position 96 44))
      (reference
        (source (document "d0") (qualified-name "SI::minute::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 96 44) (end 96 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::minute::unitConversion") (range (start 96 44) (end 96 140)))
        )
      )
    )
    (query (range (start 97 39) (end 97 57)) (probe (position 97 39))
      (reference
        (source (document "d0") (qualified-name "SI::litre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 97 39) (end 97 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::litre::unitConversion") (range (start 97 39) (end 97 143)))
        )
      )
    )
    (query (range (start 98 33) (end 98 51)) (probe (position 98 33))
      (reference
        (source (document "d0") (qualified-name "SI::tonne::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 98 33) (end 98 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::tonne::unitConversion") (range (start 98 33) (end 98 134)))
        )
      )
    )
    (query (range (start 100 50) (end 100 68)) (probe (position 100 50))
      (reference
        (source (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 100 50) (end 100 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::atomic mass unit::unitConversion") (range (start 100 50) (end 100 148)))
        )
      )
    )
    (query (range (start 101 54) (end 101 72)) (probe (position 101 54))
      (reference
        (source (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 101 54) (end 101 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::astronomical unit::unitConversion") (range (start 101 54) (end 101 184)))
        )
      )
    )
    (query (range (start 102 57) (end 102 75)) (probe (position 102 57))
      (reference
        (source (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 102 57) (end 102 75))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion") (range (start 102 57) (end 102 156)))
        )
      )
    )
    (query (range (start 103 51) (end 103 69)) (probe (position 103 51))
      (reference
        (source (document "d0") (qualified-name "SI::degree::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 103 51) (end 103 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::degree::unitConversion") (range (start 103 51) (end 103 180)))
        )
      )
    )
    (query (range (start 104 62) (end 104 80)) (probe (position 104 62))
      (reference
        (source (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 104 62) (end 104 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::minute (angle)::unitConversion") (range (start 104 62) (end 104 191)))
        )
      )
    )
    (query (range (start 106 62) (end 106 80)) (probe (position 106 62))
      (reference
        (source (document "d0") (qualified-name "SI::second (angle)::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 106 62) (end 106 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::second (angle)::unitConversion") (range (start 106 62) (end 106 191)))
        )
      )
    )
    (query (range (start 313 44) (end 313 62)) (probe (position 313 44))
      (reference
        (source (document "d0") (qualified-name "SI::nanometre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 313 44) (end 313 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::nanometre::unitConversion") (range (start 313 44) (end 313 128)))
        )
      )
    )
    (query (range (start 314 45) (end 314 63)) (probe (position 314 45))
      (reference
        (source (document "d0") (qualified-name "SI::millimetre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 314 45) (end 314 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::millimetre::unitConversion") (range (start 314 45) (end 314 130)))
        )
      )
    )
    (query (range (start 315 45) (end 315 63)) (probe (position 315 45))
      (reference
        (source (document "d0") (qualified-name "SI::centimetre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 315 45) (end 315 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::centimetre::unitConversion") (range (start 315 45) (end 315 130)))
        )
      )
    )
    (query (range (start 316 44) (end 316 62)) (probe (position 316 44))
      (reference
        (source (document "d0") (qualified-name "SI::kilometre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 316 44) (end 316 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::kilometre::unitConversion") (range (start 316 44) (end 316 128)))
        )
      )
    )
    (query (range (start 319 45) (end 319 63)) (probe (position 319 45))
      (reference
        (source (document "d0") (qualified-name "SI::millilitre::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 319 45) (end 319 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::millilitre::unitConversion") (range (start 319 45) (end 319 130)))
        )
      )
    )
    (query (range (start 322 45) (end 322 63)) (probe (position 322 45))
      (reference
        (source (document "d0") (qualified-name "SI::millinewton::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 322 45) (end 322 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::millinewton::unitConversion") (range (start 322 45) (end 322 130)))
        )
      )
    )
    (query (range (start 325 44) (end 325 62)) (probe (position 325 44))
      (reference
        (source (document "d0") (qualified-name "SI::kilojoule::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 325 44) (end 325 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::kilojoule::unitConversion") (range (start 325 44) (end 325 128)))
        )
      )
    )
    (query (range (start 326 44) (end 326 62)) (probe (position 326 44))
      (reference
        (source (document "d0") (qualified-name "SI::megajoule::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 326 44) (end 326 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::megajoule::unitConversion") (range (start 326 44) (end 326 128)))
        )
      )
    )
    (query (range (start 327 44) (end 327 62)) (probe (position 327 44))
      (reference
        (source (document "d0") (qualified-name "SI::gigajoule::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 327 44) (end 327 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::gigajoule::unitConversion") (range (start 327 44) (end 327 128)))
        )
      )
    )
    (query (range (start 330 42) (end 330 60)) (probe (position 330 42))
      (reference
        (source (document "d0") (qualified-name "SI::kilowatt::unitConversion"))
        (kind redefinition) (ordinal 0) (authored-target "unitConversion")
        (range (start 330 42) (end 330 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::kilowatt::unitConversion") (range (start 330 42) (end 330 126)))
        )
      )
    )
    (query (range (start 369 22) (end 369 42)) (probe (position 369 22))
      (reference
        (source (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))
        (kind redefinition) (ordinal 0) (authored-target "quantityValueMapping")
        (range (start 369 22) (end 369 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping") (range (start 369 8) (end 369 73)))
        )
      )
    )
    (query (range (start 9 19) (end 9 40)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "SI::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 9 19) (end 9 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 2) (end 43 24)) (probe (position 43 2))
      (reference
        (source (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))
        (kind redefinition) (ordinal 0) (authored-target "systemOfQuantities")
        (range (start 43 2) (end 43 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities") (range (start 43 2) (end 43 31)))
        )
      )
    )
    (query (range (start 30 22) (end 30 48)) (probe (position 30 22))
      (reference
        (source (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalQuantityValues")
        (range (start 30 22) (end 30 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues") (range (start 30 8) (end 30 86)))
        )
      )
    )
    (query (range (start 368 22) (end 368 48)) (probe (position 368 22))
      (reference
        (source (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))
        (kind redefinition) (ordinal 0) (authored-target "definitionalQuantityValues")
        (range (start 368 22) (end 368 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues") (range (start 368 8) (end 368 122)))
        )
      )
    )
    (query (range (start 256 60) (end 256 89)) (probe (position 256 60))
      (reference
        (source (document "d0") (qualified-name "SI::newton metre::quantityDimension"))
        (kind redefinition) (ordinal 1) (authored-target "TorqueUnit::quantityDimension")
        (range (start 256 60) (end 256 89))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 22) (end 61 54)) (probe (position 61 22))
      (reference
        (source (document "d0") (qualified-name "SI::henry::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "PermeanceUnit::quantityDimension")
        (range (start 61 22) (end 61 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 56) (end 61 89)) (probe (position 61 56))
      (reference
        (source (document "d0") (qualified-name "SI::henry::quantityDimension"))
        (kind redefinition) (ordinal 1) (authored-target "InductanceUnit::quantityDimension")
        (range (start 61 56) (end 61 89))
        (outcome (status unresolved))
      )
    )
    (query (range (start 256 22) (end 256 58)) (probe (position 256 22))
      (reference
        (source (document "d0") (qualified-name "SI::newton metre::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "MomentOfForceUnit::quantityDimension")
        (range (start 256 22) (end 256 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 71) (end 31 115)) (probe (position 31 71))
      (reference
        (source (document "d0") (qualified-name "SI::kelvin::quantityDimension"))
        (kind redefinition) (ordinal 1) (authored-target "TemperatureDifferenceUnit::quantityDimension")
        (range (start 31 71) (end 31 115))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 22) (end 31 69)) (probe (position 31 22))
      (reference
        (source (document "d0") (qualified-name "SI::kelvin::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "ThermodynamicTemperatureUnit::quantityDimension")
        (range (start 31 22) (end 31 69))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
