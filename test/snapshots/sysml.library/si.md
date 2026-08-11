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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "952d591133b74150c878a2f0b1627331ea8ec95a2f708d0978fc2304abcef40f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SI"))) (kind "package") (name "SI") (declared-name "SI") (range (start (line 0) (character 0)) (end (line 0) (character 30923))))
    (element (id (node (document "d0") (qualified-name "SI::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 4)) (end (line 9) (character 44))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 40))))))
    (element (id (node (document "d0") (qualified-name "SI::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 4)) (end (line 10) (character 25))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "public") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 18)) (end (line 10) (character 21))))))
    (element (id (node (document "d0") (qualified-name "SI::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 11) (character 4)) (end (line 11) (character 32))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Import) (visibility "public") (import (reference "SIPrefixes::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 11) (character 18)) (end (line 11) (character 28))))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (kind "attribute def") (name "ISO/IEC 80000 International System of Units") (declared-name "ISO/IEC 80000 International System of Units") (range (start (line 42) (character 1)) (end (line 42) (character 159))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SystemOfUnits") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (kind "attribute") (name "baseUnits") (declared-name "baseUnits") (range (start (line 44) (character 2)) (end (line 44) (character 44))) (parent (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseUnits") (range (start (line 44) (character 2)) (end (line 44) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (kind "attribute") (name "systemOfQuantities") (declared-name "systemOfQuantities") (range (start (line 43) (character 2)) (end (line 43) (character 31))) (parent (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "systemOfQuantities") (range (start (line 43) (character 2)) (end (line 43) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "SI::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 30923))) (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::ampere"))) (kind "attribute def") (name "ampere") (declared-name "ampere") (range (start (line 24) (character 4)) (end (line 24) (character 47))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricCurrentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre squared"))) (kind "attribute def") (name "ampere metre squared") (declared-name "ampere metre squared") (range (start (line 113) (character 4)) (end (line 113) (character 78))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticMomentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "ampere metre squared joule to the power minus 1 second to the power minus 1") (declared-name "ampere metre squared joule to the power minus 1 second to the power minus 1") (range (start (line 114) (character 4)) (end (line 114) (character 164))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (kind "attribute def") (name "ampere metre to the power minus 2 kelvin to the power minus 2") (declared-name "ampere metre to the power minus 2 kelvin to the power minus 2") (range (start (line 112) (character 4)) (end (line 112) (character 141))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RichardsonConstantUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere per metre"))) (kind "attribute def") (name "ampere per metre") (declared-name "ampere per metre") (range (start (line 116) (character 4)) (end (line 116) (character 82))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearElectricCurrentDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere per square metre"))) (kind "attribute def") (name "ampere per square metre") (declared-name "ampere per square metre") (range (start (line 117) (character 4)) (end (line 117) (character 87))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricCurrentDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (kind "attribute def") (name "ampere second per kilogram") (declared-name "ampere second per kilogram") (range (start (line 115) (character 4)) (end (line 115) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::arcmin"))) (kind "alias") (name "arcmin") (declared-name "arcmin") (range (start (line 105) (character 4)) (end (line 105) (character 27))) (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::arcsec"))) (kind "alias") (name "arcsec") (declared-name "arcsec") (range (start (line 107) (character 4)) (end (line 107) (character 27))) (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::astronomical unit"))) (kind "attribute def") (name "astronomical unit") (declared-name "astronomical unit") (range (start (line 101) (character 4)) (end (line 101) (character 186))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 101) (character 54)) (end (line 101) (character 184))) (parent (node (document "d0") (qualified-name "SI::astronomical unit"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 101) (character 54)) (end (line 101) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SI::atomic mass unit"))) (kind "attribute def") (name "atomic mass unit") (declared-name "atomic mass unit") (range (start (line 100) (character 4)) (end (line 100) (character 150))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 100) (character 50)) (end (line 100) (character 148))) (parent (node (document "d0") (qualified-name "SI::atomic mass unit"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 100) (character 50)) (end (line 100) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SI::barn"))) (kind "attribute def") (name "barn") (declared-name "barn") (range (start (line 91) (character 4)) (end (line 91) (character 143))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 91) (character 36)) (end (line 91) (character 141))) (parent (node (document "d0") (qualified-name "SI::barn"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 91) (character 36)) (end (line 91) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "SI::baud"))) (kind "attribute def") (name "baud") (declared-name "baud") (range (start (line 51) (character 4)) (end (line 51) (character 52))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ModulationRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel"))) (kind "attribute def") (name "becquerel") (declared-name "becquerel") (range (start (line 53) (character 4)) (end (line 53) (character 58))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "NuclearActivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (kind "attribute def") (name "becquerel per cubic metre") (declared-name "becquerel per cubic metre") (range (start (line 122) (character 4)) (end (line 122) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (kind "attribute def") (name "becquerel per kilogram") (declared-name "becquerel per kilogram") (range (start (line 120) (character 4)) (end (line 120) (character 80))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificActivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (kind "attribute def") (name "becquerel per square metre") (declared-name "becquerel per square metre") (range (start (line 121) (character 4)) (end (line 121) (character 92))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceActivityDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::bit"))) (kind "attribute def") (name "bit") (declared-name "bit") (range (start (line 52) (character 4)) (end (line 52) (character 52))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::bit per second"))) (kind "attribute def") (name "bit per second") (declared-name "bit per second") (range (start (line 119) (character 4)) (end (line 119) (character 71))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "BinaryDigitRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::byte"))) (kind "attribute def") (name "byte") (declared-name "byte") (range (start (line 50) (character 4)) (end (line 50) (character 51))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::byte per second"))) (kind "attribute def") (name "byte per second") (declared-name "byte per second") (range (start (line 118) (character 4)) (end (line 118) (character 65))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransferRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela"))) (kind "attribute def") (name "candela") (declared-name "candela") (range (start (line 36) (character 4)) (end (line 36) (character 51))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (kind "attribute def") (name "candela metre to the power minus 2") (declared-name "candela metre to the power minus 2") (range (start (line 129) (character 4)) (end (line 129) (character 93))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian"))) (kind "attribute def") (name "candela steradian") (declared-name "candela steradian") (range (start (line 130) (character 4)) (end (line 130) (character 73))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (kind "attribute def") (name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (declared-name "candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3") (range (start (line 131) (character 4)) (end (line 131) (character 207))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfRadiationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (kind "attribute def") (name "candela steradian metre to the power minus 2") (declared-name "candela steradian metre to the power minus 2") (range (start (line 132) (character 4)) (end (line 132) (character 113))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (kind "attribute def") (name "candela steradian metre to the power minus 2 second") (declared-name "candela steradian metre to the power minus 2 second") (range (start (line 133) (character 4)) (end (line 133) (character 131))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::candela steradian second"))) (kind "attribute def") (name "candela steradian second") (declared-name "candela steradian second") (range (start (line 134) (character 4)) (end (line 134) (character 88))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::centimetre"))) (kind "attribute def") (name "centimetre") (declared-name "centimetre") (range (start (line 315) (character 4)) (end (line 315) (character 132))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 315) (character 45)) (end (line 315) (character 130))) (parent (node (document "d0") (qualified-name "SI::centimetre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 315) (character 45)) (end (line 315) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb"))) (kind "attribute def") (name "coulomb") (declared-name "coulomb") (range (start (line 54) (character 4)) (end (line 54) (character 53))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb metre"))) (kind "attribute def") (name "coulomb metre") (declared-name "coulomb metre") (range (start (line 123) (character 4)) (end (line 123) (character 73))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricDipoleMomentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (kind "attribute def") (name "coulomb per cubic metre") (declared-name "coulomb per cubic metre") (range (start (line 128) (character 4)) (end (line 128) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (kind "attribute def") (name "coulomb per kilogram") (declared-name "coulomb per kilogram") (range (start (line 125) (character 4)) (end (line 125) (character 68))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (kind "attribute def") (name "coulomb per kilogram second") (declared-name "coulomb per kilogram second") (range (start (line 124) (character 4)) (end (line 124) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per metre"))) (kind "attribute def") (name "coulomb per metre") (declared-name "coulomb per metre") (range (start (line 126) (character 4)) (end (line 126) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearDensityOfElectricChargeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (kind "attribute def") (name "coulomb per square metre") (declared-name "coulomb per square metre") (range (start (line 127) (character 4)) (end (line 127) (character 96))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceDensityOfElectricChargeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::dalton"))) (kind "attribute def") (name "dalton") (declared-name "dalton") (range (start (line 93) (character 4)) (end (line 93) (character 174))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 93) (character 39)) (end (line 93) (character 172))) (parent (node (document "d0") (qualified-name "SI::dalton"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 93) (character 39)) (end (line 93) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "SI::day"))) (kind "attribute def") (name "day") (declared-name "day") (range (start (line 92) (character 4)) (end (line 92) (character 139))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 92) (character 38)) (end (line 92) (character 137))) (parent (node (document "d0") (qualified-name "SI::day"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 92) (character 38)) (end (line 92) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "SI::decade"))) (kind "attribute def") (name "decade") (declared-name "decade") (range (start (line 56) (character 4)) (end (line 56) (character 65))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicFrequencyRangeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::decibel"))) (kind "attribute def") (name "decibel") (declared-name "decibel") (range (start (line 55) (character 4)) (end (line 55) (character 58))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundPressureLevelUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree"))) (kind "attribute def") (name "degree") (declared-name "degree") (range (start (line 103) (character 4)) (end (line 103) (character 182))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (kind "attribute def") (name "degree celsius (absolute temperature scale)") (declared-name "degree celsius (absolute temperature scale)") (range (start (line 348) (character 4)) (end (line 348) (character 1647))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IntervalScale") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::_documentation"))) (kind "documentation") (name "") (range (start (line 348) (character 4)) (end (line 348) (character 1647))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::celsiusToKelvinScaleMapping"))) (kind "attribute") (name "celsiusToKelvinScaleMapping") (declared-name "celsiusToKelvinScaleMapping") (range (start (line 364) (character 8)) (end (line 364) (character 239))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityValueMapping") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (range (start (line 368) (character 8)) (end (line 368) (character 122))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues") (range (start (line 368) (character 22)) (end (line 368) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (kind "attribute") (name "quantityValueMapping") (declared-name "quantityValueMapping") (range (start (line 369) (character 8)) (end (line 369) (character 73))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityValueMapping") (range (start (line 369) (character 22)) (end (line 369) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtFreezingPointInC"))) (kind "attribute") (name "temperatureWaterAtFreezingPointInC") (declared-name "temperatureWaterAtFreezingPointInC") (range (start (line 358) (character 8)) (end (line 358) (character 198))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtTriplePointInC"))) (kind "attribute") (name "temperatureWaterAtTriplePointInC") (declared-name "temperatureWaterAtTriplePointInC") (range (start (line 361) (character 8)) (end (line 361) (character 210))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "DefinitionalQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (range (start (line 357) (character 8)) (end (line 357) (character 35))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unit") (range (start (line 357) (character 22)) (end (line 357) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusInKelvin"))) (kind "attribute") (name "zeroDegreeCelsiusInKelvin") (declared-name "zeroDegreeCelsiusInKelvin") (range (start (line 372) (character 8)) (end (line 372) (character 96))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "ThermodynamicTemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind "attribute") (name "zeroDegreeCelsiusToKelvinShift") (declared-name "zeroDegreeCelsiusToKelvinShift") (range (start (line 373) (character 8)) (end (line 373) (character 172))) (parent (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateFramePlacement") (range none)) (redefinition (reference "transformation") (range (start (line 373) (character 80)) (end (line 373) (character 94)))))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (kind "attribute def") (name "degree celsius (temperature difference)") (declared-name "degree celsius (temperature difference)") (range (start (line 339) (character 4)) (end (line 339) (character 340))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TemperatureDifferenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::_documentation"))) (kind "documentation") (name "") (range (start (line 339) (character 4)) (end (line 339) (character 340))) (parent (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))))
    (element (id (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 345) (character 8)) (end (line 345) (character 113))) (parent (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 345) (character 22)) (end (line 345) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 103) (character 51)) (end (line 103) (character 180))) (parent (node (document "d0") (qualified-name "SI::degree"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 103) (character 51)) (end (line 103) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt"))) (kind "attribute def") (name "electronvolt") (declared-name "electronvolt") (range (start (line 94) (character 4)) (end (line 94) (character 179))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (kind "attribute def") (name "electronvolt joule kilogram metre squared second to the power minus 2") (declared-name "electronvolt joule kilogram metre squared second to the power minus 2") (range (start (line 135) (character 4)) (end (line 135) (character 156))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HartreeEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (kind "attribute def") (name "electronvolt metre to the power minus 2 per kilogram") (declared-name "electronvolt metre to the power minus 2 per kilogram") (range (start (line 136) (character 4)) (end (line 136) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (kind "attribute def") (name "electronvolt per metre") (declared-name "electronvolt per metre") (range (start (line 137) (character 4)) (end (line 137) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalLinearStoppingPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (kind "attribute def") (name "electronvolt per square metre") (declared-name "electronvolt per square metre") (range (start (line 138) (character 4)) (end (line 138) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyFluenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 94) (character 47)) (end (line 94) (character 177))) (parent (node (document "d0") (qualified-name "SI::electronvolt"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 94) (character 47)) (end (line 94) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SI::erlang"))) (kind "attribute def") (name "erlang") (declared-name "erlang") (range (start (line 57) (character 4)) (end (line 57) (character 54))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TrafficIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::farad"))) (kind "attribute def") (name "farad") (declared-name "farad") (range (start (line 58) (character 4)) (end (line 58) (character 48))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CapacitanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::farad per metre"))) (kind "attribute def") (name "farad per metre") (declared-name "farad per metre") (range (start (line 139) (character 4)) (end (line 139) (character 69))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricConstantUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::g per l"))) (kind "attribute def") (name "g per l") (declared-name "g per l") (range (start (line 140) (character 4)) (end (line 140) (character 62))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassConcentrationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::g per mole"))) (kind "attribute def") (name "g per mole") (declared-name "g per mole") (range (start (line 141) (character 4)) (end (line 141) (character 61))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::gigajoule"))) (kind "attribute def") (name "gigajoule") (declared-name "gigajoule") (range (start (line 327) (character 4)) (end (line 327) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 327) (character 44)) (end (line 327) (character 128))) (parent (node (document "d0") (qualified-name "SI::gigajoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 327) (character 44)) (end (line 327) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::gram"))) (kind "attribute def") (name "gram") (declared-name "gram") (range (start (line 16) (character 4)) (end (line 16) (character 34))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::gray"))) (kind "attribute def") (name "gray") (declared-name "gray") (range (start (line 59) (character 4)) (end (line 59) (character 50))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorbedDoseUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::gray per second"))) (kind "attribute def") (name "gray per second") (declared-name "gray per second") (range (start (line 142) (character 4)) (end (line 142) (character 71))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AbsorbedDoseRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::hartley"))) (kind "attribute def") (name "hartley") (declared-name "hartley") (range (start (line 65) (character 4)) (end (line 65) (character 60))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::hartley per second"))) (kind "attribute def") (name "hartley per second") (declared-name "hartley per second") (range (start (line 145) (character 4)) (end (line 145) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::henry"))) (kind "attribute def") (name "henry") (declared-name "henry") (range (start (line 60) (character 4)) (end (line 60) (character 301))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PermeanceUnit") (range none)) (typing (reference "InductanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::henry per metre"))) (kind "attribute def") (name "henry per metre") (declared-name "henry per metre") (range (start (line 143) (character 4)) (end (line 143) (character 69))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticConstantUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (kind "attribute def") (name "henry to the power minus 1") (declared-name "henry to the power minus 1") (range (start (line 144) (character 4)) (end (line 144) (character 78))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ReluctanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 61) (character 8)) (end (line 61) (character 230))) (parent (node (document "d0") (qualified-name "SI::henry"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "PermeanceUnit::quantityDimension") (range (start (line 61) (character 22)) (end (line 61) (character 54)))) (redefinition (reference "InductanceUnit::quantityDimension") (range (start (line 61) (character 56)) (end (line 61) (character 89)))))))
    (element (id (node (document "d0") (qualified-name "SI::hertz"))) (kind "attribute def") (name "hertz") (declared-name "hertz") (range (start (line 66) (character 4)) (end (line 66) (character 48))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "FrequencyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::hour"))) (kind "attribute def") (name "hour") (declared-name "hour") (range (start (line 95) (character 4)) (end (line 95) (character 139))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 95) (character 39)) (end (line 95) (character 137))) (parent (node (document "d0") (qualified-name "SI::hour"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 95) (character 39)) (end (line 95) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "SI::joule"))) (kind "attribute def") (name "joule") (declared-name "joule") (range (start (line 67) (character 4)) (end (line 67) (character 43))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (kind "attribute def") (name "joule metre squared per kilogram") (declared-name "joule metre squared per kilogram") (range (start (line 146) (character 4)) (end (line 146) (character 104))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (kind "attribute def") (name "joule per cubic metre") (declared-name "joule per cubic metre") (range (start (line 158) (character 4)) (end (line 158) (character 91))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectromagneticEnergyDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (kind "attribute def") (name "joule per cubic metre nm") (declared-name "joule per cubic metre nm") (range (start (line 152) (character 4)) (end (line 152) (character 125))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kelvin"))) (kind "attribute def") (name "joule per kelvin") (declared-name "joule per kelvin") (range (start (line 154) (character 4)) (end (line 154) (character 66))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kilogram"))) (kind "attribute def") (name "joule per kilogram") (declared-name "joule per kilogram") (range (start (line 155) (character 4)) (end (line 155) (character 72))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (kind "attribute def") (name "joule per kilogram kelvin") (declared-name "joule per kilogram kelvin") (range (start (line 150) (character 4)) (end (line 150) (character 95))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per metre"))) (kind "attribute def") (name "joule per metre") (declared-name "joule per metre") (range (start (line 156) (character 4)) (end (line 156) (character 77))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalLinearStoppingPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per mole"))) (kind "attribute def") (name "joule per mole") (declared-name "joule per mole") (range (start (line 159) (character 4)) (end (line 159) (character 75))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (kind "attribute def") (name "joule per mole kelvin") (declared-name "joule per mole kelvin") (range (start (line 153) (character 4)) (end (line 153) (character 90))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per nm"))) (kind "attribute def") (name "joule per nm") (declared-name "joule per nm") (range (start (line 160) (character 4)) (end (line 160) (character 73))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per second"))) (kind "attribute def") (name "joule per second") (declared-name "joule per second") (range (start (line 161) (character 4)) (end (line 161) (character 66))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per square metre"))) (kind "attribute def") (name "joule per square metre") (declared-name "joule per square metre") (range (start (line 157) (character 4)) (end (line 157) (character 111))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (kind "attribute def") (name "joule per square metre nm") (declared-name "joule per square metre nm") (range (start (line 151) (character 4)) (end (line 151) (character 102))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule second"))) (kind "attribute def") (name "joule second") (declared-name "joule second") (range (start (line 147) (character 4)) (end (line 147) (character 66))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActionQuantityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (kind "attribute def") (name "joule second electronvolt second") (declared-name "joule second electronvolt second") (range (start (line 148) (character 4)) (end (line 148) (character 106))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalAngularMomentumUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (kind "attribute def") (name "joule second to the power minus 1") (declared-name "joule second to the power minus 1") (range (start (line 149) (character 4)) (end (line 149) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (kind "attribute def") (name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (declared-name "joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3") (range (start (line 162) (character 4)) (end (line 162) (character 221))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDensityOfStatesUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin"))) (kind "attribute def") (name "kelvin") (declared-name "kelvin") (range (start (line 25) (character 4)) (end (line 25) (character 673))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermodynamicTemperatureUnit") (range none)) (typing (reference "TemperatureDifferenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (kind "attribute def") (name "kelvin per pascal") (declared-name "kelvin per pascal") (range (start (line 163) (character 4)) (end (line 163) (character 80))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleThomsonCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin per watt"))) (kind "attribute def") (name "kelvin per watt") (declared-name "kelvin per watt") (range (start (line 164) (character 4)) (end (line 164) (character 70))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (kind "attribute def") (name "kelvin to the power minus 1") (declared-name "kelvin to the power minus 1") (range (start (line 165) (character 4)) (end (line 165) (character 95))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearExpansionCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (range (start (line 30) (character 8)) (end (line 30) (character 86))) (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "definitionalQuantityValues") (range (start (line 30) (character 22)) (end (line 30) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 31) (character 8)) (end (line 31) (character 282))) (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "ThermodynamicTemperatureUnit::quantityDimension") (range (start (line 31) (character 22)) (end (line 31) (character 69)))) (redefinition (reference "TemperatureDifferenceUnit::quantityDimension") (range (start (line 31) (character 71)) (end (line 31) (character 115)))))))
    (element (id (node (document "d0") (qualified-name "SI::kelvin::temperatureOfWaterAtTriplePointInK"))) (kind "attribute") (name "temperatureOfWaterAtTriplePointInK") (declared-name "temperatureOfWaterAtTriplePointInK") (range (start (line 26) (character 8)) (end (line 26) (character 212))) (parent (node (document "d0") (qualified-name "SI::kelvin"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram"))) (kind "attribute def") (name "kilogram") (declared-name "kilogram") (range (start (line 22) (character 4)) (end (line 22) (character 127))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (kind "attribute def") (name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (declared-name "kilogram metre cubed second to the power minus 3 ampere to the power minus 2") (range (start (line 193) (character 4)) (end (line 193) (character 161))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 1") (declared-name "kilogram metre second to the power minus 1") (range (start (line 166) (character 4)) (end (line 166) (character 106))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentumUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (kind "attribute def") (name "kilogram metre second to the power minus 2") (declared-name "kilogram metre second to the power minus 2") (range (start (line 167) (character 4)) (end (line 167) (character 103))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (kind "attribute def") (name "kilogram metre second to the power minus 3") (declared-name "kilogram metre second to the power minus 3") (range (start (line 168) (character 4)) (end (line 168) (character 117))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre second to the power minus 3 kelvin to the power minus 1") (range (start (line 169) (character 4)) (end (line 169) (character 159))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre second to the power minus 3 steradian to the power minus 1") (range (start (line 170) (character 4)) (end (line 170) (character 169))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (kind "attribute def") (name "kilogram metre squared") (declared-name "kilogram metre squared") (range (start (line 182) (character 4)) (end (line 182) (character 83))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfInertiaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 1") (declared-name "kilogram metre squared second to the power minus 1") (range (start (line 183) (character 4)) (end (line 183) (character 125))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMomentumUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2") (declared-name "kilogram metre squared second to the power minus 2") (range (start (line 184) (character 4)) (end (line 184) (character 123))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1") (range (start (line 185) (character 4)) (end (line 185) (character 164))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1") (range (start (line 186) (character 4)) (end (line 186) (character 213))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (declared-name "kilogram metre squared second to the power minus 2 mole to the power minus 1") (range (start (line 187) (character 4)) (end (line 187) (character 173))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarInternalEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3") (declared-name "kilogram metre squared second to the power minus 3") (range (start (line 188) (character 4)) (end (line 188) (character 115))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1") (range (start (line 189) (character 4)) (end (line 189) (character 179))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialDifferenceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1") (range (start (line 190) (character 4)) (end (line 190) (character 230))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 kelvin to the power minus 1") (range (start (line 191) (character 4)) (end (line 191) (character 170))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre squared second to the power minus 3 steradian to the power minus 1") (range (start (line 192) (character 4)) (end (line 192) (character 173))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1") (declared-name "kilogram metre to the power minus 1") (range (start (line 171) (character 4)) (end (line 171) (character 102))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LinearMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 1") (range (start (line 172) (character 4)) (end (line 172) (character 143))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 2") (declared-name "kilogram metre to the power minus 1 second to the power minus 2") (range (start (line 173) (character 4)) (end (line 173) (character 135))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1") (range (start (line 174) (character 4)) (end (line 174) (character 188))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 3") (declared-name "kilogram metre to the power minus 1 second to the power minus 3") (range (start (line 175) (character 4)) (end (line 175) (character 145))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralIrradianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1") (range (start (line 176) (character 4)) (end (line 176) (character 190))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 2") (declared-name "kilogram metre to the power minus 2") (range (start (line 177) (character 4)) (end (line 177) (character 103))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceMassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 2 second to the power minus 1") (declared-name "kilogram metre to the power minus 2 second to the power minus 1") (range (start (line 178) (character 4)) (end (line 178) (character 135))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (kind "attribute def") (name "kilogram metre to the power minus 2 second to the power minus 2") (declared-name "kilogram metre to the power minus 2 second to the power minus 2") (range (start (line 179) (character 4)) (end (line 179) (character 174))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (kind "attribute def") (name "kilogram metre to the power minus 3") (declared-name "kilogram metre to the power minus 3") (range (start (line 180) (character 4)) (end (line 180) (character 96))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (kind "attribute def") (name "kilogram metre to the power minus 4 second to the power minus 1") (declared-name "kilogram metre to the power minus 4 second to the power minus 1") (range (start (line 181) (character 4)) (end (line 181) (character 145))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcousticImpedanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (kind "attribute def") (name "kilogram mole to the power minus 1") (declared-name "kilogram mole to the power minus 1") (range (start (line 194) (character 4)) (end (line 194) (character 97))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarMassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 1") (declared-name "kilogram second to the power minus 1") (range (start (line 195) (character 4)) (end (line 195) (character 98))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (kind "attribute def") (name "kilogram second to the power minus 2") (declared-name "kilogram second to the power minus 2") (range (start (line 196) (character 4)) (end (line 196) (character 100))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 2 ampere to the power minus 1") (declared-name "kilogram second to the power minus 2 ampere to the power minus 1") (range (start (line 197) (character 4)) (end (line 197) (character 147))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (kind "attribute def") (name "kilogram second to the power minus 3") (declared-name "kilogram second to the power minus 3") (range (start (line 198) (character 4)) (end (line 198) (character 107))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 3 kelvin to the power minus 1") (declared-name "kilogram second to the power minus 3 kelvin to the power minus 1") (range (start (line 199) (character 4)) (end (line 199) (character 153))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (kind "attribute def") (name "kilogram second to the power minus 3 steradian to the power minus 1") (declared-name "kilogram second to the power minus 3 steradian to the power minus 1") (range (start (line 200) (character 4)) (end (line 200) (character 141))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (kind "attribute def") (name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (declared-name "kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2") (range (start (line 216) (character 4)) (end (line 216) (character 239))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorenzCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (kind "attribute def") (name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (declared-name "kilogram to the power 2 metre to the power minus 2 second to the power minus 3") (range (start (line 215) (character 4)) (end (line 215) (character 159))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 ampere") (declared-name "kilogram to the power minus 1 ampere") (range (start (line 201) (character 4)) (end (line 201) (character 98))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ExposureRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (kind "attribute def") (name "kilogram to the power minus 1 metre cubed") (declared-name "kilogram to the power minus 1 metre cubed") (range (start (line 208) (character 4)) (end (line 208) (character 109))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificVolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre second to the power 2") (declared-name "kilogram to the power minus 1 metre second to the power 2") (range (start (line 202) (character 4)) (end (line 202) (character 132))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 metre second to the power 2 kelvin") (declared-name "kilogram to the power minus 1 metre second to the power 2 kelvin") (range (start (line 203) (character 4)) (end (line 203) (character 153))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "JouleThomsonCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (kind "attribute def") (name "kilogram to the power minus 1 metre squared") (declared-name "kilogram to the power minus 1 metre squared") (range (start (line 207) (character 4)) (end (line 207) (character 123))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassAttenuationCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin") (range (start (line 204) (character 4)) (end (line 204) (character 176))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2") (range (start (line 205) (character 4)) (end (line 205) (character 202))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectrolyticConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (declared-name "kilogram to the power minus 1 metre to the power minus 5 second to the power 2") (range (start (line 206) (character 4)) (end (line 206) (character 168))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDensityOfStatesUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 second ampere") (declared-name "kilogram to the power minus 1 second ampere") (range (start (line 209) (character 4)) (end (line 209) (character 116))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "GyromagneticRatioUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 2") (declared-name "kilogram to the power minus 1 second to the power 2") (range (start (line 211) (character 4)) (end (line 211) (character 137))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDistributionOfCrossSectionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 2 ampere") (declared-name "kilogram to the power minus 1 second to the power 2 ampere") (range (start (line 212) (character 4)) (end (line 212) (character 126))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MobilityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1") (range (start (line 213) (character 4)) (end (line 213) (character 198))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power 3 kelvin") (declared-name "kilogram to the power minus 1 second to the power 3 kelvin") (range (start (line 214) (character 4)) (end (line 214) (character 134))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (kind "attribute def") (name "kilogram to the power minus 1 second to the power minus 1") (declared-name "kilogram to the power minus 1 second to the power minus 1") (range (start (line 210) (character 4)) (end (line 210) (character 131))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificActivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 22) (character 41)) (end (line 22) (character 125))) (parent (node (document "d0") (qualified-name "SI::kilogram"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 22) (character 41)) (end (line 22) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SI::kilojoule"))) (kind "attribute def") (name "kilojoule") (declared-name "kilojoule") (range (start (line 325) (character 4)) (end (line 325) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 325) (character 44)) (end (line 325) (character 128))) (parent (node (document "d0") (qualified-name "SI::kilojoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 325) (character 44)) (end (line 325) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre"))) (kind "attribute def") (name "kilometre") (declared-name "kilometre") (range (start (line 316) (character 4)) (end (line 316) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre per hour"))) (kind "attribute def") (name "kilometre per hour") (declared-name "kilometre per hour") (range (start (line 333) (character 4)) (end (line 333) (character 62))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 316) (character 44)) (end (line 316) (character 128))) (parent (node (document "d0") (qualified-name "SI::kilometre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 316) (character 44)) (end (line 316) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::kilowatt"))) (kind "attribute def") (name "kilowatt") (declared-name "kilowatt") (range (start (line 330) (character 4)) (end (line 330) (character 128))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 330) (character 42)) (end (line 330) (character 126))) (parent (node (document "d0") (qualified-name "SI::kilowatt"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 330) (character 42)) (end (line 330) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "SI::litre"))) (kind "attribute def") (name "litre") (declared-name "litre") (range (start (line 97) (character 4)) (end (line 97) (character 145))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 97) (character 39)) (end (line 97) (character 143))) (parent (node (document "d0") (qualified-name "SI::litre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 97) (character 39)) (end (line 97) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "SI::lumen"))) (kind "attribute def") (name "lumen") (declared-name "lumen") (range (start (line 69) (character 4)) (end (line 69) (character 52))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::lumen per square metre"))) (kind "attribute def") (name "lumen per square metre") (declared-name "lumen per square metre") (range (start (line 218) (character 4)) (end (line 218) (character 82))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExitanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::lumen per watt"))) (kind "attribute def") (name "lumen per watt") (declared-name "lumen per watt") (range (start (line 219) (character 4)) (end (line 219) (character 81))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEfficacyOfRadiationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::lumen second"))) (kind "attribute def") (name "lumen second") (declared-name "lumen second") (range (start (line 217) (character 4)) (end (line 217) (character 68))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::lux"))) (kind "attribute def") (name "lux") (declared-name "lux") (range (start (line 70) (character 4)) (end (line 70) (character 50))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IlluminanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::lux second"))) (kind "attribute def") (name "lux second") (declared-name "lux second") (range (start (line 220) (character 4)) (end (line 220) (character 68))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LuminousExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::m/s²"))) (kind "alias") (name "m/s²") (declared-name "m/s²") (range (start (line 306) (character 4)) (end (line 306) (character 35))) (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::megajoule"))) (kind "attribute def") (name "megajoule") (declared-name "megajoule") (range (start (line 326) (character 4)) (end (line 326) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 326) (character 44)) (end (line 326) (character 128))) (parent (node (document "d0") (qualified-name "SI::megajoule"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 326) (character 44)) (end (line 326) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::metre"))) (kind "attribute def") (name "metre") (declared-name "metre") (range (start (line 21) (character 4)) (end (line 21) (character 37))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed"))) (kind "attribute def") (name "metre cubed") (declared-name "metre cubed") (range (start (line 243) (character 4)) (end (line 243) (character 55))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (kind "attribute def") (name "metre cubed mole to the power minus 1") (declared-name "metre cubed mole to the power minus 1") (range (start (line 244) (character 4)) (end (line 244) (character 104))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarVolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (kind "attribute def") (name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (declared-name "metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1") (range (start (line 246) (character 4)) (end (line 246) (character 186))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "HallCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (kind "attribute def") (name "metre cubed second to the power minus 1") (declared-name "metre cubed second to the power minus 1") (range (start (line 245) (character 4)) (end (line 245) (character 105))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre per second"))) (kind "attribute def") (name "metre per second") (declared-name "metre per second") (range (start (line 223) (character 4)) (end (line 223) (character 59))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (kind "attribute def") (name "metre second to the power minus 1") (declared-name "metre second to the power minus 1") (range (start (line 221) (character 4)) (end (line 221) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpeedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (kind "attribute def") (name "metre second to the power minus 2") (declared-name "metre second to the power minus 2") (range (start (line 222) (character 4)) (end (line 222) (character 93))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AccelerationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared"))) (kind "attribute def") (name "metre squared") (declared-name "metre squared") (range (start (line 231) (character 4)) (end (line 231) (character 55))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared ampere"))) (kind "attribute def") (name "metre squared ampere") (declared-name "metre squared ampere") (range (start (line 232) (character 4)) (end (line 232) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticDipoleMomentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (kind "attribute def") (name "metre squared kelvin per watt") (declared-name "metre squared kelvin per watt") (range (start (line 233) (character 4)) (end (line 233) (character 93))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalInsulanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (kind "attribute def") (name "metre squared mole to the power minus 1") (declared-name "metre squared mole to the power minus 1") (range (start (line 234) (character 4)) (end (line 234) (character 121))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarAbsorptionCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per joule"))) (kind "attribute def") (name "metre squared per joule") (declared-name "metre squared per joule") (range (start (line 242) (character 4)) (end (line 242) (character 97))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyDistributionOfCrossSectionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (kind "attribute def") (name "metre squared per joule steradian") (declared-name "metre squared per joule steradian") (range (start (line 240) (character 4)) (end (line 240) (character 131))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DirectionAndEnergyDistributionOfCrossSectionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (kind "attribute def") (name "metre squared per volt second") (declared-name "metre squared per volt second") (range (start (line 241) (character 4)) (end (line 241) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MobilityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (kind "attribute def") (name "metre squared second to the power minus 1") (declared-name "metre squared second to the power minus 1") (range (start (line 235) (character 4)) (end (line 235) (character 111))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "KinematicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (kind "attribute def") (name "metre squared second to the power minus 2") (declared-name "metre squared second to the power minus 2") (range (start (line 236) (character 4)) (end (line 236) (character 107))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificEnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind "attribute def") (name "metre squared second to the power minus 2 kelvin to the power minus 1") (declared-name "metre squared second to the power minus 2 kelvin to the power minus 1") (range (start (line 237) (character 4)) (end (line 237) (character 155))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificHeatCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (kind "attribute def") (name "metre squared second to the power minus 3") (declared-name "metre squared second to the power minus 3") (range (start (line 238) (character 4)) (end (line 238) (character 107))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (kind "attribute def") (name "metre squared steradian to the power minus 1") (declared-name "metre squared steradian to the power minus 1") (range (start (line 239) (character 4)) (end (line 239) (character 133))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DirectionDistributionOfCrossSectionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power 4"))) (kind "attribute def") (name "metre to the power 4") (declared-name "metre to the power 4") (range (start (line 247) (character 4)) (end (line 247) (character 82))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SecondAxialMomentOfAreaUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (kind "attribute def") (name "metre to the power 4 second to the power minus 2") (declared-name "metre to the power 4 second to the power minus 2") (range (start (line 248) (character 4)) (end (line 248) (character 123))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TotalMassStoppingPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 1") (declared-name "metre to the power minus 1") (range (start (line 224) (character 4)) (end (line 224) (character 77))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CurvatureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (kind "attribute def") (name "metre to the power minus 2") (declared-name "metre to the power minus 2") (range (start (line 225) (character 4)) (end (line 225) (character 82))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 2 second to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1") (range (start (line 226) (character 4)) (end (line 226) (character 126))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIrradianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (declared-name "metre to the power minus 2 second to the power minus 1 steradian to the power minus 1") (range (start (line 227) (character 4)) (end (line 227) (character 171))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonRadianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (kind "attribute def") (name "metre to the power minus 3") (declared-name "metre to the power minus 3") (range (start (line 228) (character 4)) (end (line 228) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ParticleConcentrationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (kind "attribute def") (name "metre to the power minus 3 second") (declared-name "metre to the power minus 3 second") (range (start (line 229) (character 4)) (end (line 229) (character 107))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfVibrationalStatesUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (kind "attribute def") (name "metre to the power minus 3 second to the power minus 1") (declared-name "metre to the power minus 3 second to the power minus 1") (range (start (line 230) (character 4)) (end (line 230) (character 125))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ActivityDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::metric ton"))) (kind "alias") (name "metric ton") (declared-name "metric ton") (range (start (line 99) (character 4)) (end (line 99) (character 33))) (parent (node (document "d0") (qualified-name "SI"))))
    (element (id (node (document "d0") (qualified-name "SI::millilitre"))) (kind "attribute def") (name "millilitre") (declared-name "millilitre") (range (start (line 319) (character 4)) (end (line 319) (character 132))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 319) (character 45)) (end (line 319) (character 130))) (parent (node (document "d0") (qualified-name "SI::millilitre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 319) (character 45)) (end (line 319) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SI::millimetre"))) (kind "attribute def") (name "millimetre") (declared-name "millimetre") (range (start (line 314) (character 4)) (end (line 314) (character 132))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 314) (character 45)) (end (line 314) (character 130))) (parent (node (document "d0") (qualified-name "SI::millimetre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 314) (character 45)) (end (line 314) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SI::millinewton"))) (kind "attribute def") (name "millinewton") (declared-name "millinewton") (range (start (line 322) (character 4)) (end (line 322) (character 132))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 322) (character 45)) (end (line 322) (character 130))) (parent (node (document "d0") (qualified-name "SI::millinewton"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 322) (character 45)) (end (line 322) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SI::minute"))) (kind "attribute def") (name "minute") (declared-name "minute") (range (start (line 96) (character 4)) (end (line 96) (character 142))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::minute (angle)"))) (kind "attribute def") (name "minute (angle)") (declared-name "minute (angle)") (range (start (line 104) (character 4)) (end (line 104) (character 193))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 104) (character 62)) (end (line 104) (character 191))) (parent (node (document "d0") (qualified-name "SI::minute (angle)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 104) (character 62)) (end (line 104) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 96) (character 44)) (end (line 96) (character 140))) (parent (node (document "d0") (qualified-name "SI::minute"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 96) (character 44)) (end (line 96) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::ml per l"))) (kind "attribute def") (name "ml per l") (declared-name "ml per l") (range (start (line 249) (character 4)) (end (line 249) (character 63))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeFractionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole"))) (kind "attribute def") (name "mole") (declared-name "mole") (range (start (line 35) (character 4)) (end (line 35) (character 49))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (kind "attribute def") (name "mole kilogram to the power minus 1") (declared-name "mole kilogram to the power minus 1") (range (start (line 250) (character 4)) (end (line 250) (character 101))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "IonicStrengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (kind "attribute def") (name "mole metre to the power minus 3") (declared-name "mole metre to the power minus 3") (range (start (line 251) (character 4)) (end (line 251) (character 113))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (kind "attribute def") (name "mole per cubic metre") (declared-name "mole per cubic metre") (range (start (line 254) (character 4)) (end (line 254) (character 105))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EquilibriumConstantOnConcentrationBasisUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole per kilogram"))) (kind "attribute def") (name "mole per kilogram") (declared-name "mole per kilogram") (range (start (line 252) (character 4)) (end (line 252) (character 69))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolalityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::mole per l"))) (kind "attribute def") (name "mole per l") (declared-name "mole per l") (range (start (line 253) (character 4)) (end (line 253) (character 82))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AmountOfSubstanceConcentrationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::nanometre"))) (kind "attribute def") (name "nanometre") (declared-name "nanometre") (range (start (line 313) (character 4)) (end (line 313) (character 130))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 313) (character 44)) (end (line 313) (character 128))) (parent (node (document "d0") (qualified-name "SI::nanometre"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByPrefix") (range none)) (redefinition (reference "unitConversion") (range (start (line 313) (character 44)) (end (line 313) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SI::natural unit of information"))) (kind "attribute def") (name "natural unit of information") (declared-name "natural unit of information") (range (start (line 72) (character 4)) (end (line 72) (character 81))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (kind "attribute def") (name "natural unit of information per second") (declared-name "natural unit of information per second") (range (start (line 265) (character 4)) (end (line 265) (character 102))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton"))) (kind "attribute def") (name "newton") (declared-name "newton") (range (start (line 71) (character 4)) (end (line 71) (character 48))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ForceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre"))) (kind "attribute def") (name "newton metre") (declared-name "newton metre") (range (start (line 255) (character 4)) (end (line 255) (character 315))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MomentOfForceUnit") (range none)) (typing (reference "TorqueUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre second"))) (kind "attribute def") (name "newton metre second") (declared-name "newton metre second") (range (start (line 260) (character 4)) (end (line 260) (character 79))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularImpulseUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (kind "attribute def") (name "newton metre second to the power minus 1") (declared-name "newton metre second to the power minus 1") (range (start (line 261) (character 4)) (end (line 261) (character 99))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (kind "attribute def") (name "newton metre to the power minus 1") (declared-name "newton metre to the power minus 1") (range (start (line 262) (character 4)) (end (line 262) (character 95))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SurfaceTensionUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (kind "attribute def") (name "newton metre to the power minus 2") (declared-name "newton metre to the power minus 2") (range (start (line 263) (character 4)) (end (line 263) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 256) (character 8)) (end (line 256) (character 230))) (parent (node (document "d0") (qualified-name "SI::newton metre"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "MomentOfForceUnit::quantityDimension") (range (start (line 256) (character 22)) (end (line 256) (character 58)))) (redefinition (reference "TorqueUnit::quantityDimension") (range (start (line 256) (character 60)) (end (line 256) (character 89)))))))
    (element (id (node (document "d0") (qualified-name "SI::newton second"))) (kind "attribute def") (name "newton second") (declared-name "newton second") (range (start (line 264) (character 4)) (end (line 264) (character 60))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ImpulseUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::octave"))) (kind "attribute def") (name "octave") (declared-name "octave") (range (start (line 74) (character 4)) (end (line 74) (character 65))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LogarithmicFrequencyRangeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::octet"))) (kind "attribute def") (name "octet") (declared-name "octet") (range (start (line 73) (character 4)) (end (line 73) (character 52))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "StorageCapacityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::octet per second"))) (kind "attribute def") (name "octet per second") (declared-name "octet per second") (range (start (line 266) (character 4)) (end (line 266) (character 66))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "TransferRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ohm"))) (kind "attribute def") (name "ohm") (declared-name "ohm") (range (start (line 85) (character 4)) (end (line 85) (character 48))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ohm metre"))) (kind "attribute def") (name "ohm metre") (declared-name "ohm metre") (range (start (line 304) (character 4)) (end (line 304) (character 64))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ResistivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal"))) (kind "attribute def") (name "pascal") (declared-name "pascal") (range (start (line 75) (character 4)) (end (line 75) (character 49))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (kind "attribute def") (name "pascal per kelvin") (declared-name "pascal per kelvin") (range (start (line 270) (character 4)) (end (line 270) (character 76))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PressureCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second"))) (kind "attribute def") (name "pascal second") (declared-name "pascal second") (range (start (line 267) (character 4)) (end (line 267) (character 71))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DynamicViscosityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (kind "attribute def") (name "pascal second per cubic metre") (declared-name "pascal second per cubic metre") (range (start (line 269) (character 4)) (end (line 269) (character 96))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AcousticImpedanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal second per metre"))) (kind "attribute def") (name "pascal second per metre") (declared-name "pascal second per metre") (range (start (line 268) (character 4)) (end (line 268) (character 121))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (kind "attribute def") (name "pascal to the power 2 second") (declared-name "pascal to the power 2 second") (range (start (line 272) (character 4)) (end (line 272) (character 87))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SoundExposureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (kind "attribute def") (name "pascal to the power minus 1") (declared-name "pascal to the power minus 1") (range (start (line 271) (character 4)) (end (line 271) (character 86))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CompressibilityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian"))) (kind "attribute def") (name "radian") (declared-name "radian") (range (start (line 76) (character 4)) (end (line 76) (character 54))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (kind "attribute def") (name "radian metre squared per kilogram to the power 1") (declared-name "radian metre squared per kilogram to the power 1") (range (start (line 273) (character 4)) (end (line 273) (character 134))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpecificOpticalRotatoryPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (kind "attribute def") (name "radian metre squared per mole") (declared-name "radian metre squared per mole") (range (start (line 274) (character 4)) (end (line 274) (character 110))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarOpticalRotatoryPowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian per metre"))) (kind "attribute def") (name "radian per metre") (declared-name "radian per metre") (range (start (line 277) (character 4)) (end (line 277) (character 74))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhaseCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (kind "attribute def") (name "radian second to the power minus 1") (declared-name "radian second to the power minus 1") (range (start (line 275) (character 4)) (end (line 275) (character 101))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (kind "attribute def") (name "radian second to the power minus 2") (declared-name "radian second to the power minus 2") (range (start (line 276) (character 4)) (end (line 276) (character 105))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second"))) (kind "attribute def") (name "second") (declared-name "second") (range (start (line 23) (character 4)) (end (line 23) (character 40))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DurationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second (angle)"))) (kind "attribute def") (name "second (angle)") (declared-name "second (angle)") (range (start (line 106) (character 4)) (end (line 106) (character 193))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 106) (character 62)) (end (line 106) (character 191))) (parent (node (document "d0") (qualified-name "SI::second (angle)"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 106) (character 62)) (end (line 106) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "SI::second ampere"))) (kind "attribute def") (name "second ampere") (declared-name "second ampere") (range (start (line 278) (character 4)) (end (line 278) (character 67))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (kind "attribute def") (name "second to the power minus 1") (declared-name "second to the power minus 1") (range (start (line 281) (character 4)) (end (line 281) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularVelocityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (kind "attribute def") (name "second to the power minus 1 steradian to the power minus 1") (declared-name "second to the power minus 1 steradian to the power minus 1") (range (start (line 282) (character 4)) (end (line 282) (character 131))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PhotonIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (kind "attribute def") (name "second to the power minus 2") (declared-name "second to the power minus 2") (range (start (line 283) (character 4)) (end (line 283) (character 88))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AngularAccelerationUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::shannon"))) (kind "attribute def") (name "shannon") (declared-name "shannon") (range (start (line 78) (character 4)) (end (line 78) (character 58))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "InformationContentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::shannon per second"))) (kind "attribute def") (name "shannon per second") (declared-name "shannon per second") (range (start (line 284) (character 4)) (end (line 284) (character 80))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "AverageInformationRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::siemens"))) (kind "attribute def") (name "siemens") (declared-name "siemens") (range (start (line 77) (character 4)) (end (line 77) (character 54))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ConductanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (kind "attribute def") (name "siemens metre squared per mole") (declared-name "siemens metre squared per mole") (range (start (line 279) (character 4)) (end (line 279) (character 99))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MolarConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::siemens per metre"))) (kind "attribute def") (name "siemens per metre") (declared-name "siemens per metre") (range (start (line 280) (character 4)) (end (line 280) (character 67))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::sievert"))) (kind "attribute def") (name "sievert") (declared-name "sievert") (range (start (line 80) (character 4)) (end (line 80) (character 55))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::sievert per second"))) (kind "attribute def") (name "sievert per second") (declared-name "sievert per second") (range (start (line 285) (character 4)) (end (line 285) (character 72))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::steradian"))) (kind "attribute def") (name "steradian") (declared-name "steradian") (range (start (line 79) (character 4)) (end (line 79) (character 65))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SolidAngularMeasureUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::tesla"))) (kind "attribute def") (name "tesla") (declared-name "tesla") (range (start (line 81) (character 4)) (end (line 81) (character 59))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxDensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::tonne"))) (kind "attribute def") (name "tonne") (declared-name "tonne") (range (start (line 98) (character 4)) (end (line 98) (character 136))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 98) (character 33)) (end (line 98) (character 134))) (parent (node (document "d0") (qualified-name "SI::tonne"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 98) (character 33)) (end (line 98) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "SI::volt"))) (kind "attribute def") (name "volt") (declared-name "volt") (range (start (line 82) (character 4)) (end (line 82) (character 53))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricPotentialUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere"))) (kind "attribute def") (name "volt ampere") (declared-name "volt ampere") (range (start (line 286) (character 4)) (end (line 286) (character 56))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (kind "attribute def") (name "volt ampere reactive") (declared-name "volt ampere reactive") (range (start (line 102) (character 4)) (end (line 102) (character 158))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 102) (character 57)) (end (line 102) (character 156))) (parent (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 102) (character 57)) (end (line 102) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "SI::volt per kelvin"))) (kind "attribute def") (name "volt per kelvin") (declared-name "volt per kelvin") (range (start (line 287) (character 4)) (end (line 287) (character 89))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SeebeckCoefficientForSubstancesAAndBUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::volt per metre"))) (kind "attribute def") (name "volt per metre") (declared-name "volt per metre") (range (start (line 288) (character 4)) (end (line 288) (character 73))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricFieldStrengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (kind "attribute def") (name "volt to the power 2 per kelvin to the power 2") (declared-name "volt to the power 2 per kelvin to the power 2") (range (start (line 289) (character 4)) (end (line 289) (character 108))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LorenzCoefficientUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt"))) (kind "attribute def") (name "watt") (declared-name "watt") (range (start (line 83) (character 4)) (end (line 83) (character 41))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt hour"))) (kind "attribute def") (name "watt hour") (declared-name "watt hour") (range (start (line 290) (character 4)) (end (line 290) (character 55))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "EnergyUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per kelvin"))) (kind "attribute def") (name "watt per kelvin") (declared-name "watt per kelvin") (range (start (line 297) (character 4)) (end (line 297) (character 71))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductanceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per kilogram"))) (kind "attribute def") (name "watt per kilogram") (declared-name "watt per kilogram") (range (start (line 298) (character 4)) (end (line 298) (character 71))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DoseEquivalentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (kind "attribute def") (name "watt per metre kelvin") (declared-name "watt per metre kelvin") (range (start (line 291) (character 4)) (end (line 291) (character 88))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "ThermalConductivityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per nm"))) (kind "attribute def") (name "watt per nm") (declared-name "watt per nm") (range (start (line 300) (character 4)) (end (line 300) (character 70))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre"))) (kind "attribute def") (name "watt per square metre") (declared-name "watt per square metre") (range (start (line 299) (character 4)) (end (line 299) (character 84))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "DensityOfHeatFlowRateUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (kind "attribute def") (name "watt per square metre kelvin") (declared-name "watt per square metre kelvin") (range (start (line 292) (character 4)) (end (line 292) (character 105))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoefficientOfHeatTransferUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (kind "attribute def") (name "watt per square metre nm") (declared-name "watt per square metre nm") (range (start (line 293) (character 4)) (end (line 293) (character 96))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralIrradianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian"))) (kind "attribute def") (name "watt per steradian") (declared-name "watt per steradian") (range (start (line 301) (character 4)) (end (line 301) (character 74))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadiantIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (kind "attribute def") (name "watt per steradian nm") (declared-name "watt per steradian nm") (range (start (line 296) (character 4)) (end (line 296) (character 97))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadiantIntensityUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (kind "attribute def") (name "watt per steradian square metre") (declared-name "watt per steradian square metre") (range (start (line 294) (character 4)) (end (line 294) (character 93))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "RadianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (kind "attribute def") (name "watt per steradian square metre nm") (declared-name "watt per steradian square metre nm") (range (start (line 295) (character 4)) (end (line 295) (character 112))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "SpectralRadianceUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::weber"))) (kind "attribute def") (name "weber") (declared-name "weber") (range (start (line 84) (character 4)) (end (line 84) (character 50))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticFluxUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::weber metre"))) (kind "attribute def") (name "weber metre") (declared-name "weber metre") (range (start (line 302) (character 4)) (end (line 302) (character 73))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticDipoleMomentUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::weber per metre"))) (kind "attribute def") (name "weber per metre") (declared-name "weber per metre") (range (start (line 303) (character 4)) (end (line 303) (character 78))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "MagneticVectorPotentialUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ångström"))) (kind "attribute def") (name "ångström") (declared-name "ångström") (range (start (line 90) (character 4)) (end (line 90) (character 152))) (parent (node (document "d0") (qualified-name "SI"))) (authored (membership (kind Owning)) (relationships (typing (reference "LengthUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (range (start (line 90) (character 49)) (end (line 90) (character 150))) (parent (node (document "d0") (qualified-name "SI::ångström"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConversionByConvention") (range none)) (redefinition (reference "unitConversion") (range (start (line 90) (character 49)) (end (line 90) (character 67)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SI::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 9) (character 19)) (end (line 9) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 10) (character 18)) (end (line 10) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SIPrefixes::*") (range (start (line 11) (character 18)) (end (line 11) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units"))) (kind featureTyping) (ordinal 0)) (authored-target "SystemOfUnits") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits"))) (kind redefinition) (ordinal 0)) (authored-target "baseUnits") (range (start (line 44) (character 2)) (end (line 44) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::baseUnits")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities"))) (kind redefinition) (ordinal 0)) (authored-target "systemOfQuantities") (range (start (line 43) (character 2)) (end (line 43) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ISO/IEC 80000 International System of Units::systemOfQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticMomentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre squared joule to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere metre to the power minus 2 kelvin to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "RichardsonConstantUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearElectricCurrentDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricCurrentDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ampere second per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 101) (character 54)) (end (line 101) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::astronomical unit::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 100) (character 50)) (end (line 100) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::atomic mass unit::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::barn::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 91) (character 36)) (end (line 91) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::barn::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::baud"))) (kind featureTyping) (ordinal 0)) (authored-target "ModulationRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel"))) (kind featureTyping) (ordinal 0)) (authored-target "NuclearActivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificActivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::becquerel per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceActivityDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::bit"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::bit per second"))) (kind featureTyping) (ordinal 0)) (authored-target "BinaryDigitRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::byte"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::byte per second"))) (kind featureTyping) (ordinal 0)) (authored-target "TransferRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian kilogram to the power minus 1 metre to the power minus 2 second to the power 3"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian metre to the power minus 2 second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::candela steradian second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::centimetre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 315) (character 45)) (end (line 315) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::centimetre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricDipoleMomentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per kilogram second"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearDensityOfElectricChargeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::coulomb per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceDensityOfElectricChargeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::dalton::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 93) (character 39)) (end (line 93) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::dalton::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::day"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::day::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 92) (character 38)) (end (line 92) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::day::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::decade"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::decibel"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundPressureLevelUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)"))) (kind featureTyping) (ordinal 0)) (authored-target "IntervalScale") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::celsiusToKelvinScaleMapping"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityValueMapping") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (range (start (line 368) (character 22)) (end (line 368) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping"))) (kind redefinition) (ordinal 0)) (authored-target "quantityValueMapping") (range (start (line 369) (character 22)) (end (line 369) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::quantityValueMapping")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtFreezingPointInC"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::temperatureWaterAtTriplePointInC"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit"))) (kind redefinition) (ordinal 0)) (authored-target "unit") (range (start (line 357) (character 22)) (end (line 357) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::unit")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusInKelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFramePlacement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (absolute temperature scale)::zeroDegreeCelsiusToKelvinShift"))) (kind redefinition) (ordinal 0)) (authored-target "transformation") (range (start (line 373) (character 80)) (end (line 373) (character 94))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureDifferenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 345) (character 22)) (end (line 345) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree celsius (temperature difference)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::degree::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 103) (character 51)) (end (line 103) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::degree::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt joule kilogram metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "HartreeEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt metre to the power minus 2 per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalLinearStoppingPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyFluenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::electronvolt::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 94) (character 47)) (end (line 94) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::electronvolt::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::erlang"))) (kind featureTyping) (ordinal 0)) (authored-target "TrafficIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::farad"))) (kind featureTyping) (ordinal 0)) (authored-target "CapacitanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::farad per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricConstantUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::g per l"))) (kind featureTyping) (ordinal 0)) (authored-target "MassConcentrationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::g per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gigajoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 327) (character 44)) (end (line 327) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::gigajoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::gram"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gray"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorbedDoseUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::gray per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AbsorbedDoseRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hartley"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hartley per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry"))) (kind featureTyping) (ordinal 0)) (authored-target "PermeanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry"))) (kind featureTyping) (ordinal 1)) (authored-target "InductanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticConstantUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ReluctanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "PermeanceUnit::quantityDimension") (range (start (line 61) (character 22)) (end (line 61) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::henry::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "InductanceUnit::quantityDimension") (range (start (line 61) (character 56)) (end (line 61) (character 89))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hertz"))) (kind featureTyping) (ordinal 0)) (authored-target "FrequencyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::hour::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 95) (character 39)) (end (line 95) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::hour::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule metre squared per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectromagneticEnergyDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per cubic metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per kilogram kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalLinearStoppingPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per mole kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per second"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavenumberUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule per square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second"))) (kind featureTyping) (ordinal 0)) (authored-target "ActionQuantityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second electronvolt second"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalAngularMomentumUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::joule to the power minus 1 metre to the power minus 3 electronvolt to the power minus 1 metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureDifferenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin per pascal"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearExpansionCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues"))) (kind redefinition) (ordinal 0)) (authored-target "definitionalQuantityValues") (range (start (line 30) (character 22)) (end (line 30) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kelvin::definitionalQuantityValues")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "ThermodynamicTemperatureUnit::quantityDimension") (range (start (line 31) (character 22)) (end (line 31) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "TemperatureDifferenceUnit::quantityDimension") (range (start (line 31) (character 71)) (end (line 31) (character 115))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kelvin::temperatureOfWaterAtTriplePointInK"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre cubed second to the power minus 3 ampere to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentumUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfInertiaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMomentumUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "HeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 kelvin to the power minus 1 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 2 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarInternalEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialDifferenceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 ampere to the power minus 1 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre squared second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "LinearMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 1 second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceMassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 2 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantEnergyDensityInTermsOfWavelengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "MassDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram metre to the power minus 4 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarMassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 2 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram second to the power minus 3 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power 4 second to the power minus 6 ampere to the power minus 2 kelvin to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power 2 metre to the power minus 2 second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ExposureRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre cubed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificVolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre second to the power 2 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "JouleThomsonCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAttenuationCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 2 second to the power 3 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 3 second to the power 3 ampere to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectrolyticConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 metre to the power minus 5 second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDensityOfStatesUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "GyromagneticRatioUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDistributionOfCrossSectionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 2 ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "MobilityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 ampere to the power 2 mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power 3 kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram to the power minus 1 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificActivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilogram::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 22) (character 41)) (end (line 22) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilogram::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilojoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 325) (character 44)) (end (line 325) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilojoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre per hour"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilometre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 316) (character 44)) (end (line 316) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilometre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::kilowatt::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 330) (character 42)) (end (line 330) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::kilowatt::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::litre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 97) (character 39)) (end (line 97) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::litre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExitanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEfficacyOfRadiationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lumen second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lux"))) (kind featureTyping) (ordinal 0)) (authored-target "IlluminanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::lux second"))) (kind featureTyping) (ordinal 0)) (authored-target "LuminousExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::megajoule::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 326) (character 44)) (end (line 326) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::megajoule::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarVolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed per coulomb cubic metre second to the power minus 1 ampere to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "HallCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre cubed second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre per second"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared"))) (kind featureTyping) (ordinal 0)) (authored-target "AreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticDipoleMomentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared kelvin per watt"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalInsulanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared mole to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarAbsorptionCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per joule"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyDistributionOfCrossSectionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per joule steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "DirectionAndEnergyDistributionOfCrossSectionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared per volt second"))) (kind featureTyping) (ordinal 0)) (authored-target "MobilityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "KinematicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificEnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 2 kelvin to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificHeatCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared second to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre squared steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "DirectionDistributionOfCrossSectionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power 4"))) (kind featureTyping) (ordinal 0)) (authored-target "SecondAxialMomentOfAreaUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power 4 second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "TotalMassStoppingPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CurvatureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIrradianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 2 second to the power minus 1 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonRadianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "ParticleConcentrationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3 second"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfVibrationalStatesUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::metre to the power minus 3 second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "ActivityDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millilitre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 319) (character 45)) (end (line 319) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millilitre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millimetre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 314) (character 45)) (end (line 314) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millimetre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::millinewton::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 322) (character 45)) (end (line 322) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::millinewton::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 104) (character 62)) (end (line 104) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::minute (angle)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::minute::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 96) (character 44)) (end (line 96) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::minute::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::ml per l"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeFractionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole kilogram to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "IonicStrengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole metre to the power minus 3"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "EquilibriumConstantOnConcentrationBasisUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "MolalityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::mole per l"))) (kind featureTyping) (ordinal 0)) (authored-target "AmountOfSubstanceConcentrationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByPrefix") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::nanometre::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 313) (character 44)) (end (line 313) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::nanometre::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::natural unit of information"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::natural unit of information per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton"))) (kind featureTyping) (ordinal 0)) (authored-target "ForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MomentOfForceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre"))) (kind featureTyping) (ordinal 1)) (authored-target "TorqueUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre second"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularImpulseUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SurfaceTensionUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "MomentOfForceUnit::quantityDimension") (range (start (line 256) (character 22)) (end (line 256) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton metre::quantityDimension"))) (kind redefinition) (ordinal 1)) (authored-target "TorqueUnit::quantityDimension") (range (start (line 256) (character 60)) (end (line 256) (character 89))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::newton second"))) (kind featureTyping) (ordinal 0)) (authored-target "ImpulseUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octave"))) (kind featureTyping) (ordinal 0)) (authored-target "LogarithmicFrequencyRangeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octet"))) (kind featureTyping) (ordinal 0)) (authored-target "StorageCapacityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::octet per second"))) (kind featureTyping) (ordinal 0)) (authored-target "TransferRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ohm"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ohm metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ResistivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicViscosityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second per cubic metre"))) (kind featureTyping) (ordinal 0)) (authored-target "AcousticImpedanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal second per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal to the power 2 second"))) (kind featureTyping) (ordinal 0)) (authored-target "SoundExposureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::pascal to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "CompressibilityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian metre squared per kilogram to the power 1"))) (kind featureTyping) (ordinal 0)) (authored-target "SpecificOpticalRotatoryPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian metre squared per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarOpticalRotatoryPowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "PhaseCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::radian second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second (angle)::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 106) (character 62)) (end (line 106) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::second (angle)::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::second ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularVelocityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 1 steradian to the power minus 1"))) (kind featureTyping) (ordinal 0)) (authored-target "PhotonIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::second to the power minus 2"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularAccelerationUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::shannon"))) (kind featureTyping) (ordinal 0)) (authored-target "InformationContentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::shannon per second"))) (kind featureTyping) (ordinal 0)) (authored-target "AverageInformationRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens"))) (kind featureTyping) (ordinal 0)) (authored-target "ConductanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens metre squared per mole"))) (kind featureTyping) (ordinal 0)) (authored-target "MolarConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::siemens per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::sievert"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::sievert per second"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "SolidAngularMeasureUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tesla"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxDensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne"))) (kind featureTyping) (ordinal 0)) (authored-target "MassUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::tonne::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 98) (character 33)) (end (line 98) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::tonne::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricPotentialUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 102) (character 57)) (end (line 102) (character 75))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::volt ampere reactive::unitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "SeebeckCoefficientForSubstancesAAndBUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricFieldStrengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::volt to the power 2 per kelvin to the power 2"))) (kind featureTyping) (ordinal 0)) (authored-target "LorenzCoefficientUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt hour"))) (kind featureTyping) (ordinal 0)) (authored-target "EnergyUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductanceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per kilogram"))) (kind featureTyping) (ordinal 0)) (authored-target "DoseEquivalentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per metre kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "ThermalConductivityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "DensityOfHeatFlowRateUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre kelvin"))) (kind featureTyping) (ordinal 0)) (authored-target "CoefficientOfHeatTransferUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralIrradianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian"))) (kind featureTyping) (ordinal 0)) (authored-target "RadiantIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadiantIntensityUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian square metre"))) (kind featureTyping) (ordinal 0)) (authored-target "RadianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::watt per steradian square metre nm"))) (kind featureTyping) (ordinal 0)) (authored-target "SpectralRadianceUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticFluxUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticDipoleMomentUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::weber per metre"))) (kind featureTyping) (ordinal 0)) (authored-target "MagneticVectorPotentialUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "ConversionByConvention") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SI::ångström::unitConversion"))) (kind redefinition) (ordinal 0)) (authored-target "unitConversion") (range (start (line 90) (character 49)) (end (line 90) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SI::ångström::unitConversion")))))
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
